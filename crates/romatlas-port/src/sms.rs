//! Independent MIT SMS native-port helpers. No console emulator dependencies.
pub mod audio;
pub mod vdp;
pub const WIDTH: u32 = 256;
pub const HEIGHT: u32 = 192;
/// SMS four interleaved bitplanes per row, MSB at the left.
pub fn tile(bytes: &[u8]) -> anyhow::Result<[u8; 64]> {
    anyhow::ensure!(bytes.len() == 32, "SMS tile requires 32 bytes");
    let mut pixels = [0; 64];
    for y in 0..8 {
        for x in 0..8 {
            for plane in 0..4 {
                pixels[y * 8 + x] |= ((bytes[y * 4 + plane] >> (7 - x)) & 1) << plane;
            }
        }
    }
    Ok(pixels)
}
pub fn color(value: u8) -> [u8; 4] {
    [
        (value & 3) * 85,
        ((value >> 2) & 3) * 85,
        ((value >> 4) & 3) * 85,
        255,
    ]
}
pub fn default_settings() -> crate::settings::Settings {
    let mut s = crate::settings::Settings {
        width: 768,
        height: 576,
        ..Default::default()
    };
    for (old, new) in [("a", "button1"), ("b", "button2"), ("start", "pause")] {
        if let Some(v) = s.bindings.remove(old) {
            s.bindings.insert(new.into(), v);
        }
    }
    s.bindings.remove("select");
    s
}
