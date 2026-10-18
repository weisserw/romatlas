//! SNES platform adapter. Shared crates never depend on this implementation.
pub mod image;
pub use image::Rom;

pub mod decode;

pub mod emulator;

pub mod disassembly;
pub mod mapping;

pub mod analysis;
pub mod session;
