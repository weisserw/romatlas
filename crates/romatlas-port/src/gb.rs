//! Selected Game Boy graphics effects, independent of the emulator adapter.
use crate::graphics::Color;
use anyhow::{Result, ensure};
pub const GRAYSCALE: [Color; 4] = [
    [255, 255, 255, 255],
    [170, 170, 170, 255],
    [85, 85, 85, 255],
    [0, 0, 0, 255],
];
pub fn tile_2bpp(bytes: &[u8]) -> Result<[u8; 64]> {
    ensure!(bytes.len() == 16, "Game Boy tile must contain 16 bytes");
    let mut pixels = [0; 64];
    for row in 0..8 {
        for x in 0..8 {
            let bit = 7 - x;
            pixels[row * 8 + x] =
                ((bytes[row * 2] >> bit) & 1) | (((bytes[row * 2 + 1] >> bit) & 1) << 1);
        }
    }
    Ok(pixels)
}
pub fn dmg_palette(register: u8) -> [Color; 4] {
    std::array::from_fn(|index| GRAYSCALE[usize::from((register >> (index * 2)) & 3)])
}
pub fn cgb_color(value: u16) -> Color {
    let expand = |v: u16| ((v * 255 + 15) / 31) as u8;
    [
        expand(value & 31),
        expand((value >> 5) & 31),
        expand((value >> 10) & 31),
        255,
    ]
}
