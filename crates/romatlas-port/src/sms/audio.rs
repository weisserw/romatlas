//! Original MIT native synthesis helpers, driven by port-owned register writes.
//! PSG is clock driven; FM is an explicitly approximate two-operator renderer.
use anyhow::{Result, ensure};
#[derive(Clone, Debug)]
pub struct Psg {
    pub tone: [u16; 3],
    pub attenuation: [u8; 4],
    pub noise: u8,
    latch: u8,
    counter: [u16; 4],
    polarity: [bool; 4],
    lfsr: u16,
    divider: u8,
}
impl Default for Psg {
    fn default() -> Self {
        Self {
            tone: [0; 3],
            attenuation: [15; 4],
            noise: 0,
            latch: 0,
            counter: [1; 4],
            polarity: [false; 4],
            lfsr: 0x8000,
            divider: 0,
        }
    }
}
impl Psg {
    pub fn write(&mut self, v: u8) {
        if v & 128 != 0 {
            self.latch = (v >> 4) & 7;
        }
        let channel = usize::from(self.latch >> 1);
        if self.latch & 1 != 0 {
            self.attenuation[channel] = v & 15;
        } else if channel == 3 {
            self.noise = v & 7;
            self.lfsr = 0x8000;
            self.counter[3] = 1;
            self.polarity[3] = false;
        } else if v & 128 != 0 {
            self.tone[channel] = (self.tone[channel] & 0x3f0) | u16::from(v & 15);
        } else {
            self.tone[channel] = (self.tone[channel] & 15) | (u16::from(v & 63) << 4);
        }
    }
    /// Advance input clocks (NTSC 3,579,545 Hz), preserving fractional output phase externally.
    pub fn clock(&mut self, clocks: u32) {
        for _ in 0..clocks {
            self.divider += 1;
            if self.divider < 16 {
                continue;
            }
            self.divider = 0;
            for c in 0..3 {
                self.counter[c] = self.counter[c].saturating_sub(1);
                if self.counter[c] == 0 {
                    self.counter[c] = self.tone[c].max(1);
                    self.polarity[c] = !self.polarity[c];
                }
            }
            let period = match self.noise & 3 {
                0 => 16,
                1 => 32,
                2 => 64,
                _ => self.tone[2].max(1),
            };
            self.counter[3] = self.counter[3].saturating_sub(1);
            if self.counter[3] == 0 {
                self.counter[3] = period;
                self.polarity[3] = !self.polarity[3];
                if self.polarity[3] {
                    let feedback = if self.noise & 4 != 0 {
                        (self.lfsr ^ (self.lfsr >> 3)) & 1
                    } else {
                        self.lfsr & 1
                    };
                    self.lfsr = (self.lfsr >> 1) | (feedback << 15);
                }
            }
        }
    }
    pub fn sample(&self) -> f32 {
        self.channels().into_iter().sum::<f32>() * 0.25
    }
    /// Unmixed PSG channels for independent handheld stereo routing.
    pub fn channels(&self) -> [f32; 4] {
        std::array::from_fn(|c| {
            if self.attenuation[c] >= 15 {
                return 0.0;
            }
            let amplitude = 10f32.powf(-f32::from(self.attenuation[c]) / 10.0);
            amplitude
                * if if c == 3 {
                    self.lfsr & 1 != 0
                } else {
                    self.polarity[c]
                } {
                    1.0
                } else {
                    -1.0
                }
        })
    }
}
/// Eight OPLL instrument register bytes. Supply a reviewed hardware patch bank or
/// game-specific equivalent instruments. Patch zero is always the custom registers.
#[derive(Clone, Copy, Debug, Default)]
pub struct Patch(pub [u8; 8]);
// Preset data only: emu2413 MIT, see LICENSE-emu2413.txt and docs/sms-native.md.
pub const OPLL_PATCHES: [Patch; 19] = [
    Patch([0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]),
    Patch([0x71, 0x61, 0x1e, 0x17, 0xd0, 0x78, 0x00, 0x17]),
    Patch([0x13, 0x41, 0x1a, 0x0d, 0xd8, 0xf7, 0x23, 0x13]),
    Patch([0x13, 0x01, 0x99, 0x00, 0xf2, 0xc4, 0x21, 0x23]),
    Patch([0x11, 0x61, 0x0e, 0x07, 0x8d, 0x64, 0x70, 0x27]),
    Patch([0x32, 0x21, 0x1e, 0x06, 0xe1, 0x76, 0x01, 0x28]),
    Patch([0x31, 0x22, 0x16, 0x05, 0xe0, 0x71, 0x00, 0x18]),
    Patch([0x21, 0x61, 0x1d, 0x07, 0x82, 0x81, 0x11, 0x07]),
    Patch([0x33, 0x21, 0x2d, 0x13, 0xb0, 0x70, 0x00, 0x07]),
    Patch([0x61, 0x61, 0x1b, 0x06, 0x64, 0x65, 0x10, 0x17]),
    Patch([0x41, 0x61, 0x0b, 0x18, 0x85, 0xf0, 0x81, 0x07]),
    Patch([0x33, 0x01, 0x83, 0x11, 0xea, 0xef, 0x10, 0x04]),
    Patch([0x17, 0xc1, 0x24, 0x07, 0xf8, 0xf8, 0x22, 0x12]),
    Patch([0x61, 0x50, 0x0c, 0x05, 0xd2, 0xf5, 0x40, 0x42]),
    Patch([0x01, 0x01, 0x55, 0x03, 0xe9, 0x90, 0x03, 0x02]),
    Patch([0x41, 0x41, 0x89, 0x03, 0xf1, 0xe4, 0xc0, 0x13]),
    Patch([0x01, 0x01, 0x18, 0x0f, 0xdf, 0xf8, 0x6a, 0x6d]),
    Patch([0x01, 0x01, 0x00, 0x00, 0xc8, 0xd8, 0xa7, 0x68]),
    Patch([0x05, 0x01, 0x00, 0x00, 0xf8, 0xaa, 0x59, 0x55]),
];

#[derive(Clone, Copy, Debug, Default)]
struct Operator {
    phase: f64,
    envelope: f64,
    stage: u8,
    feedback: f64,
}
#[derive(Clone, Debug)]
pub struct Fm {
    pub registers: [u8; 64],
    pub patches: [Patch; 16],
    operators: [[Operator; 2]; 9],
    keys: [bool; 9],
    selected: u8,
    noise: u32,
    lfo: f64,
}
impl Fm {
    pub fn new(patches: [Patch; 16]) -> Self {
        Self {
            registers: [0; 64],
            patches,
            operators: [[Operator::default(); 2]; 9],
            keys: [false; 9],
            selected: 0,
            noise: 1,
            lfo: 0.0,
        }
    }
    pub fn select(&mut self, r: u8) {
        self.selected = r & 63;
    }
    pub fn data(&mut self, v: u8) {
        self.write(self.selected, v);
    }
    pub fn write(&mut self, r: u8, v: u8) {
        let r = usize::from(r & 63);
        self.registers[r] = v;
        if r < 8 {
            self.patches[0].0[r] = v;
        }
    }
    /// Generate one mono output sample. This models register control and musical
    /// envelopes, not OPLL logarithmic tables, slot timing or analog output.
    pub fn sample(&mut self, sample_rate: u32, clock_hz: f64) -> Result<f32> {
        ensure!(
            sample_rate > 0 && clock_hz.is_finite() && (1.0..=20_000_000.0).contains(&clock_hz),
            "invalid FM clock/rate"
        );
        let dt = 1.0 / f64::from(sample_rate);
        self.lfo = (self.lfo + dt * 6.1).fract();
        let rhythm = self.registers[14] & 32 != 0;
        self.noise = (self.noise >> 1) | (((self.noise ^ (self.noise >> 5)) & 1) << 22);
        let mut sum = 0.0;
        for channel in 0..9 {
            let high = self.registers[0x20 + channel];
            let key = if rhythm && channel >= 6 {
                self.registers[14]
                    & match channel {
                        6 => 16,
                        7 => 9,
                        _ => 6,
                    }
                    != 0
            } else {
                high & 16 != 0
            };
            if key && !self.keys[channel] {
                for op in &mut self.operators[channel] {
                    op.stage = 1;
                    op.phase = 0.0;
                }
            }
            if !key && self.keys[channel] {
                for op in &mut self.operators[channel] {
                    op.stage = 4;
                }
            }
            self.keys[channel] = key;
            let patch = if rhythm && channel >= 6 {
                OPLL_PATCHES[16 + channel - 6].0
            } else {
                self.patches[usize::from(self.registers[0x30 + channel] >> 4)].0
            };
            let frequency =
                f64::from(u16::from(self.registers[0x10 + channel]) | (u16::from(high & 1) << 8))
                    * 2f64.powi(i32::from((high >> 1) & 7))
                    * clock_hz
                    / (72.0 * 524288.0);
            let mut modulation = 0.0;
            for n in 0..2 {
                let op = &mut self.operators[channel][n];
                let rates = patch[4 + n];
                let sustain_release = patch[6 + n];
                let rate = |r: u8| {
                    if r == 0 {
                        0.0
                    } else {
                        2f64.powf(f64::from(r) * 0.7) * dt
                    }
                };
                match op.stage {
                    1 => {
                        op.envelope += (1.0 - op.envelope) * rate(rates >> 4) * 16.0;
                        if op.envelope >= 0.999 {
                            op.envelope = 1.0;
                            op.stage = 2;
                        }
                    }
                    2 => {
                        op.envelope -= rate(rates & 15);
                        if op.envelope <= 10f64.powf(-f64::from(sustain_release >> 4) * 3.0 / 20.0)
                        {
                            op.stage = 3;
                        }
                    }
                    3 => {
                        if patch[n] & 32 == 0 {
                            op.envelope -= rate(sustain_release & 15);
                        }
                    }
                    4 => {
                        op.envelope -= rate(if high & 32 != 0 {
                            5
                        } else {
                            (sustain_release & 15).max(1)
                        })
                    }
                    _ => {}
                }
                op.envelope = op.envelope.clamp(0.0, 1.0);
                let multiplier = [
                    0.5, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 10.0, 12.0, 12.0, 15.0,
                    15.0,
                ][usize::from(patch[n] & 15)];
                let vibrato = if patch[n] & 64 != 0 {
                    (self.lfo * std::f64::consts::TAU).sin() * 0.007
                } else {
                    0.0
                };
                op.phase = (op.phase + frequency * multiplier * (1.0 + vibrato) * dt).fract();
                let feedback = if n == 0 {
                    op.feedback * f64::from(patch[3] & 7) * 0.2
                } else {
                    modulation
                };
                let mut wave = (op.phase * std::f64::consts::TAU + feedback).sin();
                if patch[3] & (8 << n) != 0 {
                    wave = wave.max(0.0);
                }
                let attenuation = if n == 0 {
                    f64::from(patch[2] & 63) * 0.75
                } else {
                    f64::from(self.registers[0x30 + channel] & 15) * 3.0
                };
                let tremolo = if patch[n] & 128 != 0 {
                    1.0 - 0.15 * (self.lfo * std::f64::consts::TAU).sin().abs()
                } else {
                    1.0
                };
                modulation = wave * op.envelope * 10f64.powf(-attenuation / 20.0) * tremolo;
                op.feedback = modulation;
            }
            if rhythm && channel >= 7 {
                modulation *= if self.noise & 1 != 0 { 1.0 } else { -1.0 };
            }
            sum += modulation;
        }
        Ok((sum / 9.0) as f32)
    }
}
/// Sample-clock bridge and port $F2 PSG/FM gating. Ports own music drivers.
#[derive(Clone, Debug)]
pub struct Mixer {
    pub psg: Psg,
    pub fm: Fm,
    pub control: u8,
    pub clock_hz: f64,
    remainder: f64,
}
impl Mixer {
    pub fn new(patches: [Patch; 16]) -> Self {
        Self {
            psg: Psg::default(),
            fm: Fm::new(patches),
            control: 0,
            clock_hz: 3_579_545.0,
            remainder: 0.0,
        }
    }
    pub fn sample(&mut self, sample_rate: u32) -> Result<f32> {
        ensure!(
            (1..=384000).contains(&sample_rate)
                && self.clock_hz.is_finite()
                && (1.0..=20_000_000.0).contains(&self.clock_hz),
            "invalid mixer clock/rate"
        );
        self.remainder += self.clock_hz / f64::from(sample_rate);
        let clocks = self.remainder.floor() as u32;
        self.remainder -= f64::from(clocks);
        self.psg.clock(clocks);
        let fm = self.fm.sample(sample_rate, self.clock_hz)?;
        Ok((if matches!(self.control & 3, 0 | 3) {
            self.psg.sample()
        } else {
            0.0
        }) + if self.control & 1 != 0 { fm } else { 0.0 })
    }
}

impl Default for Fm {
    fn default() -> Self {
        Self::new(std::array::from_fn(|i| OPLL_PATCHES[i]))
    }
}
impl Default for Mixer {
    fn default() -> Self {
        Self::new(std::array::from_fn(|i| OPLL_PATCHES[i]))
    }
}
