//! Original cartridge bytes and normalized big-endian bus image remain distinct.
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
    pub rom_size: usize,
    pub rom_offset: usize,
    pub layout: String,
    pub banks: usize,
    pub mapper: String,
    pub save_hardware: String,
    pub eeprom_range: Option<(u32, u32)>,
    pub mapper_basis: String,
    pub system: String,
    pub domestic_title: String,
    pub overseas_title: String,
    pub serial: String,
    pub regions: String,
    pub checksum: u16,
    pub checksum_valid: bool,
    pub initial_sp: u32,
    pub reset_vector: u32,
    pub declared_rom_start: u32,
    pub declared_rom_end: u32,
    pub ram_start: Option<u32>,
    pub ram_end: Option<u32>,
    pub ram_type: Option<u8>,
}
#[derive(Clone)]
pub struct Rom {
    pub bytes: Vec<u8>,
    original: Vec<u8>,
    pub info: Info,
    payload: Vec<u8>,
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
    pub fn open_as(path: &Path, platform: Option<&str>) -> Result<Self> {
        ensure!(
            platform.is_none_or(|p| p == "genesis"),
            "invalid Genesis platform"
        );
        Self::open(path)
    }
    pub fn from_bytes(bytes: Vec<u8>) -> Result<Self> {
        ensure!(
            (512..=16 * 1024 * 1024 + 512).contains(&bytes.len()),
            "Genesis image size outside 512 bytes..16 MiB"
        );
        let prefix = if bytes.len() % 16384 == 512 { 512 } else { 0 };
        let mut payload = bytes[prefix..].to_vec();
        let layout = if prefix == 512
            && payload.len().is_multiple_of(16384)
            && bytes.get(8..11) == Some(&[0xaa, 0xbb, 6])
        {
            let raw = payload.clone();
            for (src, dst) in raw.chunks_exact(16384).zip(payload.chunks_exact_mut(16384)) {
                for i in 0..8192 {
                    dst[2 * i] = src[8192 + i];
                    dst[2 * i + 1] = src[i];
                }
            }
            "smd_interleaved"
        } else if payload.get(0x100..0x104) == Some(b"ESAG") {
            for pair in payload.chunks_exact_mut(2) {
                pair.swap(0, 1);
            }
            "word_swapped"
        } else {
            "big_endian"
        };
        ensure!(
            payload.len() >= 512 && payload.len().is_multiple_of(2),
            "Genesis payload must have an even size >=512"
        );
        let word = |at| u16::from_be_bytes(payload[at..at + 2].try_into().unwrap());
        let long = |at| u32::from_be_bytes(payload[at..at + 4].try_into().unwrap());
        let text = |a, b| {
            String::from_utf8_lossy(&payload[a..b])
                .trim_matches(|c: char| c == '\0' || c.is_whitespace())
                .to_owned()
        };
        let serial = text(0x180, 0x18e);
        let system = text(0x100, 0x110);
        let (mapper, save_hardware) = genesis_core::cartridge::romatlas_identify(&payload);
        let ssf = mapper == "ssf";
        let has_ram = &payload[0x1b0..0x1b2] == b"RA";
        let info = Info {
            platform: "genesis".into(),
            sha256: format!("{:x}", Sha256::digest(&bytes)),
            size: bytes.len(),
            rom_size: payload.len(),
            rom_offset: prefix,
            layout: layout.into(),
            banks: payload.len().div_ceil(0x80000),
            mapper: mapper.into(),
            save_hardware: save_hardware.into(),
            eeprom_range: genesis_core::cartridge::romatlas_eeprom_range(&payload),
            mapper_basis: if !matches!(mapper, "ssf" | "linear") {
                "recognized unsupported enhancement/board by pinned serial/CRC conventions"
            } else if ssf {
                "SEGA SSF/DOA system or recognized SSF2 serial"
            } else {
                "linear cartridge hypothesis; Genesis has no universal mapper ID"
            }
            .into(),
            system,
            domestic_title: text(0x120, 0x150),
            overseas_title: text(0x150, 0x180),
            serial,
            regions: text(0x1f0, 0x200),
            checksum: word(0x18e),
            checksum_valid: word(0x18e)
                == payload[512..].chunks_exact(2).fold(0u16, |s, b| {
                    s.wrapping_add(u16::from_be_bytes([b[0], b[1]]))
                }),
            initial_sp: long(0),
            reset_vector: long(4),
            declared_rom_start: long(0x1a0),
            declared_rom_end: long(0x1a4),
            ram_start: has_ram.then(|| long(0x1b4)),
            ram_end: has_ram.then(|| long(0x1b8)),
            ram_type: has_ram.then(|| payload[0x1b2]),
        };
        if has_ram {
            ensure!(
                info.ram_start <= info.ram_end && info.ram_end.unwrap() < 0x1000000,
                "invalid SRAM extent"
            );
        }
        let mut canonical = bytes[..prefix].to_vec();
        canonical.extend_from_slice(&payload);
        Ok(Self {
            bytes: canonical,
            original: bytes,
            info,
            payload,
        })
    }
    pub fn original(&self) -> &[u8] {
        &self.original
    }

    pub fn data(&self) -> &[u8] {
        &self.payload
    }
    pub fn map_format(&self) -> String {
        "romatlas-genesis-map".into()
    }
    pub fn check_range(&self, o: usize, n: usize) -> Result<&[u8]> {
        self.bytes
            .get(o..o.checked_add(n).context("range overflow")?)
            .context("range outside image")
    }
    pub fn file_offset(&self, at: usize) -> Option<usize> {
        if at >= self.payload.len() {
            return None;
        }
        Some(
            self.info.rom_offset
                + match self.info.layout.as_str() {
                    "word_swapped" => at ^ 1,
                    "smd_interleaved" => {
                        (at / 16384) * 16384
                            + (at % 16384) / 2
                            + if at.is_multiple_of(2) { 8192 } else { 0 }
                    }
                    _ => at,
                },
        )
    }
    pub fn payload_offset(&self, at: usize) -> Option<usize> {
        let at = at.checked_sub(self.info.rom_offset)?;
        (at < self.payload.len()).then_some(at)
    }
    pub fn canonical_cpu(&self, offset: usize) -> Result<u32> {
        let at = self.payload_offset(offset).context("not cartridge ROM")?;
        ensure!(
            self.info.mapper == "linear" && at < 0x400000,
            "banked address needs CPU context"
        );
        Ok(at as u32)
    }
    pub fn translate(&self, pc: u32) -> Option<Address> {
        if pc >= 0x1000000 {
            return None;
        }
        if pc >= 0xe00000 {
            return Some(address("ram", u64::from(pc & 65535)));
        }
        if pc < 0x400000 && self.info.mapper == "linear" {
            return ((pc as usize) < self.payload.len())
                .then(|| address("rom", self.info.rom_offset as u64 + u64::from(pc)));
        }
        None
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
        "genesis"
    }
    fn image_size(&self) -> u64 {
        self.bytes.len() as u64
    }
    fn physical_address(&self, offset: u64) -> Option<Address> {
        (offset < self.image_size()).then(|| address("rom", offset))
    }
    fn peek(&self, a: &Address, n: usize) -> Result<Vec<u8>, String> {
        if a.space != "rom" {
            return Err("image exposes physical ROM only".into());
        }
        self.check_range(a.offset as usize, n)
            .map(Vec::from)
            .map_err(|e| e.to_string())
    }
}
