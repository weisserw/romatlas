//! Independently authored MIT HuCard native helpers. No emulator code/dependency.
//! Call scanline and register writes at game-owned raster/audio scheduling points.
use anyhow::{Result, ensure};
pub const WIDTH: u32 = 256;
pub const HEIGHT: u32 = 240;
pub fn default_settings() -> crate::settings::Settings {
    let mut s = crate::settings::Settings {
        width: WIDTH * 3,
        height: HEIGHT * 3,
        ..Default::default()
    };
    for (old, new) in [("a", "i"), ("b", "ii"), ("start", "run")] {
        if let Some(v) = s.bindings.remove(old) {
            s.bindings.insert(new.into(), v);
        }
    }
    s
}
/// VCE GRB333, converted linearly; greyscale suppresses chroma using luma.
pub fn color(v: u16, greyscale: bool) -> [u8; 4] {
    let expand = |n: u16| ((n & 7) * 255 / 7) as u8;
    let (r, g, b) = (expand(v >> 3), expand(v >> 6), expand(v));
    if greyscale {
        let y = ((u32::from(r) * 299 + u32::from(g) * 587 + u32::from(b) * 114) / 1000) as u8;
        [y, y, y, 255]
    } else {
        [r, g, b, 255]
    }
}
/// BG tile: eight little-endian plane-0/1 row words, then plane-2/3 words.
pub fn tile(bytes: &[u8]) -> Result<[u8; 64]> {
    ensure!(bytes.len() == 32, "PC Engine BG tile requires 32 bytes");
    let mut out = [0; 64];
    for y in 0..8 {
        for x in 0..8 {
            for p in 0..4 {
                out[y * 8 + x] |= ((bytes[(p / 2) * 16 + y * 2 + p % 2] >> (7 - x)) & 1) << p;
            }
        }
    }
    Ok(out)
}
/// Sprite cell: four 16-word planes, 16 pixels per row.
pub fn sprite_tile(bytes: &[u8]) -> Result<[u8; 256]> {
    ensure!(
        bytes.len() == 128,
        "PC Engine sprite cell requires 128 bytes"
    );
    let mut out = [0; 256];
    for y in 0..16 {
        for x in 0..16 {
            for p in 0..4 {
                let at = p * 32 + y * 2;
                let word = u16::from_le_bytes([bytes[at], bytes[at + 1]]);
                out[y * 16 + x] |= (((word >> (15 - x)) & 1) as u8) << p;
            }
        }
    }
    Ok(out)
}
#[derive(Clone, Debug)]
pub struct Vce {
    pub palette: [u16; 512],
    pub control: u8,
    pub address: u16,
}
impl Default for Vce {
    fn default() -> Self {
        Self {
            palette: [0; 512],
            control: 0,
            address: 0,
        }
    }
}
impl Vce {
    pub fn write(&mut self, port: u8, v: u8) {
        match port & 7 {
            0 => self.control = v,
            2 => self.address = (self.address & 256) | u16::from(v),
            3 => self.address = (self.address & 255) | ((u16::from(v) & 1) << 8),
            4 => {
                let n = self.address as usize;
                self.palette[n] = (self.palette[n] & 256) | u16::from(v);
            }
            5 => {
                let n = self.address as usize;
                self.palette[n] = (self.palette[n] & 255) | ((u16::from(v) & 1) << 8);
                self.address = (self.address + 1) & 511;
            }
            _ => {}
        }
    }
    pub fn read(&mut self, port: u8) -> u8 {
        match port & 7 {
            4 => self.palette[self.address as usize] as u8,
            5 => {
                let v = (self.palette[self.address as usize] >> 8) as u8 | 0xfe;
                self.address = (self.address + 1) & 511;
                v
            }
            _ => 0xff,
        }
    }
    pub fn rgba(&self, index: u16) -> [u8; 4] {
        color(
            self.palette[(index & 511) as usize],
            self.control & 128 != 0,
        )
    }
    pub fn dot_divider(&self) -> u32 {
        match self.control & 3 {
            0 => 4,
            1 => 3,
            _ => 2,
        }
    }
    pub fn tick_rate(&self) -> crate::timing::TickRate {
        crate::timing::TickRate {
            numerator: 236_250_000,
            denominator: 11 * 1365 * (262 + u32::from(self.control & 4 != 0)),
        }
    }
}
#[derive(Clone, Debug)]
pub struct Vdc {
    pub vram: Vec<u16>,
    pub satb: [u16; 256],
    pub registers: [u16; 32],
    pub selected: u8,
    pub status: u8,
    low_latch: u8,
    read_latch: u16,
    satb_pending: bool,
}
impl Default for Vdc {
    fn default() -> Self {
        Self {
            vram: vec![0; 32768],
            satb: [0; 256],
            registers: [0; 32],
            selected: 0,
            status: 0,
            low_latch: 0,
            read_latch: 0,
            satb_pending: false,
        }
    }
}
#[derive(Clone, Debug)]
pub struct Scanline {
    pub indices: Vec<u16>,
    pub sprite_collision: bool,
    pub sprite_overflow: bool,
    pub raster_irq: bool,
}
impl Vdc {
    fn increment(&self) -> u16 {
        [1, 32, 64, 128][((self.registers[5] >> 11) & 3) as usize]
    }
    fn read_word(&self, at: u16) -> u16 {
        self.vram.get(at as usize).copied().unwrap_or(0xffff)
    }
    pub fn read(&mut self, port: u8) -> u8 {
        match port & 3 {
            0 => {
                let v = self.status;
                self.status = 0;
                v
            }
            2 => self.read_latch as u8,
            3 => {
                let v = (self.read_latch >> 8) as u8;
                self.registers[1] = self.registers[1].wrapping_add(self.increment());
                self.read_latch = self.read_word(self.registers[1]);
                v
            }
            _ => 0xff,
        }
    }
    pub fn write(&mut self, port: u8, v: u8) {
        match port & 3 {
            0 => self.selected = v & 31,
            2 => {
                let n = self.selected as usize;
                if n == 2 {
                    self.low_latch = v;
                } else {
                    self.registers[n] = (self.registers[n] & 0xff00) | u16::from(v);
                }
            }
            3 => {
                let n = self.selected as usize;
                let low = if n == 2 {
                    self.low_latch
                } else {
                    self.registers[n] as u8
                };
                let value = u16::from_le_bytes([low, v]);
                self.registers[n] = value;
                match n {
                    1 => self.read_latch = self.read_word(value),
                    2 => {
                        let at = self.registers[0] as usize;
                        if at < 32768 {
                            self.vram[at] = value;
                        }
                        self.registers[0] = self.registers[0].wrapping_add(self.increment());
                    }
                    18 => self.vram_dma(),
                    19 => self.satb_pending = true,
                    _ => {}
                }
            }
            _ => {}
        }
    }
    /// Complete scheduled VRAM DMA; ports decide when CPU/video can observe completion.
    pub fn vram_dma(&mut self) {
        let source_step = if self.registers[15] & 4 != 0 {
            u16::MAX
        } else {
            1
        };
        let dest_step = if self.registers[15] & 8 != 0 {
            u16::MAX
        } else {
            1
        };
        let count = u32::from(self.registers[18]) + 1;
        for _ in 0..count {
            let value = self.read_word(self.registers[16]);
            let dst = self.registers[17] as usize;
            if dst < 32768 {
                self.vram[dst] = value;
            }
            self.registers[16] = self.registers[16].wrapping_add(source_step);
            self.registers[17] = self.registers[17].wrapping_add(dest_step);
        }
        self.registers[18] = 0xffff;
        self.status |= 16;
    }
    pub fn vblank(&mut self) {
        self.status |= 32;
        if self.satb_pending || self.registers[15] & 16 != 0 {
            for n in 0..256 {
                self.satb[n] = self.read_word(self.registers[19].wrapping_add(n as u16));
            }
            self.satb_pending = false;
            self.status |= 8;
        }
    }
    pub fn irq_pending(&self) -> bool {
        let cr = self.registers[5];
        let dcr = self.registers[15];
        self.status & 1 != 0 && cr & 1 != 0
            || self.status & 2 != 0 && cr & 2 != 0
            || self.status & 4 != 0 && cr & 4 != 0
            || self.status & 32 != 0 && cr & 8 != 0
            || self.status & 8 != 0 && dcr & 1 != 0
            || self.status & 16 != 0 && dcr & 2 != 0
    }
    /// Active-display line. Call between writes for split scroll/palette/raster effects.
    pub fn scanline(&mut self, y: u16, width: usize) -> Result<Scanline> {
        ensure!((1..=1024).contains(&width), "VDC line width outside bounds");
        let cr = self.registers[5];
        let mwr = self.registers[9];
        let map_w = match (mwr >> 4) & 3 {
            0 => 32,
            1 => 64,
            _ => 128,
        };
        let map_h = if mwr & 64 != 0 { 64 } else { 32 };
        let mut indices = vec![0; width];
        let mut opaque = vec![false; width];
        if cr & 128 != 0 {
            for x in 0..width {
                let px = (x + usize::from(self.registers[7] & 1023)) % (map_w * 8);
                let py = (usize::from(y) + usize::from(self.registers[8] & 511)) % (map_h * 8);
                let attr = self.vram[((py / 8) * map_w + px / 8) & 32767];
                let at = ((attr as usize & 4095) * 16 + (py & 7)) & 32767;
                let mut a = self.vram[at];
                let mut b = self.vram[(at + 8) & 32767];
                if mwr & 3 == 3 {
                    if mwr & 128 == 0 {
                        b = 0;
                    } else {
                        a = 0;
                    }
                }
                let shift = 7 - (px & 7);
                let pixel = ((a >> shift) & 1)
                    | (((a >> (shift + 8)) & 1) << 1)
                    | (((b >> shift) & 1) << 2)
                    | (((b >> (shift + 8)) & 1) << 3);
                opaque[x] = pixel != 0;
                if pixel != 0 {
                    indices[x] = (attr >> 12) * 16 + pixel;
                }
            }
        }
        let mut owner = vec![None; width];
        let mut collision = false;
        let mut overflow = false;
        let mut cells = 0;
        if cr & 64 != 0 {
            for n in 0..64 {
                let at = n * 4;
                let sy = i32::from(self.satb[at] & 1023) - 64;
                let sx = i32::from(self.satb[at + 1] & 1023) - 32;
                let pattern = self.satb[at + 2];
                let flags = self.satb[at + 3];
                let wide = if flags & 256 != 0 { 32 } else { 16 };
                let high = match (flags >> 12) & 3 {
                    0 => 16,
                    1 => 32,
                    _ => 64,
                };
                let mut row = i32::from(y) - sy;
                if row < 0 || row >= high {
                    continue;
                }
                let available = (16 - cells).max(0);
                cells += wide / 16;
                if cells > 16 {
                    overflow = true;
                }
                if available == 0 {
                    continue;
                }
                let visible_columns = wide.min(available * 16);
                if flags & 32768 != 0 {
                    row = high - 1 - row;
                }
                let mut base = usize::from((pattern >> 1) & 1023);
                if wide == 32 {
                    base &= !1;
                }
                if high >= 32 {
                    base &= !2;
                }
                if high == 64 {
                    base &= !6;
                }
                for col in 0..visible_columns {
                    let x = sx + col;
                    if x < 0 || x >= width as i32 {
                        continue;
                    }
                    let cx = if flags & 2048 != 0 {
                        wide - 1 - col
                    } else {
                        col
                    };
                    let cell = base + (row as usize / 16) * 2 + cx as usize / 16;
                    let mut pixel = 0;
                    for p in 0..4 {
                        if (mwr >> 2) & 3 >= 2 && (p / 2) != usize::from(pattern & 1) {
                            continue;
                        }
                        let addr = (cell * 64 + p * 16 + row as usize % 16) & 32767;
                        pixel |= ((self.vram[addr] >> (15 - cx as usize % 16)) & 1) << p;
                    }
                    if pixel == 0 {
                        continue;
                    }
                    let x = x as usize;
                    if let Some(old) = owner[x] {
                        if old == 0 {
                            collision = true;
                        }
                        continue;
                    }
                    owner[x] = Some(n);
                    if flags & 128 != 0 || !opaque[x] {
                        indices[x] = 256 + (flags & 15) * 16 + pixel;
                    }
                }
            }
        }
        let raster_irq = self.registers[6] == y.wrapping_add(64);
        if raster_irq {
            self.status |= 4;
        }
        if collision {
            self.status |= 1;
        }
        if overflow {
            self.status |= 2;
        }
        Ok(Scanline {
            indices,
            sprite_collision: collision,
            sprite_overflow: overflow,
            raster_irq,
        })
    }
}

/// Six independent 32x5-bit wave channels, DDA, stereo attenuation, noise and LFO.
#[derive(Clone, Debug)]
pub struct Channel {
    pub wave: [u8; 32],
    pub frequency: u16,
    pub control: u8,
    pub balance: u8,
    pub noise: u8,
    pub dda: u8,
    index: u8,
    write_index: u8,
    counter: u32,
    noise_counter: u32,
    lfsr: u32,
}
impl Default for Channel {
    fn default() -> Self {
        Self {
            wave: [0; 32],
            frequency: 0,
            control: 0,
            balance: 0,
            noise: 0,
            dda: 0,
            index: 0,
            write_index: 0,
            counter: 0,
            noise_counter: 0,
            lfsr: 1,
        }
    }
}
#[derive(Clone, Debug, Default)]
pub struct Psg {
    pub channels: [Channel; 6],
    pub selected: u8,
    pub balance: u8,
    pub lfo_frequency: u8,
    pub lfo_control: u8,
}
impl Psg {
    pub fn write(&mut self, port: u8, value: u8) {
        match port & 15 {
            0 => self.selected = value & 7,
            1 => self.balance = value,
            8 => self.lfo_frequency = value,
            9 => {
                self.lfo_control = value;
                if value & 128 != 0 {
                    self.channels[1].index = 0;
                    self.channels[1].counter = 0;
                }
            }
            2..=7 => {
                let Some(c) = self.channels.get_mut(self.selected as usize) else {
                    return;
                };
                match port & 15 {
                    2 => c.frequency = (c.frequency & 0xf00) | u16::from(value),
                    3 => c.frequency = (c.frequency & 255) | ((u16::from(value) & 15) << 8),
                    4 => {
                        if c.control & 64 != 0 && value & 64 == 0 {
                            c.write_index = 0;
                            c.index = 0;
                        }
                        c.control = value;
                    }
                    5 => c.balance = value,
                    6 => {
                        c.dda = value & 31;
                        if c.control & 64 == 0 {
                            c.wave[c.write_index as usize] = value & 31;
                            if c.control & 128 == 0 {
                                c.write_index = (c.write_index + 1) & 31;
                            }
                        }
                    }
                    7 => c.noise = value,
                    _ => {}
                }
            }
            _ => {}
        }
    }
    /// PSG input clock (~3.58 MHz), independent of CPU CSL/CSH selection.
    pub fn clock(&mut self, ticks: u32) {
        for _ in 0..ticks {
            let modulation = if self.lfo_control & 3 != 0 && self.lfo_control & 128 == 0 {
                let value = i32::from(self.channels[1].wave[self.channels[1].index as usize]) - 16;
                value
                    << match self.lfo_control & 3 {
                        1 => 0,
                        2 => 4,
                        _ => 8,
                    }
            } else {
                0
            };
            for (n, c) in self.channels.iter_mut().enumerate() {
                if n >= 4 && c.noise & 128 != 0 {
                    if c.noise_counter == 0 {
                        c.noise_counter = u32::from(31 - (c.noise & 31)).max(1) * 64;
                        let feedback =
                            (c.lfsr ^ (c.lfsr >> 1) ^ (c.lfsr >> 11) ^ (c.lfsr >> 12)) & 1;
                        c.lfsr = (c.lfsr >> 1) | (feedback << 17);
                    }
                    c.noise_counter -= 1;
                }
                let lfo = n == 1 && self.lfo_control & 3 != 0;
                if c.control & 64 != 0 || (c.control & 128 == 0 && !lfo) {
                    continue;
                }
                if lfo && self.lfo_control & 128 != 0 {
                    continue;
                }
                if c.counter == 0 {
                    let frequency = if n == 0 {
                        (i32::from(c.frequency) + modulation) & 4095
                    } else {
                        i32::from(c.frequency)
                    };
                    let period = if frequency == 0 {
                        4096
                    } else {
                        frequency as u32
                    };
                    c.counter = period
                        * if lfo {
                            u32::from(self.lfo_frequency).max(1)
                        } else {
                            1
                        };
                    c.index = (c.index + 1) & 31;
                }
                c.counter -= 1;
            }
        }
    }
    pub fn output(&self) -> [f32; 2] {
        let mut out = [0.0; 2];
        for (n, c) in self.channels.iter().enumerate() {
            if c.control & 128 == 0 || n == 1 && self.lfo_control & 3 != 0 {
                continue;
            }
            let value = if c.control & 64 != 0 {
                c.dda
            } else if n >= 4 && c.noise & 128 != 0 {
                if c.lfsr & 1 != 0 { 31 } else { 0 }
            } else {
                c.wave[c.index as usize]
            };
            for (side, sample) in out.iter_mut().enumerate() {
                let shift = if side == 0 { 4 } else { 0 };
                let global = (self.balance >> shift) & 15;
                let balance = (c.balance >> shift) & 15;
                let attenuation = (31 - (c.control & 31)) as u32
                    + u32::from(15 - global) * 2
                    + u32::from(15 - balance) * 2;
                if attenuation < 62 {
                    *sample += (f32::from(value) - 15.5) / 15.5
                        * 10.0_f32.powf(-(attenuation as f32) * 1.5 / 20.0)
                        / 6.0;
                }
            }
        }
        out
    }
}
#[derive(Clone, Debug)]
pub struct Mixer {
    pub psg: Psg,
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
            clock_hz,
            sample_rate,
            phase: 0,
        })
    }
    pub fn sample(&mut self) -> [f32; 2] {
        self.phase += u64::from(self.clock_hz);
        let ticks = self.phase / u64::from(self.sample_rate);
        self.phase %= u64::from(self.sample_rate);
        self.psg.clock(ticks as u32);
        self.psg.output()
    }
}
