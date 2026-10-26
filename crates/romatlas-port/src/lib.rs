//! Native game-port helpers. Core modules never initialize display/audio devices.
//! Platform modules reproduce selected effects; this is not a console emulator.
pub mod assets;
pub mod audio;
pub mod browser;
#[cfg(not(target_arch = "wasm32"))]
pub mod debug;
#[cfg(feature = "desktop")]
pub mod desktop;
pub mod gb;
pub mod gg;
pub mod graphics;
pub mod input;
pub mod nes;
pub mod saves;
pub mod settings;
pub mod sms;
pub mod snes;
#[cfg(feature = "snapshots")]
pub mod states;
pub mod timing;

use anyhow::{Result, ensure};
use graphics::Frame;
use input::{InputState, MultiInputState};

/// A port owns its rules/state and can run identically with or without a window.
pub trait Game {
    fn tick(&mut self, input: &InputState);
    /// Optional full native state, separate from battery persistence. No replay.
    fn snapshot_json(&self) -> Result<String> {
        anyhow::bail!("save states unavailable for this port")
    }
    /// Validate before mutation; errors must leave the current game intact.
    fn restore_snapshot_json(&mut self, _json: &str) -> Result<()> {
        anyhow::bail!("save states unavailable for this port")
    }
    fn reset(&mut self) -> Result<()> {
        anyhow::bail!("reset unavailable for this port")
    }
    /// Original in-game battery persistence only, never a full runtime snapshot.
    /// Increment the version when the game-owned persistent representation changes.
    fn save_version(&self) -> Option<u32> {
        None
    }
    fn load_save(&mut self, _bytes: &[u8]) -> Result<()> {
        anyhow::bail!("this port does not support battery saves")
    }
    fn save_data(&self) -> Result<Vec<u8>> {
        Ok(Vec::new())
    }
    /// Remains true until the frontend successfully stores and acknowledges data.
    fn save_dirty(&self) -> bool {
        false
    }
    fn save_acknowledged(&mut self) {}
    /// Advance with independent controller slots. Existing ports continue to
    /// receive the primary player's actions until they opt in to multiplayer.
    fn tick_players(&mut self, input: &MultiInputState) {
        self.tick(&input.primary);
    }
    /// Optional presentation aspect for ports whose native pixels are stretched.
    /// None preserves the framebuffer's aspect and existing frontend behavior.
    fn display_aspect_ratio(&self) -> Option<(u32, u32)> {
        None
    }
    fn render(&self, frame: &mut Frame);
    /// Append interleaved stereo f32 samples at the requested rate. This default
    /// is silence, not a substitute for implementing the original game's audio.
    fn audio(&mut self, frames: usize, _sample_rate: u32, samples: &mut Vec<f32>) {
        samples.resize(samples.len() + frames * 2, 0.0);
    }
}

/// Scripted advancement with no SDL, wall clock, display or audio device.
pub fn headless(
    game: &mut impl Game,
    frames: usize,
    mut input: impl FnMut(usize) -> InputState,
    size: (u32, u32),
) -> Result<Frame> {
    headless_players(
        game,
        frames,
        |index| {
            let primary = input(index);
            let players = std::array::from_fn(|player| {
                if player == 0 {
                    crate::input::PlayerInput {
                        held: primary.held.clone(),
                        pressed: primary.pressed.clone(),
                        released: primary.released.clone(),
                    }
                } else {
                    crate::input::PlayerInput::default()
                }
            });
            MultiInputState { primary, players }
        },
        size,
    )
}

/// Scripted advancement with independent controller slots and no frontend devices.
pub fn headless_players(
    game: &mut impl Game,
    frames: usize,
    mut input: impl FnMut(usize) -> MultiInputState,
    size: (u32, u32),
) -> Result<Frame> {
    ensure!(frames <= 1_000_000, "headless frame limit is 1000000");
    let mut frame = Frame::new(size.0, size.1)?;
    for index in 0..frames {
        game.tick_players(&input(index));
    }
    game.render(&mut frame);
    Ok(frame)
}

pub mod genesis;

pub mod pce;
