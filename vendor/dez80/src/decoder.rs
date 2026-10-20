//! Contains an interface for stateful decoding that does not require
//! all bytes of an instruction to be available at once.
//!
//! # Example
//!
//! ```
//! use dez80::InstructionDecoder;
//!
//! // Initialize a stateful instruction decoder.
//! let mut decoder = InstructionDecoder::default();
//!
//! // Decode a single byte with the decoder.
//! decoder.push_byte(0x00); // NOP
//! let result = decoder.try_decode();
//!
//! // This is a single-byte instruction, so we can verify
//! // that a valid instruction was returned.
//! assert!(result.is_ok());
//! assert_eq!("NOP", result.unwrap().to_string());
//!
//! // Decode the first byte of a multi-byte instruction.
//! decoder.push_byte(0xED); // extended instruction
//! let result = decoder.try_decode();
//!
//! // No instruction can be decoded from this byte alone.
//! assert!(result.is_err());
//!
//! // Decode the second byte of the instruction.
//! // This time, the instruction can finish decoding.
//! decoder.push_byte(0x44); // NEG
//! let result = decoder.try_decode();
//! assert!(result.is_ok());
//! assert_eq!("NEG", result.unwrap().to_string());
//! ```

use crate::instruction::*;

/// Represents an instruction decoder that maintains a state.
/// Its main use case is to decode instructions progressively
/// byte by byte, when a data source cannot implement `Read`.
#[derive(Clone, Debug, Default)]
pub struct InstructionDecoder {
    received_bytes: Vec<u8>,
}

impl InstructionDecoder {
    /// Attempts to decode one instruction from the decoder's source.
    /// If there is not enough data to complete the decoding process,
    /// an `Err<DecodingState>` is returned, which describes the state
    /// of the decoder prior to running out of data.
    /// If an instruction is successfully decoded, an `Ok<Instruction>`
    /// is returned, and its bytes are drained from the source.
    pub fn try_decode(&mut self) -> Result<Instruction, DecodingState> {
        let result = Instruction::decode_one(&mut self.received_bytes.as_slice());
        if let Ok(instruction) = result.clone() {
            self.received_bytes.drain(0..instruction.to_bytes().len());
        }

        result
    }

    /// Pushes an opcode byte to the decoder source.
    pub fn push_byte(&mut self, byte: u8) {
        self.received_bytes.push(byte);
    }

    /// Pushes a slice of opcode bytes to the decoder source.
    pub fn push_slice(&mut self, slice: &[u8]) {
        self.received_bytes.extend_from_slice(slice);
    }

    /// Resets the decoder state, allowing decoding to start from scratch.
    pub fn reset(&mut self) {
        self.received_bytes.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decode_single_byte_with_decoder() {
        let instruction_byte = 0x00;
        let mut decoder = InstructionDecoder::default();
        decoder.push_byte(instruction_byte);
        let result = decoder.try_decode();
        assert!(result.is_ok());
        assert_eq!(&[instruction_byte], result.unwrap().to_bytes().as_slice());
        assert_eq!(0, decoder.received_bytes.len());
    }

    #[test]
    fn decode_multiple_bytes_with_decoder() {
        let instruction_bytes = &[0x01, 0x02, 0x03];
        let mut decoder = InstructionDecoder::default();
        decoder.push_byte(instruction_bytes[0]); // LD BC, **
        assert!(decoder.try_decode().is_err());
        decoder.push_byte(instruction_bytes[1]); // LD BC, 0x**02
        assert!(decoder.try_decode().is_err());
        decoder.push_byte(instruction_bytes[2]); // LD BC, 0x0302
        let result = decoder.try_decode();
        assert!(result.is_ok());
        assert_eq!(instruction_bytes, result.unwrap().to_bytes().as_slice());
        assert_eq!(0, decoder.received_bytes.len());
    }

    #[test]
    fn decode_slice_with_decoder() {
        let instruction_bytes = &[0x01, 0x33, 0x22];
        let mut decoder = InstructionDecoder::default();
        decoder.push_slice(instruction_bytes);
        let result = decoder.try_decode();
        assert!(result.is_ok());
        assert_eq!(instruction_bytes, result.unwrap().to_bytes().as_slice());
        assert_eq!(0, decoder.received_bytes.len());
    }

    #[test]
    fn decode_two_instruction_slice_with_decoder() {
        let instruction_bytes = &[0x06, 0x11, 0x00];
        let mut decoder = InstructionDecoder::default();

        // LD B, *
        decoder.push_slice(instruction_bytes);
        let result = decoder.try_decode();
        assert!(result.is_ok());
        assert_eq!(
            &instruction_bytes[0..=1],
            result.unwrap().to_bytes().as_slice()
        );
        assert_eq!(1, decoder.received_bytes.len());

        // NOP
        decoder.push_slice(&[]);
        let result = decoder.try_decode();
        assert!(result.is_ok());
        assert_eq!(
            &instruction_bytes[2..],
            result.unwrap().to_bytes().as_slice()
        );
        assert_eq!(0, decoder.received_bytes.len());
    }
}
