//! Pointer hypotheses from explicit word ranges. No address value proves a pointer or code.
use crate::Rom;
use anyhow::{Result, ensure};
use romatlas_core::{Address, ImageAdapter, Provenance, Reference, Region, RegionKind};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};
#[derive(Default, Clone, Copy)]
pub struct BankContext {
    pub fixed_bank: Option<usize>,
    pub switch_bank: Option<usize>,
}
/// Bank fields name effective physical banks, never raw mapper registers.
#[derive(Default, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Layout {
    #[default]
    WordLe,
    Bank8WordLe,
    WordLeBank8,
    Bank16WordLe,
    WordLeBank16,
}
impl Layout {
    pub fn width(self) -> usize {
        match self {
            Self::WordLe => 2,
            Self::Bank8WordLe | Self::WordLeBank8 => 3,
            Self::Bank16WordLe | Self::WordLeBank16 => 4,
        }
    }
    fn read(self, bytes: &[u8]) -> (u16, Option<usize>) {
        let word = |offset| u16::from_le_bytes([bytes[offset], bytes[offset + 1]]);
        match self {
            Self::WordLe => (word(0), None),
            Self::Bank8WordLe => (word(1), Some(bytes[0] as usize)),
            Self::WordLeBank8 => (word(0), Some(bytes[2] as usize)),
            Self::Bank16WordLe => (word(2), Some(word(0) as usize)),
            Self::WordLeBank16 => (word(0), Some(word(2) as usize)),
        }
    }
}
#[derive(Serialize)]
pub struct PointerAnalysis {
    pub candidate: Option<Region>,
    pub references: Vec<Reference>,
    pub words: usize,
    pub resolved_rom: usize,
    pub ambiguous_rom: usize,
    pub non_rom: usize,
    pub invalid_rom: usize,
    pub unique_targets: usize,
    pub diagnostics: Vec<String>,
}
pub fn analyze(
    rom: &Rom,
    offset: usize,
    count: usize,
    context: BankContext,
) -> Result<PointerAnalysis> {
    analyze_layout(rom, offset, count, context, Layout::WordLe)
}
pub fn analyze_layout(
    rom: &Rom,
    offset: usize,
    count: usize,
    context: BankContext,
    layout: Layout,
) -> Result<PointerAnalysis> {
    ensure!(
        (1..=4096).contains(&count),
        "pointer count must be 1..4096 entries"
    );
    let end = offset
        .checked_add(count * layout.width())
        .ok_or_else(|| anyhow::anyhow!("pointer range overflow"))?;
    ensure!(end <= rom.bytes.len(), "pointer range outside ROM");
    ensure!(
        offset / 0x4000 == (end - 1) / 0x4000,
        "pointer table crosses physical bank; split it"
    );
    for bank in [context.fixed_bank, context.switch_bank]
        .into_iter()
        .flatten()
    {
        ensure!(bank < rom.info.banks, "asserted bank outside ROM");
    }
    if matches!(rom.info.controller.as_str(), "ROM" | "MBC3" | "MBC5") {
        ensure!(
            context.fixed_bank.is_none_or(|b| b == 0),
            "this controller fixes the lower CPU window to bank 0"
        );
    }
    if rom.info.controller == "ROM" {
        ensure!(
            context.switch_bank.is_none_or(|b| b == 1),
            "ROM-only upper CPU window is bank 1"
        );
    }
    let fixed = context.fixed_bank.or_else(|| {
        if matches!(rom.info.controller.as_str(), "ROM" | "MBC3" | "MBC5")
            || rom.info.controller == "MBC1" && rom.info.banks <= 16
        {
            Some(0)
        } else {
            None
        }
    });
    let switch = context.switch_bank.or(if rom.info.controller == "ROM" {
        Some(1)
    } else {
        None
    });
    let mut result = PointerAnalysis {
        candidate: None,
        references: Vec::new(),
        words: count,
        resolved_rom: 0,
        ambiguous_rom: 0,
        non_rom: 0,
        invalid_rom: 0,
        unique_targets: 0,
        diagnostics: Vec::new(),
    };
    let mut unique = BTreeSet::new();
    for index in 0..count {
        let source = offset + index * layout.width();
        let (cpu, encoded_bank) = layout.read(&rom.bytes[source..source + layout.width()]);
        let target = if cpu < 0x8000 {
            let bank = encoded_bank.or(if cpu < 0x4000 { fixed } else { switch });
            if let Some(bank) = bank {
                let invalid = bank >= rom.info.banks
                    || cpu < 0x4000
                        && matches!(rom.info.controller.as_str(), "ROM" | "MBC3" | "MBC5")
                        && bank != 0
                    || cpu >= 0x4000 && rom.info.controller == "ROM" && bank != 1
                    || encoded_bank.is_some()
                        && (if cpu < 0x4000 {
                            context.fixed_bank
                        } else {
                            context.switch_bank
                        })
                        .is_some_and(|asserted| asserted != bank);
                if invalid {
                    result.invalid_rom += 1;
                    None
                } else {
                    result.resolved_rom += 1;
                    rom.physical_address((bank * 0x4000 + (cpu as usize & 0x3fff)) as u64)
                }
            } else {
                result.ambiguous_rom += 1;
                None
            }
        } else {
            result.non_rom += 1;
            Some(Address {
                space: "cpu".into(),
                offset: cpu as u64,
                context: BTreeMap::new(),
            })
        };
        if let Some(target) = &target {
            unique.insert(target.clone());
        }
        result.references.push(Reference{source:rom.physical_address(source as u64).unwrap(),target,kind:"pointer-candidate".into(),context:serde_json::json!({"cpu_address":cpu,"provenance":"Static","confidence":0.5,"width":layout.width(),"layout":layout,"encoded_bank":encoded_bank,"bank_encoding":"effective-physical","byte_order":"little","fixed_bank_asserted":context.fixed_bank,"switch_bank_asserted":context.switch_bank})});
    }
    result.unique_targets = unique.len();
    if result.ambiguous_rom > 0 {
        result.diagnostics.push("ROM pointer words need effective bank context; table location does not select a target bank".into());
    }
    if result.non_rom > 0 {
        result.diagnostics.push("Non-ROM values retain CPU-visible targets; they do not resolve physical RAM/VRAM banks".into());
    }
    if result.invalid_rom > 0 {
        result.diagnostics.push("Encoded bank is outside ROM, conflicts with asserted context, or cannot occupy this CPU window; target stays unresolved".into());
    }
    if result.resolved_rom >= 2 && result.resolved_rom * 5 >= count * 4 && unique.len() >= 2 {
        result.candidate = Some(Region {
            start: rom.physical_address(offset as u64).unwrap(),
            length: (count * layout.width()) as u64,
            kind: RegionKind::Data,
            provenance: Provenance::Static,
            confidence: 0.5,
        });
    }
    result.diagnostics.push("Pointer candidates require confirmation. No target is execution coverage or a code region merely because a word points to it".into());
    Ok(result)
}
#[cfg(test)]
mod tests {
    use super::*;
    fn rom(kind: u8) -> Rom {
        let mut bytes = vec![0; 0x10000];
        bytes[0x147] = kind;
        bytes[0x200..0x206].copy_from_slice(&[0, 0x40, 0x20, 0x40, 0, 0xc0]);
        Rom::from_bytes(bytes).unwrap()
    }
    #[test]
    fn bank_ambiguity_and_asserted_context() {
        let r = rom(1);
        let a = analyze(&r, 0x200, 3, Default::default()).unwrap();
        assert_eq!(a.ambiguous_rom, 2);
        assert!(a.candidate.is_none());
        assert!(a.references[0].target.is_none());
        let a = analyze(
            &r,
            0x200,
            2,
            BankContext {
                fixed_bank: None,
                switch_bank: Some(2),
            },
        )
        .unwrap();
        assert_eq!(a.references[0].target.as_ref().unwrap().offset, 0x8000);
        assert_eq!(a.references[1].target.as_ref().unwrap().offset, 0x8020);
        assert!(a.candidate.is_some());
        assert_eq!(a.candidate.unwrap().provenance, Provenance::Static);
    }
    #[test]
    fn bounds_and_literals() {
        let mut r = rom(0);
        r.bytes[0x200..0x204].fill(0);
        assert!(
            analyze(&r, 0x200, 2, Default::default())
                .unwrap()
                .candidate
                .is_none()
        );
        assert!(analyze(&r, 0x3fff, 1, Default::default()).is_err());
        assert!(analyze(&r, 0, 0, Default::default()).is_err());
        assert!(
            analyze(
                &r,
                0x200,
                2,
                BankContext {
                    fixed_bank: None,
                    switch_bank: Some(2)
                }
            )
            .is_err()
        );
    }
    #[test]
    fn far_layouts_keep_bank_and_word_order_explicit() {
        for (layout, bytes) in [
            (Layout::Bank8WordLe, vec![2, 0, 0x40, 3, 0x20, 0x40]),
            (Layout::WordLeBank8, vec![0, 0x40, 2, 0x20, 0x40, 3]),
            (Layout::Bank16WordLe, vec![2, 0, 0, 0x40, 3, 0, 0x20, 0x40]),
            (Layout::WordLeBank16, vec![0, 0x40, 2, 0, 0x20, 0x40, 3, 0]),
        ] {
            let mut r = rom(0x19);
            r.bytes[0x200..0x200 + bytes.len()].copy_from_slice(&bytes);
            let result = analyze_layout(&r, 0x200, 2, Default::default(), layout).unwrap();
            assert_eq!(result.resolved_rom, 2);
            assert_eq!(result.references[0].target.as_ref().unwrap().offset, 0x8000);
            assert_eq!(result.references[1].target.as_ref().unwrap().offset, 0xc020);
            assert_eq!(
                result.references[1].source.offset,
                (0x200 + layout.width()) as u64
            );
            assert_eq!(result.candidate.unwrap().length, bytes.len() as u64);
        }
    }
    #[test]
    fn far_banks_validate_windows_and_context() {
        let mut r = rom(0x19);
        // Outside image; impossible nonzero fixed bank; CPU RAM target.
        r.bytes[0x200..0x209].copy_from_slice(&[4, 0, 0x40, 2, 0, 0x10, 255, 0, 0xc0]);
        let a = analyze_layout(&r, 0x200, 3, Default::default(), Layout::Bank8WordLe).unwrap();
        assert_eq!(a.invalid_rom, 2);
        assert_eq!(a.non_rom, 1);
        assert_eq!(a.references[2].target.as_ref().unwrap().space, "cpu");
        assert!(a.candidate.is_none());
        r.bytes[0x200..0x203].copy_from_slice(&[2, 0, 0x40]);
        let a = analyze_layout(
            &r,
            0x200,
            1,
            BankContext {
                fixed_bank: None,
                switch_bank: Some(3),
            },
            Layout::Bank8WordLe,
        )
        .unwrap();
        assert_eq!(a.invalid_rom, 1);
        assert!(a.references[0].target.is_none());
        assert!(analyze_layout(&r, 0x3ffe, 1, Default::default(), Layout::Bank8WordLe).is_err());
        assert!(
            analyze_layout(&r, usize::MAX, 1, Default::default(), Layout::Bank16WordLe).is_err()
        );
    }
    #[test]
    fn far_word_bank_retains_mbc5_ninth_bit() {
        let mut bytes = vec![0; 512 * 0x4000];
        bytes[0x147] = 0x19;
        bytes[0x200..0x204].copy_from_slice(&[0, 1, 0x23, 0x41]);
        let r = Rom::from_bytes(bytes).unwrap();
        let a = analyze_layout(&r, 0x200, 1, Default::default(), Layout::Bank16WordLe).unwrap();
        assert_eq!(a.references[0].target.as_ref().unwrap().offset, 0x400123);
        assert_eq!(a.references[0].context["encoded_bank"], 256);
    }
}
