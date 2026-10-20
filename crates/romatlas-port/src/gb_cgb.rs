//! Game Boy Color rendering effects for native ports, independent of an emulator.
//! Input is physical VRAM (bank 0 then bank 1), raw OAM and palette RAM.
//! Ports own register timing, DMA, window triggering and game logic.
use crate::graphics::{Color, Frame};
use anyhow::{Result, ensure};

pub const WIDTH: u32 = 160;
pub const HEIGHT: u32 = 144;

/// Eight four-color palettes in the hardware's little-endian RGB555 format.
#[derive(Clone)]
pub struct Palettes {
    pub bytes: [u8; 64],
    index: u8,
}
impl Default for Palettes {
    fn default() -> Self {
        Self {
            bytes: [0; 64],
            index: 0,
        }
    }
}
impl Palettes {
    pub fn from_bytes(bytes: &[u8]) -> Result<Self> {
        ensure!(bytes.len() == 64, "CGB palette RAM must contain 64 bytes");
        let mut result = Self::default();
        result.bytes.copy_from_slice(bytes);
        Ok(result)
    }
    /// BCPS/OCPS: bit 6 reads high; bit 7 enables data-write auto increment.
    pub fn read_index(&self) -> u8 {
        self.index | 0x40
    }
    pub fn write_index(&mut self, value: u8) {
        self.index = value & 0xbf;
    }
    pub fn read_data(&self, blocked: bool) -> u8 {
        if blocked {
            0xff
        } else {
            self.bytes[usize::from(self.index & 63)]
        }
    }
    /// A mode-3 write is discarded but still advances an enabled index.
    pub fn write_data(&mut self, value: u8, blocked: bool) {
        if !blocked {
            self.bytes[usize::from(self.index & 63)] = value;
        }
        if self.index & 128 != 0 {
            self.index = 128 | (self.index.wrapping_add(1) & 63);
        }
    }
    pub fn color(&self, palette: u8, index: u8) -> Color {
        let offset = usize::from(palette & 7) * 8 + usize::from(index & 3) * 2;
        super::cgb_color(u16::from_le_bytes([
            self.bytes[offset],
            self.bytes[offset + 1],
        ]))
    }
}

/// Register values for one rendered line. Defaults select unsigned BG tiles,
/// BG map $9800, enabled LCD/BG, disabled objects/window and CGB OAM priority.
#[derive(Clone, Copy)]
pub struct Registers {
    pub lcdc: u8,
    pub scx: u8,
    pub scy: u8,
    pub wx: u8,
    pub wy: u8,
    /// OPRI bit 0: false uses OAM order; true uses X then OAM order.
    pub coordinate_priority: bool,
}
impl Default for Registers {
    fn default() -> Self {
        Self {
            lcdc: 0x91,
            scx: 0,
            scy: 0,
            wx: 7,
            wy: 0,
            coordinate_priority: false,
        }
    }
}

pub struct Video<'a> {
    pub vram: &'a [u8],
    /// Zero to forty four-byte entries, in OAM order.
    pub oam: &'a [[u8; 4]],
    pub background: &'a Palettes,
    pub objects: &'a Palettes,
}
impl Video<'_> {
    fn validate(&self, frame: &Frame) -> Result<()> {
        ensure!(
            self.vram.len() == 0x4000,
            "CGB VRAM must contain two 8 KiB banks"
        );
        ensure!(self.oam.len() <= 40, "CGB OAM has at most 40 entries");
        ensure!(
            frame.width == WIDTH && frame.height == HEIGHT,
            "CGB frame must be 160x144"
        );
        Ok(())
    }
    fn pixel(&self, bank: u8, tile: usize, x: usize, y: usize) -> u8 {
        let offset = usize::from(bank) * 0x2000 + tile + y * 2;
        let bit = 7 - x;
        ((self.vram[offset] >> bit) & 1) | (((self.vram[offset + 1] >> bit) & 1) << 1)
    }
    fn background_pixel(&self, x: usize, y: usize, map: usize, unsigned: bool) -> (u8, u8) {
        let offset = map + (y / 8) * 32 + x / 8;
        let tile = self.vram[offset];
        let attributes = self.vram[offset + 0x2000];
        let tile = if unsigned {
            usize::from(tile) * 16
        } else {
            (0x1000i32 + i32::from(tile as i8) * 16) as usize
        };
        let px = if attributes & 32 != 0 {
            7 - x % 8
        } else {
            x % 8
        };
        let py = if attributes & 64 != 0 {
            7 - y % 8
        } else {
            y % 8
        };
        (self.pixel((attributes >> 3) & 1, tile, px, py), attributes)
    }
    /// Render a line using the supplied internal window line, or disable its
    /// window with None. The caller tracks WY triggering and increments only
    /// on lines where the window renders. Returns whether it rendered here.
    pub fn render_scanline(
        &self,
        frame: &mut Frame,
        y: u8,
        registers: Registers,
        window_line: Option<u8>,
    ) -> Result<bool> {
        self.validate(frame)?;
        ensure!(u32::from(y) < HEIGHT, "CGB scanline must be 0..143");
        if registers.lcdc & 128 == 0 {
            for x in 0..WIDTH {
                frame.background_pixel(x as i32, i32::from(y), 0, [255; 4]);
            }
            return Ok(false);
        }
        let window = registers.lcdc & 32 != 0 && registers.wx <= 166 && window_line.is_some();
        let height = if registers.lcdc & 4 != 0 { 16 } else { 8 };
        // X-hidden entries still consume the ten-object selection budget.
        let mut selected: Vec<_> = self
            .oam
            .iter()
            .enumerate()
            .filter(|(_, o)| {
                let row = i32::from(y) + 16 - i32::from(o[0]);
                (0..height).contains(&row)
            })
            .take(10)
            .collect();
        if registers.coordinate_priority {
            selected.sort_by_key(|(i, o)| (o[1], *i));
        }
        for x in 0..WIDTH as usize {
            let in_window = window && x as i32 >= i32::from(registers.wx) - 7;
            let (px, py, map) = if in_window {
                (
                    (x as i32 + 7 - i32::from(registers.wx)) as usize,
                    usize::from(window_line.unwrap()),
                    if registers.lcdc & 64 != 0 {
                        0x1c00
                    } else {
                        0x1800
                    },
                )
            } else {
                (
                    (x + usize::from(registers.scx)) & 255,
                    (usize::from(y) + usize::from(registers.scy)) & 255,
                    if registers.lcdc & 8 != 0 {
                        0x1c00
                    } else {
                        0x1800
                    },
                )
            };
            let (bg, attributes) = self.background_pixel(px, py, map, registers.lcdc & 16 != 0);
            frame.background_pixel(
                x as i32,
                i32::from(y),
                bg,
                self.background.color(attributes & 7, bg),
            );
            if registers.lcdc & 2 == 0 {
                continue;
            }
            for (_, object) in &selected {
                let dx = x as i32 + 8 - i32::from(object[1]);
                if !(0..8).contains(&dx) {
                    continue;
                }
                let flags = object[3];
                let dx = if flags & 32 != 0 { 7 - dx } else { dx };
                let row = i32::from(y) + 16 - i32::from(object[0]);
                let row = if flags & 64 != 0 {
                    height - 1 - row
                } else {
                    row
                };
                let tile = if height == 16 {
                    object[2] & 0xfe
                } else {
                    object[2]
                };
                let pixel = self.pixel(
                    (flags >> 3) & 1,
                    usize::from(tile) * 16 + (row as usize / 8) * 16,
                    dx as usize,
                    row as usize % 8,
                );
                if pixel == 0 {
                    continue;
                }
                if bg == 0 || registers.lcdc & 1 == 0 || (attributes | flags) & 128 == 0 {
                    frame.pixel(x as i32, i32::from(y), self.objects.color(flags & 7, pixel));
                }
                // Resolve the winning object before testing BG priority.
                break;
            }
        }
        Ok(window)
    }
    /// Static-register frame convenience. For raster effects call render_scanline
    /// with game-owned register updates and window-counter state instead.
    pub fn render(&self, frame: &mut Frame, registers: Registers) -> Result<()> {
        self.validate(frame)?;
        let mut window_line = 0;
        for y in 0..HEIGHT as u8 {
            let window = (y >= registers.wy).then_some(window_line);
            if self.render_scanline(frame, y, registers, window)? {
                window_line += 1;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn palettes() -> Palettes {
        let mut result = Palettes::default();
        for palette in 0..8 {
            for color in 0..4 {
                let value = (palette * 4 + color) as u16;
                let offset = palette * 8 + color * 2;
                result.bytes[offset..offset + 2].copy_from_slice(&value.to_le_bytes());
            }
        }
        result
    }
    fn color(frame: &Frame, x: usize, y: usize) -> &[u8] {
        &frame.pixels[(y * 160 + x) * 4..(y * 160 + x + 1) * 4]
    }
    fn solid(vram: &mut [u8], bank: usize, tile: usize, index: u8) {
        for row in 0..8 {
            vram[bank * 0x2000 + tile * 16 + row * 2] = if index & 1 != 0 { 255 } else { 0 };
            vram[bank * 0x2000 + tile * 16 + row * 2 + 1] = if index & 2 != 0 { 255 } else { 0 };
        }
    }
    #[test]
    fn palette_blocked_access_wraps_and_preserves_raw_bytes() {
        let mut palette = Palettes::default();
        palette.write_index(0xbf);
        palette.write_data(0x12, false);
        assert_eq!(palette.read_index(), 0xc0);
        palette.write_data(0x34, true);
        assert_eq!(palette.read_index(), 0xc1);
        assert_eq!(palette.bytes[63], 0x12);
        assert_eq!(palette.bytes[0], 0);
        assert_eq!(palette.read_data(true), 255);
        palette.write_index(63);
        assert_eq!(palette.read_data(false), 0x12);
        assert!(Palettes::from_bytes(&[0; 63]).is_err());
    }
    #[test]
    fn banked_signed_tiles_flips_scrolling_and_window_counter() {
        let mut vram = vec![0; 0x4000];
        // Signed tile -1 is $8ff0 in bank 1; only its bottom-right pixel is 3.
        vram[0x2000 + 0xff0 + 14] = 1;
        vram[0x2000 + 0xff0 + 15] = 1;
        vram[0x1800] = 255;
        vram[0x3800] = 0x6b; // bank 1, both flips, palette 3
        vram[0x1c00] = 1;
        vram[0x3c00] = 4;
        solid(&mut vram, 0, 1, 2);
        let palette = palettes();
        let video = Video {
            vram: &vram,
            oam: &[],
            background: &palette,
            objects: &palette,
        };
        let mut frame = Frame::new(WIDTH, HEIGHT).unwrap();
        let mut regs = Registers {
            lcdc: 0x81,
            ..Registers::default()
        };
        video.render_scanline(&mut frame, 0, regs, None).unwrap();
        assert_eq!(color(&frame, 0, 0), palette.color(3, 3));
        assert_eq!(color(&frame, 1, 0), palette.color(3, 0));
        regs.scx = 255;
        video.render_scanline(&mut frame, 0, regs, None).unwrap();
        assert_eq!(color(&frame, 1, 0), palette.color(3, 3)); // wrap to x=0
        regs = Registers {
            lcdc: 0xf1,
            wx: 15,
            wy: 2,
            ..Registers::default()
        };
        assert!(!video.render_scanline(&mut frame, 1, regs, None).unwrap());
        assert!(!video.render_scanline(&mut frame, 2, regs, None).unwrap());
        assert!(video.render_scanline(&mut frame, 3, regs, Some(0)).unwrap());
        assert_eq!(color(&frame, 8, 3), palette.color(4, 2));
        assert_eq!(frame.background_index(8, 3), Some(2));
        // Once triggered, a game-owned window counter can remain active even
        // after WY changes. Static render() supplies the ordinary WY gate.
        assert!(video.render_scanline(&mut frame, 1, regs, Some(0)).unwrap());
    }
    #[test]
    fn cgb_priority_truth_table_and_hidden_object_winner() {
        let palette = palettes();
        for bg_index in 0..4 {
            for lcd_priority in [0, 1] {
                for bg_priority in [0, 128] {
                    for obj_priority in [0, 128] {
                        let mut vram = vec![0; 0x4000];
                        solid(&mut vram, 0, 0, bg_index);
                        solid(&mut vram, 1, 1, 2);
                        solid(&mut vram, 0, 2, 3);
                        vram[0x3800] = bg_priority;
                        // First OAM entry wins even though second has smaller X.
                        let oam = [[16, 10, 1, obj_priority | 8 | 2], [16, 8, 2, 3]];
                        let video = Video {
                            vram: &vram,
                            oam: &oam,
                            background: &palette,
                            objects: &palette,
                        };
                        let mut frame = Frame::new(WIDTH, HEIGHT).unwrap();
                        let regs = Registers {
                            lcdc: 0x92 | lcd_priority,
                            ..Registers::default()
                        };
                        video.render_scanline(&mut frame, 0, regs, None).unwrap();
                        let visible =
                            bg_index == 0 || lcd_priority == 0 || (bg_priority | obj_priority) == 0;
                        assert_eq!(
                            color(&frame, 2, 0),
                            if visible {
                                palette.color(2, 2)
                            } else {
                                palette.color(0, bg_index)
                            }
                        );
                        // OPRI coordinate mode changes the winner before BG priority.
                        video
                            .render_scanline(
                                &mut frame,
                                0,
                                Registers {
                                    coordinate_priority: true,
                                    lcdc: 0x92,
                                    ..regs
                                },
                                None,
                            )
                            .unwrap();
                        assert_eq!(color(&frame, 2, 0), palette.color(3, 3));
                    }
                }
            }
        }
    }
    #[test]
    fn tall_flipped_objects_and_hidden_entries_consume_selection_slots() {
        let mut vram = vec![0; 0x4000];
        solid(&mut vram, 1, 2, 1);
        solid(&mut vram, 1, 3, 2);
        let palette = palettes();
        let mut oam = vec![[16, 0, 0, 0]; 10];
        oam.push([16, 8, 3, 0x48]); // odd tile ignored, vertical flip across both tiles
        let mut frame = Frame::new(WIDTH, HEIGHT).unwrap();
        let regs = Registers {
            lcdc: 0x97,
            ..Registers::default()
        };
        Video {
            vram: &vram,
            oam: &oam,
            background: &palette,
            objects: &palette,
        }
        .render_scanline(&mut frame, 0, regs, None)
        .unwrap();
        assert_eq!(color(&frame, 0, 0), palette.color(0, 0));
        Video {
            vram: &vram,
            oam: &oam[10..],
            background: &palette,
            objects: &palette,
        }
        .render(&mut frame, regs)
        .unwrap();
        assert_eq!(color(&frame, 0, 0), palette.color(0, 2));
        assert_eq!(color(&frame, 0, 8), palette.color(0, 1));
        assert_eq!(color(&frame, 0, 16), palette.color(0, 0));
    }
}
