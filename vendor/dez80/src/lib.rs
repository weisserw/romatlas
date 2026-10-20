//! Provides Z80 disassembly and analysis models and functions.

extern crate strum;
extern crate strum_macros;

mod decoder;
pub mod instruction;
pub mod register;

pub use decoder::InstructionDecoder;
pub use instruction::{DecodingState, Instruction};
