//! Rational console cadence, independent of display refresh and wall-clock RTC.
use anyhow::{Result, ensure};
use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct TickRate {
    pub numerator: u32,
    pub denominator: u32,
}
impl TickRate {
    pub const DMG: Self = Self {
        numerator: 4_194_304,
        denominator: 70_224,
    };
    /// Nominal non-interlaced NTSC frame cadence (master clock / frame clocks).
    pub const SNES_NTSC: Self = Self {
        numerator: 21_477_272,
        denominator: 357_366,
    };
    /// NTSC PPU cadence averaged over the rendering-enabled odd-frame skipped dot.
    pub const NES_NTSC: Self = Self {
        numerator: 10_738_636,
        denominator: 178_683,
    };
    /// SMS NTSC master clock / (342 dots * 10 clocks * 262 lines).
    pub const SMS_NTSC: Self = Self {
        numerator: 53_693_175,
        denominator: 896_040,
    };
    pub const SMS_PAL: Self = Self {
        numerator: 53_203_424,
        denominator: 1_070_460,
    };
    pub fn validate(self) -> Result<()> {
        ensure!(
            self.denominator > 0
                && self.numerator > 0
                && u64::from(self.numerator) <= u64::from(self.denominator) * 1000,
            "tick rate must be in (0, 1000] Hz"
        );
        Ok(())
    }
}
pub struct Clock {
    rate: TickRate,
    remainder: u128,
    max_catch_up: u32,
}
#[derive(Debug, PartialEq, Eq)]
pub struct Advancement {
    pub ticks: u32,
    pub dropped_ticks: u128,
}
impl Clock {
    pub fn new(rate: TickRate, max_catch_up: u32) -> Result<Self> {
        rate.validate()?;
        ensure!((1..=1000).contains(&max_catch_up), "invalid catch-up limit");
        Ok(Self {
            rate,
            remainder: 0,
            max_catch_up,
        })
    }
    pub fn advance(&mut self, elapsed: Duration) -> Advancement {
        self.remainder += elapsed.as_nanos() * u128::from(self.rate.numerator);
        let period = u128::from(self.rate.denominator) * 1_000_000_000;
        let due = self.remainder / period;
        self.remainder %= period;
        let ticks = due.min(u128::from(self.max_catch_up)) as u32;
        Advancement {
            ticks,
            dropped_ticks: due - u128::from(ticks),
        }
    }
    pub fn reset(&mut self) {
        self.remainder = 0;
    }
}
