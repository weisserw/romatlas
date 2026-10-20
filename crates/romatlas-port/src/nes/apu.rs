//! Independent NTSC 2A03 sound primitives for native ports. CPU-clocked pulse,
//! triangle, noise and DMC, register writes, envelopes, sweep, length/linear
//! counters and nonlinear mixing. No CPU/bus emulator or expansion audio.
//! DMC reads use a caller callback; DMA stalls are returned as read requests,
//! leaving game/driver scheduling to the port. Analog filtering is caller-owned.
const CPU_HZ: f64 = 1_789_773.0;
const LENGTH: [u8; 32] = [
    10, 254, 20, 2, 40, 4, 80, 6, 160, 8, 60, 10, 14, 12, 26, 14, 12, 16, 24, 18, 48, 20, 96, 22,
    192, 24, 72, 26, 16, 28, 32, 30,
];
const NOISE: [u16; 16] = [
    4, 8, 16, 32, 64, 96, 128, 160, 202, 254, 380, 508, 762, 1016, 2034, 4068,
];
const DMC: [u16; 16] = [
    428, 380, 340, 320, 286, 254, 226, 214, 190, 160, 142, 128, 106, 84, 72, 54,
];
const DUTY: [[u8; 8]; 4] = [
    [0, 1, 0, 0, 0, 0, 0, 0],
    [0, 1, 1, 0, 0, 0, 0, 0],
    [0, 1, 1, 1, 1, 0, 0, 0],
    [1, 0, 0, 1, 1, 1, 1, 1],
];
#[derive(Clone, Default, Debug)]
struct Envelope {
    start: bool,
    divider: u8,
    decay: u8,
}
impl Envelope {
    fn clock(&mut self, control: u8) {
        if self.start {
            self.start = false;
            self.decay = 15;
            self.divider = control & 15;
        } else if self.divider != 0 {
            self.divider -= 1;
        } else {
            self.divider = control & 15;
            if self.decay != 0 {
                self.decay -= 1;
            } else if control & 32 != 0 {
                self.decay = 15;
            }
        }
    }
    fn output(&self, control: u8) -> u8 {
        if control & 16 != 0 {
            control & 15
        } else {
            self.decay
        }
    }
}
#[derive(Clone, Default, Debug)]
struct Pulse {
    control: u8,
    sweep: u8,
    period: u16,
    timer: u16,
    phase: u8,
    length: u8,
    envelope: Envelope,
    sweep_divider: u8,
    sweep_reload: bool,
}
impl Pulse {
    fn target(&self, first: bool) -> i32 {
        let delta = i32::from(self.period >> (self.sweep & 7));
        if self.sweep & 8 != 0 {
            i32::from(self.period) - delta - i32::from(first)
        } else {
            i32::from(self.period) + delta
        }
    }
    fn half(&mut self, first: bool) {
        if self.control & 32 == 0 {
            self.length = self.length.saturating_sub(1);
        }
        let target = self.target(first);
        if self.sweep_divider == 0
            && self.sweep & 128 != 0
            && self.sweep & 7 != 0
            && self.period >= 8
            && (0..=2047).contains(&target)
        {
            self.period = target as u16;
        }
        if self.sweep_divider == 0 || self.sweep_reload {
            self.sweep_divider = (self.sweep >> 4) & 7;
            self.sweep_reload = false;
        } else {
            self.sweep_divider -= 1;
        }
    }
    fn tick(&mut self) {
        if self.timer == 0 {
            self.timer = self.period;
            self.phase = (self.phase + 1) & 7;
        } else {
            self.timer -= 1;
        }
    }
    fn output(&self, first: bool) -> u8 {
        if self.length == 0
            || self.period < 8
            || self.target(first) > 2047
            || DUTY[usize::from(self.control >> 6)][usize::from(self.phase)] == 0
        {
            0
        } else {
            self.envelope.output(self.control)
        }
    }
}
#[derive(Clone, Debug)]
pub struct Apu {
    pulse: [Pulse; 2],
    regs: [u8; 24],
    enabled: u8,
    triangle_length: u8,
    linear: u8,
    linear_reload: bool,
    triangle_timer: u16,
    triangle_phase: u8,
    noise_length: u8,
    noise_timer: u16,
    noise_shift: u16,
    noise_envelope: Envelope,
    dmc_timer: u16,
    dmc_level: u8,
    dmc_address: u16,
    dmc_remaining: u16,
    dmc_shift: u8,
    dmc_bits: u8,
    dmc_buffer: Option<u8>,
    dmc_silent: bool,
    cycles: u64,
    frame_cycle: u32,
    frame_mode: u8,
    frame_reset: Option<u8>,
    frame_irq: bool,
    dmc_irq: bool,
    sample_phase: f64,
}
impl Default for Apu {
    fn default() -> Self {
        Self {
            pulse: Default::default(),
            regs: [0; 24],
            enabled: 0,
            triangle_length: 0,
            linear: 0,
            linear_reload: false,
            triangle_timer: 0,
            triangle_phase: 0,
            noise_length: 0,
            noise_timer: 0,
            noise_shift: 1,
            noise_envelope: Default::default(),
            dmc_timer: 0,
            dmc_level: 0,
            dmc_address: 0xc000,
            dmc_remaining: 0,
            dmc_shift: 0,
            dmc_bits: 8,
            dmc_buffer: None,
            dmc_silent: true,
            cycles: 0,
            frame_cycle: 0,
            frame_mode: 0,
            frame_reset: None,
            frame_irq: false,
            dmc_irq: false,
            sample_phase: 0.0,
        }
    }
}
impl Apu {
    /// Register writes use CPU addresses $4000..$4017; joypad registers are rejected.
    pub fn write(&mut self, address: u16, value: u8) -> anyhow::Result<()> {
        anyhow::ensure!(
            (0x4000..=0x4017).contains(&address)
                && !matches!(address, 0x4009 | 0x400d | 0x4014 | 0x4016),
            "unsupported APU register"
        );
        let at = usize::from(address - 0x4000);
        self.regs[at] = value;
        match at {
            0..=7 => {
                let i = at / 4;
                let p = &mut self.pulse[i];
                match at & 3 {
                    0 => p.control = value,
                    1 => {
                        p.sweep = value;
                        p.sweep_reload = true;
                    }
                    2 => p.period = (p.period & 0x700) | u16::from(value),
                    3 => {
                        p.period = (p.period & 255) | (u16::from(value & 7) << 8);
                        if self.enabled & (1 << i) != 0 {
                            p.length = LENGTH[usize::from(value >> 3)];
                        }
                        p.phase = 0;
                        p.envelope.start = true;
                    }
                    _ => unreachable!(),
                }
            }
            11 => {
                if self.enabled & 4 != 0 {
                    self.triangle_length = LENGTH[usize::from(value >> 3)];
                }
                self.linear_reload = true;
            }
            15 => {
                if self.enabled & 8 != 0 {
                    self.noise_length = LENGTH[usize::from(value >> 3)];
                }
                self.noise_envelope.start = true;
            }
            16 => {
                if value & 128 == 0 {
                    self.dmc_irq = false;
                }
            }
            17 => self.dmc_level = value & 127,
            21 => {
                self.enabled = value & 31;
                for i in 0..2 {
                    if value & (1 << i) == 0 {
                        self.pulse[i].length = 0;
                    }
                }
                if value & 4 == 0 {
                    self.triangle_length = 0;
                }
                if value & 8 == 0 {
                    self.noise_length = 0;
                }
                if value & 16 == 0 {
                    self.dmc_remaining = 0;
                } else if self.dmc_remaining == 0 {
                    self.restart_dmc();
                }
                self.dmc_irq = false;
            }
            23 => {
                self.frame_mode = value;
                if value & 64 != 0 {
                    self.frame_irq = false;
                }
                self.frame_reset = Some(if self.cycles & 1 == 0 { 4 } else { 3 });
            }
            _ => {}
        }
        Ok(())
    }
    fn restart_dmc(&mut self) {
        self.dmc_address = 0xc000 + u16::from(self.regs[18]) * 64;
        self.dmc_remaining = u16::from(self.regs[19]) * 16 + 1;
    }
    fn quarter(&mut self) {
        for p in &mut self.pulse {
            p.envelope.clock(p.control);
        }
        self.noise_envelope.clock(self.regs[12]);
        if self.linear_reload {
            self.linear = self.regs[8] & 127;
        } else {
            self.linear = self.linear.saturating_sub(1);
        }
        if self.regs[8] & 128 == 0 {
            self.linear_reload = false;
        }
    }
    fn half(&mut self) {
        for (i, p) in self.pulse.iter_mut().enumerate() {
            p.half(i == 0);
        }
        if self.regs[8] & 128 == 0 {
            self.triangle_length = self.triangle_length.saturating_sub(1);
        }
        if self.regs[12] & 32 == 0 {
            self.noise_length = self.noise_length.saturating_sub(1);
        }
    }
    /// Advance one NTSC CPU clock. Returns the DMC memory address fetched on this
    /// tick, allowing the caller to account for timing without embedding a CPU.
    pub fn clock(&mut self, read: &mut impl FnMut(u16) -> u8) -> Option<u16> {
        self.cycles = self.cycles.wrapping_add(1);
        self.frame_cycle += 1;
        if let Some(delay) = self.frame_reset {
            if delay == 1 {
                self.frame_cycle = 0;
                self.frame_reset = None;
                if self.frame_mode & 128 != 0 {
                    self.quarter();
                    self.half();
                }
            } else {
                self.frame_reset = Some(delay - 1);
            }
        }
        match self.frame_cycle {
            7457 | 22371 => self.quarter(),
            14913 => {
                self.quarter();
                self.half();
            }
            29829 if self.frame_mode & 128 == 0 => {
                self.quarter();
                self.half();
            }
            37281 if self.frame_mode & 128 != 0 => {
                self.quarter();
                self.half();
            }
            _ => {}
        }
        if self.frame_mode & 0xc0 == 0 && (29828..=29830).contains(&self.frame_cycle) {
            self.frame_irq = true;
        }
        if self.frame_cycle
            >= if self.frame_mode & 128 != 0 {
                37282
            } else {
                29830
            }
        {
            self.frame_cycle = 0;
        }
        if self.cycles & 1 == 0 {
            for p in &mut self.pulse {
                p.tick();
            }
        }
        let tri_period = u16::from(self.regs[10]) | (u16::from(self.regs[11] & 7) << 8);
        if self.triangle_timer == 0 {
            self.triangle_timer = tri_period;
            if self.triangle_length != 0 && self.linear != 0 {
                self.triangle_phase = (self.triangle_phase + 1) & 31;
            }
        } else {
            self.triangle_timer -= 1;
        }
        if self.noise_timer == 0 {
            self.noise_timer = NOISE[usize::from(self.regs[14] & 15)] - 1;
            let tap = if self.regs[14] & 128 != 0 { 6 } else { 1 };
            let feedback = (self.noise_shift ^ (self.noise_shift >> tap)) & 1;
            self.noise_shift = (self.noise_shift >> 1) | (feedback << 14);
        } else {
            self.noise_timer -= 1;
        }
        let mut fetched = None;
        if self.dmc_buffer.is_none() && self.dmc_remaining != 0 {
            let addr = self.dmc_address;
            self.dmc_buffer = Some(read(addr));
            fetched = Some(addr);
            self.dmc_address = if addr == 0xffff { 0x8000 } else { addr + 1 };
            self.dmc_remaining -= 1;
            if self.dmc_remaining == 0 {
                if self.regs[16] & 64 != 0 {
                    self.restart_dmc();
                } else if self.regs[16] & 128 != 0 {
                    self.dmc_irq = true;
                }
            }
        }
        if self.dmc_timer == 0 {
            self.dmc_timer = DMC[usize::from(self.regs[16] & 15)] - 1;
            if !self.dmc_silent {
                if self.dmc_shift & 1 != 0 {
                    if self.dmc_level <= 125 {
                        self.dmc_level += 2;
                    }
                } else if self.dmc_level >= 2 {
                    self.dmc_level -= 2;
                }
            }
            self.dmc_shift >>= 1;
            self.dmc_bits -= 1;
            if self.dmc_bits == 0 {
                self.dmc_bits = 8;
                if let Some(byte) = self.dmc_buffer.take() {
                    self.dmc_shift = byte;
                    self.dmc_silent = false;
                } else {
                    self.dmc_silent = true;
                }
            }
        } else {
            self.dmc_timer -= 1;
        }
        fetched
    }
    /// Reading status clears only the frame IRQ, matching the driver-visible latch.
    pub fn status(&mut self) -> u8 {
        let v = u8::from(self.pulse[0].length != 0)
            | u8::from(self.pulse[1].length != 0) << 1
            | u8::from(self.triangle_length != 0) << 2
            | u8::from(self.noise_length != 0) << 3
            | u8::from(self.dmc_remaining != 0) << 4
            | u8::from(self.frame_irq) << 6
            | u8::from(self.dmc_irq) << 7;
        self.frame_irq = false;
        v
    }
    pub fn irq(&self) -> bool {
        self.frame_irq || self.dmc_irq
    }
    /// Raw DAC values: pulse 1, pulse 2, triangle, noise, DMC. Triangle holds its
    /// current level when stopped, as the hardware DAC does.
    pub fn levels(&self) -> [u8; 5] {
        let phase = self.triangle_phase;
        [
            self.pulse[0].output(true),
            self.pulse[1].output(false),
            if phase < 16 { 15 - phase } else { phase - 16 },
            if self.noise_length != 0 && self.noise_shift & 1 == 0 {
                self.noise_envelope.output(self.regs[12])
            } else {
                0
            },
            self.dmc_level,
        ]
    }
    pub fn output(&self) -> f32 {
        mix(self.levels())
    }
    /// Append interleaved stereo PCM. The port schedules register changes between
    /// calls (or uses clock for sub-sample changes). Box averaging limits aliasing;
    /// this helper does not model the analog high/low-pass output circuit.
    pub fn render(
        &mut self,
        frames: usize,
        sample_rate: u32,
        mut read: impl FnMut(u16) -> u8,
        out: &mut Vec<f32>,
    ) -> anyhow::Result<()> {
        anyhow::ensure!(
            (8000..=192000).contains(&sample_rate) && frames <= 1_000_000,
            "audio output limits"
        );
        for _ in 0..frames {
            self.sample_phase += CPU_HZ / f64::from(sample_rate);
            let ticks = self.sample_phase.floor() as usize;
            self.sample_phase -= ticks as f64;
            let mut sum = 0.0;
            for _ in 0..ticks {
                self.clock(&mut read);
                sum += self.output();
            }
            let sample = sum / ticks as f32;
            out.extend_from_slice(&[sample, sample]);
        }
        Ok(())
    }
}
/// NES pulse and triangle/noise/DMC nonlinear DAC approximation.
pub fn mix([p1, p2, t, n, d]: [u8; 5]) -> f32 {
    let pulse = f32::from(p1.min(15) + p2.min(15));
    let tnd = f32::from(t.min(15)) / 8227.0
        + f32::from(n.min(15)) / 12241.0
        + f32::from(d.min(127)) / 22638.0;
    (if pulse == 0.0 {
        0.0
    } else {
        95.88 / (8128.0 / pulse + 100.0)
    }) + (if tnd == 0.0 {
        0.0
    } else {
        159.79 / (1.0 / tnd + 100.0)
    })
}
