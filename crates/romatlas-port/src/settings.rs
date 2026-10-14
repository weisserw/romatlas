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
    pub integer_scale: bool,
    pub volume: f32,
    pub dead_zone: f32,
    /// SDL GUID#ordinal among identical models; None selects the first available.
    /// A bare GUID is also accepted and selects ordinal zero.
    pub controller: Option<String>,
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
            height: 576,
            fullscreen: false,
            integer_scale: true,
            volume: 0.7,
            dead_zone: 0.2,
            controller: None,
            pause_on_focus_loss: true,
            bindings,
        }
    }
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
        let settings: Self = serde_json::from_slice(&read_bounded(path, 1024 * 1024)?)?;
        settings.validate()?;
        Ok(settings)
    }
    pub fn save(&self, path: &Path) -> Result<()> {
        self.validate()?;
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
            file.write_all(&serde_json::to_vec_pretty(self)?)?;
            file.sync_all()?;
            fs::rename(&temp, path).context("publish settings")?;
            Ok(())
        })();
        if result.is_err() {
            let _ = fs::remove_file(temp);
        }
        result
    }
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
        let controller = settings
            .controller
            .as_ref()
            .map(|id| {
                controllers
                    .iter()
                    .find(|(guid, _)| guid == id)
                    .map(|(_, name)| name.as_str())
                    .unwrap_or("DISCONNECTED")
            })
            .unwrap_or("AUTO");
        let mut labels = vec![
            format!("WINDOW {} X {}", settings.width, settings.height),
            format!("FULLSCREEN {}", settings.fullscreen),
            format!("INTEGER SCALE {}", settings.integer_scale),
            format!("VOLUME {:.0}%", settings.volume * 100.0),
            format!("DEAD ZONE {:.0}%", settings.dead_zone * 100.0),
            format!("CONTROLLER {controller}"),
            format!("PAUSE ON FOCUS LOSS {}", settings.pause_on_focus_loss),
            "RESET DEFAULTS".into(),
        ];
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
    /// Reserve Escape/F1 for cancelling the menu and Backspace for clearing.
    pub fn capture(&mut self, binding: Binding, settings: &mut Settings) -> bool {
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
        let count = 8 + settings.bindings.len();
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
        if self.selected >= 8 {
            let action = settings
                .bindings
                .keys()
                .nth(self.selected - 8)
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
                let sizes = [
                    (640, 576),
                    (800, 720),
                    (960, 864),
                    (1280, 720),
                    (1920, 1080),
                ];
                let index = sizes
                    .iter()
                    .position(|size| *size == (settings.width, settings.height))
                    .unwrap_or(0);
                (settings.width, settings.height) =
                    sizes[(index as i32 + step).rem_euclid(sizes.len() as i32) as usize];
            }
            1 => settings.fullscreen = !settings.fullscreen,
            2 => settings.integer_scale = !settings.integer_scale,
            3 => settings.volume = (settings.volume + step as f32 * 0.05).clamp(0.0, 1.0),
            4 => settings.dead_zone = (settings.dead_zone + step as f32 * 0.05).clamp(0.0, 0.95),
            5 => {
                let mut choices = vec![None];
                for (id, _) in controllers {
                    if !choices.contains(&Some(id.clone())) {
                        choices.push(Some(id.clone()));
                    }
                }
                let index = choices
                    .iter()
                    .position(|c| c == &settings.controller)
                    .unwrap_or(0);
                settings.controller = choices
                    [(index as i32 + step).rem_euclid(choices.len() as i32) as usize]
                    .clone();
            }
            6 => settings.pause_on_focus_loss = !settings.pause_on_focus_loss,
            7 => *settings = defaults.clone(),
            _ => unreachable!(),
        }
        true
    }
}
