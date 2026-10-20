//! Independent MIT helpers for NES native ports. No emulator dependency.
//! Games supply game rules, mapper/timing decisions and ROM-backed assets.
pub mod apu;
pub mod ppu;
/// Decode the NES two-plane 8x8 CHR layout (low plane then high plane).
pub fn tile(bytes: &[u8]) -> anyhow::Result<[u8; 64]> {
    anyhow::ensure!(bytes.len() == 16, "NES tile requires 16 bytes");
    let mut out = [0; 64];
    for y in 0..8 {
        for x in 0..8 {
            out[y * 8 + x] = ((bytes[y] >> (7 - x)) & 1) | (((bytes[y + 8] >> (7 - x)) & 1) << 1);
        }
    }
    Ok(out)
}

/// Standard NES actions use the shared eight-button defaults with NES window size.
pub fn default_settings() -> crate::settings::Settings {
    crate::settings::Settings {
        width: 768,
        height: 720,
        ..Default::default()
    }
}
