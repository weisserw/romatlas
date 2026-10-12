//! SM83 structural interpretation with conservative bank identity resolution.
use crate::{
    Rom,
    decode::{Flow, decode},
};
use romatlas_core::{Address, DecodeAdapter, ImageAdapter, Reference, StructuralInstruction};
#[derive(Clone, Copy, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CpuContext {
    pub cpu_address: u16,
    pub fixed_bank: Option<usize>,
    pub switch_bank: Option<usize>,
}
pub struct ContextDecoder<'a>(pub &'a Rom);
impl ContextDecoder<'_> {
    pub fn node(mut address: Address, context: CpuContext) -> Address {
        address.context.insert(
            "gb.cpu_context".into(),
            serde_json::to_string(&context).unwrap(),
        );
        address
    }
}
impl ImageAdapter for ContextDecoder<'_> {
    fn platform(&self) -> &str {
        self.0.platform()
    }
    fn image_size(&self) -> u64 {
        self.0.image_size()
    }
    fn physical_address(&self, offset: u64) -> Option<Address> {
        self.0.physical_address(offset)
    }
    fn peek(&self, address: &Address, length: usize) -> Result<Vec<u8>, String> {
        self.0.peek(&self.canonical_address(address), length)
    }
}
impl DecodeAdapter for ContextDecoder<'_> {
    fn canonical_address(&self, address: &Address) -> Address {
        let mut physical = address.clone();
        physical.context.remove("gb.cpu_context");
        physical
    }
    fn decode_structure(&self, address: &Address) -> Result<StructuralInstruction, String> {
        let context: CpuContext = serde_json::from_str(
            address
                .context
                .get("gb.cpu_context")
                .ok_or("missing CPU traversal context")?,
        )
        .map_err(|e| e.to_string())?;
        self.0
            .decode_structure_in(&self.canonical_address(address), Some(context), true)
    }
}
impl DecodeAdapter for Rom {
    fn decode_structure(&self, address: &Address) -> Result<StructuralInstruction, String> {
        self.decode_structure_in(address, None, false)
    }
}
impl Rom {
    pub fn decode_structure_with_context(
        &self,
        address: &Address,
        context: CpuContext,
    ) -> Result<StructuralInstruction, String> {
        self.decode_structure_in(address, Some(context), false)
    }
    fn decode_structure_in(
        &self,
        address: &Address,
        context: Option<CpuContext>,
        contextual_successors: bool,
    ) -> Result<StructuralInstruction, String> {
        if self.physical_address(address.offset).as_ref() != Some(address) {
            return Err("expected canonical physical ROM address".into());
        }
        let offset = address.offset as usize;
        let bank = offset / 0x4000;
        let (_, default_pc) = Self::sym_location(offset);
        let pc = context.map_or(default_pc as u16, |c| c.cpu_address);
        if let Some(c) = context {
            if c.cpu_address >= 0x8000 || c.cpu_address as usize % 0x4000 != offset % 0x4000 {
                return Err("CPU address does not match physical ROM offset".into());
            }
            if [c.fixed_bank, c.switch_bank]
                .into_iter()
                .flatten()
                .any(|bank| bank >= self.info.banks)
            {
                return Err("effective bank outside ROM".into());
            }
            if matches!(self.info.controller.as_str(), "ROM" | "MBC3" | "MBC5")
                && c.fixed_bank.is_some_and(|b| b != 0)
            {
                return Err("controller fixes lower CPU window to bank zero".into());
            }
            if self.info.controller == "ROM" && c.switch_bank.is_some_and(|b| b != 1) {
                return Err("ROM-only upper CPU window must use bank one".into());
            }
            let source_bank = if pc < 0x4000 {
                c.fixed_bank.or(
                    if matches!(self.info.controller.as_str(), "ROM" | "MBC3" | "MBC5") {
                        Some(0)
                    } else {
                        None
                    },
                )
            } else {
                c.switch_bank.or(if self.info.controller == "ROM" {
                    Some(1)
                } else {
                    None
                })
            };
            if source_bank != Some(bank) {
                return Err(
                    "CPU window bank does not match physical source; supply its effective bank"
                        .into(),
                );
            }
        }
        if !["ROM", "MBC1", "MBC3", "MBC5"].contains(&self.info.controller.as_str()) {
            return Err("static bank rules for this controller are not yet validated".into());
        }
        if context.is_none()
            && self.info.controller == "MBC1"
            && bank > 0
            && bank.is_multiple_of(16)
        {
            return Err("physical bank may occupy a fixed or switchable CPU window; explicit CPU context is required".into());
        }
        let end = ((bank + 1) * 0x4000).min(self.bytes.len());
        let d = decode(&self.bytes[offset..end], pc).map_err(|e| e.to_string())?;
        if d.flow == Flow::Invalid || !d.warnings.is_empty() {
            return Err("invalid or noncanonical instruction encoding; retain raw bytes".into());
        }
        let resolve = |cpu: u16| -> Option<Address> {
            if let Some(c) = context {
                let target_bank = if cpu < 0x4000 {
                    c.fixed_bank.or(
                        if matches!(self.info.controller.as_str(), "ROM" | "MBC3" | "MBC5") {
                            Some(0)
                        } else {
                            None
                        },
                    )
                } else if cpu < 0x8000 {
                    c.switch_bank.or(if self.info.controller == "ROM" {
                        Some(1)
                    } else {
                        None
                    })
                } else {
                    None
                };
                return target_bank.and_then(|bank| {
                    self.physical_address((bank * 0x4000 + cpu as usize % 0x4000) as u64)
                });
            }
            let target_bank = if cpu < 0x4000 {
                if bank == 0
                    || matches!(self.info.controller.as_str(), "ROM" | "MBC3" | "MBC5")
                    || self.info.banks <= 16
                {
                    Some(0)
                } else {
                    None
                }
            } else if cpu < 0x8000 {
                if bank != 0 {
                    Some(bank)
                } else if self.info.controller == "ROM" {
                    Some(1)
                } else {
                    None
                }
            } else {
                None
            };
            target_bank.and_then(|bank| {
                self.physical_address((bank * 0x4000 + (cpu as usize % 0x4000)) as u64)
            })
        };
        let mut successors = Vec::new();
        let successor = |physical: &Address, cpu_address: u16| {
            if contextual_successors {
                ContextDecoder::node(
                    physical.clone(),
                    CpuContext {
                        cpu_address,
                        ..context.unwrap()
                    },
                )
            } else {
                physical.clone()
            }
        };
        let mut references = Vec::new();
        let mut diagnostics = Vec::new();
        if let Some(target) = d.target {
            let physical = resolve(target);
            if let Some(a) = &physical {
                successors.push(successor(a, target));
            } else {
                diagnostics.push(format!(
                    "unresolved bank or non-ROM control target ${target:04x}"
                ));
            }
            references.push(Reference{source:address.clone(),target:physical,kind:"control".into(),context:serde_json::json!({"cpu_address":target,"conditional":d.conditional,"flow":d.flow})});
        }
        // A known or register-indirect controller write may change the next fetch bank.
        let controller_write = d.mnemonic == "LD"
            && d.operands.first().is_some_and(|o| {
                !o.spec.immediate
                    && (matches!(o.spec.name.as_str(), "BC" | "DE" | "HL")
                        || o.spec.name == "a16" && o.value.is_some_and(|v| v < 0x8000))
            });
        if let Some(next) = d.fallthrough {
            if controller_write && self.info.controller != "ROM" {
                diagnostics.push("possible bank-controller write: stopped static fallthrough; use observed targets".into());
            } else if matches!(d.flow, Flow::Halt | Flow::Stop) {
                diagnostics.push("HALT/STOP continuation requires runtime evidence".into());
            } else if let Some(a) = resolve(next) {
                successors.push(successor(&a, next));
            } else {
                diagnostics.push(format!("unresolved fallthrough bank at ${next:04x}"));
            }
        }
        if d.flow == Flow::IndirectJump {
            diagnostics.push(
                "indirect jump target needs observed execution or a confirmed pointer table".into(),
            );
        }
        for cpu in d.memory_references {
            references.push(Reference{source:address.clone(),target:if controller_write&&cpu<0x8000{None}else{resolve(cpu)},kind:if controller_write&&cpu<0x8000{"controller".into()}else{"memory".into()},context:serde_json::json!({"cpu_address":cpu,"bank_ambiguous":resolve(cpu).is_none()&&cpu<0x8000})});
        }
        if let Some(c) = context {
            for reference in &mut references {
                reference.context["source_cpu_address"] = serde_json::json!(c.cpu_address);
                reference.context["fixed_bank_asserted"] = serde_json::json!(c.fixed_bank);
                reference.context["switch_bank_asserted"] = serde_json::json!(c.switch_bank);
            }
        }
        Ok(StructuralInstruction {
            length: d.length as u64,
            successors,
            references,
            diagnostics,
        })
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeSet;
    #[test]
    fn bank_write_and_indirect_stop() {
        let mut bytes = vec![0; 0x10000];
        bytes[0x147] = 1;
        bytes[0x100..0x103].copy_from_slice(&[0xea, 0, 0x20]);
        bytes[0x200] = 0xe9;
        let rom = Rom::from_bytes(bytes).unwrap();
        let write = rom
            .decode_structure(&rom.physical_address(0x100).unwrap())
            .unwrap();
        assert!(write.successors.is_empty());
        assert!(write.references[0].target.is_none());
        assert_eq!(write.references[0].kind, "controller");
        let indirect = rom
            .decode_structure(&rom.physical_address(0x200).unwrap())
            .unwrap();
        assert!(indirect.successors.is_empty());
        assert!(!indirect.diagnostics.is_empty());
    }
    #[test]
    fn bank_ambiguity_and_data_barrier() {
        let mut bytes = vec![0; 0x10000];
        bytes[0x147] = 1;
        bytes[0x100..0x103].copy_from_slice(&[0xc3, 0, 0x40]);
        let rom = Rom::from_bytes(bytes).unwrap();
        let a = rom.physical_address(0x100).unwrap();
        let d = rom.decode_structure(&a).unwrap();
        assert!(d.successors.is_empty());
        assert!(d.references[0].target.is_none());
        let result =
            romatlas_analysis::discovery::discover(&rom, vec![a], &BTreeSet::new(), &[], 100)
                .unwrap();
        assert_eq!(result.candidates.len(), 1);
        assert_eq!(result.references.len(), 1);
    }
    #[test]
    fn mbc3_mbc5_static_windows_and_controller_barriers() {
        for controller in [0x11, 0x19] {
            let mut bytes = vec![0; 32 * 0x4000];
            bytes[0x147] = controller;
            bytes[0x100..0x103].copy_from_slice(&[0xc3, 0, 0x40]);
            bytes[0x8000..0x8003].copy_from_slice(&[0xc3, 0x00, 0x01]);
            bytes[0x8100..0x8103].copy_from_slice(&[0xc3, 0x20, 0x41]);
            bytes[0x8200..0x8203].copy_from_slice(&[0xea, 0, 0x20]);
            let rom = Rom::from_bytes(bytes).unwrap();
            let at = |offset| {
                rom.decode_structure(&rom.physical_address(offset).unwrap())
                    .unwrap()
            };
            assert!(at(0x100).references[0].target.is_none());
            assert_eq!(at(0x8000).successors[0].offset, 0x100);
            assert_eq!(at(0x8100).successors[0].offset, 0x8120);
            assert!(at(0x8200).successors.is_empty());
            assert_eq!(at(0x8200).references[0].kind, "controller");
        }
    }
    #[test]
    fn explicit_fixed_and_switch_aliases_use_cpu_pc() {
        let mut bytes = vec![0; 64 * 0x4000];
        bytes[0x147] = 1;
        bytes[0x80000..0x80002].copy_from_slice(&[0x18, 0x20]);
        let rom = Rom::from_bytes(bytes).unwrap();
        let address = rom.physical_address(0x80000).unwrap();
        assert!(rom.decode_structure(&address).is_err());
        let c = CpuContext {
            cpu_address: 0,
            fixed_bank: Some(32),
            switch_bank: Some(33),
        };
        let d = rom.decode_structure_with_context(&address, c).unwrap();
        assert_eq!(d.references[0].context["cpu_address"], 0x22);
        assert_eq!(d.successors[0].offset, 0x80022);
        assert!(
            rom.decode_structure_with_context(
                &address,
                CpuContext {
                    cpu_address: 1,
                    ..c
                }
            )
            .is_err()
        );
        let mut bytes = vec![0; 0x8000];
        bytes[0x147] = 0x19;
        bytes[0x100..0x103].copy_from_slice(&[0xc3, 0x20, 0x40]);
        let rom = Rom::from_bytes(bytes).unwrap();
        let address = rom.physical_address(0x100).unwrap();
        let d = rom
            .decode_structure_with_context(
                &address,
                CpuContext {
                    cpu_address: 0x4100,
                    fixed_bank: Some(0),
                    switch_bank: Some(0),
                },
            )
            .unwrap();
        assert_eq!(d.successors[0].offset, 0x20);
        assert_eq!(d.references[0].context["cpu_address"], 0x4020);
    }
    #[test]
    fn recursive_context_crosses_windows_and_keeps_physical_barriers() {
        let mut bytes = vec![0; 64 * 0x4000];
        bytes[0x147] = 1;
        bytes[0x80000..0x80003].copy_from_slice(&[0xc3, 0x20, 0x40]);
        bytes[0x84020..0x84023].copy_from_slice(&[0xc3, 0x10, 0]);
        bytes[0x80010] = 0x76;
        let rom = Rom::from_bytes(bytes).unwrap();
        let adapter = ContextDecoder(&rom);
        let root = ContextDecoder::node(
            rom.physical_address(0x80000).unwrap(),
            CpuContext {
                cpu_address: 0,
                fixed_bank: Some(32),
                switch_bank: Some(33),
            },
        );
        let result = romatlas_analysis::discovery::discover(
            &adapter,
            vec![root.clone()],
            &BTreeSet::new(),
            &[],
            10,
        )
        .unwrap();
        assert_eq!(result.candidates.len(), 3);
        assert_eq!(result.references[1].context["source_cpu_address"], 0x4020);
        assert!(
            result
                .candidates
                .iter()
                .all(|r| !r.start.context.contains_key("gb.cpu_context"))
        );
        let barrier = romatlas_core::Region {
            start: rom.physical_address(0x84020).unwrap(),
            length: 3,
            kind: romatlas_core::RegionKind::Data,
            provenance: romatlas_core::Provenance::Manual,
            confidence: 1.0,
        };
        let blocked = romatlas_analysis::discovery::discover(
            &adapter,
            vec![root],
            &BTreeSet::new(),
            &[barrier],
            10,
        )
        .unwrap();
        assert_eq!(blocked.candidates.len(), 1);
        assert!(
            blocked
                .diagnostics
                .iter()
                .any(|d| d.message.contains("non-code"))
        );
    }
    #[test]
    fn shared_physical_instruction_visits_both_cpu_aliases() {
        let mut bytes = vec![0; 0x8000];
        bytes[0x147] = 0x19;
        bytes[0] = 0x76;
        let rom = Rom::from_bytes(bytes).unwrap();
        let physical = rom.physical_address(0x3fff).unwrap();
        let roots = [0x3fff, 0x7fff].map(|cpu_address| {
            ContextDecoder::node(
                physical.clone(),
                CpuContext {
                    cpu_address,
                    fixed_bank: Some(0),
                    switch_bank: Some(0),
                },
            )
        });
        let result = romatlas_analysis::discovery::discover(
            &ContextDecoder(&rom),
            roots.to_vec(),
            &BTreeSet::new(),
            &[],
            10,
        )
        .unwrap();
        assert_eq!(result.visited, 3);
        assert_eq!(result.candidates.len(), 2);
        assert!(
            result
                .diagnostics
                .iter()
                .any(|d| d.message.contains("$8000"))
        );
    }
}
