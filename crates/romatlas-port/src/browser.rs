//! Device-free browser driver. The JS shell only translates host I/O; games use
//! the same Game/input/frame/audio/save contracts as SDL desktop ports.
use crate::{
    Game,
    audio::SampleClock,
    graphics::Frame,
    input::{InputLatch, InputMapper, PhysicalInput},
    saves::BatterySave,
    settings::Settings,
    timing::{Clock, TickRate},
};
use anyhow::{Result, ensure};
use std::time::Duration;

pub struct Runtime<G: Game> {
    pub game: G,
    pub frame: Frame,
    pub settings: Settings,
    pub samples: Vec<f32>,
    clock: Clock,
    sample_clock: SampleClock,
    sample_rate: u32,
    mapper: InputMapper,
    latches: [InputLatch; 4],
    rom_sha256: String,
}
impl<G: Game> Runtime<G> {
    pub fn new(
        game: G,
        size: (u32, u32),
        rate: TickRate,
        settings: Settings,
        sample_rate: u32,
        rom_sha256: String,
    ) -> Result<Self> {
        settings.validate()?;
        ensure!((8000..=192000).contains(&sample_rate), "invalid audio rate");
        Ok(Self {
            game,
            frame: Frame::new(size.0, size.1)?,
            settings,
            samples: Vec::new(),
            clock: Clock::new(rate, 8)?,
            sample_clock: SampleClock::new(rate, sample_rate)?,
            sample_rate,
            mapper: InputMapper::default(),
            latches: Default::default(),
            rom_sha256,
        })
    }
    pub fn configure(&mut self, json: &str) -> Result<()> {
        ensure!(json.len() <= 1024 * 1024, "settings exceed 1 MiB");
        let settings: Settings = serde_json::from_str(json)?;
        settings.validate()?;
        self.settings = settings;
        self.mapper.clear();
        Ok(())
    }
    pub fn boundary(&mut self, physical: &[PhysicalInput; 4]) {
        self.clock.reset();
        self.mapper.clear();
        self.samples.clear();
        for (latch, input) in self.latches.iter_mut().zip(physical) {
            latch.suppress(input);
        }
    }
    pub fn advance(&mut self, elapsed_ms: f64, mut physical: [PhysicalInput; 4]) -> Result<u32> {
        ensure!(
            elapsed_ms.is_finite() && (0.0..=60000.0).contains(&elapsed_ms),
            "invalid browser elapsed time"
        );
        for (latch, input) in self.latches.iter_mut().zip(&mut physical) {
            *input = latch.filter(input.clone(), self.settings.dead_zone);
        }
        self.samples.clear();
        let ticks = self
            .clock
            .advance(Duration::from_secs_f64(elapsed_ms / 1000.0))
            .ticks;
        for _ in 0..ticks {
            self.game
                .tick_players(&self.mapper.sample_players(&physical, &self.settings));
            let count = self.sample_clock.next_tick();
            let before = self.samples.len();
            self.game.audio(count, self.sample_rate, &mut self.samples);
            ensure!(
                self.samples.len() == before + count * 2,
                "game audio must supply requested stereo sample count"
            );
        }
        for sample in &mut self.samples {
            *sample = if sample.is_finite() {
                (*sample * self.settings.volume).clamp(-1.0, 1.0)
            } else {
                0.0
            };
        }
        self.game.render(&mut self.frame);
        Ok(ticks)
    }
    pub fn save_json(&self) -> Result<String> {
        if !self.game.save_dirty() {
            return Ok(String::new());
        }
        Ok(BatterySave::capture(&self.game, &self.rom_sha256)?
            .map(|save| serde_json::to_string(&save))
            .transpose()?
            .unwrap_or_default())
    }
    pub fn restore(&mut self, json: &str) -> Result<()> {
        ensure!(
            json.len() <= crate::saves::MAX_SAVE_BYTES * 4 + 1024,
            "battery save too large"
        );
        let save: BatterySave = serde_json::from_str(json)?;
        save.restore(&mut self.game, &self.rom_sha256)
    }
}

#[cfg(feature = "snapshots")]
#[derive(serde::Deserialize)]
struct DriverSnapshot {
    clock: Clock,
    sample_clock: SampleClock,
    mapper: InputMapper,
    latches: [InputLatch; 4],
    sample_rate: u32,
    rom_sha256: String,
}
#[cfg(feature = "snapshots")]
impl<G: Game> Runtime<G> {
    pub fn driver_snapshot_json(&self) -> Result<String> {
        Ok(serde_json::to_string(&serde_json::json!({
            "clock": self.clock, "sample_clock": self.sample_clock,
            "mapper": self.mapper, "latches": self.latches,
            "sample_rate": self.sample_rate, "rom_sha256": self.rom_sha256,
        }))?)
    }
    fn validated_driver(&self, json: &str) -> Result<DriverSnapshot> {
        ensure!(json.len() <= 1024 * 1024, "driver snapshot too large");
        let saved: DriverSnapshot = serde_json::from_str(json)?;
        ensure!(
            saved.sample_rate == self.sample_rate && saved.rom_sha256 == self.rom_sha256,
            "driver snapshot identity mismatch"
        );
        ensure!(
            saved.clock.snapshot_compatible(&self.clock)
                && saved.sample_clock.snapshot_compatible(&self.sample_clock),
            "invalid snapshot clock"
        );
        Ok(saved)
    }
    /// Validate driver compatibility before invoking the game's atomic restore.
    pub fn restore_state_json(&mut self, game: &str, driver: &str) -> Result<()> {
        let saved = self.validated_driver(driver)?;
        self.game.restore_snapshot_json(game)?;
        self.apply_driver(saved);
        self.game.render(&mut self.frame);
        Ok(())
    }
    pub fn restore_driver_snapshot_json(&mut self, json: &str) -> Result<()> {
        let saved = self.validated_driver(json)?;
        self.apply_driver(saved);
        Ok(())
    }
    fn apply_driver(&mut self, saved: DriverSnapshot) {
        self.clock = saved.clock;
        self.sample_clock = saved.sample_clock;
        self.mapper = saved.mapper;
        self.latches = saved.latches;
        self.samples.clear();
    }
}
