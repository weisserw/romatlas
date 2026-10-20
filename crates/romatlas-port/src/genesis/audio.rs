//! Original MIT OPN2-oriented native synthesis. Register semantics and routing are
//! useful to ports; analog ladder, exact envelope/phase tables and bus timing are approximations.
pub use crate::sms::audio::Psg;
use anyhow::{Result, ensure};
const TAU: f64 = std::f64::consts::TAU;
#[derive(Clone, Copy, Debug, Default)]
enum Stage {
    Attack,
    Decay,
    Sustain,
    Release,
    #[default]
    Off,
}
#[derive(Clone, Copy, Debug, Default)]
struct Operator {
    phase: f64,
    envelope: f64,
    stage: Stage,
    key: bool,
    ssg_invert: bool,
    previous: [f64; 2],
}
#[derive(Clone, Debug)]
pub struct Ym2612 {
    registers: [[u8; 256]; 2],
    address: [u8; 2],
    operators: [[Operator; 4]; 6],
    timer_clock: u64,
    timer_a: u32,
    timer_b: u32,
    pub status: u8,
    lfo_phase: f64,
    sample_rate: u32,
    clock_hz: u32,
}
impl Ym2612 {
    pub fn new(clock_hz: u32, sample_rate: u32) -> Result<Self> {
        ensure!(
            clock_hz > 0 && (8000..=192000).contains(&sample_rate),
            "invalid FM clock/sample rate"
        );
        Ok(Self {
            registers: [[0; 256]; 2],
            address: [0; 2],
            operators: [[Operator::default(); 4]; 6],
            timer_clock: 0,
            timer_a: 0,
            timer_b: 0,
            status: 0,
            lfo_phase: 0.0,
            sample_rate,
            clock_hz,
        })
    }
    pub fn read_status(&self) -> u8 {
        self.status & 3
    }
    pub fn write_port(&mut self, port: u8, value: u8) {
        let bank = usize::from((port >> 1) & 1);
        if port & 1 == 0 {
            self.address[bank] = value
        } else {
            self.write(bank as u8, self.address[bank], value)
        }
    }
    fn key(&mut self, ch: usize, mask: u8) {
        for (op, state) in self.operators[ch].iter_mut().enumerate() {
            let key = mask & (1 << op) != 0;
            if key && !state.key {
                state.stage = Stage::Attack;
                state.phase = 0.0;
                state.ssg_invert = false;
            } else if !key && state.key {
                state.stage = Stage::Release;
            }
            state.key = key;
        }
    }
    pub fn write(&mut self, bank: u8, register: u8, value: u8) {
        let bank = usize::from(bank & 1);
        let old = self.registers[bank][register as usize];
        self.registers[bank][register as usize] = value;
        if bank == 0 {
            match register {
                0x28 => {
                    let ch = usize::from(value & 3) + if value & 4 != 0 { 3 } else { 0 };
                    if value & 3 != 3 {
                        self.key(ch, value >> 4)
                    }
                }
                0x27 => {
                    if value & 16 != 0 {
                        self.status &= !1
                    }
                    if value & 32 != 0 {
                        self.status &= !2
                    }
                    if value & 1 != 0 && old & 1 == 0 {
                        self.timer_a = 0
                    }
                    if value & 2 != 0 && old & 2 == 0 {
                        self.timer_b = 0
                    }
                }
                _ => {}
            }
        }
    }
    /// FM input clocks (master clock / 7). Timers remain deterministic when muted.
    pub fn clock(&mut self, clocks: u32) {
        self.timer_clock += u64::from(clocks);
        let ticks = (self.timer_clock / 72) as u32;
        self.timer_clock %= 72;
        let mode = self.registers[0][0x27];
        if mode & 1 != 0 {
            let preset =
                (u32::from(self.registers[0][0x24]) << 2) | u32::from(self.registers[0][0x25] & 3);
            let period = 1024 - preset;
            self.timer_a += ticks;
            while self.timer_a >= period {
                self.timer_a -= period;
                if mode & 4 != 0 {
                    self.status |= 1
                }
                if mode & 128 != 0 {
                    self.key(2, 15);
                }
            }
        }
        if mode & 2 != 0 {
            let period = 16 * (256 - u32::from(self.registers[0][0x26]));
            self.timer_b += ticks;
            if self.timer_b >= period {
                self.timer_b %= period;
                if mode & 8 != 0 {
                    self.status |= 2
                }
            }
        }
    }
    fn frequency(&self, ch: usize, op: usize) -> f64 {
        let bank = ch / 3;
        let n = ch % 3;
        let (low, high) = if ch == 2 && op < 3 && self.registers[0][0x27] & 0xc0 != 0 {
            let index = [1, 2, 0][op];
            (
                self.registers[0][0xa8 + index],
                self.registers[0][0xac + index],
            )
        } else {
            (
                self.registers[bank][0xa0 + n],
                self.registers[bank][0xa4 + n],
            )
        };
        let fnum = (u16::from(high & 7) << 8) | u16::from(low);
        let block = (high >> 3) & 7;
        f64::from(fnum) * 2f64.powi(i32::from(block) - 1) * f64::from(self.clock_hz)
            / (144.0 * 1_048_576.0)
    }
    fn operator(&mut self, ch: usize, op: usize, modulation: f64, lfo: f64) -> f64 {
        let slot = [0, 2, 1, 3][op];
        let n = ch % 3 + slot * 4;
        let bank = ch / 3;
        let r = &self.registers[bank];
        let dt_mul = r[0x30 + n];
        let tl = r[0x40 + n] & 127;
        let ar = r[0x50 + n] & 31;
        let dr = r[0x60 + n] & 31;
        let sr = r[0x70 + n] & 31;
        let rr = (r[0x80 + n] & 15) * 2 + 1;
        let sl = r[0x80 + n] >> 4;
        let ssg = r[0x90 + n] & 15;
        let am = r[0x60 + n] & 128 != 0;
        let sensitivity = r[0xb4 + ch % 3];
        let freq = self.frequency(ch, op);
        let key_scale = (r[0x50 + n] >> 6) as f64;
        let op = &mut self.operators[ch][op];
        let step = |rate: u8| {
            if rate == 0 {
                0.0
            } else {
                2f64.powf((f64::from(rate) + key_scale * 2.0) / 4.0)
                    / (f64::from(self.sample_rate) * 4.0)
            }
        };
        match op.stage {
            Stage::Attack => {
                op.envelope += (1.0 - op.envelope) * step(ar) * 8.0;
                if op.envelope >= 0.999 || ar == 31 {
                    op.envelope = 1.0;
                    op.stage = Stage::Decay
                }
            }
            Stage::Decay => {
                op.envelope -= step(dr);
                let sustain = 10f64.powf(-if sl == 15 { 93.0 } else { f64::from(sl) * 3.0 } / 20.0);
                if op.envelope <= sustain {
                    op.envelope = sustain;
                    op.stage = Stage::Sustain
                }
            }
            Stage::Sustain => {
                op.envelope -= step(sr);
                if op.envelope <= 0.0 && ssg & 8 != 0 {
                    if ssg & 2 != 0 {
                        op.ssg_invert = !op.ssg_invert
                    }
                    if ssg & 1 != 0 {
                        op.envelope = if ssg & 4 != 0 { 1.0 } else { 0.0 };
                        op.stage = Stage::Off
                    } else {
                        op.envelope = 1.0;
                    }
                }
            }
            Stage::Release => op.envelope -= step(rr),
            Stage::Off => {}
        }
        op.envelope = op.envelope.clamp(0.0, 1.0);
        if op.envelope == 0.0 && !matches!(op.stage, Stage::Attack | Stage::Sustain) {
            op.stage = Stage::Off
        }
        let mult = if dt_mul & 15 == 0 {
            0.5
        } else {
            f64::from(dt_mul & 15)
        };
        let dt = dt_mul >> 4 & 7;
        let detune = if dt & 4 != 0 { -1.0 } else { 1.0 } * f64::from(dt & 3) * 0.2;
        let pm = 1.0 + lfo * f64::from(sensitivity & 7) * 0.001;
        op.phase = (op.phase + TAU * (freq * mult + detune) * pm / f64::from(self.sample_rate))
            .rem_euclid(TAU);
        let env = if ssg & 8 != 0 && (op.ssg_invert ^ (ssg & 4 != 0)) {
            1.0 - op.envelope
        } else {
            op.envelope
        };
        let amp = if am {
            (1.0 - (lfo + 1.0) * 0.5 * f64::from((sensitivity >> 4) & 3) * 0.15).max(0.0)
        } else {
            1.0
        };
        (op.phase + modulation * 4.0).sin() * env * 10f64.powf(-f64::from(tl) * 0.75 / 20.0) * amp
    }
    /// Produce a stereo sample. A port supplies writes and elapsed clocks separately.
    pub fn sample(&mut self) -> [f32; 2] {
        let lfo_reg = self.registers[0][0x22];
        let lfo = if lfo_reg & 8 != 0 {
            let hz = [3.98, 5.56, 6.02, 6.37, 6.88, 9.63, 48.1, 72.2][usize::from(lfo_reg & 7)];
            self.lfo_phase = (self.lfo_phase + TAU * hz / f64::from(self.sample_rate)) % TAU;
            self.lfo_phase.sin()
        } else {
            0.0
        };
        let mut out = [0.0f64; 2];
        for ch in 0..6 {
            let r = self.registers[ch / 3][0xb0 + ch % 3];
            let algorithm = r & 7;
            let feedback = (r >> 3) & 7;
            let history = self.operators[ch][0].previous;
            let fb = if feedback == 0 {
                0.0
            } else {
                (history[0] + history[1]) * 2f64.powi(i32::from(feedback) - 7)
            };
            let a = self.operator(ch, 0, fb, lfo);
            self.operators[ch][0].previous = [a, history[0]];
            let (b, c, d, result) = match algorithm {
                0 => {
                    let b = self.operator(ch, 1, a, lfo);
                    let c = self.operator(ch, 2, b, lfo);
                    let d = self.operator(ch, 3, c, lfo);
                    (b, c, d, d)
                }
                1 => {
                    let b = self.operator(ch, 1, 0.0, lfo);
                    let c = self.operator(ch, 2, a + b, lfo);
                    let d = self.operator(ch, 3, c, lfo);
                    (b, c, d, d)
                }
                2 => {
                    let b = self.operator(ch, 1, 0.0, lfo);
                    let c = self.operator(ch, 2, b, lfo);
                    let d = self.operator(ch, 3, a + c, lfo);
                    (b, c, d, d)
                }
                3 => {
                    let b = self.operator(ch, 1, a, lfo);
                    let c = self.operator(ch, 2, 0.0, lfo);
                    let d = self.operator(ch, 3, b + c, lfo);
                    (b, c, d, d)
                }
                4 => {
                    let b = self.operator(ch, 1, a, lfo);
                    let c = self.operator(ch, 2, 0.0, lfo);
                    let d = self.operator(ch, 3, c, lfo);
                    (b, c, d, b + d)
                }
                5 => {
                    let b = self.operator(ch, 1, a, lfo);
                    let c = self.operator(ch, 2, a, lfo);
                    let d = self.operator(ch, 3, a, lfo);
                    (b, c, d, b + c + d)
                }
                6 => {
                    let b = self.operator(ch, 1, a, lfo);
                    let c = self.operator(ch, 2, 0.0, lfo);
                    let d = self.operator(ch, 3, 0.0, lfo);
                    (b, c, d, b + c + d)
                }
                _ => {
                    let b = self.operator(ch, 1, 0.0, lfo);
                    let c = self.operator(ch, 2, 0.0, lfo);
                    let d = self.operator(ch, 3, 0.0, lfo);
                    (b, c, d, a + b + c + d)
                }
            };
            let _ = (b, c, d);
            let value = if ch == 5 && self.registers[0][0x2b] & 128 != 0 {
                (f64::from(self.registers[0][0x2a]) - 128.0) / 128.0
            } else {
                result * 0.25
            };
            let pan = self.registers[ch / 3][0xb4 + ch % 3];
            if pan & 128 != 0 {
                out[0] += value / 6.0
            }
            if pan & 64 != 0 {
                out[1] += value / 6.0
            }
        }
        [
            out[0].clamp(-1.0, 1.0) as f32,
            out[1].clamp(-1.0, 1.0) as f32,
        ]
    }
}
#[derive(Clone, Debug)]
pub struct Mixer {
    pub fm: Ym2612,
    pub psg: Psg,
    pub fm_gain: f32,
    pub psg_gain: f32,
    master_hz: u32,
    sample_rate: u32,
    fm_phase: u64,
    psg_phase: u64,
}
impl Mixer {
    pub fn new(master_hz: u32, sample_rate: u32) -> Result<Self> {
        Ok(Self {
            fm: Ym2612::new(master_hz / 7, sample_rate)?,
            psg: Default::default(),
            fm_gain: 1.0,
            psg_gain: 0.25,
            master_hz,
            sample_rate,
            fm_phase: 0,
            psg_phase: 0,
        })
    }
    pub fn sample(&mut self) -> [f32; 2] {
        self.fm_phase += u64::from(self.master_hz);
        let denom = u64::from(self.sample_rate) * 7;
        self.fm.clock((self.fm_phase / denom) as u32);
        self.fm_phase %= denom;
        self.psg_phase += u64::from(self.master_hz);
        let denom = u64::from(self.sample_rate) * 15;
        self.psg.clock((self.psg_phase / denom) as u32);
        self.psg_phase %= denom;
        let fm = self.fm.sample();
        let psg = self.psg.sample() * self.psg_gain;
        [
            (fm[0] * self.fm_gain + psg).clamp(-1.0, 1.0),
            (fm[1] * self.fm_gain + psg).clamp(-1.0, 1.0),
        ]
    }
}
