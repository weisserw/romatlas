//! Sample budgets and simple synthesis building blocks; no audio device access.
use crate::timing::TickRate;
use anyhow::{Result, ensure};

pub struct SampleClock {
    rate: TickRate,
    sample_rate: u32,
    remainder: u64,
}
impl SampleClock {
    pub fn new(rate: TickRate, sample_rate: u32) -> Result<Self> {
        rate.validate()?;
        ensure!(
            (8000..=192000).contains(&sample_rate),
            "unsupported sample rate"
        );
        Ok(Self {
            rate,
            sample_rate,
            remainder: 0,
        })
    }
    pub fn next_tick(&mut self) -> usize {
        self.remainder += u64::from(self.sample_rate) * u64::from(self.rate.denominator);
        let samples = self.remainder / u64::from(self.rate.numerator);
        self.remainder %= u64::from(self.rate.numerator);
        samples as usize
    }
}

/// A convenience oscillator, not an accurate Game Boy APU or sound driver.
#[derive(Default)]
pub struct Pulse {
    phase: f64,
}
impl Pulse {
    pub fn sample(&mut self, hz: f64, duty: f64, sample_rate: u32) -> f32 {
        if sample_rate == 0 || !hz.is_finite() || hz <= 0.0 || !duty.is_finite() {
            return 0.0;
        }
        self.phase = (self.phase + hz / f64::from(sample_rate)).fract();
        if self.phase < duty.clamp(0.0, 1.0) {
            1.0
        } else {
            -1.0
        }
    }
}
