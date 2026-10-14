//! CPU-side RGBA buffers. Ports can use these or render directly in a frontend.
use anyhow::{Context, Result, ensure};
pub type Color = [u8; 4];
pub struct Frame {
    pub width: u32,
    pub height: u32,
    pub pixels: Vec<u8>,
    background_indices: Vec<u8>,
}
impl Frame {
    pub fn new(width: u32, height: u32) -> Result<Self> {
        ensure!(
            width > 0 && height > 0 && width <= 4096 && height <= 4096,
            "invalid frame dimensions"
        );
        Ok(Self {
            width,
            height,
            pixels: vec![0; width as usize * height as usize * 4],
            background_indices: vec![0; width as usize * height as usize],
        })
    }
    pub fn clear(&mut self, color: Color) {
        self.background_indices.fill(0);
        for pixel in self.pixels.chunks_exact_mut(4) {
            pixel.copy_from_slice(&color);
        }
    }
    pub fn pixel(&mut self, x: i32, y: i32, color: Color) {
        if x >= 0 && y >= 0 && (x as u32) < self.width && (y as u32) < self.height {
            let index = (y as usize * self.width as usize + x as usize) * 4;
            self.pixels[index..index + 4].copy_from_slice(&color);
        }
    }
    /// Preserve a background's source index independently of its palette color.
    pub fn background_index(&self, x: i32, y: i32) -> Option<u8> {
        if x < 0 || y < 0 || x as u32 >= self.width || y as u32 >= self.height {
            return None;
        }
        Some(self.background_indices[y as usize * self.width as usize + x as usize])
    }
    /// Draw and retain a background index without changing sprite state.
    pub fn background_pixel(&mut self, x: i32, y: i32, index: u8, color: Color) {
        self.pixel(x, y, color);
        if x >= 0 && y >= 0 && (x as u32) < self.width && (y as u32) < self.height {
            self.background_indices[y as usize * self.width as usize + x as usize] = index;
        }
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct TileStyle {
    pub flip_x: bool,
    pub flip_y: bool,
    pub transparent: Option<u8>,
}
pub fn tile(
    frame: &mut Frame,
    indices: &[u8; 64],
    palette: &[Color],
    x: i32,
    y: i32,
    style: TileStyle,
) -> Result<()> {
    draw_tile(frame, indices, palette, x, y, style, Layer::Color)
}
/// Draw indexed background pixels for later sprite-priority decisions.
pub fn background_tile(
    frame: &mut Frame,
    indices: &[u8; 64],
    palette: &[Color],
    x: i32,
    y: i32,
    style: TileStyle,
) -> Result<()> {
    draw_tile(frame, indices, palette, x, y, style, Layer::Background)
}
/// Draw a sprite; priority compares source BG index, never rendered color.
pub fn sprite_tile(
    frame: &mut Frame,
    indices: &[u8; 64],
    palette: &[Color],
    x: i32,
    y: i32,
    style: TileStyle,
    behind_background: bool,
) -> Result<()> {
    draw_tile(
        frame,
        indices,
        palette,
        x,
        y,
        style,
        Layer::Sprite(behind_background),
    )
}
enum Layer {
    Color,
    Background,
    Sprite(bool),
}
fn draw_tile(
    frame: &mut Frame,
    indices: &[u8; 64],
    palette: &[Color],
    x: i32,
    y: i32,
    style: TileStyle,
    layer: Layer,
) -> Result<()> {
    // Validate even pixels that will be clipped or transparent.
    for index in indices {
        ensure!(
            usize::from(*index) < palette.len(),
            "palette index out of range"
        );
    }
    for row in 0..8 {
        for column in 0..8 {
            let source_x = if style.flip_x { 7 - column } else { column };
            let source_y = if style.flip_y { 7 - row } else { row };
            let index = indices[source_y * 8 + source_x];
            if Some(index) != style.transparent {
                let px = x
                    .checked_add(column as i32)
                    .context("tile position overflow")?;
                let py = y
                    .checked_add(row as i32)
                    .context("tile position overflow")?;
                if px < 0 || py < 0 || px as u32 >= frame.width || py as u32 >= frame.height {
                    continue;
                }
                let position = py as usize * frame.width as usize + px as usize;
                match layer {
                    Layer::Background => {
                        frame.background_pixel(px, py, index, palette[usize::from(index)])
                    }
                    Layer::Sprite(true) if frame.background_indices[position] != 0 => {}
                    _ => frame.pixel(px, py, palette[usize::from(index)]),
                }
            }
        }
    }
    Ok(())
}

pub fn tilemap(
    frame: &mut Frame,
    tiles: &[[u8; 64]],
    map: &[usize],
    columns: usize,
    palette: &[Color],
    origin: (i32, i32),
) -> Result<()> {
    ensure!(
        columns > 0 && columns <= 512 && map.len() <= 512 * 512,
        "invalid tile map dimensions"
    );
    for (position, index) in map.iter().enumerate() {
        let indices = tiles.get(*index).context("tile map index out of range")?;
        let x = origin
            .0
            .checked_add((position % columns * 8) as i32)
            .context("tile map overflow")?;
        let y = origin
            .1
            .checked_add((position / columns * 8) as i32)
            .context("tile map overflow")?;
        tile(frame, indices, palette, x, y, TileStyle::default())?;
    }
    Ok(())
}

#[cfg(test)]
mod priority_tests {
    use super::*;
    #[test]
    fn priority_uses_source_index_even_when_palette_colors_match() {
        let mut frame = Frame::new(8, 8).unwrap();
        let palette = [[255, 255, 255, 255]; 4];
        let mut background = [0; 64];
        background[1] = 2;
        background_tile(
            &mut frame,
            &background,
            &palette,
            0,
            0,
            TileStyle::default(),
        )
        .unwrap();
        sprite_tile(
            &mut frame,
            &[1; 64],
            &[[0, 0, 0, 255]; 4],
            0,
            0,
            TileStyle {
                transparent: Some(0),
                ..Default::default()
            },
            true,
        )
        .unwrap();
        assert_eq!(&frame.pixels[..4], &[0, 0, 0, 255]);
        assert_eq!(&frame.pixels[4..8], &[255, 255, 255, 255]);
        frame.clear([255, 255, 255, 255]);
        sprite_tile(
            &mut frame,
            &[1; 64],
            &[[0, 0, 0, 255]; 4],
            0,
            0,
            TileStyle::default(),
            true,
        )
        .unwrap();
        assert_eq!(&frame.pixels[4..8], &[0, 0, 0, 255]);
    }
}
