//! Raw SMS/Game Gear cartridges, optional 512-byte copier prefix and immutable file identity.
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
    pub rom_offset: usize,
    pub rom_size: usize,
    pub banks: usize,
    pub mapper: String,
    pub mapper_basis: String,
    pub header_offset: Option<usize>,
    pub checksum: Option<u16>,
    pub checksum_valid: Option<bool>,
    pub product_code: Option<u32>,
    pub version: Option<u8>,
    pub region_code: Option<u8>,
    pub declared_size: Option<usize>,
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
        let mut rom = Self::from_bytes(std::fs::read(path)?)?;
        if path
            .extension()
            .is_some_and(|e| e.eq_ignore_ascii_case("gg"))
        {
            rom.info.platform = "gg".into();
        }
        Ok(rom)
    }
    pub fn from_bytes(bytes: Vec<u8>) -> Result<Self> {
        let prefix = if bytes.len() % 8192 == 512 { 512 } else { 0 };
        let data = &bytes[prefix..];
        ensure!(
            data.len() >= 8192 && data.len() <= 4 * 1024 * 1024 && data.len().is_multiple_of(8192),
            "expected SMS/Game Gear ROM in 8 KiB units, up to 4 MiB, with optional 512-byte copier prefix"
        );
        let header = [0x7ff0, 0x3ff0, 0x1ff0]
            .into_iter()
            .find(|&i| data.get(i..i + 8) == Some(b"TMR SEGA"));
        let mut info = Info {
            platform: "sms".into(),
            sha256: format!("{:x}", Sha256::digest(&bytes)),
            size: bytes.len(),
            rom_offset: prefix,
            rom_size: data.len(),
            banks: data.len().div_ceil(16384),
            mapper: "sega".into(),
            mapper_basis: "default hypothesis: Sega headers do not encode a mapper".into(),
            header_offset: header.map(|i| i + prefix),
            checksum: None,
            checksum_valid: None,
            product_code: None,
            version: None,
            region_code: None,
            declared_size: None,
        };
        if data.len() >= 32768 {
            let expected = u16::from_le_bytes([data[0x7fe6], data[0x7fe7]]);
            let sum = (0..data.len())
                .step_by(2)
                .filter(|i| !(0x7ff0..0x8000).contains(i))
                .fold(0u16, |n, i| {
                    n.wrapping_add(u16::from_le_bytes([data[i], data[i + 1]]))
                });
            if sum == expected {
                info.mapper = "codemasters".into();
                info.mapper_basis =
                    "Codemasters word-checksum heuristic (not an encoded mapper ID)".into();
            }
        }
        if let Some(h) = header {
            info.checksum = Some(u16::from_le_bytes([data[h + 10], data[h + 11]]));
            info.product_code = Some(
                u32::from(data[h + 12])
                    + u32::from(data[h + 13]) * 256
                    + u32::from(data[h + 14] >> 4) * 65536,
            );
            info.version = Some(data[h + 14] & 15);
            info.region_code = Some(data[h + 15] >> 4);
            info.declared_size = match data[h + 15] & 15 {
                0xa => Some(8192),
                0xb => Some(16384),
                0xc => Some(32768),
                0xd => Some(49152),
                0xe => Some(65536),
                0xf => Some(131072),
                0 => Some(262144),
                1 => Some(524288),
                2 => Some(1048576),
                _ => None,
            };
            if let Some(n) = info.declared_size.filter(|&n| n <= data.len()) {
                info.checksum_valid = Some(
                    Some(
                        (0..n)
                            .filter(|i| !(h..h + 16).contains(i))
                            .fold(0u16, |s, i| s.wrapping_add(u16::from(data[i]))),
                    ) == info.checksum,
                );
            }
        }
        if info.region_code.is_some_and(|r| matches!(r, 5..=7)) {
            info.platform = "gg".into();
        }
        Ok(Self { bytes, info })
    }
    pub fn open_as(path: &Path, platform: Option<&str>) -> Result<Self> {
        let mut rom = Self::open(path)?;
        if let Some(platform) = platform {
            ensure!(matches!(platform, "sms" | "gg"), "invalid Sega platform");
            rom.info.platform = platform.into();
        }
        Ok(rom)
    }
    pub fn is_gg(&self) -> bool {
        self.info.platform == "gg"
    }
    pub fn map_format(&self) -> String {
        format!("romatlas-{}-map", self.info.platform)
    }
    pub fn data(&self) -> &[u8] {
        &self.bytes[self.info.rom_offset..]
    }
    pub fn check_range(&self, offset: usize, length: usize) -> Result<&[u8]> {
        self.bytes
            .get(offset..offset.checked_add(length).context("range overflow")?)
            .context("range outside image")
    }
    pub fn translate(&self, pc: u32) -> Option<Address> {
        if pc > 65535 {
            return None;
        }
        if pc >= 0xc000 {
            return Some(address("ram", u64::from(pc & 8191)));
        }
        if self.info.mapper == "sega" && pc < 1024 {
            return Some(address("rom", self.info.rom_offset as u64 + u64::from(pc)));
        }
        None
    }
    pub fn canonical_cpu(&self, offset: usize) -> Result<u32> {
        ensure!(
            offset >= self.info.rom_offset && offset < self.bytes.len(),
            "offset is not cartridge ROM"
        );
        let at = offset - self.info.rom_offset;
        ensure!(
            at < 1024 && self.info.mapper == "sega",
            "banked decoding needs explicit CPU address/context"
        );
        Ok(at as u32)
    }
    pub fn binding(&self, b: (&str, u64)) -> Address {
        if b.0 == "rom" {
            address("rom", self.info.rom_offset as u64 + b.1)
        } else {
            address(b.0, b.1)
        }
    }
}
impl ImageAdapter for Rom {
    fn platform(&self) -> &str {
        &self.info.platform
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
