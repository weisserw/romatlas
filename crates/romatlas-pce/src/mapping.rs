//! ROM-free PC Engine mappings. Physical roles are separate from execution views.
use crate::{
    Rom,
    decode::{self, Context, Cpu, Instruction},
};
use anyhow::{Context as _, Result, ensure};
use romatlas_core::{Provenance, RegionKind};
use serde::{Deserialize, Serialize};
use std::{collections::BTreeSet, path::Path};
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct View {
    pub cpu: Cpu,
    pub cpu_address: u32,
    #[serde(default)]
    pub context: Context,
    #[serde(default)]
    pub uploaded: bool,
}
fn certain() -> f32 {
    1.0
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Entry {
    pub id: u64,
    pub offset: usize,
    pub length: usize,
    pub kind: RegionKind,
    pub provenance: Provenance,
    #[serde(default = "certain")]
    pub confidence: f32,
    #[serde(default)]
    pub label: Option<String>,
    #[serde(default)]
    pub comment: String,
    #[serde(default)]
    pub evidence: String,
    #[serde(default)]
    pub views: Vec<View>,
}
fn first_id() -> u64 {
    1
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Mapping {
    pub format: String,
    pub version: u32,
    pub image_sha256: String,
    pub image_size: usize,
    pub regions: Vec<Entry>,
    #[serde(default = "first_id")]
    pub next_id: u64,
}
impl Mapping {
    pub fn new(rom: &Rom) -> Self {
        Self {
            format: "romatlas-pce-map".into(),
            version: 1,
            image_sha256: rom.info.sha256.clone(),
            image_size: rom.bytes.len(),
            regions: Vec::new(),
            next_id: 1,
        }
    }
    pub fn load(path: &Path) -> Result<Self> {
        ensure!(
            std::fs::metadata(path)?.len() <= 16 * 1024 * 1024,
            "mapping exceeds 16 MiB"
        );
        let value: serde_json::Value = serde_json::from_slice(&std::fs::read(path)?)?;
        let missing = value.get("next_id").is_none();
        let mut map: Self = serde_json::from_value(value)?;
        if missing {
            map.next_id = map
                .regions
                .iter()
                .map(|r| r.id)
                .max()
                .unwrap_or(0)
                .checked_add(1)
                .context("region id exhausted")?;
        }
        Ok(map)
    }
    pub fn save(&self, path: &Path) -> Result<()> {
        std::fs::write(path, serde_json::to_vec_pretty(self)?)?;
        Ok(())
    }
    pub fn validate(&self, rom: &Rom) -> Result<()> {
        ensure!(
            self.format == "romatlas-pce-map" && self.version == 1,
            "unsupported PC Engine map format"
        );
        ensure!(
            self.image_sha256 == rom.info.sha256 && self.image_size == rom.bytes.len(),
            "map ROM identity mismatch"
        );
        ensure!(self.regions.len() <= 100000, "region limit exceeded");
        ensure!(
            self.next_id > 0 && self.regions.iter().all(|r| r.id < self.next_id),
            "invalid next region id"
        );
        let mut sorted: Vec<_> = self.regions.iter().collect();
        sorted.sort_by_key(|e| e.offset);
        let mut end = 0;
        let mut ids = BTreeSet::new();
        let mut labels = BTreeSet::new();
        for entry in sorted {
            ensure!(
                entry.id > 0 && ids.insert(entry.id),
                "duplicate/invalid region id"
            );
            ensure!(
                entry.length > 0 && entry.offset >= end,
                "empty or overlapping physical regions; split existing regions first"
            );
            end = entry
                .offset
                .checked_add(entry.length)
                .context("region overflow")?;
            ensure!(end <= rom.bytes.len(), "region outside image");
            if let Some(label) = &entry.label {
                ensure!(
                    valid_label(label) && labels.insert(label),
                    "duplicate or invalid label"
                );
            }
            ensure!(
                entry.comment.len() <= 65536 && entry.evidence.len() <= 65536,
                "annotation text exceeds limit"
            );
            ensure!(
                entry.confidence.is_finite() && (0.0..=1.0).contains(&entry.confidence),
                "invalid confidence"
            );
            ensure!(entry.views.len() <= 16, "execution view limit exceeds 16");
            if entry.kind == RegionKind::Code {
                ensure!(
                    !entry.views.is_empty(),
                    "code needs an explicit CPU execution view"
                );
                for view in &entry.views {
                    decode_region(rom, entry, view)?;
                }
            } else {
                ensure!(
                    entry.views.is_empty(),
                    "execution views belong to code regions only"
                );
            }
        }
        Ok(())
    }
    pub fn annotate(&mut self, rom: &Rom, mut entry: Entry) -> Result<u64> {
        let mut candidate = self.clone();
        if entry.id == 0 {
            entry.id = candidate.next_id;
            candidate.next_id = candidate
                .next_id
                .checked_add(1)
                .context("region id exhausted")?;
            candidate.regions.push(entry.clone());
        } else {
            let old = candidate
                .regions
                .iter_mut()
                .find(|e| e.id == entry.id)
                .context("unknown region id")?;
            *old = entry.clone();
        }
        candidate.validate(rom)?;
        *self = candidate;
        Ok(entry.id)
    }
    pub fn unknown(&self) -> Vec<(usize, usize)> {
        let mut ranges: Vec<_> = self
            .regions
            .iter()
            .filter(|r| r.kind != RegionKind::Unknown)
            .map(|r| (r.offset, r.offset + r.length))
            .collect();
        ranges.sort();
        let mut out = Vec::new();
        let mut cursor = 0;
        for (start, end) in ranges {
            if start > cursor {
                out.push((cursor, start - cursor));
            }
            cursor = cursor.max(end);
        }
        if cursor < self.image_size {
            out.push((cursor, self.image_size - cursor));
        }
        out
    }
}
pub fn valid_label(s: &str) -> bool {
    !s.is_empty()
        && s.len() <= 128
        && s.bytes()
            .enumerate()
            .all(|(i, b)| b.is_ascii_alphabetic() || b == b'_' || i > 0 && b.is_ascii_digit())
}
pub fn decode_region(rom: &Rom, entry: &Entry, view: &View) -> Result<Vec<(usize, Instruction)>> {
    let mut result = Vec::new();
    let mut offset = entry.offset;
    let end = offset + entry.length;
    let mut pc = view.cpu_address;
    let mut ctx = view.context;
    ensure!(
        !view.uploaded,
        "uploaded execution views are not yet supported; RAM traces do not prove ROM origins"
    );
    while offset < end {
        ensure!(pc <= 65535, "HuC6280 PC exceeds 16 bits");
        let resolved = resolve(rom, pc, ctx);
        ensure!(
            resolved.is_some_and(|a| a.space == "rom" && a.offset == offset as u64),
            "CPU slot context does not identify this physical PRG byte"
        );
        let i = decode::decode(view.cpu, &rom.bytes[offset..end], pc, ctx)
            .with_context(|| format!("code view at file offset {offset}:"))?;
        for byte in 0..i.length {
            ensure!(
                resolve(rom, (pc + byte as u32) & 65535, ctx)
                    .is_some_and(|a| a.space == "rom" && a.offset == (offset + byte) as u64),
                "instruction operand crosses a noncontiguous CPU slot"
            );
        }
        ctx = i.after;
        pc = (pc & 0xff0000) | ((pc + i.length as u32) & 0xffff);
        offset += i.length;
        result.push((offset - i.length, i));
    }
    Ok(result)
}

/// Resolve effective MPR bus pages without treating I/O/unmapped pages as ROM.
pub fn resolve(rom: &Rom, pc: u32, context: Context) -> Option<romatlas_core::Address> {
    if pc > 65535 {
        return None;
    }
    let page = context.mpr[(pc as usize) >> 13]?;
    rom.bus_binding(u32::from(page) * 8192 + (pc & 8191), context.sf2_bank)
}
