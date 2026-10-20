//! Original scanline renderer and explicit DMA/port helpers, independent of emulator code.
use super::{Brightness, color};
use anyhow::{Result, ensure};
#[derive(Clone)]
pub struct Vdp {
    pub vram: Vec<u8>,
    pub cram: [u16; 64],
    pub vsram: [u16; 40],
    pub registers: [u8; 24],
    address: u16,
    code: u8,
    first: Option<u16>,
    fill: bool,
}
#[derive(Clone, Debug)]
pub struct Line {
    pub rgba: Vec<[u8; 4]>,
    pub sprite_overflow: bool,
    pub sprite_collision: bool,
}
#[derive(Clone, Copy, Default)]
struct Pixel {
    index: u8,
    priority: bool,
}
impl Default for Vdp {
    fn default() -> Self {
        Self {
            vram: vec![0; 65536],
            cram: [0; 64],
            vsram: [0; 40],
            registers: [0; 24],
            address: 0,
            code: 0,
            first: None,
            fill: false,
        }
    }
}
impl Vdp {
    fn word(&self, a: usize) -> u16 {
        u16::from_be_bytes([self.vram[a & 65535], self.vram[(a + 1) & 65535]])
    }
    fn put_word(&mut self, a: usize, v: u16) {
        let b = v.to_be_bytes();
        self.vram[a & 65535] = b[0];
        self.vram[(a ^ 1) & 65535] = b[1];
    }
    pub fn width(&self) -> usize {
        if self.registers[12] & 1 != 0 {
            320
        } else {
            256
        }
    }
    pub fn height(&self) -> usize {
        if self.registers[1] & 8 != 0 { 240 } else { 224 }
    }
    pub fn interlace(&self) -> bool {
        self.registers[12] & 6 == 6
    }
    pub fn control(&mut self, value: u16) {
        if let Some(first) = self.first.take() {
            self.address = (first & 0x3fff) | ((value & 3) << 14);
            self.code = ((first >> 14) & 3) as u8 | ((value >> 2) & 0x3c) as u8;
            self.fill =
                self.code & 32 != 0 && self.registers[23] >> 6 == 2 && self.registers[1] & 16 != 0;
        } else if value & 0xc000 == 0x8000 {
            let r = ((value >> 8) & 31) as usize;
            if r < 24 {
                self.registers[r] = value as u8;
            }
        } else {
            self.first = Some(value);
            self.address = (self.address & 0xc000) | (value & 0x3fff);
            self.code = (self.code & !3) | ((value >> 14) as u8 & 3);
        }
    }
    fn advance(&mut self) {
        self.address = self.address.wrapping_add(u16::from(self.registers[15]));
    }
    fn write_target(&mut self, v: u16) {
        match self.code & 15 {
            1 => self.put_word(self.address as usize, v),
            3 => self.cram[(self.address as usize >> 1) & 63] = v & 0xeee,
            5 => {
                let at = (self.address as usize >> 1) & 63;
                if at < 40 {
                    self.vsram[at] = v & 0x7ff
                }
            }
            _ => {}
        }
        self.advance();
    }
    pub fn data_write(&mut self, value: u16) {
        self.first = None;
        if self.fill {
            self.put_word(self.address as usize, value);
            let count = self.dma_length();
            for _ in 0..count {
                self.vram[(self.address as usize ^ 1) & 65535] = (value >> 8) as u8;
                self.advance();
            }
            self.finish_dma();
            self.fill = false;
        } else {
            self.write_target(value)
        }
    }
    pub fn data_read(&mut self) -> u16 {
        self.first = None;
        let v = match self.code & 15 {
            0 => self.word(self.address as usize),
            8 => self.cram[(self.address as usize >> 1) & 63],
            4 => self
                .vsram
                .get((self.address as usize >> 1) & 63)
                .copied()
                .unwrap_or(0),
            _ => 0xffff,
        };
        self.advance();
        v
    }
    fn dma_length(&self) -> usize {
        let n = u16::from_le_bytes([self.registers[19], self.registers[20]]);
        if n == 0 { 65536 } else { n as usize }
    }
    fn finish_dma(&mut self) {
        self.registers[19] = 0;
        self.registers[20] = 0;
        self.code &= !32;
    }
    /// Invoke after a DMA command at the source game's scheduled raster time.
    /// Source callback reads big-endian 68000 words. Transfer timing is caller-owned.
    pub fn dma(&mut self, mut read_word: impl FnMut(u32) -> u16) {
        if self.code & 32 == 0 || self.registers[1] & 16 == 0 || self.registers[23] >> 6 == 2 {
            return;
        }
        let mut source = u32::from(self.registers[21])
            | (u32::from(self.registers[22]) << 8)
            | (u32::from(self.registers[23] & 127) << 16);
        let copy = self.registers[23] & 0xc0 == 0xc0;
        for _ in 0..self.dma_length() {
            if copy {
                let b = self.vram[source as usize & 65535];
                self.vram[self.address as usize] = b;
                source = (source + 1) & 65535;
                self.advance();
            } else {
                self.write_target(read_word(source << 1));
                source = (source & 0x7f0000) | ((source + 1) & 65535);
            }
        }
        self.registers[21] = source as u8;
        self.registers[22] = (source >> 8) as u8;
        if !copy {
            self.registers[23] = (self.registers[23] & 128) | ((source >> 16) as u8 & 127);
        }
        self.finish_dma();
    }
    fn pattern(&self, entry: u16, x: usize, y: usize) -> Pixel {
        let h = if self.interlace() { 16 } else { 8 };
        let x = if entry & 0x800 != 0 {
            7 - (x & 7)
        } else {
            x & 7
        };
        let y = if entry & 0x1000 != 0 {
            h - 1 - (y % h)
        } else {
            y % h
        };
        let at = (usize::from(entry & 0x7ff) * h * 4 + y * 4 + x / 2) & 65535;
        let n = (self.vram[at] >> if x % 2 == 0 { 4 } else { 0 }) & 15;
        Pixel {
            index: if n == 0 {
                0
            } else {
                n | ((entry >> 9) as u8 & 48)
            },
            priority: entry & 0x8000 != 0,
        }
    }
    fn dimensions(&self) -> (usize, usize) {
        let n = |v| match v & 3 {
            0 => 32,
            1 => 64,
            3 => 128,
            _ => 32,
        };
        (n(self.registers[16]), n(self.registers[16] >> 4))
    }
    fn plane(&self, which: usize, x: usize, y: usize) -> Pixel {
        let h = if self.interlace() { 16 } else { 8 };
        let (w, rows) = self.dimensions();
        let hrow = match self.registers[11] & 3 {
            0 => 0,
            2 => y & !7,
            3 => y,
            _ => y & 7,
        };
        let hs =
            self.word((usize::from(self.registers[13] & 63) << 10) + hrow * 4 + which * 2) & 1023;
        let vs_index = if self.registers[11] & 4 != 0 {
            (x / 16) * 2 + which
        } else {
            which
        };
        let vs = self.vsram.get(vs_index).copied().unwrap_or(0) as usize;
        let px = x.wrapping_sub(hs as usize) & (w * 8 - 1);
        let py = (y + vs) & (rows * h - 1);
        let base = if which == 0 {
            usize::from(self.registers[2] & 56) << 10
        } else {
            usize::from(self.registers[4] & 7) << 13
        };
        self.pattern(self.word(base + ((py / h) * w + px / 8) * 2), px, py)
    }
    fn window(&self, x: usize, y: usize) -> bool {
        let rx = usize::from(self.registers[17] & 31) * 16;
        let ry = usize::from(self.registers[18] & 31) * 8;
        (if self.registers[17] & 128 != 0 {
            x >= rx
        } else {
            x < rx
        }) || (if self.registers[18] & 128 != 0 {
            y >= ry
        } else {
            y < ry
        })
    }
    fn window_pixel(&self, x: usize, y: usize) -> Pixel {
        let w = if self.width() == 320 { 64 } else { 32 };
        let h = if self.interlace() { 16 } else { 8 };
        let base = usize::from(self.registers[3] & if w == 64 { 60 } else { 62 }) << 10;
        self.pattern(self.word(base + ((y / h) * w + x / 8) * 2), x, y)
    }
    pub fn scanline(&self, y: usize) -> Result<Line> {
        ensure!(self.vram.len() == 65536, "VDP VRAM must contain 64 KiB");
        let height = self.height() * if self.interlace() { 2 } else { 1 };
        ensure!(y < height, "scanline outside active display");
        let width = self.width();
        let mut sprites = vec![Pixel::default(); width];
        let mut overflow = false;
        let mut collision = false;
        if self.registers[1] & 64 == 0 {
            return Ok(Line {
                rgba: vec![
                    color(
                        self.cram[usize::from(self.registers[7] & 63)],
                        Brightness::Normal
                    );
                    width
                ],
                sprite_overflow: false,
                sprite_collision: false,
            });
        }
        let base = usize::from(self.registers[5] & if width == 320 { 126 } else { 127 }) << 9;
        let mut link = 0;
        let mut seen = [false; 80];
        let mut selected = 0;
        let mut pixels = 0;
        let mut had_visible = false;
        for _ in 0..if width == 320 { 80 } else { 64 } {
            if link >= seen.len() || seen[link] {
                break;
            }
            seen[link] = true;
            let at = base + link * 8;
            let sy = i32::from(self.word(at) & if self.interlace() { 1023 } else { 511 })
                - if self.interlace() { 256 } else { 128 };
            let sz = self.word(at + 2);
            let cells_x = usize::from((sz >> 10) & 3) + 1;
            let cells_y = usize::from((sz >> 8) & 3) + 1;
            let entry = self.word(at + 4);
            let raw_x = self.word(at + 6) & 511;
            let sx = i32::from(raw_x) - 128;
            let tile_h = if self.interlace() { 16 } else { 8 };
            if y as i32 >= sy && (y as i32) < sy + (cells_y * tile_h) as i32 {
                selected += 1;
                if selected > if width == 320 { 20 } else { 16 } {
                    overflow = true;
                    break;
                }
                if raw_x == 0 && had_visible {
                    break;
                }
                had_visible |= raw_x != 0;
                let local_y = (y as i32 - sy) as usize;
                let yy = if entry & 0x1000 != 0 {
                    cells_y * tile_h - 1 - local_y
                } else {
                    local_y
                };
                for dx in 0..cells_x * 8 {
                    pixels += 1;
                    if pixels > width {
                        overflow = true;
                        break;
                    }
                    let screen_x = sx + dx as i32;
                    if !(0..width as i32).contains(&screen_x) {
                        continue;
                    }
                    let xx = if entry & 0x800 != 0 {
                        cells_x * 8 - 1 - dx
                    } else {
                        dx
                    };
                    let tile =
                        (usize::from(entry & 0x7ff) + (xx / 8) * cells_y + yy / tile_h) & 0x7ff;
                    let pixel = self.pattern((entry & 0xe000) | tile as u16, xx & 7, yy % tile_h);
                    if pixel.index & 15 != 0 {
                        let dst = &mut sprites[screen_x as usize];
                        if dst.index & 15 != 0 {
                            collision = true
                        } else {
                            *dst = pixel
                        }
                    }
                }
                if overflow {
                    break;
                }
            }
            link = usize::from(sz & 127);
            if link == 0 {
                break;
            }
        }
        let shadow = self.registers[12] & 8 != 0;
        let backdrop = self.registers[7] & 63;
        let mut rgba = Vec::with_capacity(width);
        for (x, s) in sprites.into_iter().enumerate() {
            let a = if self.window(x, y) {
                self.window_pixel(x, y)
            } else {
                self.plane(0, x, y)
            };
            let b = self.plane(1, x, y);
            let mut chosen = Pixel {
                index: backdrop,
                priority: false,
            };
            let mut level = if shadow {
                Brightness::Shadow
            } else {
                Brightness::Normal
            };
            for high in [false, true] {
                for (layer, p) in [b, a, s].into_iter().enumerate() {
                    if p.index & 15 != 0 && p.priority == high {
                        if shadow && p.index == 62 && layer == 2 {
                            level = Brightness::Highlight;
                        } else if shadow && p.index == 63 && layer == 2 {
                            level = Brightness::Shadow;
                        } else {
                            chosen = p;
                            level = if shadow && !p.priority && !(layer == 2 && p.index & 15 == 14)
                            {
                                Brightness::Shadow
                            } else {
                                Brightness::Normal
                            };
                        }
                    }
                }
            }
            rgba.push(color(self.cram[usize::from(chosen.index & 63)], level));
        }
        Ok(Line {
            rgba,
            sprite_overflow: overflow,
            sprite_collision: collision,
        })
    }
}
