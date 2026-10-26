//! Physical bindings map to game-owned action names, never console registers.
use crate::settings::Settings;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum Binding {
    Key { name: String },
    Button { name: String },
    Axis { name: String, direction: i8 },
}
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PhysicalInput {
    pub keys: BTreeSet<String>,
    pub buttons: BTreeSet<String>,
    /// Normalized signed values. An absent controller supplies no values.
    pub axes: BTreeMap<String, f32>,
}
#[derive(Debug, Clone, Default, PartialEq, Eq)]
#[cfg_attr(feature = "snapshots", derive(serde::Serialize, serde::Deserialize))]
pub struct InputState {
    pub held: BTreeSet<String>,
    pub pressed: BTreeSet<String>,
    pub released: BTreeSet<String>,
}
#[derive(Debug, Clone, Default, PartialEq, Eq)]
#[cfg_attr(feature = "snapshots", derive(serde::Serialize, serde::Deserialize))]
pub struct PlayerInput {
    pub held: BTreeSet<String>,
    pub pressed: BTreeSet<String>,
    pub released: BTreeSet<String>,
}
impl PlayerInput {
    pub fn held(&self, action: &str) -> bool {
        self.held.contains(action)
    }
    pub fn pressed(&self, action: &str) -> bool {
        self.pressed.contains(action)
    }
}
#[derive(Debug, Clone, Default, PartialEq, Eq)]
#[cfg_attr(feature = "snapshots", derive(serde::Serialize, serde::Deserialize))]
pub struct MultiInputState {
    /// Player zero mirrors the legacy single-player input state.
    pub primary: InputState,
    /// Independent controller actions; index zero is the primary controls.
    pub players: [PlayerInput; 4],
}
impl InputState {
    pub fn held(&self, action: &str) -> bool {
        self.held.contains(action)
    }
    pub fn pressed(&self, action: &str) -> bool {
        self.pressed.contains(action)
    }
}
#[derive(Default)]
#[cfg_attr(feature = "snapshots", derive(serde::Serialize, serde::Deserialize))]
pub struct InputMapper {
    previous: [BTreeSet<String>; 4],
}
impl InputMapper {
    pub fn sample(&mut self, physical: &PhysicalInput, settings: &Settings) -> InputState {
        let mut physical_players: [PhysicalInput; 4] = Default::default();
        physical_players[0] = physical.clone();
        self.sample_players(&physical_players, settings).primary
    }

    /// Map independent physical inputs into four controller slots. Bindings
    /// are shared; devices remain isolated so a port can poll each player.
    pub fn sample_players(
        &mut self,
        physical: &[PhysicalInput; 4],
        settings: &Settings,
    ) -> MultiInputState {
        let players = std::array::from_fn(|player| {
            let physical = &physical[player];
            let held: BTreeSet<_> = settings
                .bindings
                .iter()
                .filter(|(_, bindings)| {
                    bindings.iter().any(|binding| match binding {
                        Binding::Key { name } => physical.keys.contains(name),
                        Binding::Button { name } => physical.buttons.contains(name),
                        Binding::Axis { name, direction } => {
                            physical.axes.get(name).is_some_and(|value| {
                                value.is_finite()
                                    && *value * f32::from(*direction) > settings.dead_zone
                            })
                        }
                    })
                })
                .map(|(action, _)| action.clone())
                .collect();
            PlayerInput {
                pressed: held.difference(&self.previous[player]).cloned().collect(),
                released: self.previous[player].difference(&held).cloned().collect(),
                held,
            }
        });
        for (previous, state) in self.previous.iter_mut().zip(&players) {
            *previous = state.held.clone();
        }
        MultiInputState {
            primary: InputState {
                held: players[0].held.clone(),
                pressed: players[0].pressed.clone(),
                released: players[0].released.clone(),
            },
            players,
        }
    }
    /// Focus/pause boundaries discard held state and stale press edges.
    pub fn clear(&mut self) {
        self.previous.iter_mut().for_each(BTreeSet::clear);
    }
}

/// Prevent controls held across focus/menu/device transitions from becoming
/// fresh game actions. Each suppressed control rearms after returning to neutral.
#[derive(Default)]
#[cfg_attr(feature = "snapshots", derive(serde::Serialize, serde::Deserialize))]
pub struct InputLatch {
    suppressed: PhysicalInput,
}
impl InputLatch {
    pub fn suppress(&mut self, physical: &PhysicalInput) {
        self.suppressed = physical.clone();
    }
    pub fn filter(&mut self, mut physical: PhysicalInput, dead_zone: f32) -> PhysicalInput {
        self.suppressed
            .keys
            .retain(|key| physical.keys.contains(key));
        self.suppressed
            .buttons
            .retain(|button| physical.buttons.contains(button));
        self.suppressed
            .axes
            .retain(|axis, _| physical.axes.get(axis).is_some_and(|v| v.abs() > dead_zone));
        physical
            .keys
            .retain(|key| !self.suppressed.keys.contains(key));
        physical
            .buttons
            .retain(|button| !self.suppressed.buttons.contains(button));
        physical
            .axes
            .retain(|axis, _| !self.suppressed.axes.contains_key(axis));
        physical
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn player_inputs_keep_actions_and_edges_independent() {
        let settings = Settings::default();
        let mut physical: [PhysicalInput; 4] = Default::default();
        physical[0].keys.insert("Up".into());
        physical[1].buttons.insert("a".into());
        physical[2].buttons.insert("start".into());
        let mut mapper = InputMapper::default();

        let state = mapper.sample_players(&physical, &settings);
        assert!(state.primary.held("up"));
        assert_eq!(state.primary.held, state.players[0].held);
        assert!(state.players[0].pressed("up"));
        assert!(state.players[1].pressed("a"));
        assert!(state.players[2].pressed("start"));
        assert!(!state.players[1].held("up"));
        assert!(!state.players[0].held("a"));

        physical[0].keys.clear();
        let state = mapper.sample_players(&physical, &settings);
        assert!(state.players[0].released.contains("up"));
        assert!(state.players[1].held("a"));
        assert!(state.players[1].pressed.is_empty());
    }

    #[test]
    fn legacy_sample_populates_only_primary_player() {
        let settings = Settings::default();
        let mut physical = PhysicalInput::default();
        physical.keys.insert("Return".into());
        let state = InputMapper::default().sample(&physical, &settings);
        assert!(state.held("start"));
        let mut physical_players: [PhysicalInput; 4] = Default::default();
        physical_players[0] = physical;
        let multi = InputMapper::default().sample_players(&physical_players, &settings);
        assert!(multi.players[0].held("start"));
        assert!(
            multi.players[1..]
                .iter()
                .all(|player| player.held.is_empty())
        );
    }
}
