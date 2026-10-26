//! Persisted player settings and backend-independent settings-menu behavior.
use crate::{assets::read_bounded, input::Binding};
use anyhow::{Context, Result, ensure};
use serde::{Deserialize, Serialize};
use std::{
    collections::BTreeMap,
    fs::{self, OpenOptions},
    io::Write,
    path::Path,
    sync::atomic::{AtomicU64, Ordering},
};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Settings {
    pub version: u32,
    pub width: u32,
    pub height: u32,
    pub fullscreen: bool,
    /// Legacy input only; display always scales freely.
    #[serde(default, skip_serializing)]
    pub integer_scale: bool,
    pub volume: f32,
    pub dead_zone: f32,
    /// SDL GUID#ordinal among identical models; None selects the first available.
    /// A bare GUID is also accepted and selects ordinal zero.
    pub controller: Option<String>,
    #[serde(default = "automatic_players")]
    pub players: [String; 4],
    pub pause_on_focus_loss: bool,
    pub bindings: BTreeMap<String, Vec<Binding>>,
}

/// Per-port user configuration without requiring SDL or a display server.
pub fn default_path(game: &str) -> Result<std::path::PathBuf> {
    ensure!(
        !game.is_empty() && game.bytes().all(|b| b.is_ascii_alphanumeric() || b == b'-'),
        "invalid game name"
    );
    let root = if cfg!(target_os = "windows") {
        std::env::var_os("APPDATA").map(std::path::PathBuf::from)
    } else if cfg!(target_os = "macos") {
        std::env::var_os("HOME")
            .map(|home| std::path::PathBuf::from(home).join("Library/Application Support"))
    } else {
        std::env::var_os("XDG_CONFIG_HOME")
            .filter(|path| Path::new(path).is_absolute())
            .map(std::path::PathBuf::from)
            .or_else(|| {
                std::env::var_os("HOME").map(|home| std::path::PathBuf::from(home).join(".config"))
            })
    }
    .context("cannot locate user configuration directory; supply --settings")?;
    Ok(root.join("romatlas-ports").join(game).join("settings.json"))
}
fn automatic_players() -> [String; 4] {
    std::array::from_fn(|_| "auto".into())
}

impl Default for Settings {
    fn default() -> Self {
        let mut bindings = BTreeMap::new();
        for (action, key, button) in [
            ("up", "Up", "dpup"),
            ("down", "Down", "dpdown"),
            ("left", "Left", "dpleft"),
            ("right", "Right", "dpright"),
            ("a", "Z", "a"),
            ("b", "X", "b"),
            ("start", "Return", "start"),
            ("select", "Backspace", "back"),
        ] {
            bindings.insert(
                action.into(),
                vec![
                    Binding::Key { name: key.into() },
                    Binding::Button {
                        name: button.into(),
                    },
                ],
            );
        }
        for (action, direction) in [("left", -1), ("right", 1)] {
            bindings.get_mut(action).unwrap().push(Binding::Axis {
                name: "leftx".into(),
                direction,
            });
        }
        for (action, direction) in [("up", -1), ("down", 1)] {
            bindings.get_mut(action).unwrap().push(Binding::Axis {
                name: "lefty".into(),
                direction,
            });
        }
        Self {
            version: 1,
            width: 640,
            height: 480,
            fullscreen: false,
            integer_scale: false,
            volume: 0.7,
            dead_zone: 0.2,
            controller: None,
            players: automatic_players(),
            pause_on_focus_loss: true,
            bindings,
        }
    }
}
fn valid_controller(id: &str) -> bool {
    let (guid, ordinal) = id.split_once('#').map_or((id, None), |(g, o)| (g, Some(o)));
    guid.len() == 32
        && guid.bytes().all(|b| b.is_ascii_hexdigit())
        && ordinal.is_none_or(|o| o.parse::<u16>().is_ok())
}
impl Settings {
    pub fn validate(&self) -> Result<()> {
        ensure!(self.version == 1, "unsupported settings version");
        ensure!(
            (160..=8192).contains(&self.width) && (144..=8192).contains(&self.height),
            "invalid window dimensions"
        );
        ensure!(
            self.volume.is_finite() && (0.0..=1.0).contains(&self.volume),
            "invalid audio volume"
        );
        ensure!(
            self.dead_zone.is_finite() && (0.0..=0.95).contains(&self.dead_zone),
            "invalid stick dead zone"
        );
        ensure!(
            self.controller.as_ref().is_none_or(|s| {
                let (guid, ordinal) = s
                    .split_once('#')
                    .map_or((s.as_str(), None), |(guid, ordinal)| (guid, Some(ordinal)));
                guid.len() == 32
                    && guid.bytes().all(|b| b.is_ascii_hexdigit())
                    && ordinal.is_none_or(|o| o.parse::<u16>().is_ok())
            }),
            "invalid controller selector"
        );
        let mut explicit = std::collections::BTreeSet::new();
        for id in &self.players {
            ensure!(
                id == "auto" || id == "none" || valid_controller(id),
                "invalid player controller"
            );
            ensure!(
                id == "auto" || id == "none" || explicit.insert(id.to_ascii_lowercase()),
                "controller assigned twice"
            );
        }
        ensure!(
            !self.bindings.is_empty() && self.bindings.len() <= 128,
            "invalid action count"
        );
        for (action, bindings) in &self.bindings {
            ensure!(
                !action.is_empty()
                    && action.len() <= 64
                    && action
                        .bytes()
                        .all(|b| b.is_ascii_alphanumeric() || b"_-".contains(&b)),
                "invalid action name"
            );
            ensure!(bindings.len() <= 16, "too many bindings for action");
            for binding in bindings {
                let name = match binding {
                    Binding::Key { name } | Binding::Button { name } => name,
                    Binding::Axis { name, direction } => {
                        ensure!(
                            *direction == -1 || *direction == 1,
                            "invalid axis direction"
                        );
                        name
                    }
                };
                ensure!(!name.is_empty() && name.len() <= 64, "invalid binding name");
            }
        }
        Ok(())
    }
    pub fn load(path: &Path) -> Result<Self> {
        if !path.try_exists()? {
            return Ok(Self::default());
        }
        let mut settings: Self = serde_json::from_slice(&read_bounded(path, 1024 * 1024)?)?;
        settings.integer_scale = false;
        settings.validate()?;
        Ok(settings)
    }
    pub fn save(&self, path: &Path) -> Result<()> {
        self.validate()?;
        write_atomic(path, &serde_json::to_vec_pretty(self)?)
    }
}

pub(crate) fn write_atomic(path: &Path, bytes: &[u8]) -> Result<()> {
    let parent = path
        .parent()
        .filter(|p| !p.as_os_str().is_empty())
        .unwrap_or(Path::new("."));
    fs::create_dir_all(parent)?;
    static SERIAL: AtomicU64 = AtomicU64::new(0);
    let temp = parent.join(format!(
        ".romatlas-settings-{}-{}",
        std::process::id(),
        SERIAL.fetch_add(1, Ordering::Relaxed)
    ));
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&temp)?;
    let result = (|| {
        file.write_all(bytes)?;
        file.sync_all()?;
        fs::rename(&temp, path).context("publish settings")?;
        Ok(())
    })();
    if result.is_err() {
        let _ = fs::remove_file(temp);
    }
    result
}

#[derive(Clone, Copy)]
pub enum MenuInput {
    Up,
    Down,
    Left,
    Right,
    Accept,
    Cancel,
    ClearBinding,
}
#[derive(Default)]
pub struct Menu {
    pub open: bool,
    pub selected: usize,
    pub rebinding: Option<String>,
}
impl Menu {
    pub fn toggle(&mut self) {
        self.open = !self.open;
        self.rebinding = None;
    }
    pub fn labels(&self, settings: &Settings, controllers: &[(String, String)]) -> Vec<String> {
        let mut labels = vec![
            format!("HEIGHT {} (WIDTH AUTOMATIC)", settings.height),
            format!("FULLSCREEN {}", settings.fullscreen),
            format!("VOLUME {:.0}%", settings.volume * 100.0),
            format!("DEAD ZONE {:.0}%", settings.dead_zone * 100.0),
        ];
        for (player, id) in settings.players.iter().enumerate() {
            let name = match id.as_str() {
                "auto" => "AUTO",
                "none" => "NONE",
                _ => controllers
                    .iter()
                    .find(|(key, _)| key == id)
                    .map_or("DISCONNECTED", |(_, name)| name.as_str()),
            };
            labels.push(format!("PLAYER {}: {}", player + 1, name));
        }
        labels.push(format!(
            "PAUSE ON FOCUS LOSS {}",
            settings.pause_on_focus_loss
        ));
        labels.push("RESET DEFAULTS".into());
        for (action, bindings) in &settings.bindings {
            let names = bindings
                .iter()
                .map(|binding| match binding {
                    Binding::Key { name } => format!("KEY {name}"),
                    Binding::Button { name } => format!("PAD {name}"),
                    Binding::Axis { name, direction } => format!("AXIS {name} {direction}"),
                })
                .collect::<Vec<_>>()
                .join(", ");
            labels.push(format!("BIND {action}: {names}"));
        }
        labels
    }
    /// Rebinding adds an alternative. ClearBinding explicitly clears the action.
    /// Reserve Escape/F6 for cancelling the menu and Backspace for clearing.
    pub fn capture(&mut self, binding: Binding, settings: &mut Settings) -> bool {
        if matches!(&binding, Binding::Key { name } if matches!(name.as_str(), "F1"|"F2"|"F3"|"F4"|"F5"|"F6"))
        {
            return false;
        }
        if let Some(action) = self.rebinding.take() {
            let bindings = settings.bindings.entry(action).or_default();
            if bindings.len() < 16 && !bindings.contains(&binding) {
                bindings.push(binding);
                return true;
            }
        }
        false
    }
    pub fn handle(
        &mut self,
        input: MenuInput,
        settings: &mut Settings,
        defaults: &Settings,
        controllers: &[(String, String)],
    ) -> bool {
        if !self.open {
            return false;
        }
        if self.rebinding.is_some() {
            if matches!(input, MenuInput::Cancel) {
                self.rebinding = None;
            }
            return false;
        }
        let count = 10 + settings.bindings.len();
        self.selected = self.selected.min(count - 1);
        match input {
            MenuInput::Up => {
                self.selected = (self.selected + count - 1) % count;
                return false;
            }
            MenuInput::Down => {
                self.selected = (self.selected + 1) % count;
                return false;
            }
            MenuInput::Cancel => {
                self.open = false;
                return false;
            }
            _ => {}
        }
        if self.selected >= 10 {
            let action = settings
                .bindings
                .keys()
                .nth(self.selected - 10)
                .unwrap()
                .clone();
            if matches!(input, MenuInput::Accept) {
                self.rebinding = Some(action);
            } else if matches!(input, MenuInput::ClearBinding) {
                settings.bindings.get_mut(&action).unwrap().clear();
                return true;
            }
            return false;
        }
        if !matches!(
            input,
            MenuInput::Accept | MenuInput::Left | MenuInput::Right
        ) {
            return false;
        }
        let step: i32 = if matches!(input, MenuInput::Left) {
            -1
        } else {
            1
        };
        match self.selected {
            0 => {
                let heights = [240, 360, 480, 540, 720, 900, 1080, 1440, 2160];
                let index = heights
                    .iter()
                    .position(|height| *height == settings.height)
                    .unwrap_or(2);
                settings.height =
                    heights[(index as i32 + step).rem_euclid(heights.len() as i32) as usize];
            }
            1 => settings.fullscreen = !settings.fullscreen,
            2 => settings.volume = (settings.volume + step as f32 * 0.05).clamp(0.0, 1.0),
            3 => settings.dead_zone = (settings.dead_zone + step as f32 * 0.05).clamp(0.0, 0.95),
            4..=7 => {
                let player = self.selected - 4;
                let mut choices = vec!["auto".to_owned(), "none".to_owned()];
                choices.extend(controllers.iter().map(|(id, _)| id.clone()).filter(|id| {
                    !settings
                        .players
                        .iter()
                        .enumerate()
                        .any(|(other, assigned)| other != player && assigned == id)
                }));
                let index = choices
                    .iter()
                    .position(|id| id == &settings.players[player])
                    .unwrap_or(0);
                settings.players[player] = choices
                    [(index as i32 + step).rem_euclid(choices.len() as i32) as usize]
                    .clone();
                settings.controller = None;
            }
            8 => settings.pause_on_focus_loss = !settings.pause_on_focus_loss,
            9 => *settings = defaults.clone(),
            _ => unreachable!(),
        }
        true
    }
}
