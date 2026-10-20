//! Independent MIT Genesis port helpers. Games own scheduling and behavior.
use anyhow::{Result, ensure};
pub mod audio;
pub mod vdp;
pub const WIDTH: u32 = 320;
pub const HEIGHT: u32 = 224;
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Brightness {
    Shadow,
    Normal,
    Highlight,
}
/// CRAM $0BBB0GGG0RRR0, with digital shadow/highlight levels.
pub fn color(word: u16, level: Brightness) -> [u8; 4] {
    let component = |shift: u32| {
        let v = ((word >> shift) & 7) as u32;
        let n = match level {
            Brightness::Shadow => v,
            Brightness::Normal => v * 2,
            Brightness::Highlight => v + 8,
        };
        ((n * 255 + 7) / 14).min(255) as u8
    };
    [component(1), component(5), component(9), 255]
}
/// Packed 4-bpp, high nibble first; 8x8 tile, 32 bytes.
pub fn tile(bytes: &[u8]) -> Result<[u8; 64]> {
    ensure!(bytes.len() >= 32, "Genesis tile needs 32 bytes");
    Ok(std::array::from_fn(|pixel| {
        (bytes[pixel / 2] >> if pixel % 2 == 0 { 4 } else { 0 }) & 15
    }))
}
pub fn default_settings() -> crate::settings::Settings {
    use crate::input::Binding;
    let mut s = crate::settings::Settings {
        width: WIDTH * 3,
        height: HEIGHT * 3,
        ..Default::default()
    };
    s.bindings.remove("select");
    for (a, k, b) in [
        ("c", "C", "rightshoulder"),
        ("x", "A", "x"),
        ("y", "S", "y"),
        ("z", "D", "leftshoulder"),
        ("mode", "Backspace", "back"),
    ] {
        s.bindings.insert(
            a.into(),
            vec![
                Binding::Key { name: k.into() },
                Binding::Button { name: b.into() },
            ],
        );
    }
    s
}
