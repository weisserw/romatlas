//! SMS platform adapter; GPL core remains outside shared/native-port crates.
pub mod image;
pub use image::Rom;
pub mod analysis;
pub mod decode;
pub mod disassembly;
pub mod emulator;
pub mod mapping;
pub mod session;
