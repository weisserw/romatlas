//! Selected Game Boy register-driven sound generation, without a CPU or audio device.
//! This is a native-port component, not a claim of cycle-exact APU emulation.
#[derive(Clone)]
pub struct Sound {
    registers: [u8; 48],
    active: [bool; 4],
    length: [u16; 4],
    volume: [u8; 4],
    envelope_ticks: [u8; 4],
    timers: [u32; 4],
    phase: [u8; 4],
    wave_buffer: u8,
    noise: u16,
    frame_timer: u32,
    frame_step: u8,
    sample_remainder: u64,
    capacitor: [f32; 2],
    sweep_shadow: u16,
    sweep_timer: u8,
    sweep_enabled: bool,
    sweep_subtracted: bool,
}
impl Default for Sound {
    fn default() -> Self {
        Self {
            registers: [0; 48],
            active: [false; 4],
            length: [0; 4],
            volume: [0; 4],
            envelope_ticks: [0; 4],
            timers: [1; 4],
            phase: [0; 4],
            wave_buffer: 0,
            noise: 0,
            frame_timer: 8192,
            frame_step: 0,
            sample_remainder: 0,
            capacitor: [0.; 2],
            sweep_shadow: 0,
            sweep_timer: 0,
            sweep_enabled: false,
            sweep_subtracted: false,
        }
    }
}
impl Sound {
    /// Address is the low byte of FF10..FF3F. Unused addresses are ignored.
    pub fn write(&mut self, address: u8, value: u8) {
        if !(0x10..=0x3f).contains(&address) {
            return;
        }
        if address == 0x26 {
            let enabled = value & 128 != 0;
            if !enabled {
                *self = Self::default();
            }
            self.registers[22] = if enabled { 128 } else { 0 };
            return;
        }
        if address < 0x30 && self.registers[22] & 128 == 0 {
            return;
        }
        let previous = self.registers[(address - 16) as usize];
        self.registers[(address - 16) as usize] = value;
        if matches!(address, 0x14 | 0x19 | 0x1e | 0x23)
            && previous & 64 == 0
            && value & 64 != 0
            && self.frame_step & 1 != 0
        {
            let channel = match address {
                0x14 => 0,
                0x19 => 1,
                0x1e => 2,
                _ => 3,
            };
            if self.length[channel] != 0 {
                self.length[channel] -= 1;
                if self.length[channel] == 0 {
                    self.active[channel] = false;
                }
            }
        }
        match address {
            0x10 if previous & 8 != 0 && value & 8 == 0 && self.sweep_subtracted => {
                self.active[0] = false
            }
            0x11 => self.length[0] = 64 - u16::from(value & 63),
            0x16 => self.length[1] = 64 - u16::from(value & 63),
            0x1b => self.length[2] = 256 - u16::from(value),
            0x20 => self.length[3] = 64 - u16::from(value & 63),
            0x12 | 0x17 | 0x21 => {
                let channel = match address {
                    0x12 => 0,
                    0x17 => 1,
                    _ => 3,
                };
                if value & 0xf8 == 0 {
                    self.active[channel] = false;
                }
            }
            0x1a if value & 128 == 0 => self.active[2] = false,
            0x14 | 0x19 | 0x1e | 0x23 if value & 128 != 0 => {
                let channel = match address {
                    0x14 => 0,
                    0x19 => 1,
                    0x1e => 2,
                    _ => 3,
                };
                self.active[channel] = self.dac(channel);
                if self.length[channel] == 0 {
                    self.length[channel] = if channel == 2 { 256 } else { 64 };
                    if value & 64 != 0 && self.frame_step & 1 != 0 {
                        self.length[channel] -= 1;
                    }
                }
                self.timers[channel] = self.period(channel);
                if channel == 2 {
                    self.phase[2] = 0;
                } else {
                    let envelope = self.envelope(channel);
                    self.volume[channel] = envelope >> 4;
                    self.envelope_ticks[channel] =
                        (envelope & 7).max(8 * u8::from(envelope & 7 == 0));
                    if channel == 3 {
                        self.noise = 0;
                    }
                }
                if channel == 0 {
                    self.sweep_shadow =
                        u16::from(self.registers[3]) | (u16::from(self.registers[4] & 7) << 8);
                    self.sweep_timer = ((self.registers[0] >> 4) & 7)
                        .max(8 * u8::from(self.registers[0] & 0x70 == 0));
                    self.sweep_enabled = self.registers[0] & 0x77 != 0;
                    self.sweep_subtracted = false;
                    if self.registers[0] & 7 != 0 {
                        self.calculate_sweep();
                    }
                }
            }
            _ => {}
        }
    }
    fn envelope(&self, channel: usize) -> u8 {
        self.registers[match channel {
            0 => 2,
            1 => 7,
            _ => 17,
        }]
    }
    fn dac(&self, channel: usize) -> bool {
        if channel == 2 {
            self.registers[10] & 128 != 0
        } else {
            self.envelope(channel) & 0xf8 != 0
        }
    }
    fn period(&self, channel: usize) -> u32 {
        if channel == 3 {
            let value = self.registers[18];
            let divisor = [8, 16, 32, 48, 64, 80, 96, 112][(value & 7) as usize];
            return divisor << (value >> 4);
        }
        let low = [3, 8, 13][channel];
        let frequency =
            u16::from(self.registers[low]) | (u16::from(self.registers[low + 1] & 7) << 8);
        u32::from(2048 - frequency) * if channel == 2 { 2 } else { 4 }
    }
    fn frame_clock(&mut self) {
        if self.frame_step & 1 == 0 {
            for channel in 0..4 {
                let control = self.registers[[4, 9, 14, 19][channel]];
                if control & 64 != 0 && self.length[channel] > 0 {
                    self.length[channel] -= 1;
                    if self.length[channel] == 0 {
                        self.active[channel] = false;
                    }
                }
            }
        }
        if matches!(self.frame_step, 2 | 6) {
            self.clock_sweep();
        }
        if self.frame_step == 7 {
            for channel in [0, 1, 3] {
                let envelope = self.envelope(channel);
                if envelope & 7 == 0 {
                    continue;
                }
                self.envelope_ticks[channel] = self.envelope_ticks[channel].saturating_sub(1);
                if self.envelope_ticks[channel] == 0 {
                    self.envelope_ticks[channel] = envelope & 7;
                    self.volume[channel] = if envelope & 8 != 0 {
                        self.volume[channel].saturating_add(1).min(15)
                    } else {
                        self.volume[channel].saturating_sub(1)
                    };
                }
            }
        }
        self.frame_step = (self.frame_step + 1) & 7;
    }
    fn calculate_sweep(&mut self) -> Option<u16> {
        let shifted = self.sweep_shadow >> (self.registers[0] & 7);
        let next = if self.registers[0] & 8 != 0 {
            self.sweep_subtracted = true;
            self.sweep_shadow - shifted
        } else {
            self.sweep_shadow + shifted
        };
        if next > 2047 {
            self.active[0] = false;
            None
        } else {
            Some(next)
        }
    }
    fn clock_sweep(&mut self) {
        self.sweep_timer = self.sweep_timer.saturating_sub(1);
        if self.sweep_timer != 0 {
            return;
        }
        let pace = (self.registers[0] >> 4) & 7;
        self.sweep_timer = if pace == 0 { 8 } else { pace };
        if !self.sweep_enabled || pace == 0 {
            return;
        }
        if let Some(next) = self.calculate_sweep()
            && self.registers[0] & 7 != 0
        {
            self.sweep_shadow = next;
            self.registers[3] = next as u8;
            self.registers[4] = (self.registers[4] & 0xf8) | (next >> 8) as u8;
            self.calculate_sweep(); // Second overflow check does not write back.
        }
    }
    /// Advance 4,194,304 Hz APU ticks on DMG or CGB. CGB CPU double speed does
    /// not double this clock. No host clock or device is consulted.
    pub fn advance(&mut self, cycles: u32) {
        if self.registers[22] & 128 == 0 {
            return;
        }
        for _ in 0..cycles {
            self.frame_timer -= 1;
            if self.frame_timer == 0 {
                self.frame_timer = 8192;
                self.frame_clock();
            }
            for channel in 0..4 {
                self.timers[channel] -= 1;
                if self.timers[channel] != 0 {
                    continue;
                }
                self.timers[channel] = self.period(channel);
                match channel {
                    0 | 1 => self.phase[channel] = (self.phase[channel] + 1) & 7,
                    2 => {
                        self.phase[2] = (self.phase[2] + 1) & 31;
                        let byte = self.registers[32 + usize::from(self.phase[2] / 2)];
                        self.wave_buffer = if self.phase[2] & 1 == 0 {
                            byte >> 4
                        } else {
                            byte & 15
                        };
                    }
                    _ => {
                        let bit = (!(self.noise ^ (self.noise >> 1))) & 1;
                        self.noise = (self.noise >> 1) | (bit << 14);
                        if self.registers[18] & 8 != 0 {
                            self.noise = (self.noise & !(1 << 6)) | (bit << 6);
                        }
                    }
                }
            }
        }
    }
    fn output(&self, channel: usize) -> f32 {
        if !self.dac(channel) {
            return 0.;
        }
        let digital = if !self.active[channel] {
            0
        } else {
            match channel {
                0 | 1 => {
                    let duty = self.registers[if channel == 0 { 1 } else { 6 }] >> 6;
                    let bits = [0b00000001, 0b10000001, 0b10000111, 0b01111110][duty as usize];
                    if bits & (1 << self.phase[channel]) != 0 {
                        self.volume[channel]
                    } else {
                        0
                    }
                }
                2 => match (self.registers[12] >> 5) & 3 {
                    0 => 0,
                    1 => self.wave_buffer,
                    2 => self.wave_buffer >> 1,
                    _ => self.wave_buffer >> 2,
                },
                _ => {
                    if self.noise & 1 != 0 {
                        self.volume[3]
                    } else {
                        0
                    }
                }
            }
        };
        1. - f32::from(digital) / 7.5
    }
    /// Append normalized stereo samples. The HPF is a20Hz approximation.
    pub fn render(&mut self, frames: usize, sample_rate: u32, samples: &mut Vec<f32>) {
        if sample_rate == 0 {
            samples.resize(samples.len() + frames * 2, 0.);
            return;
        }
        let retention = (-2. * std::f32::consts::PI * 20. / sample_rate as f32).exp();
        for _ in 0..frames {
            self.sample_remainder += 4_194_304;
            let cycles = self.sample_remainder / u64::from(sample_rate);
            self.sample_remainder %= u64::from(sample_rate);
            self.advance(cycles as u32);
            let route = self.registers[21];
            let master = self.registers[20];
            for side in [1, 0] {
                // Left bits4..7, then right bits0..3.
                let mut mixed = 0.;
                for channel in 0..4 {
                    if route & (1 << (channel + side * 4)) != 0 {
                        mixed += self.output(channel);
                    }
                }
                mixed *= f32::from(((master >> (side * 4)) & 7) + 1) / 32.;
                let output = mixed - self.capacitor[side];
                self.capacitor[side] = mixed - output * retention;
                samples.push(output);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn sweep_shadow_second_overflow_and_negate_clear_disable_channel() {
        let mut sound = Sound::default();
        for (address, value) in [
            (0x26, 128),
            (0x10, 0x11),
            (0x12, 0xf0),
            (0x13, 0xe8),
            (0x14, 0x83),
        ] {
            sound.write(address, value);
        }
        assert!(sound.active[0]);
        sound.advance(8192 * 3);
        assert_eq!(sound.sweep_shadow, 1500);
        assert!(!sound.active[0]); // Second calculation2250 overflows.
        sound.write(0x10, 0x19);
        sound.write(0x14, 0x83);
        assert!(sound.active[0]);
        sound.write(0x10, 0x11);
        assert!(!sound.active[0]);
    }
    #[test]
    fn enabling_length_between_length_clocks_decrements_immediately() {
        let mut sound = Sound::default();
        sound.write(0x26, 128);
        sound.advance(8192); // Next frame step1 does not clock length.
        sound.write(0x16, 63);
        sound.write(0x17, 0xf0);
        sound.write(0x19, 0x80);
        assert!(sound.active[1]);
        sound.write(0x19, 0x40);
        assert!(!sound.active[1]);
        sound.write(0x19, 0xc0);
        assert_eq!(sound.length[1], 63);
    }
    #[test]
    fn routing_dac_shutdown_and_length_expiry() {
        let mut sound = Sound::default();
        for (address, value) in [
            (0x26, 128),
            (0x24, 0x77),
            (0x25, 0x20),
            (0x16, 0xbf),
            (0x17, 0xf0),
            (0x18, 0),
            (0x19, 0xc7),
        ] {
            sound.write(address, value);
        }
        let mut samples = Vec::new();
        sound.render(40, 48000, &mut samples);
        assert!(samples.chunks_exact(2).all(|s| s[1] == 0.));
        assert!(samples.iter().any(|&s| s != 0.));
        sound.advance(8192);
        assert!(!sound.active[1]);
        sound.write(0x17, 0);
        assert_eq!(sound.output(1), 0.);
    }
    #[test]
    fn rendering_in_chunks_preserves_clock_and_samples() {
        let mut sound = Sound::default();
        for (address, value) in [
            (0x26, 128),
            (0x24, 0x77),
            (0x25, 0xff),
            (0x1a, 128),
            (0x30, 0xf0),
            (0x1c, 0x20),
            (0x1d, 0),
            (0x1e, 0x87),
        ] {
            sound.write(address, value);
        }
        let mut other = sound.clone();
        let mut once = Vec::new();
        let mut chunks = Vec::new();
        sound.render(100, 44100, &mut once);
        other.render(37, 44100, &mut chunks);
        other.render(63, 44100, &mut chunks);
        assert_eq!(once, chunks);
    }
}
