//! HuC6280 operands derived from the pinned upstream mnemonic table, not NMOS opcodes.
use anyhow::{Result, ensure};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Cpu {
    Main,
}
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Context {
    #[serde(default)]
    pub mpr: [Option<u8>; 8],
    #[serde(default)]
    pub sf2_bank: Option<u8>,
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
pub fn decode(_: Cpu, bytes: &[u8], pc: u32, context: Context) -> Result<Instruction> {
    ensure!(
        pc <= 65535 && !bytes.is_empty(),
        "missing opcode or PC exceeds 16 bits"
    );
    let op = bytes[0];
    let template = huc6280_emu::disassemble(op).to_ascii_lowercase();
    let (mnemonic, mode) = template.split_once(' ').unwrap_or((&template, ""));
    let bitbranch = mnemonic.starts_with("bbr") || mnemonic.starts_with("bbs");
    let bitmem = mnemonic.starts_with("rmb") || mnemonic.starts_with("smb");
    let branch = matches!(
        op,
        0x10 | 0x30 | 0x50 | 0x70 | 0x90 | 0xb0 | 0xd0 | 0xf0 | 0x80 | 0x44
    );
    let block = matches!(op, 0x73 | 0xc3 | 0xd3 | 0xe3 | 0xf3);
    let tst = mnemonic == "tst";
    let immediate = mode == "#imm" || matches!(op, 0x03 | 0x13 | 0x23 | 0x43 | 0x53);
    let length = if block {
        7
    } else if tst {
        if mode.starts_with("abs") { 4 } else { 3 }
    } else if bitbranch {
        3
    } else if branch || bitmem || immediate || mode.contains("zp") {
        2
    } else if mode.contains("abs") || op == 0x20 {
        3
    } else {
        1
    };
    ensure!(bytes.len() >= length, "truncated HuC6280 instruction");
    let word = |i: usize| u32::from(u16::from_le_bytes([bytes[i], bytes[i + 1]]));
    let next = (pc + length as u32) & 65535;
    let target = if branch || bitbranch {
        ((i64::from(next) + i64::from(bytes[length - 1] as i8)) & 65535) as u32
    } else {
        0
    };
    let mut refs = Vec::new();
    let mut operand = mode.to_string();
    if block {
        operand = format!("${:04x},${:04x},${:04x}", word(1), word(3), word(5));
        for (at, kind) in [(1, "block_source"), (3, "block_destination")] {
            refs.push(json!({"cpu_address":word(at),"kind":kind,"indirect":false,"indexed":false,"length":word(5)}));
        }
    } else if tst {
        operand = mode.replace("zp", &format!("z:${:02x}", bytes[2])).replace(
            "abs",
            &format!("a:${:04x}", if length == 4 { word(2) } else { 0 }),
        );
        operand = format!("#${:02x},{operand}", bytes[1]);
        refs.push(json!({"cpu_address":if length==4 {word(2)}else{0x2000+u32::from(bytes[2])},"kind":"memory","indexed":mode.contains(',') ,"indirect":false}));
    } else if bitbranch {
        operand = format!("${:02x},${target:04x}", bytes[1]);
        refs.push(json!({"cpu_address":0x2000+u32::from(bytes[1]),"kind":"memory","indexed":false,"indirect":false}));
    } else if branch {
        operand = format!("${target:04x}");
    } else if immediate {
        operand = format!("#${:02x}", bytes[1]);
    } else if bitmem {
        operand = format!("${:02x}", bytes[1]);
    } else if length > 1 {
        operand = mode.replace("zp", &format!("${:02x}", bytes[1])).replace(
            "abs",
            &format!("a:${:04x}", if length == 3 { word(1) } else { 0 }),
        );
        if op == 0x20 {
            operand = format!("a:${:04x}", word(1));
        }
    }
    if !block && !tst && !immediate && !branch && length > 1 && !bitbranch {
        refs.push(json!({"cpu_address":if length==3{word(1)}else{0x2000+u32::from(bytes[1])},"kind":if matches!(op,0x20|0x4c|0x6c|0x7c){"control"}else{"memory"},"indirect":mode.contains('('),"indexed":mode.contains(",x")||mode.contains(",y")}));
    }
    if branch || bitbranch {
        refs.push(json!({"cpu_address":target,"kind":"control","indirect":false,"indexed":false}));
    }
    let terminal =
        matches!(op, 0x00 | 0x40 | 0x60 | 0x4c | 0x6c | 0x7c | 0x80) || mnemonic == "illegal";
    let mut successors = Vec::new();
    if !terminal {
        successors.push(next);
    }
    if branch || bitbranch {
        successors.push(target);
    }
    if matches!(op, 0x20 | 0x4c) {
        successors.push(word(1));
    }
    let operand = operand.replace("(a:", "(");
    let text = if operand.is_empty() {
        mnemonic.into()
    } else {
        format!("{mnemonic} {operand}")
    };
    let mut after = context;
    if op == 0x53 {
        for slot in 0..8 {
            if bytes[1] & (1 << slot) != 0 {
                after.mpr[slot] = None;
            }
        }
    }
    // Wrapped relative branches cannot be expressed as a ca65 numeric displacement.
    let assembly = if mnemonic == "illegal"
        || (branch || bitbranch)
            && !(-128..=127).contains(&(i64::from(target) - i64::from(pc + length as u32)))
    {
        None
    } else {
        Some(text.clone())
    };
    Ok(Instruction {
        opcode: op,
        length,
        text,
        assembly,
        after,
        successors,
        references: refs,
        terminal,
    })
}
