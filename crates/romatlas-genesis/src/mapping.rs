//! ROM-free Genesis mappings. Physical roles are separate from execution views.
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
            format: rom.map_format(),
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
        self.validate_with_budget(rom, &romatlas_disasm::Budget::new(120000)?)
    }
    pub fn validate_with_budget(&self, rom: &Rom, budget: &romatlas_disasm::Budget) -> Result<()> {
        ensure!(
            self.format == rom.map_format() && self.version == 1,
            "unsupported map format for {}",
            rom.info.platform
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
                    decode_region_with_budget(rom, entry, view, budget)?;
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
    decode_region_with_budget(rom, entry, view, &romatlas_disasm::Budget::new(120000)?)
}
pub fn decode_region_with_budget(
    rom: &Rom,
    entry: &Entry,
    view: &View,
    budget: &romatlas_disasm::Budget,
) -> Result<Vec<(usize, Instruction)>> {
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
        ensure!(
            pc <= if view.cpu == Cpu::Z80 {
                65535
            } else {
                0xffffff
            },
            "CPU address exceeds bus width"
        );
        let resolved = resolve_cpu(rom, view.cpu, pc, view.context);
        ensure!(
            resolved.is_some_and(|a| a.space == "rom" && a.offset == offset as u64),
            "CPU slot context does not identify this physical ROM byte"
        );
        let i = decode::decode_with_budget(view.cpu, &rom.bytes[offset..end], pc, ctx, budget)
            .with_context(|| format!("code view at file offset {offset}:"))?;
        for byte in 0..i.length {
            ensure!(
                resolve_cpu(rom, view.cpu, (pc + byte as u32) & 0xffffff, view.context)
                    .is_some_and(|a| a.space == "rom" && a.offset == (offset + byte) as u64),
                "instruction operand crosses a noncontiguous CPU slot"
            );
        }
        ctx = i.after;
        pc = (pc + i.length as u32)
            & if view.cpu == Cpu::Z80 {
                65535
            } else {
                0xffffff
            };
        offset += i.length;
        result.push((offset - i.length, i));
    }
    Ok(result)
}

/// Effective 512 KiB cartridge windows. Unknown overlays never imply reset state.
pub fn resolve(rom: &Rom, pc: u32, context: Context) -> Option<romatlas_core::Address> {
    if context.rom_banks.iter().flatten().any(|&b| b > 255)
        || context.z80_bank.is_some_and(|b| b > 511)
    {
        return None;
    }
    if !matches!(rom.info.mapper.as_str(), "linear" | "ssf") {
        return None;
    }
    if rom
        .info
        .eeprom_range
        .is_some_and(|(a, b)| (a..=b).contains(&pc))
        && context.ram_enabled != Some(false)
    {
        return None;
    }
    if pc >= 0x1000000 {
        return None;
    }
    if pc >= 0xe00000 {
        return rom.translate(pc);
    }
    if pc >= 0x400000 {
        return None;
    }
    if let (Some(start), Some(end), Some(kind)) =
        (rom.info.ram_start, rom.info.ram_end, rom.info.ram_type)
    {
        let lane = kind & 0x18;
        let matches_lane =
            lane == 0 || lane == 0x10 && pc.is_multiple_of(2) || lane == 0x18 && pc % 2 == 1;
        if (start..=end).contains(&pc) && matches_lane {
            match context.ram_enabled {
                Some(true) => {
                    return Some(crate::image::address(
                        "cart_ram",
                        u64::from(if lane == 0 {
                            pc - start
                        } else {
                            (pc - start) / 2
                        }),
                    ));
                }
                None => return None,
                _ => {}
            }
        }
    }
    let at = if rom.info.mapper == "ssf" {
        context.rom_banks[(pc >> 19) as usize]? as usize * 0x80000 + (pc as usize & 0x7ffff)
    } else {
        pc as usize
    };
    (at < rom.info.rom_size)
        .then(|| crate::image::address("rom", (rom.info.rom_offset + at) as u64))
}
pub fn resolve_cpu(
    rom: &Rom,
    cpu: Cpu,
    pc: u32,
    context: Context,
) -> Option<romatlas_core::Address> {
    if cpu == Cpu::Main {
        return resolve(rom, pc, context);
    }
    match pc {
        0..=0x3fff => Some(crate::image::address("zram", u64::from(pc & 8191))),
        0x8000..=0xffff => resolve(rom, (context.z80_bank? << 15) | (pc & 0x7fff), context),
        _ => None,
    }
}
