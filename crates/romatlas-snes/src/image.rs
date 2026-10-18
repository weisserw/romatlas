//! Baseline image identity, explicit layout and side-effect-free address translation.
use anyhow::{Context, Result, ensure};
use romatlas_core::{Address, ImageAdapter};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::{collections::BTreeMap, path::Path};
#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum Layout {
    LoRom,
    HiRom,
}
#[derive(Clone, Debug, Serialize)]
pub struct Info {
    pub platform: &'static str,
    pub sha256: String,
    pub size: usize,
    pub copier_header_size: usize,
    pub payload_size: usize,
    pub layout: Layout,
    pub title: String,
    pub cartridge_type: u8,
    pub sram_size: usize,
    pub region: u8,
    pub checksum_valid: bool,
    pub reset_cpu_address: u32,
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
        ensure!(
            std::fs::metadata(path)?.len() <= 0x400200,
            "image exceeds baseline 4 MiB limit"
        );
        Self::from_bytes(std::fs::read(path)?)
    }
    pub fn from_bytes(bytes: Vec<u8>) -> Result<Self> {
        ensure!(
            (0x8000..=0x400200).contains(&bytes.len()),
            "expected 32 KiB–4 MiB SNES image"
        );
        let header = if bytes.len() % 0x8000 == 512 { 512 } else { 0 };
        let data = &bytes[header..];
        ensure!(
            data.len().is_multiple_of(0x8000),
            "incomplete 32 KiB ROM block"
        );
        let mut candidates = Vec::new();
        for (layout, h, mode) in [(Layout::LoRom, 0x7fc0, 0x20), (Layout::HiRom, 0xffc0, 0x21)] {
            if h + 64 > data.len() {
                continue;
            }
            if data[h + 0x15] & 0xef != mode {
                continue;
            }
            let checksum = u16::from_le_bytes([data[h + 0x1e], data[h + 0x1f]]);
            let complement = u16::from_le_bytes([data[h + 0x1c], data[h + 0x1d]]);
            let reset = u16::from_le_bytes([data[h + 0x3c], data[h + 0x3d]]);
            let score = u32::from(checksum ^ complement == 0xffff) * 4
                + u32::from(reset >= 0x8000) * 2
                + u32::from(data[h..h + 21].iter().all(|b| b.is_ascii()));
            candidates.push((score, layout, h));
        }
        candidates.sort_by_key(|c| std::cmp::Reverse(c.0));
        ensure!(
            !candidates.is_empty(),
            "no baseline LoROM/HiROM header; unsupported layout or cartridge"
        );
        ensure!(
            candidates.len() == 1 || candidates[0].0 != candidates[1].0,
            "ambiguous SNES header layout"
        );
        let (_, layout, h) = candidates[0];
        let cart = data[h + 0x16];
        ensure!(
            cart <= 2,
            "unsupported cartridge enhancement type ${cart:02x}"
        );
        let region = data[h + 0x19];
        ensure!(
            matches!(region, 0 | 1),
            "unsupported region {region}; NTSC Japan/USA only"
        );
        let ram = data[h + 0x18];
        ensure!(ram <= 8, "unsupported SRAM size code");
        let sram_size = if ram == 0 { 0 } else { 1024usize << ram };
        ensure!(cart != 0 || sram_size == 0, "ROM-only header declares SRAM");
        let checksum = (0..data.len().next_power_of_two()).fold(0u16, |sum, n| {
            sum.wrapping_add(u16::from(data[mirror_offset(n, data.len())]))
        });
        let expected = u16::from_le_bytes([data[h + 0x1e], data[h + 0x1f]]);
        let info = Info {
            platform: "snes",
            sha256: format!("{:x}", Sha256::digest(&bytes)),
            size: bytes.len(),
            copier_header_size: header,
            payload_size: data.len(),
            layout,
            title: String::from_utf8_lossy(&data[h..h + 21])
                .trim_end()
                .to_owned(),
            cartridge_type: cart,
            sram_size,
            region,
            checksum_valid: checksum == expected,
            reset_cpu_address: u32::from(u16::from_le_bytes([data[h + 0x3c], data[h + 0x3d]])),
        };
        Ok(Self { bytes, info })
    }
    pub fn payload(&self) -> &[u8] {
        &self.bytes[self.info.copier_header_size..]
    }
    pub fn translate(&self, cpu: u32) -> Option<Address> {
        if cpu > 0xffffff {
            return None;
        }
        let bank = cpu >> 16;
        let low = cpu & 0xffff;
        if matches!(bank, 0x7e | 0x7f) {
            return Some(address("wram", u64::from(cpu & 0x1ffff)));
        }
        if bank & 0x7f < 0x40 && low < 0x2000 {
            return Some(address("wram", u64::from(low)));
        }
        let sram = self.info.sram_size;
        let chip = match self.info.layout {
            Layout::LoRom => {
                if bank & 0x7f >= 0x70 && low < 0x8000 && sram > 0 {
                    return Some(address(
                        "sram",
                        u64::from((cpu & 0x7fff) | ((cpu & 0xf0000) >> 1)) & (sram as u64 - 1),
                    ));
                }
                if low < 0x8000 && bank & 0x7f < 0x40 {
                    return None;
                }
                ((cpu & 0x7f0000) >> 1) | (low & 0x7fff)
            }
            Layout::HiRom => {
                if (0x20..=0x3f).contains(&(bank & 0x7f))
                    && (0x6000..0x8000).contains(&low)
                    && sram > 0
                {
                    return Some(address(
                        "sram",
                        u64::from(((bank & 0x1f) << 13) | (low & 0x1fff)) & (sram as u64 - 1),
                    ));
                }
                if low < 0x8000 && bank & 0x7f < 0x40 {
                    return None;
                }
                cpu & 0x3fffff
            }
        };
        Some(address(
            "rom",
            (mirror_offset(chip as usize, self.info.payload_size) + self.info.copier_header_size)
                as u64,
        ))
    }
    pub fn canonical_cpu(&self, offset: usize) -> Result<u32> {
        let p = offset
            .checked_sub(self.info.copier_header_size)
            .context("copier header has no CPU address")?;
        ensure!(p < self.info.payload_size, "offset outside ROM");
        Ok(match self.info.layout {
            Layout::LoRom => {
                let bank = p as u32 / 0x8000;
                let bank = if bank >= 0x7e { bank | 0x80 } else { bank };
                (bank << 16) | 0x8000 | (p as u32 & 0x7fff)
            }
            Layout::HiRom => 0xc00000 | p as u32,
        })
    }
    pub fn rom_offset(&self, cpu: u32) -> Result<usize> {
        let a = self.translate(cpu).context("unmapped CPU address")?;
        ensure!(a.space == "rom", "CPU address does not map to ROM");
        Ok(a.offset as usize)
    }
    pub fn check_range(&self, offset: usize, length: usize) -> Result<&[u8]> {
        self.bytes
            .get(offset..offset.checked_add(length).context("range overflow")?)
            .context("ROM range outside image")
    }
}
impl ImageAdapter for Rom {
    fn platform(&self) -> &str {
        "snes"
    }
    fn image_size(&self) -> u64 {
        self.bytes.len() as u64
    }
    fn physical_address(&self, offset: u64) -> Option<Address> {
        (offset < self.image_size()).then(|| address("rom", offset))
    }
    fn peek(&self, a: &Address, n: usize) -> Result<Vec<u8>, String> {
        if a.space != "rom" {
            return Err("immutable ROM adapter only supports rom space".into());
        }
        self.check_range(a.offset.try_into().map_err(|_| "offset overflow")?, n)
            .map(<[u8]>::to_vec)
            .map_err(|e| e.to_string())
    }
}

fn mirror_offset(offset: usize, length: usize) -> usize {
    let offset = offset & (length.next_power_of_two() - 1);
    if offset < length {
        return offset;
    }
    let base = length.next_power_of_two() / 2;
    base + mirror_offset(offset - base, length - base)
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn physical_mirroring_matches_core() {
        for length in 1..100usize {
            let mut reference: Vec<u8> = (0..length).map(|n| n as u8).collect();
            jgenesis_common::rom::mirror_to_next_power_of_two(&mut reference);
            for (i, value) in reference.iter().enumerate() {
                assert_eq!(mirror_offset(i, length) as u8, *value);
            }
        }
    }
}
