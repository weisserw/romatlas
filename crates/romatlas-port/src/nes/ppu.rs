//! Pixel/scanline effects for native ports, independent of emulation cores.
//! Callers can vary scroll, CHR pages, palette and mask by scanline or pixel for
//! split screens, mapper IRQ effects, raster palette changes and CHR animation.
use crate::graphics::Color;
use anyhow::{Result, ensure};
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mirroring {
    Horizontal,
    Vertical,
    Single(u8),
    FourScreen,
}
pub fn nametable_index(address: u16, mirroring: Mirroring) -> usize {
    let a = usize::from(address.wrapping_sub(0x2000) & 0xfff);
    let table = a / 1024;
    let physical = match mirroring {
        Mirroring::Horizontal => table / 2,
        Mirroring::Vertical => table % 2,
        Mirroring::Single(bank) => usize::from(bank & 1),
        Mirroring::FourScreen => table,
    };
    physical * 1024 + (a & 1023)
}
/// $3F10/$14/$18/$1C mirror the background universal entries.
pub fn palette_index(address: u8) -> usize {
    let a = address & 31;
    usize::from(if a & 0x13 == 0x10 { a & 15 } else { a })
}
/// A caller-supplied 64-entry RGB palette avoids embedding game assets or tying
/// ports to one composite decoder. Emphasis is an RGB approximation.
pub fn color(palette: &[Color; 64], index: u8, mask: u8) -> Color {
    let index = if mask & 1 != 0 {
        index & 0x30
    } else {
        index & 63
    };
    let mut out = palette[usize::from(index)];
    let emphasis = mask >> 5;
    for (channel, value) in out[..3].iter_mut().enumerate() {
        let attenuations = (emphasis & !(1 << channel)).count_ones();
        for _ in 0..attenuations {
            *value = ((u16::from(*value) * 3 + 2) / 4) as u8;
        }
    }
    out
}
#[derive(Clone, Copy, Debug, Default)]
pub struct Scroll {
    pub x: u16,
    pub y: u16,
    pub nametable: u8,
}
/// Native tile sampling through an explicit eight-page CHR bank view.
fn pattern(chr: &[u8], banks: &[usize; 8], address: usize, x: usize, y: usize) -> u8 {
    let at = address + y;
    let read = |a: usize| {
        chr.get(banks[(a / 1024) & 7] * 1024 + (a & 1023))
            .copied()
            .unwrap_or(0)
    };
    ((read(at) >> (7 - x)) & 1) | (((read(at + 8) >> (7 - x)) & 1) << 1)
}
pub struct Background<'a> {
    pub chr: &'a [u8],
    pub chr_banks: [usize; 8],
    pub nametables: &'a [u8],
    pub mirroring: Mirroring,
    pub pattern_table: u16,
    pub scroll: Scroll,
}
impl Background<'_> {
    /// Return palette address 0..15. Low two bits zero mean transparent.
    pub fn pixel(&self, x: u16, y: u16) -> Result<u8> {
        ensure!(
            self.pattern_table == 0 || self.pattern_table == 0x1000,
            "invalid pattern table"
        );
        let sx =
            (u32::from(x) + u32::from(self.scroll.x) + u32::from(self.scroll.nametable & 1) * 256)
                % 512;
        let sy = (u32::from(y)
            + u32::from(self.scroll.y)
            + u32::from((self.scroll.nametable >> 1) & 1) * 240)
            % 480;
        let table = (sy / 240) * 2 + sx / 256;
        let tx = (sx % 256) / 8;
        let ty = (sy % 240) / 8;
        let base = 0x2000 + table as u16 * 1024;
        let read = |a: u16| -> Result<u8> {
            self.nametables
                .get(nametable_index(a, self.mirroring))
                .copied()
                .ok_or_else(|| anyhow::anyhow!("nametable storage too short"))
        };
        let tile = read(base + (ty * 32 + tx) as u16)?;
        let attr = read(base + 960 + ((ty / 4) * 8 + tx / 4) as u16)?;
        let shift = ((ty & 2) * 2 + (tx & 2)) as u8;
        let group = (attr >> shift) & 3;
        let pixel = pattern(
            self.chr,
            &self.chr_banks,
            usize::from(self.pattern_table) + usize::from(tile) * 16,
            (sx & 7) as usize,
            (sy & 7) as usize,
        );
        Ok(if pixel == 0 { 0 } else { group * 4 + pixel })
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SpritePixel {
    pub palette: u8,
    pub behind_background: bool,
    pub sprite_zero: bool,
}
#[derive(Clone, Debug)]
pub struct SpriteLine {
    pub pixels: [Option<SpritePixel>; 256],
    pub overflow: bool,
}
/// Select the first eight in-range OAM entries, including transparent sprites.
/// Supports 8x8/8x16 sprites, both flips, pattern tables and OAM priority.
/// Overflow means >8 in-range sprites; the hardware diagonal overflow bug is excluded.
pub fn sprites(
    chr: &[u8],
    banks: [usize; 8],
    oam: &[u8],
    y: u16,
    tall: bool,
    table: u16,
) -> Result<SpriteLine> {
    ensure!(oam.len() == 256, "OAM needs 256 bytes");
    ensure!(
        table == 0 || table == 0x1000,
        "invalid sprite pattern table"
    );
    let mut out = SpriteLine {
        pixels: [None; 256],
        overflow: false,
    };
    let height = if tall { 16 } else { 8 };
    let mut count = 0;
    for (index, s) in oam.chunks_exact(4).enumerate() {
        // Y=255 is offscreen; NES does not wrap it onto scanline zero.
        let top = u16::from(s[0]) + 1;
        if y < top || y >= top + height {
            continue;
        }
        count += 1;
        if count > 8 {
            out.overflow = true;
            continue;
        }
        let row = usize::from(if s[2] & 0x80 != 0 {
            height - 1 - (y - top)
        } else {
            y - top
        });
        let base = if tall {
            usize::from(s[1] & 1) * 4096 + usize::from(s[1] & 0xfe) * 16 + (row / 8) * 16
        } else {
            usize::from(table) + usize::from(s[1]) * 16
        };
        for column in 0..8 {
            let x = usize::from(s[3]) + column;
            if x >= 256 {
                break;
            }
            let px = pattern(
                chr,
                &banks,
                base,
                if s[2] & 0x40 != 0 { 7 - column } else { column },
                row & 7,
            );
            if px != 0 && out.pixels[x].is_none() {
                out.pixels[x] = Some(SpritePixel {
                    palette: 16 + (s[2] & 3) * 4 + px,
                    behind_background: s[2] & 0x20 != 0,
                    sprite_zero: index == 0,
                });
            }
        }
    }
    Ok(out)
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Pixel {
    pub palette: u8,
    pub sprite_zero_hit: bool,
}
/// PPUMASK enables, left-edge clipping, sprite/background priority and zero hit.
pub fn compose(background: u8, sprite: Option<SpritePixel>, x: u16, mask: u8) -> Pixel {
    let bg = if mask & 8 != 0 && (x >= 8 || mask & 2 != 0) {
        background
    } else {
        0
    };
    let sprite = if mask & 16 != 0 && (x >= 8 || mask & 4 != 0) {
        sprite
    } else {
        None
    };
    let opaque = bg & 3 != 0;
    Pixel {
        palette: match sprite {
            Some(s) if !s.behind_background || !opaque => s.palette,
            _ => {
                if opaque {
                    bg
                } else {
                    0
                }
            }
        },
        sprite_zero_hit: sprite.is_some_and(|s| s.sprite_zero) && opaque && x < 255,
    }
}
