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
#[derive(Debug, Clone, Default)]
pub struct PhysicalInput {
    pub keys: BTreeSet<String>,
    pub buttons: BTreeSet<String>,
    /// Normalized signed values. An absent controller supplies no values.
    pub axes: BTreeMap<String, f32>,
}
#[derive(Debug, Clone, Default)]
pub struct InputState {
    pub held: BTreeSet<String>,
    pub pressed: BTreeSet<String>,
    pub released: BTreeSet<String>,
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
pub struct InputMapper {
    previous: BTreeSet<String>,
}
impl InputMapper {
    pub fn sample(&mut self, physical: &PhysicalInput, settings: &Settings) -> InputState {
        let held: BTreeSet<_> = settings
            .bindings
            .iter()
            .filter(|(_, bindings)| {
                bindings.iter().any(|binding| match binding {
                    Binding::Key { name } => physical.keys.contains(name),
                    Binding::Button { name } => physical.buttons.contains(name),
                    Binding::Axis { name, direction } => {
                        physical.axes.get(name).is_some_and(|value| {
                            value.is_finite() && *value * f32::from(*direction) > settings.dead_zone
                        })
                    }
                })
            })
            .map(|(action, _)| action.clone())
            .collect();
        let state = InputState {
            pressed: held.difference(&self.previous).cloned().collect(),
            released: self.previous.difference(&held).cloned().collect(),
            held: held.clone(),
        };
        self.previous = held;
        state
    }
    /// Focus/pause boundaries discard held state and stale press edges.
    pub fn clear(&mut self) {
        self.previous.clear();
    }
}

/// Prevent controls held across focus/menu/device transitions from becoming
/// fresh game actions. Each suppressed control rearms after returning to neutral.
#[derive(Default)]
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
