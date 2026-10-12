//! SM83 decoding. Table source: gbdev/gb-opcodes (CC0), pinned in data/REVISION.
use anyhow::{Result, bail};
use serde::{Deserialize, Serialize};
use std::{collections::BTreeMap, sync::OnceLock};
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct OperandSpec {
    pub name: String,
    pub immediate: bool,
    #[serde(default)]
    pub bytes: u8,
    #[serde(default)]
    pub increment: bool,
    #[serde(default)]
    pub decrement: bool,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct OpcodeSpec {
    pub mnemonic: String,
    pub bytes: u8,
    /// T cycles: taken path first, then untaken path when conditional.
    pub cycles: Vec<u8>,
    pub operands: Vec<OperandSpec>,
    pub flags: BTreeMap<String, String>,
}
#[derive(Deserialize)]
struct Tables {
    unprefixed: BTreeMap<String, OpcodeSpec>,
    cbprefixed: BTreeMap<String, OpcodeSpec>,
}
fn tables() -> &'static Tables {
    static TABLE: OnceLock<Tables> = OnceLock::new();
    TABLE.get_or_init(|| {
        serde_json::from_str(include_str!("../data/Opcodes.json")).expect("checked opcode table")
    })
}
pub fn spec(opcode: u8, cb: bool) -> &'static OpcodeSpec {
    let table = if cb {
        &tables().cbprefixed
    } else {
        &tables().unprefixed
    };
    &table[&format!("0x{opcode:02X}")]
}
#[derive(Clone, Debug, Serialize)]
pub struct Operand {
    pub spec: OperandSpec,
    pub value: Option<i32>,
}
#[derive(Clone, Debug, Serialize, PartialEq, Eq)]
pub enum Flow {
    Next,
    Jump,
    Call,
    Return,
    Restart,
    IndirectJump,
    Halt,
    Stop,
    Invalid,
}
#[derive(Clone, Debug, Serialize)]
pub struct Instruction {
    pub pc: u16,
    pub opcode: u8,
    pub cb: bool,
    pub length: u8,
    pub mnemonic: String,
    pub operands: Vec<Operand>,
    pub cycles: Vec<u8>,
    pub flags: BTreeMap<String, String>,
    pub flow: Flow,
    pub conditional: bool,
    /// CPU-visible target only. The caller must resolve effective bank context.
    pub target: Option<u16>,
    pub fallthrough: Option<u16>,
    pub memory_references: Vec<u16>,
    pub warnings: Vec<String>,
}
pub fn decode(bytes: &[u8], pc: u16) -> Result<Instruction> {
    let Some(&first) = bytes.first() else {
        bail!("empty instruction input");
    };
    let cb = first == 0xcb;
    let opcode = if cb {
        *bytes
            .get(1)
            .ok_or_else(|| anyhow::anyhow!("truncated CB prefix"))?
    } else {
        first
    };
    let s = spec(opcode, cb);
    if bytes.len() < s.bytes as usize {
        bail!("truncated {}: needs {} bytes", s.mnemonic, s.bytes);
    }
    let mut cursor = if cb { 2 } else { 1 };
    let mut operands = Vec::new();
    for o in &s.operands {
        let value = match o.bytes {
            0 => None,
            1 => {
                let b = bytes[cursor];
                cursor += 1;
                Some(if o.name == "e8" {
                    b as i8 as i32
                } else {
                    b as i32
                })
            }
            2 => {
                let v = u16::from_le_bytes([bytes[cursor], bytes[cursor + 1]]);
                cursor += 2;
                Some(v as i32)
            }
            _ => unreachable!("table operand width"),
        };
        operands.push(Operand {
            spec: o.clone(),
            value,
        });
    }
    let conditional = !cb
        && matches!(
            opcode,
            0x20 | 0x28
                | 0x30
                | 0x38
                | 0xc0
                | 0xc2
                | 0xc4
                | 0xc8
                | 0xca
                | 0xcc
                | 0xd0
                | 0xd2
                | 0xd4
                | 0xd8
                | 0xda
                | 0xdc
        );
    let flow = if cb {
        Flow::Next
    } else {
        match s.mnemonic.as_str() {
            "JP" if opcode == 0xe9 => Flow::IndirectJump,
            "JP" | "JR" => Flow::Jump,
            "CALL" => Flow::Call,
            "RET" | "RETI" => Flow::Return,
            "RST" => Flow::Restart,
            "HALT" => Flow::Halt,
            "STOP" => Flow::Stop,
            m if m.starts_with("ILLEGAL") => Flow::Invalid,
            _ => Flow::Next,
        }
    };
    let next = pc.wrapping_add(s.bytes as u16);
    let target = match flow {
        Flow::Jump if s.mnemonic == "JR" => {
            Some(next.wrapping_add_signed(operands.last().unwrap().value.unwrap() as i16))
        }
        Flow::Jump | Flow::Call => Some(operands.last().unwrap().value.unwrap() as u16),
        Flow::Restart => {
            Some(u16::from_str_radix(&s.operands[0].name[1..], 16).expect("RST table"))
        }
        _ => None,
    };
    let fallthrough = match flow {
        Flow::Jump | Flow::Return if !conditional => None,
        Flow::IndirectJump | Flow::Invalid => None,
        _ => Some(next),
    };
    let memory_references = operands
        .iter()
        .filter_map(|o| match o.spec.name.as_str() {
            "a16" if !o.spec.immediate => o.value.map(|v| v as u16),
            "a8" => o.value.map(|v| 0xff00 | v as u16),
            _ => None,
        })
        .collect();
    let warnings = if first == 0x10 && bytes[1] != 0 {
        vec!["nonzero STOP operand: assembler may change encoding; retain raw bytes".into()]
    } else {
        vec![]
    };
    Ok(Instruction {
        pc,
        opcode,
        cb,
        length: s.bytes,
        mnemonic: s.mnemonic.clone(),
        operands,
        cycles: s.cycles.clone(),
        flags: s.flags.clone(),
        flow,
        conditional,
        target,
        fallthrough,
        memory_references,
        warnings,
    })
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn all_base_lengths_and_truncation() {
        let lengths: [u8; 256] = [
            1, 3, 1, 1, 1, 1, 2, 1, 3, 1, 1, 1, 1, 1, 2, 1, 2, 3, 1, 1, 1, 1, 2, 1, 2, 1, 1, 1, 1,
            1, 2, 1, 2, 3, 1, 1, 1, 1, 2, 1, 2, 1, 1, 1, 1, 1, 2, 1, 2, 3, 1, 1, 1, 1, 2, 1, 2, 1,
            1, 1, 1, 1, 2, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
            1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
            1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
            1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
            1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 3, 3, 3, 1, 2, 1, 1, 1, 3,
            2, 3, 3, 2, 1, 1, 1, 3, 1, 3, 1, 2, 1, 1, 1, 3, 1, 3, 1, 2, 1, 2, 1, 1, 1, 1, 1, 2, 1,
            2, 1, 3, 1, 1, 1, 2, 1, 2, 1, 1, 1, 1, 1, 2, 1, 2, 1, 3, 1, 1, 1, 2, 1,
        ];
        for op in 0..=255u8 {
            let input = [op, 0x35, 0x12];
            let d = decode(&input, 0x4000).unwrap();
            assert_eq!(d.length, lengths[op as usize], "{op:02x}");
            for n in 0..d.length as usize {
                assert!(decode(&input[..n], 0).is_err());
            }
            if op != 0xcb {
                for o in &d.operands {
                    match o.spec.bytes {
                        1 => assert_eq!(o.value, Some(0x35)),
                        2 => assert_eq!(o.value, Some(0x1235)),
                        _ => {}
                    }
                }
            }
        }
    }
    #[test]
    fn base_timing_matrix() {
        let cycles: [u8; 256] = [
            4, 12, 8, 8, 4, 4, 8, 4, 20, 8, 8, 8, 4, 4, 8, 4, 4, 12, 8, 8, 4, 4, 8, 4, 12, 8, 8, 8,
            4, 4, 8, 4, 12, 12, 8, 8, 4, 4, 8, 4, 12, 8, 8, 8, 4, 4, 8, 4, 12, 12, 8, 8, 12, 12,
            12, 4, 12, 8, 8, 8, 4, 4, 8, 4, 4, 4, 4, 4, 4, 4, 8, 4, 4, 4, 4, 4, 4, 4, 8, 4, 4, 4,
            4, 4, 4, 4, 8, 4, 4, 4, 4, 4, 4, 4, 8, 4, 4, 4, 4, 4, 4, 4, 8, 4, 4, 4, 4, 4, 4, 4, 8,
            4, 8, 8, 8, 8, 8, 8, 4, 8, 4, 4, 4, 4, 4, 4, 8, 4, 4, 4, 4, 4, 4, 4, 8, 4, 4, 4, 4, 4,
            4, 4, 8, 4, 4, 4, 4, 4, 4, 4, 8, 4, 4, 4, 4, 4, 4, 4, 8, 4, 4, 4, 4, 4, 4, 4, 8, 4, 4,
            4, 4, 4, 4, 4, 8, 4, 4, 4, 4, 4, 4, 4, 8, 4, 4, 4, 4, 4, 4, 4, 8, 4, 20, 12, 16, 16,
            24, 16, 8, 16, 20, 16, 16, 4, 24, 24, 8, 16, 20, 12, 16, 4, 24, 16, 8, 16, 20, 16, 16,
            4, 24, 4, 8, 16, 12, 12, 8, 4, 4, 16, 8, 16, 16, 4, 16, 4, 4, 4, 8, 16, 12, 12, 8, 4,
            4, 16, 8, 16, 12, 8, 16, 4, 4, 4, 8, 16,
        ];
        for op in 0..=255u8 {
            let s = spec(op, false);
            assert_eq!(s.cycles[0], cycles[op as usize], "{op:02x}");
            let untaken = match op {
                0x20 | 0x28 | 0x30 | 0x38 | 0xc0 | 0xc8 | 0xd0 | 0xd8 => Some(8),
                0xc2 | 0xca | 0xd2 | 0xda | 0xc4 | 0xcc | 0xd4 | 0xdc => Some(12),
                _ => None,
            };
            assert_eq!(s.cycles.get(1).copied(), untaken, "{op:02x}");
            assert_eq!(s.flags.len(), 4);
            for key in ["Z", "N", "H", "C"] {
                assert!(["-", "0", "1", "Z", "N", "H", "C"].contains(&s.flags[key].as_str()));
            }
        }
    }
    #[test]
    fn cb_matrix() {
        let regs = ["B", "C", "D", "E", "H", "L", "HL", "A"];
        let rotations = ["RLC", "RRC", "RL", "RR", "SLA", "SRA", "SWAP", "SRL"];
        for op in 0..=255u8 {
            let d = decode(&[0xcb, op], 0).unwrap();
            let group = op >> 6;
            let r = (op & 7) as usize;
            assert_eq!(d.length, 2);
            assert_eq!(
                d.mnemonic,
                if group == 0 {
                    rotations[(op >> 3) as usize]
                } else {
                    ["", "BIT", "RES", "SET"][group as usize]
                }
            );
            let o = d.operands.last().unwrap();
            assert_eq!(o.spec.name, regs[r]);
            assert_eq!(o.spec.immediate, r != 6);
            assert_eq!(
                d.cycles,
                vec![if r != 6 {
                    8
                } else if group == 1 {
                    12
                } else {
                    16
                }]
            );
            if group > 0 {
                assert_eq!(d.operands[0].spec.name, ((op >> 3) & 7).to_string());
            }
            assert_eq!(d.flags["N"], if group <= 1 { "0" } else { "-" });
            assert_eq!(
                d.flags["H"],
                if group == 0 {
                    "0"
                } else if group == 1 {
                    "1"
                } else {
                    "-"
                }
            );
        }
    }
    #[test]
    fn invalid_set_and_targets() {
        let invalid = [
            0xd3, 0xdb, 0xdd, 0xe3, 0xe4, 0xeb, 0xec, 0xed, 0xf4, 0xfc, 0xfd,
        ];
        for op in 0..=255u8 {
            assert_eq!(
                decode(&[op, 0, 0], 0).unwrap().flow == Flow::Invalid,
                invalid.contains(&op)
            );
        }
        assert_eq!(decode(&[0x18, 0xfe], 0xffff).unwrap().target, Some(0xffff));
        assert_eq!(decode(&[0x20, 0x80], 0x4000).unwrap().target, Some(0x3f82));
        assert_eq!(decode(&[0xcd, 0x34, 0x12], 0).unwrap().target, Some(0x1234));
        assert_eq!(decode(&[0xff], 0).unwrap().target, Some(0x38));
        assert_eq!(decode(&[0xe9], 0).unwrap().flow, Flow::IndirectJump);
        assert_eq!(
            decode(&[0xe0, 0x42], 0).unwrap().memory_references,
            vec![0xff42]
        );
        assert_eq!(
            decode(&[0xea, 0, 0xc0], 0).unwrap().memory_references,
            vec![0xc000]
        );
        assert!(!decode(&[0x10, 1], 0).unwrap().warnings.is_empty());
    }
}
