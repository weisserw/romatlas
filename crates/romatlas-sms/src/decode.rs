//! Z80 decoding delegates instruction tables to the pinned MIT DeZ80 library.
use anyhow::{Result, ensure};
use dez80::instruction::{InstructionType as T, Operand as O};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Cpu {
    Main,
}
/// Effective physical 16 KiB ROM banks. None never means the reset bank.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Context {
    #[serde(default)]
    pub rom_banks: [Option<u32>; 3],
    #[serde(default)]
    pub ram_enabled: Option<bool>,
    #[serde(default)]
    pub ram_bank: Option<u32>,
    #[serde(default)]
    pub cartridge_enabled: Option<bool>,
}
#[derive(Clone, Debug, Serialize)]
pub struct Instruction {
    pub opcode: u8,
    pub length: usize,
    pub text: String,
    pub assembly: Option<String>,
    pub after: Context,
    pub successors: Vec<u32>,
    pub references: Vec<Value>,
    pub terminal: bool,
}
fn operand(o: O, pc: u32, length: usize) -> String {
    match o {
        O::ProgramCounterRelative(d) => format!(
            "${:04x}",
            ((pc as i64 + length as i64 + i64::from(d)) & 65535)
        ),
        O::MemoryIndexed(r, d) => format!("({r}{:+})", d),
        O::MemoryIndexedAndRegister(r, d, s) => format!("({r}{:+}), {s}", d),
        O::MemoryIndexedBit(r, d, b) => format!("{b}, ({r}{:+})", d),
        O::MemoryIndexedBitAndRegister(r, d, b, s) => format!("{b}, ({r}{:+}), {s}", d),
        _ => o.to_string().replace("0x", "$"),
    }
    .to_lowercase()
}
pub fn decode(_: Cpu, bytes: &[u8], pc: u32, context: Context) -> Result<Instruction> {
    ensure!(pc <= 65535, "Z80 PC exceeds 16 bits");
    // Bound malicious prefix chains independently of the size of the image.
    let mut input = &bytes[..bytes.len().min(64)];
    let before = input.len();
    let i = dez80::Instruction::decode_one(&mut input).map_err(|e| {
        anyhow::anyhow!("truncated instruction or prefix chain exceeds 64 bytes: {e:?}")
    })?;
    let length = before - input.len();
    ensure!(
        length > 0 && i.to_bytes() == bytes[..length],
        "decoder encoding mismatch"
    );
    let op = i.opcode.value;
    let next = (pc + length as u32) & 65535;
    let mut text = i.r#type.to_string().replace("0x", "$").to_lowercase();
    let operands: Vec<_> = [i.destination, i.source]
        .into_iter()
        .flatten()
        .map(|o| operand(o, pc, length))
        .collect();
    if !operands.is_empty() {
        if matches!(i.r#type, T::Jp(Some(_)) | T::Jr(Some(_)) | T::Call(Some(_))) {
            text.push(',');
        }
        text.push(' ');
        text.push_str(&operands.join(", "));
    }
    // JP (HL/IX/IY) uses an implied register pair in DeZ80.
    if matches!(i.r#type, T::Jp(_))
        && matches!(i.source.or(i.destination), Some(O::RegisterPairImplied(_)))
    {
        text = format!(
            "jp ({})",
            operand(i.source.or(i.destination).unwrap(), pc, length)
        );
    }
    let control = matches!(
        i.r#type,
        T::Jp(_) | T::Jr(_) | T::Call(_) | T::Djnz | T::Rst(_)
    );
    let mut references = Vec::new();
    let mut target = None;
    for (index, o) in [i.destination, i.source].into_iter().enumerate() {
        let Some(o) = o else { continue };
        let (value, indirect, indexed) = match o {
            O::ProgramCounterRelative(d) => (
                Some(((i64::from(next) + i64::from(d)) & 65535) as u32),
                false,
                false,
            ),
            O::DoubletImmediate(v) if control => (Some(u32::from(v)), false, false),
            O::MemoryDirect(v) => (Some(u32::from(v)), false, false),
            O::MemoryIndirect(_) | O::MemoryIndirectBit(_, _) | O::RegisterPairImplied(_)
                if control || matches!(o, O::MemoryIndirect(_) | O::MemoryIndirectBit(_, _)) =>
            {
                (None, true, false)
            }
            O::MemoryIndexed(_, _)
            | O::MemoryIndexedAndRegister(_, _, _)
            | O::MemoryIndexedBit(_, _, _)
            | O::MemoryIndexedBitAndRegister(_, _, _, _) => (None, true, true),
            O::PortDirect(v) => (Some(u32::from(v)), false, false),
            O::PortIndirect(_) => (None, true, false),
            _ => continue,
        };
        let port = matches!(o, O::PortDirect(_) | O::PortIndirect(_));
        if control {
            target = value;
        }
        references.push(json!({"cpu_address":value,"kind":if control{"control"}else if port{"io"}else{"memory"},"indirect":indirect,"indexed":indexed,"write":index==0 && matches!(i.r#type,T::Ld|T::Inc|T::Dec|T::Res|T::Set|T::Out)}));
    }
    if let T::Rst(v) = i.r#type {
        target = Some(u32::from(v));
        references.push(json!({"cpu_address":v,"kind":"control","indirect":false,"indexed":false}));
    }
    let terminal = matches!(
        i.r#type,
        T::Ret(None) | T::Reti | T::Retn | T::Jp(None) | T::Jr(None) | T::Halt | T::Inva
    );
    let mut successors = Vec::new();
    if !terminal {
        successors.push(next);
    }
    if let Some(t) = target {
        successors.push(t);
    }
    // Canonical assembler spellings cannot retain ignored prefixes, ED aliases,
    // undocumented indexed destinations or wrapped relative targets.
    let ed_alias = bytes[0] == 0xed
        && matches!(
            op,
            0x4c | 0x54
                | 0x5c
                | 0x64
                | 0x6c
                | 0x74
                | 0x7c
                | 0x55
                | 0x5d
                | 0x65
                | 0x6d
                | 0x75
                | 0x7d
                | 0x4e
                | 0x66
                | 0x6e
                | 0x76
                | 0x7e
                | 0x63
                | 0x6b
                | 0x70
                | 0x71
        );
    let undocumented = matches!(i.r#type, T::Inva | T::Sll)
        || [i.source, i.destination].into_iter().flatten().any(|o| {
            matches!(
                o,
                O::MemoryIndexedAndRegister(_, _, _) | O::MemoryIndexedBitAndRegister(_, _, _, _)
            )
        });
    let canonical = i.ignored_prefixes.is_empty() && !undocumented && !ed_alias;
    let assembly = canonical.then(|| {
        let mut asm = text.clone();
        // WLA numeric relative operands are signed displacements; symbolic
        // operands are addresses. Keep CPU targets in the readable decoder text.
        for o in [i.source, i.destination].into_iter().flatten() {
            if let O::ProgramCounterRelative(d) = o {
                asm = asm.replace(&operand(o, pc, length), &d.to_string());
            }
        }
        asm
    });
    Ok(Instruction {
        opcode: op,
        length,
        assembly,
        text,
        after: context,
        successors,
        references,
        terminal,
    })
}
