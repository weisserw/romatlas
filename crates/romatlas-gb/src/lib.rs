//! Game Boy image parsing and physical ROM address translation.
use anyhow::{Result, bail};
use romatlas_core::{Address, ImageAdapter};
use serde::Serialize;
use sha2::{Digest, Sha256};
use std::{collections::BTreeMap, path::Path};
#[derive(Clone)]
pub struct Rom {
    pub bytes: Vec<u8>,
    pub info: Info,
}
#[derive(Clone, Serialize)]
pub struct Info {
    pub sha256: String,
    pub size: usize,
    pub banks: usize,
    pub cartridge_type: u8,
    pub cgb_flag: u8,
    pub declared_rom_size: u8,
    pub ram_size: u8,
    pub controller: String,
    pub header_checksum_valid: bool,
}
impl Rom {
    pub fn open(path: &Path) -> Result<Self> {
        Self::from_bytes(std::fs::read(path)?)
    }
    pub fn from_bytes(bytes: Vec<u8>) -> Result<Self> {
        if bytes.len() < 0x8000 || !bytes.len().is_multiple_of(0x4000) {
            bail!("ROM must contain complete 16 KiB banks and at least 32 KiB");
        }
        let cartridge_type = bytes[0x147];
        let controller = match cartridge_type {
            0 | 8 | 9 => "ROM",
            1..=3 => "MBC1",
            0x0f..=0x13 => "MBC3",
            0x19..=0x1e => "MBC5",
            _ => "unsupported",
        }
        .to_string();
        let checksum = bytes[0x134..0x14d]
            .iter()
            .fold(0u8, |a, b| a.wrapping_sub(*b).wrapping_sub(1));
        let info = Info {
            sha256: format!("{:x}", Sha256::digest(&bytes)),
            size: bytes.len(),
            banks: bytes.len() / 0x4000,
            cartridge_type,
            cgb_flag: bytes[0x143],
            declared_rom_size: bytes[0x148],
            ram_size: bytes[0x149],
            controller,
            header_checksum_valid: checksum == bytes[0x14d],
        };
        Ok(Self { bytes, info })
    }
    pub fn sym_location(offset: usize) -> (u32, u32) {
        let bank = offset / 0x4000;
        (
            bank as u32,
            (offset % 0x4000 + if bank == 0 { 0 } else { 0x4000 }) as u32,
        )
    }
}
impl ImageAdapter for Rom {
    fn platform(&self) -> &str {
        "gb"
    }
    fn image_size(&self) -> u64 {
        self.bytes.len() as u64
    }
    fn physical_address(&self, offset: u64) -> Option<Address> {
        if offset >= self.image_size() {
            return None;
        }
        let mut context = BTreeMap::new();
        context.insert("bank".into(), (offset / 0x4000).to_string());
        Some(Address {
            space: "cartridge-rom".into(),
            offset,
            context,
        })
    }
    fn peek(&self, address: &Address, length: usize) -> Result<Vec<u8>, String> {
        if address.space != "cartridge-rom" {
            return Err("unsupported address space".into());
        }
        let start = usize::try_from(address.offset).map_err(|_| "address overflow")?;
        let end = start.checked_add(length).ok_or("range overflow")?;
        self.bytes
            .get(start..end)
            .map(|s| s.to_vec())
            .ok_or("range outside ROM".into())
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn physical_banks() {
        assert_eq!(Rom::sym_location(0x8123), (2, 0x4123));
        assert_eq!(Rom::sym_location(0x123), (0, 0x123));
    }
    #[test]
    fn rejects_partial() {
        assert!(Rom::from_bytes(vec![0; 0x8001]).is_err());
    }
}

pub mod decode;

pub mod mapping;

pub mod emulator;

pub mod tracing;

pub mod discovery;
pub mod discovery_profiles;

pub mod pointers;

#[cfg(test)]
mod decode_golden;

pub mod disassembly;

pub mod execution;

pub mod edges;
