//! Ordinary planar backgrounds and OBJ sampling, windows and scanline composition.
//! Callers supply per-scanline registers and layer priority ranks. Mode 7 transforms,
//! offset-per-tile, mosaic, interlace/hires and hardware sprite overflow are separate.
use super::TilemapEntry;
use anyhow::{Result, ensure};

/// Byte address of an entry in a 32/64 by 32/64 tile map. Coordinates wrap.
pub fn tilemap_address(base: u16, wide: bool, tall: bool, x: u16, y: u16) -> u16 {
    let x = x & if wide { 63 } else { 31 };
    let y = y & if tall { 63 } else { 31 };
    let screen = (x / 32) + (y / 32) * if wide { 2 } else { 1 };
    base.wrapping_add(screen * 2048 + ((y & 31) * 32 + (x & 31)) * 2)
}
fn word(vram: &[u8; 65536], a: u16) -> u16 {
    u16::from_le_bytes([vram[a as usize], vram[a.wrapping_add(1) as usize]])
}
/// One pixel from VRAM, with 16-bit byte-address wrapping.
pub fn tile_pixel(vram: &[u8; 65536], base: u16, tile: u16, bpp: u8, x: u8, y: u8) -> Result<u8> {
    ensure!(
        matches!(bpp, 2 | 4 | 8) && x < 8 && y < 8,
        "invalid tile pixel request"
    );
    let start = base.wrapping_add(tile.wrapping_mul(u16::from(bpp) * 8));
    let mut index = 0;
    for plane in 0..bpp {
        let a =
            start.wrapping_add(u16::from(plane / 2) * 16 + u16::from(y) * 2 + u16::from(plane & 1));
        index |= ((vram[a as usize] >> (7 - x)) & 1) << plane;
    }
    Ok(index)
}
#[derive(Clone, Copy, Debug)]
pub struct Background {
    pub map_base: u16,
    pub tile_base: u16,
    pub wide: bool,
    pub tall: bool,
    pub large_tiles: bool,
    pub bpp: u8,
    /// Mode 0 uses a distinct 32-color base per BG; other ordinary modes use 0.
    pub palette_base: u8,
    pub scroll_x: u16,
    pub scroll_y: u16,
}
impl Background {
    /// Palette index and tile priority; index zero is transparent regardless of palette.
    pub fn sample(&self, vram: &[u8; 65536], x: u16, y: u16) -> Result<Option<(u8, bool)>> {
        ensure!(matches!(self.bpp, 2 | 4 | 8), "invalid background depth");
        let size = if self.large_tiles { 16 } else { 8 };
        let x = x.wrapping_add(self.scroll_x);
        let y = y.wrapping_add(self.scroll_y);
        let e = TilemapEntry::from_word(word(
            vram,
            tilemap_address(self.map_base, self.wide, self.tall, x / size, y / size),
        ));
        let mut px = x % size;
        let mut py = y % size;
        if e.flip_x {
            px = size - 1 - px;
        }
        if e.flip_y {
            py = size - 1 - py;
        }
        let tile = e.tile.wrapping_add(px / 8 + (py / 8) * 16) & 1023;
        let index = tile_pixel(
            vram,
            self.tile_base,
            tile,
            self.bpp,
            (px & 7) as u8,
            (py & 7) as u8,
        )?;
        Ok((index != 0).then(|| {
            let palette = if self.bpp == 8 {
                0
            } else {
                e.palette.wrapping_mul(1 << self.bpp)
            };
            (
                self.palette_base.wrapping_add(palette).wrapping_add(index),
                e.priority,
            )
        }))
    }
}
/// OBSEL small/large dimensions, including rectangular modes 6 and 7.
pub fn object_size(selection: u8, large: bool) -> (u8, u8) {
    const SIZES: [[(u8, u8); 2]; 8] = [
        [(8, 8), (16, 16)],
        [(8, 8), (32, 32)],
        [(8, 8), (64, 64)],
        [(16, 16), (32, 32)],
        [(16, 16), (64, 64)],
        [(32, 32), (64, 64)],
        [(16, 32), (32, 64)],
        [(16, 32), (32, 32)],
    ];
    SIZES[usize::from(selection & 7)][usize::from(large)]
}
#[derive(Clone, Copy, Debug)]
pub struct Object {
    pub x: i16,
    pub y: u8,
    pub tile: u8,
    pub name_table: bool,
    pub palette: u8,
    pub priority: u8,
    pub flip_x: bool,
    pub flip_y: bool,
    pub large: bool,
}
impl Object {
    pub fn from_oam(oam: &[u8; 544], index: u8) -> Result<Self> {
        ensure!(index < 128, "OAM index out of range");
        let i = usize::from(index);
        let high = (oam[512 + i / 4] >> ((i % 4) * 2)) & 3;
        let attr = oam[i * 4 + 3];
        let x = i16::from(oam[i * 4]) - if high & 1 != 0 { 256 } else { 0 };
        Ok(Self {
            x,
            y: oam[i * 4 + 1],
            tile: oam[i * 4 + 2],
            name_table: attr & 1 != 0,
            palette: (attr >> 1) & 7,
            priority: (attr >> 4) & 3,
            flip_x: attr & 64 != 0,
            flip_y: attr & 128 != 0,
            large: high & 2 != 0,
        })
    }
    /// Sample a non-interlaced object. OBJ tile columns wrap within a 16-tile row.
    pub fn sample(
        &self,
        vram: &[u8; 65536],
        base: u16,
        name_gap: u8,
        sizes: u8,
        x: u8,
        y: u8,
    ) -> Result<Option<u8>> {
        ensure!(
            self.palette < 8 && self.priority < 4,
            "invalid OBJ palette or priority"
        );
        let (width, height) = object_size(sizes, self.large);
        let px = i16::from(x) - self.x;
        let py = y.wrapping_sub(self.y);
        if px < 0 || px >= i16::from(width) || py >= height {
            return Ok(None);
        }
        let px = if self.flip_x {
            width - 1 - px as u8
        } else {
            px as u8
        };
        let py = if self.flip_y { height - 1 - py } else { py };
        let tile = (self.tile & 0xf0).wrapping_add((py / 8) * 16)
            | ((self.tile & 15).wrapping_add(px / 8) & 15);
        let base = base.wrapping_add(if self.name_table {
            (u16::from(name_gap & 3) + 1) * 8192
        } else {
            0
        });
        let index = tile_pixel(vram, base, u16::from(tile), 4, px & 7, py & 7)?;
        Ok((index != 0).then_some(128 + self.palette * 16 + index))
    }
}
#[derive(Clone, Copy, Debug, Default)]
pub struct Window {
    pub left: u8,
    pub right: u8,
    pub enabled: bool,
    pub invert: bool,
}
#[derive(Clone, Copy, Debug)]
pub enum WindowLogic {
    Or,
    And,
    Xor,
    Xnor,
}
pub fn window_mask(x: u8, a: Window, b: Window, logic: WindowLogic) -> bool {
    let test = |w: Window| (x >= w.left && x <= w.right) ^ w.invert;
    match (a.enabled, b.enabled) {
        (false, false) => false,
        (true, false) => test(a),
        (false, true) => test(b),
        (true, true) => match logic {
            WindowLogic::Or => test(a) | test(b),
            WindowLogic::And => test(a) & test(b),
            WindowLogic::Xor => test(a) ^ test(b),
            WindowLogic::Xnor => !(test(a) ^ test(b)),
        },
    }
}
/// Math at native five-bit precision; halve before saturation for addition.
pub fn color_math(main: u16, sub: u16, subtract: bool, half: bool) -> u16 {
    let mut out = 0;
    for shift in [0, 5, 10] {
        let a = i32::from((main >> shift) & 31);
        let b = i32::from((sub >> shift) & 31);
        let mut value = if subtract { (a - b).max(0) } else { a + b };
        if half {
            value >>= 1;
        }
        out |= (value.min(31) as u16) << shift;
    }
    out
}
#[derive(Clone, Copy, Debug)]
pub struct Pixel {
    pub color: u16,
    /// Higher wins; equal ranks preserve earlier candidates (OAM order).
    pub rank: u8,
    /// BG1..BG4 = 0..3, OBJ = 4, backdrop = 5.
    pub layer: u8,
    /// OBJ palettes 0..3 cannot participate in color math.
    pub math: bool,
}
/// Resolve one main/sub screen pixel. Supply candidates already filtered by
/// screen enables and windows. Backdrop is selected when no layer survives.
pub fn screen_pixel(candidates: impl IntoIterator<Item = Pixel>, backdrop: u16) -> Pixel {
    let mut top = Pixel {
        color: backdrop,
        rank: 0,
        layer: 5,
        math: true,
    };
    for p in candidates {
        if top.layer == 5 || p.rank > top.rank {
            top = p;
        }
    }
    top
}
#[derive(Clone, Copy, Debug, Default)]
pub struct Composition {
    pub math_layers: u8,
    pub subtract: bool,
    pub half: bool,
    pub use_subscreen: bool,
    pub fixed_color: u16,
    pub clip_main: bool,
    pub prevent_math: bool,
    pub brightness: u8,
    pub force_blank: bool,
}
impl Composition {
    /// Color-window decisions are supplied per pixel. Subscreen backdrop falls
    /// back to fixed color and disables halving; clipping also disables halving.
    pub fn compose(&self, main: Pixel, sub: Pixel) -> crate::graphics::Color {
        if self.force_blank {
            return [0, 0, 0, 255];
        }
        let mut color = if self.clip_main { 0 } else { main.color };
        if !self.prevent_math
            && main.math
            && main.layer < 6
            && self.math_layers & (1 << main.layer) != 0
        {
            let operand = if self.use_subscreen && sub.layer != 5 {
                sub.color
            } else {
                self.fixed_color
            };
            let half = self.half && !self.clip_main && (!self.use_subscreen || sub.layer != 5);
            color = color_math(color, operand, self.subtract, half);
        }
        let brightness = u32::from(self.brightness.min(15));
        let mut rgba = super::color(color);
        for c in &mut rgba[..3] {
            *c = (u32::from(*c) * brightness / 15) as u8;
        }
        rgba
    }
}

/// Mode 1 priorities, indexed by BG number 0..2 and priority bit. OBJ ranks
/// are [2, 5, 8, 11]. BG3 high priority can optionally move above all OBJ.
pub fn mode1_bg_rank(bg: u8, high: bool, bg3_high: bool) -> Result<u8> {
    ensure!(bg < 3, "Mode 1 has three backgrounds");
    Ok(match (bg, high) {
        (0, false) => 7,
        (0, true) => 10,
        (1, false) => 6,
        (1, true) => 9,
        (2, false) => 1,
        _ => {
            if bg3_high {
                12
            } else {
                3
            }
        }
    })
}
/// Compose a complete ordinary 256-pixel line from resolved main/sub pixels.
/// The callback supplies per-pixel color-window and brightness decisions.
pub fn compose_scanline(
    main: &[Pixel; 256],
    sub: &[Pixel; 256],
    settings: impl Fn(u8) -> Composition,
) -> [crate::graphics::Color; 256] {
    std::array::from_fn(|x| settings(x as u8).compose(main[x], sub[x]))
}

/// Resolve ordinary OBJ pixels in caller-supplied OAM rotation order before BG
/// priority comparison. This does not enforce scanline object/tile fetch limits.
pub fn object_scanline(
    objects: &[Object],
    vram: &[u8; 65536],
    cgram: &[u16; 256],
    obsel: u8,
    y: u8,
    ranks: [u8; 4],
) -> Result<[Option<Pixel>; 256]> {
    let mut line = [None; 256];
    let base = u16::from(obsel & 7).wrapping_mul(16384);
    for object in objects {
        ensure!(
            object.palette < 8 && object.priority < 4,
            "invalid OBJ palette or priority"
        );
        for (x, pixel) in line.iter_mut().enumerate() {
            if pixel.is_some() {
                continue;
            }
            if let Some(index) =
                object.sample(vram, base, (obsel >> 3) & 3, obsel >> 5, x as u8, y)?
            {
                *pixel = Some(Pixel {
                    color: cgram[usize::from(index)],
                    rank: ranks[usize::from(object.priority)],
                    layer: 4,
                    math: object.palette >= 4,
                });
            }
        }
    }
    Ok(line)
}
