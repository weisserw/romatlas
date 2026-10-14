//! CPU-side RGBA buffers. Ports can use these or render directly in a frontend.
use anyhow::{Context, Result, ensure};
pub type Color = [u8; 4];
pub struct Frame {
    pub width: u32,
    pub height: u32,
    pub pixels: Vec<u8>,
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
        })
    }
    pub fn clear(&mut self, color: Color) {
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
                frame.pixel(px, py, palette[usize::from(index)]);
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
