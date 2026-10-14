//! Native game-port helpers. Core modules never initialize display/audio devices.
//! Platform modules reproduce selected effects; this is not a console emulator.
pub mod assets;
pub mod audio;
#[cfg(feature = "desktop")]
pub mod desktop;
pub mod gb;
pub mod graphics;
pub mod input;
pub mod settings;
pub mod timing;

use anyhow::{Result, ensure};
use graphics::Frame;
use input::InputState;

/// A port owns its rules/state and can run identically with or without a window.
pub trait Game {
    fn tick(&mut self, input: &InputState);
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
    ensure!(frames <= 1_000_000, "headless frame limit is 1000000");
    let mut frame = Frame::new(size.0, size.1)?;
    for index in 0..frames {
        game.tick(&input(index));
    }
    game.render(&mut frame);
    Ok(frame)
}
