//! SMS2 Mode 4 scanline helpers. Ports schedule register/VRAM/palette changes.
use anyhow::{Result, ensure};
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Pixel {
    pub index: u8,
    pub priority: bool,
}
#[derive(Clone, Copy, Debug)]
pub struct Config {
    pub height: u16,
    pub name_base: u16,
    pub sprite_base: u16,
    pub sprite_pattern_base: u16,
    pub scroll_x: u8,
    pub scroll_y: u8,
    pub top_scroll_lock: bool,
    pub right_scroll_lock: bool,
    pub mask_left: bool,
    pub shift_sprites: bool,
    pub tall_sprites: bool,
    pub zoom_sprites: bool,
    pub display: bool,
    pub backdrop: u8,
}
impl Default for Config {
    fn default() -> Self {
        Self {
            height: 192,
            name_base: 0x3800,
            sprite_base: 0x3f00,
            sprite_pattern_base: 0,
            scroll_x: 0,
            scroll_y: 0,
            top_scroll_lock: false,
            right_scroll_lock: false,
            mask_left: false,
            shift_sprites: false,
            tall_sprites: false,
            zoom_sprites: false,
            display: true,
            backdrop: 0,
        }
    }
}
fn read(vram: &[u8], at: usize) -> u8 {
    vram[at & 16383]
}
fn pattern(vram: &[u8], tile: usize, x: usize, y: usize) -> u8 {
    (0..4).fold(0, |v, p| {
        v | ((read(vram, tile * 32 + y * 4 + p) >> (7 - x)) & 1) << p
    })
}
impl Config {
    pub fn validate(self) -> Result<()> {
        ensure!(
            matches!(self.height, 192 | 224 | 240),
            "unsupported Mode 4 height"
        );
        Ok(())
    }
    /// Decode the Mode 4 fields from the eleven standard VDP register bytes.
    pub fn registers(r: [u8; 11]) -> Self {
        let height = if r[0] & 2 != 0 && r[1] & 0x18 == 0x10 {
            224
        } else if r[0] & 2 != 0 && r[1] & 0x18 == 8 {
            240
        } else {
            192
        };
        Self {
            height,
            name_base: if height == 192 {
                u16::from(r[2] & 14) << 10
            } else {
                (u16::from(r[2] & 12) << 10) | 0x700
            },
            sprite_base: u16::from(r[5] & 0x7e) << 7,
            sprite_pattern_base: u16::from(r[6] & 4) << 11,
            scroll_x: r[8],
            scroll_y: r[9],
            top_scroll_lock: r[0] & 0x40 != 0,
            right_scroll_lock: r[0] & 0x80 != 0,
            mask_left: r[0] & 0x20 != 0,
            shift_sprites: r[0] & 8 != 0,
            tall_sprites: r[1] & 2 != 0,
            zoom_sprites: r[1] & 1 != 0,
            display: r[1] & 0x40 != 0,
            backdrop: r[7] & 15,
        }
    }
}
pub fn background(vram: &[u8], x: u16, y: u16, c: Config) -> Result<Pixel> {
    ensure!(vram.len() == 16384, "VDP needs 16 KiB VRAM");
    c.validate()?;
    ensure!(x < 256 && y < c.height, "pixel outside active display");
    let sx = if c.top_scroll_lock && y < 16 {
        x
    } else {
        x.wrapping_sub(u16::from(c.scroll_x)) & 255
    };
    let sy = if c.right_scroll_lock && x >= 192 {
        y
    } else {
        (y + u16::from(c.scroll_y)) % if c.height == 192 { 224 } else { 256 }
    };
    let at = usize::from(c.name_base) + (usize::from(sy) / 8 * 32 + usize::from(sx) / 8) * 2;
    let entry = u16::from_le_bytes([read(vram, at), read(vram, at + 1)]);
    let mut tx = usize::from(sx & 7);
    let mut ty = usize::from(sy & 7);
    if entry & 0x200 != 0 {
        tx = 7 - tx;
    }
    if entry & 0x400 != 0 {
        ty = 7 - ty;
    }
    let color = pattern(vram, usize::from(entry & 511), tx, ty);
    Ok(Pixel {
        index: color | if entry & 0x800 != 0 { 16 } else { 0 },
        priority: color != 0 && entry & 0x1000 != 0,
    })
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Scanline {
    pub indices: [u8; 256],
    pub collision: bool,
    pub overflow: bool,
    pub selected: Vec<u8>,
}
pub fn scanline(vram: &[u8], y: u16, c: Config) -> Result<Scanline> {
    ensure!(vram.len() == 16384, "VDP needs 16 KiB VRAM");
    c.validate()?;
    ensure!(y < c.height, "scanline outside display");
    let mut line = Scanline {
        indices: [16 | (c.backdrop & 15); 256],
        collision: false,
        overflow: false,
        selected: Vec::new(),
    };
    if !c.display {
        return Ok(line);
    }
    let mut bg = [Pixel::default(); 256];
    for (x, pixel) in bg.iter_mut().enumerate() {
        *pixel = background(vram, x as u16, y, c)?;
        line.indices[x] = pixel.index;
    }
    let scale = if c.zoom_sprites { 2 } else { 1 };
    let height = if c.tall_sprites { 16 } else { 8 };
    let mut occupied = [false; 256];
    for n in 0..64 {
        let raw_y = read(vram, usize::from(c.sprite_base) + n);
        if c.height == 192 && raw_y == 0xd0 {
            break;
        }
        let row = (i32::from(y) - i32::from(raw_y) - 1).rem_euclid(256);
        if row >= height * scale {
            continue;
        }
        if line.selected.len() == 8 {
            line.overflow = true;
            break;
        }
        line.selected.push(n as u8);
        let at = usize::from(c.sprite_base) + 128 + n * 2;
        let left = i32::from(read(vram, at)) - if c.shift_sprites { 8 } else { 0 };
        let tile = usize::from(read(vram, at + 1)) | (usize::from(c.sprite_pattern_base) / 32);
        let tile = if c.tall_sprites { tile & !1 } else { tile };
        for col in 0..8 * scale {
            let x = left + col;
            if !(0..256).contains(&x) {
                continue;
            }
            let row = (row / scale) as usize;
            let index = pattern(vram, tile + row / 8, (col / scale) as usize, row & 7);
            if index == 0 {
                continue;
            }
            let x = x as usize;
            if occupied[x] {
                line.collision = true;
                continue;
            }
            occupied[x] = true;
            if !bg[x].priority {
                line.indices[x] = 16 | index;
            }
        }
    }
    if c.mask_left {
        line.indices[..8].fill(16 | (c.backdrop & 15));
    }
    Ok(line)
}
/// Compose immediately so mid-frame palette changes can be scheduled by the port.
pub fn rgba(line: &Scanline, cram: &[u8]) -> Result<Vec<[u8; 4]>> {
    ensure!(cram.len() == 32, "SMS palette needs 32 bytes");
    Ok(line
        .indices
        .iter()
        .map(|&i| super::color(cram[usize::from(i & 31)]))
        .collect())
}
/// Deterministic line-interrupt counter; callers clock once per scanline.
#[derive(Clone, Copy, Debug)]
pub struct LineCounter {
    pub reload: u8,
    pub remaining: u8,
}
impl LineCounter {
    pub fn clock(&mut self, active: bool) -> bool {
        if !active {
            self.remaining = self.reload;
            return false;
        }
        if self.remaining == 0 {
            self.remaining = self.reload;
            true
        } else {
            self.remaining -= 1;
            false
        }
    }
}
