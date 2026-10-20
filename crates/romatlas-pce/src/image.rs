//! Original HuCard identity; no commercial database or implicit header repair.
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
    pub payload_offset: usize,
    pub payload_size: usize,
    pub physical_bank_size: usize,
    pub banks: usize,
    pub cartridge_header: Option<String>,
    pub copier_header: bool,
    pub mapper: String,
    pub mapper_basis: String,
    pub reset_vector: u16,
    pub layout: String,
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
        let payload_offset = if bytes.len() % 8192 == 512 { 512 } else { 0 };
        let size = bytes.len() - payload_offset;
        ensure!(
            (8192..=2560 * 1024).contains(&size) && size.is_multiple_of(8192),
            "expected HuCard image in 8 KiB units, optionally with 512-byte copier prefix; CD images unsupported"
        );
        let crc = crc::Crc::<u32>::new(&crc::CRC_32_ISO_HDLC).checksum(&bytes[payload_offset..]);
        let mapper = match crc {
            0xdb5f97b3 => "populous_sram",
            0x33deb700 => "sf2",
            _ if size > 1024 * 1024 => "unknown",
            _ => "linear",
        };
        let info = Info {
            platform: "pce".into(),
            sha256: format!("{:x}", Sha256::digest(&bytes)),
            size: bytes.len(),
            payload_offset,
            payload_size: size,
            physical_bank_size: 8192,
            banks: size / 8192,
            cartridge_header: None,
            copier_header: payload_offset == 512,
            mapper: mapper.into(),
            mapper_basis: if matches!(mapper, "populous_sram" | "sf2") {
                "upstream known CRC32"
            } else {
                "size heuristic; HuCards have no universal mapper/header field"
            }
            .into(),
            reset_vector: u16::from_le_bytes([
                bytes[payload_offset + 0x1ffe],
                bytes[payload_offset + 0x1fff],
            ]),
            layout: match size {
                393216 => "256 KiB mirrored twice + 128 KiB mirrored four times",
                524288 => "512 KiB compatibility layout; upper 256 KiB repeated in upper half",
                _ => "linear with next-power-of-two mirroring",
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
    pub fn canonical_cpu(&self, offset: usize) -> Result<u32> {
        ensure!(
            offset >= self.info.payload_offset && offset < self.bytes.len(),
            "not HuCard payload"
        );
        Ok(0xe000 + ((offset - self.info.payload_offset) & 8191) as u32)
    }
    /// MPR is an effective 21-bit bus page, not necessarily an immutable ROM bank.
    pub fn bus_binding(&self, bus: u32, sf2_bank: Option<u8>) -> Option<Address> {
        match bus {
            0..=0xfffff => {
                if self.info.mapper == "populous_sram" && (0x80000..0x88000).contains(&bus) {
                    return Some(address("sram", u64::from(bus & 0x7fff)));
                }
                let mut n = bus as usize;
                if self.info.mapper == "sf2" && n >= 0x80000 {
                    let bank = sf2_bank?;
                    if !(1..=4).contains(&bank) {
                        return None;
                    }
                    n = ((bank as usize) * 0x80000) + (n & 0x7ffff);
                }
                if self.info.mapper == "unknown" {
                    return None;
                }
                let size = self.info.payload_size;
                let at = match size {
                    393216 => {
                        if n < 0x80000 {
                            n & 0x3ffff
                        } else {
                            0x40000 + (n & 0x1ffff)
                        }
                    }
                    524288 => {
                        if n < 0x80000 {
                            n & 0x7ffff
                        } else {
                            0x40000 + (n & 0x3ffff)
                        }
                    }
                    _ => {
                        let n = n & (size.next_power_of_two() - 1);
                        mirror_offset(n, size)
                    }
                };
                Some(address("rom", (self.info.payload_offset + at) as u64))
            }
            0x1f0000..=0x1f7fff => Some(address("ram", u64::from(bus & 8191))),
            0x1fe000..=0x1fffff => Some(address("io", u64::from(bus & 8191))),
            _ => None,
        }
    }
}
fn mirror_offset(mut n: usize, size: usize) -> usize {
    // Mirror incomplete final chips recursively, matching upstream ROM expansion.
    let mut base = 0;
    let mut remaining = size;
    while n >= remaining {
        let bit = 1usize << (usize::BITS - 1 - n.leading_zeros());
        n -= bit;
        if remaining > bit {
            base += bit;
            remaining -= bit;
        }
    }
    base + n
}
impl ImageAdapter for Rom {
    fn platform(&self) -> &str {
        "pce"
    }
    fn image_size(&self) -> u64 {
        self.bytes.len() as u64
    }
    fn physical_address(&self, offset: u64) -> Option<Address> {
        (offset < self.image_size()).then(|| address("rom", offset))
    }
    fn peek(&self, a: &Address, n: usize) -> Result<Vec<u8>, String> {
        if a.space != "rom" {
            return Err("image exposes physical rom only".into());
        }
        self.check_range(a.offset as usize, n)
            .map(Vec::from)
            .map_err(|e| e.to_string())
    }
}
