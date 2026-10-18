//! Complete opcode formatting with explicit local width ambiguity.
use anyhow::{Context as _, Result, ensure};
use serde::{Deserialize, Serialize};
#[derive(Clone, Copy, Debug, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Context {
    pub e: Option<bool>,
    pub m: Option<bool>,
    pub x: Option<bool>,
    pub carry: Option<bool>,
    pub db: Option<u8>,
    pub direct_page: Option<u16>,
}
#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum Cpu {
    Main,
    Spc700,
}
impl Default for Cpu {
    fn default() -> Self {
        Self::Main
    }
}
#[derive(Clone, Debug, Serialize)]
pub struct Instruction {
    pub cpu: Cpu,
    pub cpu_address: u32,
    pub opcode: u8,
    pub length: usize,
    pub text: String,
    pub assembly: Option<String>,
    pub target: Option<u32>,
    pub conditional: bool,
    pub call: bool,
    pub terminal: bool,
    pub indirect: bool,
    pub after: Context,
    pub diagnostics: Vec<String>,
    pub references: Vec<serde_json::Value>,
}
fn value(data: &[u8]) -> u32 {
    data.iter()
        .enumerate()
        .fold(0, |n, (i, b)| n | (u32::from(*b) << (8 * i)))
}
pub fn decode(cpu: Cpu, data: &[u8], pc: u32, ctx: Context) -> Result<Instruction> {
    ensure!(
        pc <= if cpu == Cpu::Main { 0xffffff } else { 0xffff },
        "CPU address outside processor space"
    );
    let op = *data.first().context("missing opcode")?;
    let mut i = Instruction {
        cpu,
        cpu_address: pc,
        opcode: op,
        length: 1,
        text: String::new(),
        assembly: None,
        target: None,
        conditional: false,
        call: false,
        terminal: false,
        indirect: false,
        after: ctx,
        diagnostics: Vec::new(),
        references: Vec::new(),
    };
    if cpu == Cpu::Spc700 {
        return spc(data, i);
    }
    let template = wdc65816_emu::core::debug_instruction_template(op);
    let (name, operand) = template.split_once(' ').unwrap_or((template, ""));
    let short_branch = matches!(
        op,
        0x10 | 0x30 | 0x50 | 0x70 | 0x80 | 0x90 | 0xb0 | 0xd0 | 0xf0
    );
    let relative = short_branch || matches!(op, 0x62 | 0x82);
    let mut width = None;
    i.length = if matches!(op, 0 | 2 | 0x42) {
        2
    } else if matches!(op, 0x44 | 0x54 | 0x62 | 0x82) {
        3
    } else if short_branch {
        2
    } else if operand.contains("#<d>") {
        let flag = if matches!(op, 0xa0 | 0xa2 | 0xc0 | 0xe0) {
            ctx.x
        } else if matches!(op, 0xc2 | 0xe2) {
            Some(true)
        } else {
            ctx.m
        };
        let eight = if ctx.e == Some(true) {
            true
        } else {
            flag.context("immediate length ambiguous: supply relevant M/X or E context, statically established or observed")?
        };
        width = Some(eight);
        if eight { 2 } else { 3 }
    } else if operand.contains("$xxxxxx") {
        4
    } else if operand.contains("$xxxx") {
        3
    } else if operand.contains("$xx") {
        2
    } else {
        1
    };
    ensure!(data.len() >= i.length, "truncated instruction");
    ensure!(
        (pc & 0xffff) + i.length as u32 <= 0x10000,
        "instruction fetch crosses CPU bank boundary; use a mapped execution view"
    );
    let n = value(&data[1..i.length]);
    let mut text = operand.to_lowercase();
    let mut mnemonic = name.to_lowercase();
    if relative {
        let signed = if i.length == 2 {
            i32::from(data[1] as i8)
        } else {
            i32::from(n as i16)
        };
        let target = (pc & 0xff0000)
            | ((pc.wrapping_add(i.length as u32).wrapping_add_signed(signed)) & 0xffff);
        i.target = Some(target);
        text = format!("${n:0width$x}", width = (i.length - 1) * 2);
    } else if matches!(op, 0 | 2 | 0x42) {
        text = format!("#${n:02x}");
    } else if matches!(op, 0x44 | 0x54) {
        text = format!("${:02x},${:02x}", data[1], data[2]);
    } else if let Some(eight) = width {
        text = format!("#${n:0width$x}", width = if eight { 2 } else { 4 });
        mnemonic.push_str(if eight { ".b" } else { ".w" });
    } else if operand.contains("$xxxxxx") {
        text = text.replace("$xxxxxx", &format!("${n:06x}"));
        if op == 0x5c {
            mnemonic = "jml".into();
        } else {
            mnemonic.push_str(".l");
        }
    } else if operand.contains("$xxxx") {
        text = text.replace("$xxxx", &format!("${n:04x}"));
        mnemonic.push_str(".w");
    } else if operand.contains("$xx") {
        text = text.replace("$xx", &format!("${n:02x}"));
        mnemonic.push_str(".b");
    }
    i.text = if text.is_empty() {
        mnemonic
    } else {
        format!("{mnemonic} {text}")
    };
    i.assembly = Some(i.text.clone());
    if matches!(op, 0x20 | 0x4c) {
        i.target = Some((pc & 0xff0000) | n);
    }
    if matches!(op, 0x22 | 0x5c) {
        i.target = Some(n);
    }
    i.call = matches!(op, 0x20 | 0x22 | 0xfc);
    i.indirect = matches!(op, 0x6c | 0x7c | 0xdc | 0xfc);
    i.conditional = short_branch && op != 0x80;
    i.terminal = matches!(
        op,
        0 | 2 | 0x40 | 0x4c | 0x5c | 0x60 | 0x6b | 0x6c | 0x7c | 0x80 | 0x82 | 0xdb | 0xdc
    );
    if op == 0x62 {
        i.target = None;
    } // PER pushes an address; it does not transfer control.
    if operand.contains('$') && !relative && !matches!(op, 0x20 | 0x22 | 0x4c | 0x5c | 0xf4) {
        let indexed = operand.contains(",X") || operand.contains(",Y") || operand.contains(",S");
        let pointer = operand.contains('(') || operand.contains('[');
        let base = if operand.contains("$xxxxxx") {
            Some(n)
        } else if operand.contains("$xxxx") {
            if matches!(op, 0x20 | 0x4c | 0x7c | 0xfc) {
                Some((pc & 0xff0000) | n)
            } else if matches!(op, 0x6c | 0xdc) {
                Some(n)
            } else {
                ctx.db.map(|b| (u32::from(b) << 16) | n)
            }
        } else if operand.contains(",S") {
            None
        } else {
            ctx.direct_page.map(|d| u32::from(d.wrapping_add(n as u16)))
        };
        i.references.push(serde_json::json!({"cpu_address":if indexed{None}else{base},"base_cpu_address":base,"encoded_value":n,"indexed":indexed,"kind":if pointer{"pointer-storage"}else if matches!(name,"STA"|"STX"|"STY"|"STZ"){"write"}else if matches!(name,"ASL"|"LSR"|"ROL"|"ROR"|"INC"|"DEC"|"TRB"|"TSB"){"read-write"}else{"read"},"diagnostic":if indexed{"index register value unresolved"}else if base.is_none(){"effective bank/direct-page context unresolved"}else{""}}));
    }
    if let Some(target) = i.target {
        i.references
            .push(serde_json::json!({"kind":"control-target","cpu_address":target}));
    }
    match op {
        0x44 | 0x54 => i.after.db = Some(data[1]),
        0xc2 | 0xe2 => {
            let set = op == 0xe2;
            if n & 0x20 != 0 {
                i.after.m = if !set && ctx.e.is_none() {
                    None
                } else {
                    Some(set || ctx.e == Some(true))
                };
            }
            if n & 0x10 != 0 {
                i.after.x = if !set && ctx.e.is_none() {
                    None
                } else {
                    Some(set || ctx.e == Some(true))
                };
            }
            if n & 1 != 0 {
                i.after.carry = Some(set);
            }
        }
        0x18 => i.after.carry = Some(false),
        0x38 => i.after.carry = Some(true),
        0xfb => {
            i.after.e = ctx.carry;
            i.after.carry = ctx.e;
            if i.after.e == Some(true) {
                i.after.m = Some(true);
                i.after.x = Some(true);
            } else if i.after.e.is_none() {
                if ctx.m != Some(true) {
                    i.after.m = None;
                }
                if ctx.x != Some(true) {
                    i.after.x = None;
                }
            }
        }
        0x28 | 0x40 => {
            i.after.m = if ctx.e == Some(true) {
                Some(true)
            } else {
                None
            };
            i.after.x = i.after.m;
            i.after.carry = None;
        }
        0xab => i.after.db = None,
        0x2b | 0x5b => i.after.direct_page = None,
        _ => {
            // Preserve carry only for opcodes known not to modify it.
            if matches!(
                name,
                "ADC" | "SBC" | "CMP" | "CPX" | "CPY" | "ASL" | "LSR" | "ROL" | "ROR"
            ) {
                i.after.carry = None;
            }
        }
    }
    if i.indirect {
        i.diagnostics
            .push("indirect target requires additional reference context".into());
    }
    Ok(i)
}
fn spc(data: &[u8], mut i: Instruction) -> Result<Instruction> {
    let op = i.opcode;
    let template = spc700_emu::debug_instruction_template(op);
    let (name, operands) = template.split_once(' ').unwrap_or((template, ""));
    let branch = operands.contains(", r") || operands == "r";
    let direct_pair = operands.contains("dd") || operands.contains("d, #i");
    let absolute = operands.contains("!a") || operands.contains("m.b");
    let direct = operands
        .split(|c: char| !c.is_ascii_alphabetic())
        .any(|s| s == "d");
    i.length = if direct_pair || absolute || branch && direct {
        3
    } else if direct || operands.contains("#i") || branch || operands == "u" {
        2
    } else {
        1
    };
    ensure!(data.len() >= i.length, "truncated SPC700 instruction");
    ensure!(
        i.cpu_address + i.length as u32 <= 0x10000,
        "SPC instruction wraps ARAM; explicit execution view required"
    );
    let b = data.get(1).copied().unwrap_or(0);
    let word = if i.length == 3 {
        u16::from_le_bytes([b, data[2]])
    } else {
        u16::from(b)
    };
    let mut args = operands.to_string();
    let mut mnemonic = name.to_lowercase();
    if direct_pair {
        args = args
            .replace("dd", &format!("${:02x}", data[2]))
            .replace("ds", &format!("${b:02x}"));
        if operands.contains("#i") {
            args = args.replace("#i", &format!("#${b:02x}")).replacen(
                "d",
                &format!("${:02x}", data[2]),
                1,
            );
        }
    } else {
        args = args
            .replace("#i", &format!("#${b:02x}"))
            .replace("!a", &format!("${word:04x}"));
        if operands.contains("m.b") {
            // Asar accepts explicit .bit memory operands; preserve all 16 encoded bits.
            args = args.replace("m.b", &format!("${:04x}.{}", word & 0x1fff, word >> 13));
        } else if direct {
            args = args.replace('d', &format!("${b:02x}"));
        }
    }
    if branch {
        let disp = data[i.length - 1];
        let target =
            (i.cpu_address + i.length as u32).wrapping_add_signed(i32::from(disp as i8)) & 0xffff;
        i.target = Some(target);
        args = args.replace(
            'r',
            &if op & 0x0f == 3 {
                format!("${target:04x}")
            } else {
                format!("${disp:02x}")
            },
        );
    }
    if operands == "u" {
        args = format!("${b:02x}");
        i.target = Some(0xff00 | u32::from(b));
    }
    if matches!(op, 0x3f | 0x5f) {
        i.target = Some(u32::from(word));
    }
    if matches!(name, "BBS" | "BBC" | "SET1" | "CLR1") {
        mnemonic = match name {
            "SET1" => format!("set{}", op >> 5),
            "CLR1" => format!("clr{}", op >> 5),
            _ => format!("{}{}", name.to_lowercase(), op >> 5),
        };
        args = args.replace(&format!(".{}", op >> 5), "");
    }
    if matches!(name, "TSET1" | "TCLR1" | "TCLR") {
        mnemonic = if name == "TSET1" { "tset" } else { "tclr" }.into();
        args.push_str(",a");
    }
    args = args
        .to_lowercase()
        .replace('[', "(")
        .replace(']', ")")
        .replace("psw", "p")
        .replace("(x)+", "(x+)")
        .replace('/', "!");
    i.text = if args.is_empty() {
        mnemonic
    } else {
        format!("{mnemonic} {args}")
    };
    i.assembly = Some(i.text.clone());
    if branch && op & 0x0f == 3 {
        let target = i.cpu_address as i32 + i.length as i32 + i32::from(data[i.length - 1] as i8);
        if !(0..=65535).contains(&target) {
            i.assembly = None;
            i.diagnostics
                .push("Asar SPC bit-branch wraps address space; retain exact raw encoding".into());
        }
    }
    if (direct || absolute) && !matches!(op, 0x3f | 0x5f) {
        let indexed =
            operands.contains("+X") || operands.contains("+Y") || operands.contains("(X)");
        let base = if operands.contains("m.b") {
            Some(u32::from(word & 0x1fff))
        } else if absolute {
            Some(u32::from(word))
        } else {
            i.after
                .direct_page
                .filter(|d| matches!(*d, 0 | 0x100))
                .map(|d| u32::from(d) + u32::from(if direct_pair { data[2] } else { b }))
        };
        i.references.push(serde_json::json!({"kind":if operands.contains('['){"pointer-storage"}else{"memory"},"cpu_address":if indexed{None}else{base},"base_cpu_address":base,"indexed":indexed}));
    }
    if let Some(target) = i.target {
        i.references
            .push(serde_json::json!({"kind":"control-target","cpu_address":target}));
    }
    if direct_pair && operands.contains("dd") {
        for (kind, byte) in [("source", data[1]), ("destination", data[2])] {
            i.references.push(serde_json::json!({"kind":kind,"cpu_address":i.after.direct_page.filter(|d|matches!(*d,0|0x100)).map(|d|u32::from(d)+u32::from(byte))}));
        }
    }
    if op & 0x0f == 1 {
        i.references.push(serde_json::json!({"kind":"pointer-storage","cpu_address":0xffdeu32-(u32::from(op>>4)*2)}));
    }
    if op == 0x20 {
        i.after.direct_page = Some(0);
    } else if op == 0x40 {
        i.after.direct_page = Some(0x100);
    } else if matches!(op, 0x8e | 0x7f) {
        i.after.direct_page = None;
    }
    i.conditional = branch && op != 0x2f;
    i.call = matches!(op, 0x3f | 0x4f) || op & 0x0f == 1;
    i.terminal = matches!(op, 0x0f | 0x1f | 0x2f | 0x5f | 0x6f | 0x7f | 0xef | 0xff);
    i.indirect = op == 0x1f || op & 0x0f == 1;
    Ok(i)
}
