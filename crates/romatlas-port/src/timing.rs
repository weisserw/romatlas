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
