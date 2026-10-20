//! iNES physical file identity. Headers, trainers, PRG, CHR and trailing bytes survive rebuilding.
use anyhow::{Context, Result, ensure};
use romatlas_core::{Address, ImageAdapter};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::{collections::BTreeMap, path::Path};
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Info {
    pub platform: String,
    pub sha256: String,
    pub size: usize,
    pub mapper: u16,
    pub submapper: u8,
    pub nes2: bool,
    pub prg_offset: usize,
    pub prg_size: usize,
    pub chr_offset: usize,
    pub chr_size: usize,
    pub trainer: bool,
    pub battery: bool,
    pub mirroring: String,
}
#[derive(Clone)]
pub struct Rom {
    pub bytes: Vec<u8>,
    pub info: Info,
}
pub fn address(space: &str, offset: u64) -> Address {
    Address {
        space: space.into(),
        offset,
        context: BTreeMap::new(),
    }
}
impl Rom {
    pub fn open(path: &Path) -> Result<Self> {
        Self::from_bytes(std::fs::read(path)?)
    }
    pub fn from_bytes(bytes: Vec<u8>) -> Result<Self> {
        ensure!(
            bytes.len() >= 16 && &bytes[..4] == b"NES\x1a",
            "expected iNES/NES 2.0 image"
        );
        ensure!(bytes.len() <= 64 * 1024 * 1024, "NES image exceeds 64 MiB");
        let nes2 = bytes[7] & 12 == 8;
        ensure!(
            bytes[7] & 3 == 0,
            "VS/PlayChoice/extended console unsupported"
        );
        let mapper = u16::from(bytes[6] >> 4 | bytes[7] & 0xf0)
            | if nes2 {
                u16::from(bytes[8] & 15) << 8
            } else {
                0
            };
        let trainer = bytes[6] & 4 != 0;
        let size = |low: u8, high: u8, unit: usize| -> Result<usize> {
            if high == 15 {
                (1usize.checked_shl(u32::from(low >> 2)))
                    .and_then(|n| n.checked_mul(usize::from((low & 3) * 2 + 1)))
                    .context("NES 2.0 size overflow")
            } else {
                Ok((usize::from(low) | usize::from(high) << 8) * unit)
            }
        };
        let prg_size = size(bytes[4], if nes2 { bytes[9] & 15 } else { 0 }, 16384)?;
        let chr_size = size(bytes[5], if nes2 { bytes[9] >> 4 } else { 0 }, 8192)?;
        let prg_offset = 16 + if trainer { 512 } else { 0 };
        let chr_offset = prg_offset + prg_size;
        ensure!(
            prg_size > 0
                && chr_offset
                    .checked_add(chr_size)
                    .is_some_and(|end| end <= bytes.len()),
            "truncated cartridge sections"
        );
        let info = Info {
            platform: "nes".into(),
            sha256: format!("{:x}", Sha256::digest(&bytes)),
            size: bytes.len(),
            mapper,
            submapper: if nes2 { bytes[8] >> 4 } else { 0 },
            nes2,
            prg_offset,
            prg_size,
            chr_offset,
            chr_size,
            trainer,
            battery: bytes[6] & 2 != 0,
            mirroring: if bytes[6] & 8 != 0 {
                "four_screen"
            } else if bytes[6] & 1 != 0 {
                "vertical"
            } else {
                "horizontal"
            }
            .into(),
        };
        Ok(Self { bytes, info })
    }
    pub fn check_range(&self, offset: usize, length: usize) -> Result<&[u8]> {
        self.bytes
            .get(offset..offset.checked_add(length).context("range overflow")?)
            .context("range outside image")
    }
    /// Only NROM/CNROM have a unique static PRG mapping. Other mappers need explicit slots.
    pub fn translate(&self, pc: u32) -> Option<Address> {
        if pc < 0x2000 {
            return Some(address("ram", u64::from(pc & 0x7ff)));
        }
        if (0x8000..=0xffff).contains(&pc) && matches!(self.info.mapper, 0 | 3) {
            return Some(address(
                "rom",
                (self.info.prg_offset + (pc as usize - 0x8000) % self.info.prg_size) as u64,
            ));
        }
        None
    }
    pub fn canonical_cpu(&self, offset: usize) -> Result<u32> {
        ensure!(
            offset >= self.info.prg_offset && offset < self.info.chr_offset,
            "offset is not PRG ROM"
        );
        ensure!(
            matches!(self.info.mapper, 0 | 3),
            "banked PRG decoding needs explicit --cpu-address and bank context"
        );
        Ok(0x8000 + (offset - self.info.prg_offset) as u32)
    }
    pub fn binding(&self, binding: (&str, u64)) -> Address {
        match binding.0 {
            "prg_rom" => address("rom", self.info.prg_offset as u64 + binding.1),
            "chr_rom" => address("rom", self.info.chr_offset as u64 + binding.1),
            s => address(s, binding.1),
        }
    }
}
impl ImageAdapter for Rom {
    fn platform(&self) -> &str {
        "nes"
    }
    fn image_size(&self) -> u64 {
        self.bytes.len() as u64
    }
    fn physical_address(&self, offset: u64) -> Option<Address> {
        (offset < self.image_size()).then(|| address("rom", offset))
    }
    fn peek(&self, a: &Address, n: usize) -> Result<Vec<u8>, String> {
        if a.space != "rom" {
            return Err("image exposes only physical rom".into());
        }
        self.check_range(a.offset as usize, n)
            .map(Vec::from)
            .map_err(|e| e.to_string())
    }
}
