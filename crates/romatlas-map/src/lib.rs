//! Stock mgbdis symbol adapter. No ROM bytes belong in this format.
use anyhow::{Context, Result, bail};
use serde::{Deserialize, Serialize};
/// Bound local metadata reads, including files that grow after opening.
pub fn read_limited(path: &std::path::Path, max_bytes: u64) -> Result<Vec<u8>> {
    use std::io::Read;
    anyhow::ensure!(
        std::fs::metadata(path)?.is_file(),
        "metadata input must be a regular file"
    );
    let file = std::fs::File::open(path)?;
    let metadata = file.metadata()?;
    anyhow::ensure!(metadata.is_file(), "metadata input must be a regular file");
    anyhow::ensure!(
        metadata.len() <= max_bytes,
        "metadata file exceeds {max_bytes} bytes"
    );
    let mut bytes = Vec::new();
    file.take(
        max_bytes
            .checked_add(1)
            .context("metadata byte limit overflow")?,
    )
    .read_to_end(&mut bytes)?;
    anyhow::ensure!(
        bytes.len() as u64 <= max_bytes,
        "metadata file exceeds {max_bytes} bytes"
    );
    Ok(bytes)
}
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Entry {
    pub bank: u32,
    pub address: u32,
    pub annotation: String,
}
#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub struct Map {
    pub entries: Vec<Entry>,
}
impl Map {
    pub fn parse(text: &str) -> Result<Self> {
        let mut entries = Vec::new();
        for (i, line) in text.lines().enumerate() {
            let line = line.trim();
            if line.is_empty() || line.starts_with(';') {
                continue;
            }
            let fields: Vec<_> = line.split_whitespace().collect();
            if fields.len() != 2 {
                bail!("line {}: expected bank:address annotation", i + 1);
            }
            let (bank, address) = fields[0].split_once(':').context("expected bank:address")?;
            let entry = Entry {
                bank: u32::from_str_radix(bank, 16)?,
                address: u32::from_str_radix(address, 16)?,
                annotation: fields[1].into(),
            };
            validate_annotation(&entry.annotation).with_context(|| format!("line {}", i + 1))?;
            entries.push(entry);
        }
        let result = Self { entries };
        result.validate(None)?;
        Ok(result)
    }
    pub fn export(&self) -> Result<String> {
        self.validate(None)?;
        let mut entries = self.entries.clone();
        entries.sort();
        Ok(entries
            .iter()
            .map(|e| format!("{:02x}:{:04x} {}\n", e.bank, e.address, e.annotation))
            .collect())
    }
    pub fn raw(banks: usize) -> Self {
        Self {
            entries: (0..banks)
                .map(|bank| Entry {
                    bank: bank as u32,
                    address: if bank == 0 { 0 } else { 0x4000 },
                    annotation: ".data:4000".into(),
                })
                .collect(),
        }
    }
    pub fn validate(&self, banks: Option<usize>) -> Result<()> {
        let mut ranges = Vec::new();
        let mut labels = std::collections::BTreeSet::new();
        let mut locations = std::collections::BTreeSet::new();
        for e in &self.entries {
            validate_annotation(&e.annotation)?;
            if e.address > 0xffff {
                bail!("address exceeds Game Boy address space");
            }
            if let Some(n) = banks
                && e.bank as usize >= n
                && e.address < 0x8000
            {
                bail!("bank {} outside ROM", e.bank);
            }
            if e.annotation.starts_with('.') && e.annotation.contains(':') {
                let base = if e.bank == 0 { 0 } else { 0x4000 };
                let length = u32::from_str_radix(e.annotation.split(':').nth(1).unwrap(), 16)?;
                if length == 0
                    || e.address < base
                    || e.address
                        .checked_add(length)
                        .is_none_or(|end| end > base + 0x4000)
                {
                    bail!("region outside bank at {:x}:{:x}", e.bank, e.address);
                }
                ranges.push((e.bank, e.address, e.address + length));
            } else if !labels.insert(&e.annotation)
                || !locations.insert((
                    if e.address >= 0x8000 || e.address < 0x4000 {
                        0
                    } else {
                        e.bank
                    },
                    e.address,
                ))
            {
                bail!("duplicate label or label address");
            }
        }
        ranges.sort();
        for w in ranges.windows(2) {
            if w[0].0 == w[1].0 && w[0].2 > w[1].1 {
                bail!("overlapping regions in bank {:x} at {:x}", w[1].0, w[1].1);
            }
        }
        Ok(())
    }
}
fn validate_annotation(s: &str) -> Result<()> {
    if s.starts_with('.') && s.contains(':') {
        let parts: Vec<_> = s.split(':').collect();
        if parts.len() < 2
            || parts.len() > 3
            || ![
                ".code", ".data", ".byt", ".text", ".asc", ".padding", ".image",
            ]
            .contains(&parts[0].to_ascii_lowercase().as_str())
        {
            bail!("unsupported mgbdis directive: {s}");
        }
        u32::from_str_radix(parts[1], 16).context("invalid region length")?;
        if parts.len() == 3 && parts[2].is_empty() {
            bail!("empty directive arguments");
        }
        if let Some(args) = parts.get(2) {
            match parts[0].to_ascii_lowercase().as_str() {
                ".data" | ".byt" => {
                    let width =
                        u32::from_str_radix(args, 16).context("data width must be hexadecimal")?;
                    anyhow::ensure!(width > 0, "data width must be positive");
                }
                ".code" | ".padding" => bail!("{} does not support arguments", parts[0]),
                ".image" => {
                    let mut keys = std::collections::BTreeSet::new();
                    for arg in args.split(',') {
                        let key = if arg == "1bpp" {
                            "bpp"
                        } else if let Some(value) = arg.strip_prefix('w') {
                            let width: u32 =
                                value.parse().context("image width must be decimal")?;
                            anyhow::ensure!(
                                width > 0 && width.is_multiple_of(8),
                                "image width must be a positive multiple of 8"
                            );
                            "width"
                        } else if let Some(value) = arg.strip_prefix('p') {
                            u8::from_str_radix(value, 16)
                                .context("image palette must be one hexadecimal byte")?;
                            "palette"
                        } else {
                            bail!("unsupported image argument: {arg}");
                        };
                        anyhow::ensure!(keys.insert(key), "duplicate image argument: {arg}");
                    }
                }
                ".text" | ".asc" => {
                    let (key, value) = args.split_once('=').unwrap_or((args, "0"));
                    anyhow::ensure!(
                        ["cm", "charmap"].contains(&key)
                            && !value.is_empty()
                            && value.chars().all(|c| c.is_ascii_alphanumeric() || c == '_'),
                        "unsupported text charmap argument: {args}"
                    );
                }
                _ => unreachable!(),
            }
        }
    } else if s.is_empty()
        || !s
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || "_.#@".contains(c))
        || s.chars().next().unwrap().is_ascii_digit()
    {
        bail!("invalid label: {s}");
    }
    Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn metadata_reads_reject_large_files_before_parsing() {
        let path =
            std::env::temp_dir().join(format!("romatlas-metadata-limit-{}", std::process::id()));
        let file = std::fs::File::create(&path).unwrap();
        file.set_len(64 * 1024 * 1024 + 1).unwrap();
        assert!(
            read_limited(&path, 1024 * 1024)
                .unwrap_err()
                .to_string()
                .contains("exceeds")
        );
        assert!(
            store::Store::load(&path)
                .unwrap_err()
                .to_string()
                .contains("exceeds")
        );
        std::fs::write(&path, b"valid metadata").unwrap();
        assert_eq!(read_limited(&path, 14).unwrap(), b"valid metadata");
        assert!(read_limited(&path, 13).is_err());
        std::fs::remove_file(path).unwrap();
    }
    #[test]
    fn supported_roundtrip() {
        let s = "; community-style map\n02:4000 Tiles\n02:4000 .image:100:w128,pe4\n00:c000 wBuffer\n01:4200 .byt:20:2\n00:0200 .asc:10\n";
        let m = Map::parse(s).unwrap();
        assert_eq!(
            Map::parse(&m.export().unwrap()).unwrap().export().unwrap(),
            m.export().unwrap()
        );
    }
    #[test]
    fn overlaps() {
        assert!(Map::parse("00:0100 .code:10\n00:0108 .data:10\n").is_err());
    }
    #[test]
    fn unknown_directive() {
        assert!(Map::parse("00:0100 .unknown:10\n").is_err());
    }
    #[test]
    fn deterministic() {
        assert_eq!(
            Map::raw(2).export().unwrap(),
            "00:0000 .data:4000\n01:4000 .data:4000\n"
        );
    }
    #[test]
    fn directive_arguments_match_stock_syntax() {
        for annotation in [
            ".data:10:0",
            ".byt:10:no",
            ".code:10:2",
            ".padding:10:2",
            ".image:10:w0",
            ".image:10:w9",
            ".image:10:p100",
            ".image:10:w8,w16",
            ".image:10:2bpp",
            ".text:10:bad",
            ".asc:10:cm=",
        ] {
            assert!(
                Map::parse(&format!("00:0200 {annotation}\n")).is_err(),
                "accepted {annotation}"
            );
        }
        for annotation in [
            ".byt:10:2",
            ".image:10:w8,pe4,1bpp",
            ".text:10:cm=0",
            ".asc:10:charmap=Letters",
        ] {
            assert!(
                Map::parse(&format!("00:0200 {annotation}\n")).is_ok(),
                "rejected {annotation}"
            );
        }
    }
}

pub mod store;
