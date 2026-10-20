//! Genesis platform adapter; GPL core remains outside shared/native-port crates.
pub mod image;
pub use image::Rom;
pub mod analysis;
pub mod decode;
pub mod disassembly;
pub mod emulator;
pub mod mapping;
pub mod session;

/// The pinned locally built cross tools take precedence over system cross tools.
pub fn tool(name: &str) -> std::path::PathBuf {
    let local = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../tools/m68k/bin")
        .join(format!("m68k-elf-{name}"));
    if local.is_file() {
        local
    } else {
        std::path::PathBuf::from(format!("m68k-linux-gnu-{name}"))
    }
}
