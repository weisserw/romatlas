//! Explicit caller-confirmed invariants, never implicit register or execution claims.
use crate::{
    Rom,
    decode::{Flow, decode},
    discovery::{ContextDecoder, CpuContext},
};
use anyhow::{Result, ensure};
use romatlas_core::{Address, DecodeAdapter, ImageAdapter, Reference, StructuralInstruction};
use serde::Deserialize;
use std::collections::BTreeMap;

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ConfirmedProfiles {
    pub rom_sha256: String,
    pub entries: Vec<ConfirmedProfile>,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ConfirmedProfile {
    pub offset: u64,
    pub cpu_context: CpuContext,
    pub expected_bytes: Vec<u8>,
    pub evidence: String,
    pub scope: ProfileScope,
    pub effect: ProfileEffect,
}
#[derive(Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProfileScope {
    AllArrivalsInContext,
}
#[derive(Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum ProfileEffect {
    NonMapperStore { cpu_destination: u16 },
    InlineDispatch,
}

pub struct ProfiledContextDecoder<'a> {
    decoder: ContextDecoder<'a>,
    profiles: BTreeMap<Address, ConfirmedProfile>,
}
impl<'a> ProfiledContextDecoder<'a> {
    pub fn new(rom: &'a Rom, profiles: Option<ConfirmedProfiles>) -> Result<Self> {
        let mut by_address = BTreeMap::new();
        if let Some(profiles) = profiles {
            ensure!(
                profiles.rom_sha256 == rom.info.sha256,
                "profile ROM identity mismatch"
            );
            ensure!(profiles.entries.len() <= 1000, "profile count exceeds 1000");
            for profile in profiles.entries {
                ensure!(
                    !profile.evidence.trim().is_empty() && profile.evidence.len() <= 4096,
                    "profile requires bounded evidence text"
                );
                let physical = rom
                    .physical_address(profile.offset)
                    .ok_or_else(|| anyhow::anyhow!("profile offset outside ROM"))?;
                let address = ContextDecoder::node(physical, profile.cpu_context);
                // This validates CPU alias/effective banks, opcode validity and encoding.
                let structure = ContextDecoder(rom)
                    .decode_structure(&address)
                    .map_err(anyhow::Error::msg)?;
                ensure!(
                    profile.expected_bytes.len() == structure.length as usize,
                    "profile must match the complete source instruction"
                );
                let start = profile.offset as usize;
                ensure!(
                    rom.bytes[start..start + structure.length as usize] == profile.expected_bytes,
                    "profile source bytes mismatch"
                );
                let instruction = decode(&profile.expected_bytes, profile.cpu_context.cpu_address)?;
                match profile.effect {
                    ProfileEffect::NonMapperStore { cpu_destination } => {
                        ensure!(
                            cpu_destination >= 0x8000,
                            "profile destination may write a mapper"
                        );
                        ensure!(
                            instruction.mnemonic == "LD"
                                && instruction
                                    .operands
                                    .first()
                                    .is_some_and(|o| !o.spec.immediate
                                        && matches!(o.spec.name.as_str(), "BC" | "DE" | "HL")),
                            "non-mapper profile requires an indirect byte LD store"
                        );
                    }
                    ProfileEffect::InlineDispatch => {
                        ensure!(
                            matches!(instruction.flow, Flow::Call | Flow::Restart)
                                && !instruction.conditional
                                && instruction.target != instruction.fallthrough,
                            "inline-dispatch profile requires an unconditional CALL/RST"
                        );
                    }
                }
                ensure!(
                    by_address.insert(address, profile).is_none(),
                    "duplicate context profile"
                );
            }
        }
        Ok(Self {
            decoder: ContextDecoder(rom),
            profiles: by_address,
        })
    }
}
impl ImageAdapter for ProfiledContextDecoder<'_> {
    fn platform(&self) -> &str {
        self.decoder.platform()
    }
    fn image_size(&self) -> u64 {
        self.decoder.image_size()
    }
    fn physical_address(&self, offset: u64) -> Option<Address> {
        self.decoder.physical_address(offset)
    }
    fn peek(&self, address: &Address, length: usize) -> Result<Vec<u8>, String> {
        self.decoder.peek(address, length)
    }
}
impl DecodeAdapter for ProfiledContextDecoder<'_> {
    fn canonical_address(&self, address: &Address) -> Address {
        self.decoder.canonical_address(address)
    }
    fn decode_structure(&self, address: &Address) -> Result<StructuralInstruction, String> {
        let mut structure = self.decoder.decode_structure(address)?;
        let Some(profile) = self.profiles.get(address) else {
            return Ok(structure);
        };
        let instruction = decode(&profile.expected_bytes, profile.cpu_context.cpu_address)
            .map_err(|e| e.to_string())?;
        match profile.effect {
            ProfileEffect::InlineDispatch => {
                structure.successors.retain(|node| {
                    let context: CpuContext =
                        serde_json::from_str(&node.context["gb.cpu_context"]).unwrap();
                    Some(context.cpu_address) != instruction.fallthrough
                });
                structure.diagnostics.push(format!("caller-confirmed inline dispatch: return-site fallthrough suppressed; supply reviewed handler entrypoints. Evidence: {}", profile.evidence));
            }
            ProfileEffect::NonMapperStore { cpu_destination } => {
                structure
                    .diagnostics
                    .retain(|s| !s.starts_with("possible bank-controller write:"));
                if let Some(next) = instruction.fallthrough {
                    // Map a CPU window, never assume the physical source bank continues.
                    let bank = if next < 0x4000 {
                        profile.cpu_context.fixed_bank
                    } else if next < 0x8000 {
                        profile.cpu_context.switch_bank
                    } else {
                        None
                    };
                    if let Some(target) = bank.and_then(|b| {
                        self.physical_address((b as u64) * 0x4000 + u64::from(next % 0x4000))
                    }) {
                        let target = ContextDecoder::node(
                            target,
                            CpuContext {
                                cpu_address: next,
                                ..profile.cpu_context
                            },
                        );
                        if !structure.successors.contains(&target) {
                            structure.successors.push(target);
                        }
                    } else {
                        structure.diagnostics.push(format!(
                            "profile cannot resolve fallthrough CPU window ${next:04x}"
                        ));
                    }
                }
                structure.references.push(Reference {
                    source: self.canonical_address(address), target: None,
                    kind: "confirmed-non-mapper-store".into(),
                    context: serde_json::json!({"cpu_address":cpu_destination,"evidence":profile.evidence,
                        "provenance":"Manual","scope":"all_arrivals_in_context","physical_memory_claim":false}),
                });
                structure.diagnostics.push(format!("caller-confirmed non-mapper store; not inferred register state or observed execution. Evidence: {}", profile.evidence));
            }
        }
        Ok(structure)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use romatlas_core::{Provenance, Region, RegionKind};
    use std::collections::BTreeSet;
    fn fixture() -> Rom {
        let mut bytes = vec![0; 0x10000];
        bytes[0x147] = 1;
        bytes[0x100] = 0x77; // LD [HL],A
        bytes[0x101] = 0x76;
        bytes[0x200] = 0xef; // RST 28 with an inline table after it
        bytes[0x201] = 0x76;
        bytes[0x28] = 0xc9;
        Rom::from_bytes(bytes).unwrap()
    }
    fn context(offset: u16) -> CpuContext {
        CpuContext {
            cpu_address: offset,
            fixed_bank: Some(0),
            switch_bank: Some(1),
        }
    }
    fn profiles(rom: &Rom, offset: u64, effect: ProfileEffect) -> ConfirmedProfiles {
        ConfirmedProfiles {
            rom_sha256: rom.info.sha256.clone(),
            entries: vec![ConfirmedProfile {
                offset,
                cpu_context: context(offset as u16),
                expected_bytes: vec![rom.bytes[offset as usize]],
                evidence: "synthetic all-arrival invariant".into(),
                scope: ProfileScope::AllArrivalsInContext,
                effect,
            }],
        }
    }
    #[test]
    fn profiles_are_explicit_scoped_and_never_execution_coverage() {
        let rom = fixture();
        let root = ContextDecoder::node(rom.physical_address(0x100).unwrap(), context(0x100));
        let run = |adapter: &ProfiledContextDecoder<'_>, regions: &[Region]| {
            romatlas_analysis::discovery::discover(
                adapter,
                vec![root.clone()],
                &BTreeSet::new(),
                regions,
                10,
            )
            .unwrap()
        };
        assert_eq!(
            run(&ProfiledContextDecoder::new(&rom, None).unwrap(), &[])
                .candidates
                .len(),
            1
        );
        let decoder = ProfiledContextDecoder::new(
            &rom,
            Some(profiles(
                &rom,
                0x100,
                ProfileEffect::NonMapperStore {
                    cpu_destination: 0xa000,
                },
            )),
        )
        .unwrap();
        let result = run(&decoder, &[]);
        assert_eq!(result.candidates.len(), 2);
        assert!(
            result
                .candidates
                .iter()
                .all(|r| r.provenance == Provenance::Static)
        );
        assert!(result.references.last().unwrap().target.is_none());
        let barrier = Region {
            start: rom.physical_address(0x101).unwrap(),
            length: 1,
            kind: RegionKind::Data,
            provenance: Provenance::Manual,
            confidence: 1.0,
        };
        assert_eq!(run(&decoder, &[barrier]).candidates.len(), 1);
        let decoder = ProfiledContextDecoder::new(
            &rom,
            Some(profiles(&rom, 0x200, ProfileEffect::InlineDispatch)),
        )
        .unwrap();
        let root = ContextDecoder::node(rom.physical_address(0x200).unwrap(), context(0x200));
        let result =
            romatlas_analysis::discovery::discover(&decoder, vec![root], &BTreeSet::new(), &[], 10)
                .unwrap();
        assert_eq!(
            result
                .candidates
                .iter()
                .map(|r| r.start.offset)
                .collect::<Vec<_>>(),
            vec![0x28, 0x200]
        );
    }
    #[test]
    fn stale_and_unsafe_profiles_are_rejected() {
        let rom = fixture();
        assert!(
            ProfiledContextDecoder::new(
                &rom,
                Some(profiles(
                    &rom,
                    0x100,
                    ProfileEffect::NonMapperStore {
                        cpu_destination: 0x2000
                    }
                ))
            )
            .is_err()
        );
        assert!(
            ProfiledContextDecoder::new(
                &rom,
                Some(profiles(&rom, 0x101, ProfileEffect::InlineDispatch))
            )
            .is_err()
        );
        let mut p = profiles(&rom, 0x200, ProfileEffect::InlineDispatch);
        p.rom_sha256 = "wrong".into();
        assert!(ProfiledContextDecoder::new(&rom, Some(p)).is_err());
        let mut p = profiles(&rom, 0x200, ProfileEffect::InlineDispatch);
        p.entries[0].expected_bytes[0] = 0;
        assert!(ProfiledContextDecoder::new(&rom, Some(p)).is_err());
        let mut p = profiles(&rom, 0x200, ProfileEffect::InlineDispatch);
        p.entries[0].cpu_context.fixed_bank = Some(1);
        assert!(ProfiledContextDecoder::new(&rom, Some(p)).is_err());
    }
}
