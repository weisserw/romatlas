//! Independent MIT Game Gear graphics and stereo PSG helpers.
//! Ports own raster scheduling, register drivers and game behavior.
pub use crate::sms::tile;
pub mod vdp {
    pub use crate::sms::vdp::{Config, LineCounter, Pixel, Scanline, background, scanline};
    use anyhow::{Result, ensure};
    /// Convert a full Mode 4 line with the handheld's 64-byte palette.
    pub fn rgba(line: &Scanline, cram: &[u8]) -> Result<Vec<[u8; 4]>> {
        ensure!(cram.len() == 64, "Game Gear palette requires 64 bytes");
        Ok(line
            .indices
            .iter()
            .map(|&index| {
                let at = usize::from(index & 31) * 2;
                super::color(u16::from_le_bytes([cram[at], cram[at + 1]]))
            })
            .collect())
    }
}
use anyhow::{Result, ensure};
pub const WIDTH: u32 = 160;
pub const HEIGHT: u32 = 144;
pub const VIEWPORT_X: usize = 48;
pub const VIEWPORT_Y: usize = 24;
/// Little-endian CRAM word: low nibbles are red, green, blue.
pub fn color(value: u16) -> [u8; 4] {
    [
        (value as u8 & 15) * 17,
        ((value >> 4) as u8 & 15) * 17,
        ((value >> 8) as u8 & 15) * 17,
        255,
    ]
}
/// Compose a visible LCD scanline using the shared Mode 4 effects and sprite rules.
/// Full-width rendering before cropping preserves offscreen sprite selection/collisions.
pub fn scanline(vram: &[u8], cram: &[u8], config: vdp::Config, y: usize) -> Result<Vec<[u8; 4]>> {
    ensure!(y < HEIGHT as usize, "LCD scanline outside display");
    ensure!(cram.len() == 64, "Game Gear palette requires 64 bytes");
    let top = VIEWPORT_Y + if config.height == 224 { 16 } else { 0 };
    let line = vdp::scanline(vram, (y + top) as u16, config)?;
    let colors = vdp::rgba(&line, cram)?;
    Ok(colors[VIEWPORT_X..VIEWPORT_X + WIDTH as usize].to_vec())
}
pub fn default_settings() -> crate::settings::Settings {
    let mut s = crate::sms::default_settings();
    s.width = WIDTH * 3;
    s.height = HEIGHT * 3;
    if let Some(binding) = s.bindings.remove("pause") {
        s.bindings.insert("start".into(), binding);
    }
    s
}
pub mod audio {
    pub use crate::sms::audio::Psg;
    use anyhow::{Result, ensure};
    /// Game Gear I/O $06: low nibble enables right channels, high nibble left.
    #[derive(Clone, Debug)]
    pub struct Mixer {
        pub psg: Psg,
        pub stereo: u8,
        clock_hz: u32,
        sample_rate: u32,
        phase: u64,
    }
    impl Mixer {
        pub fn new(clock_hz: u32, sample_rate: u32) -> Result<Self> {
            ensure!(
                clock_hz > 0 && sample_rate > 0,
                "audio clocks must be nonzero"
            );
            Ok(Self {
                psg: Psg::default(),
                stereo: 255,
                clock_hz,
                sample_rate,
                phase: 0,
            })
        }
        pub fn write(&mut self, value: u8) {
            self.psg.write(value);
        }
        pub fn write_stereo(&mut self, value: u8) {
            self.stereo = value;
        }
        pub fn sample(&mut self) -> [f32; 2] {
            self.phase += u64::from(self.clock_hz);
            let clocks = self.phase / u64::from(self.sample_rate);
            self.phase %= u64::from(self.sample_rate);
            self.psg.clock(clocks as u32);
            let channels = self.psg.channels();
            std::array::from_fn(|side| {
                channels
                    .iter()
                    .enumerate()
                    .filter_map(|(c, &v)| {
                        (self.stereo & (1 << (c + if side == 0 { 4 } else { 0 })) != 0).then_some(v)
                    })
                    .sum::<f32>()
                    * 0.25
            })
        }
    }
    impl Default for Mixer {
        fn default() -> Self {
            Self::new(3_579_545, 48_000).expect("constant audio clocks")
        }
    }
}
