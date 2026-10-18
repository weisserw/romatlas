//! Native-port SNES asset helpers. Original implementation, no emulator dependency.
//! Ports own layer composition, timing, gameplay and sound behavior.
use crate::graphics::Color;
use anyhow::{Result, ensure};
/// Decode an 8x8 SNES planar tile. Planes are stored in pairs 16 bytes apart.
pub fn tile(bytes: &[u8], bits_per_pixel: u8) -> Result<[u8; 64]> {
    ensure!(
        matches!(bits_per_pixel, 2 | 4 | 8),
        "SNES tile depth must be 2, 4 or 8"
    );
    ensure!(
        bytes.len() == usize::from(bits_per_pixel) * 8,
        "wrong SNES tile byte length"
    );
    let mut out = [0; 64];
    for plane in 0..usize::from(bits_per_pixel) {
        for row in 0..8 {
            let data = bytes[(plane / 2) * 16 + row * 2 + (plane & 1)];
            for x in 0..8 {
                out[row * 8 + x] |= ((data >> (7 - x)) & 1) << plane;
            }
        }
    }
    Ok(out)
}
/// Convert a little-endian CGRAM BGR555 value to linear full-range RGBA8.
/// Display-specific gamma/color correction is a separate frontend policy.
pub fn color(value: u16) -> Color {
    let expand = |v: u16| ((v * 255 + 15) / 31) as u8;
    [
        expand(value & 31),
        expand((value >> 5) & 31),
        expand((value >> 10) & 31),
        255,
    ]
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TilemapEntry {
    pub tile: u16,
    pub palette: u8,
    pub priority: bool,
    pub flip_x: bool,
    pub flip_y: bool,
}
impl TilemapEntry {
    pub fn from_word(word: u16) -> Self {
        Self {
            tile: word & 0x3ff,
            palette: ((word >> 10) & 7) as u8,
            priority: word & 0x2000 != 0,
            flip_x: word & 0x4000 != 0,
            flip_y: word & 0x8000 != 0,
        }
    }
}
/// Decode Mode 7's chunky 8bpp tile from the character bytes (not the interleaved
/// VRAM tilemap/character storage). Zero transparency remains a composition rule.
pub fn mode7_tile(bytes: &[u8]) -> Result<[u8; 64]> {
    ensure!(bytes.len() == 64, "Mode 7 tile needs 64 character bytes");
    Ok(bytes.try_into().unwrap())
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn separated_plane_pairs_and_palette_bits() {
        let mut bytes = [0; 64];
        bytes[0] = 0x80;
        bytes[17] = 0x80;
        bytes[32] = 0x80;
        bytes[49] = 0x80;
        let decoded = tile(&bytes, 8).unwrap();
        assert_eq!(decoded[0], 1 | 8 | 16 | 128);
        assert_eq!(decoded[1], 0);
        assert!(tile(&bytes, 4).is_err());
        assert!(tile(&bytes, 3).is_err());
        assert_eq!(color(0x001f), [255, 0, 0, 255]);
        assert_eq!(color(0x7c00), [0, 0, 255, 255]);
        assert_eq!(
            TilemapEntry::from_word(0xffff),
            TilemapEntry {
                tile: 1023,
                palette: 7,
                priority: true,
                flip_x: true,
                flip_y: true
            }
        );
    }
}

/// Ordinary SNES-style actions for a new port; games may replace these with their
/// own action names. This does not change the Game Boy/default settings profile.
pub fn default_settings() -> crate::settings::Settings {
    use crate::input::Binding;
    let mut settings = crate::settings::Settings {
        width: 768,
        height: 672,
        ..Default::default()
    };
    for (action, key, button) in [
        ("a", "X", "b"),
        ("b", "Z", "a"),
        ("x", "S", "y"),
        ("y", "A", "x"),
        ("l", "Q", "leftshoulder"),
        ("r", "W", "rightshoulder"),
    ] {
        settings.bindings.insert(
            action.into(),
            vec![
                Binding::Key { name: key.into() },
                Binding::Button {
                    name: button.into(),
                },
            ],
        );
    }
    settings
}

#[cfg(test)]
mod input_defaults_tests {
    use super::*;
    #[test]
    fn ordinary_controller_layout_is_independent_of_gb_defaults() {
        let settings = default_settings();
        settings.validate().unwrap();
        assert_eq!(settings.bindings.len(), 12);
        let mut physical = crate::input::PhysicalInput::default();
        physical.buttons.insert("a".into());
        let state = crate::input::InputMapper::default().sample(&physical, &settings);
        assert!(state.held("b") && !state.held("a"));
        assert_eq!(crate::settings::Settings::default().bindings.len(), 8);
    }
}
