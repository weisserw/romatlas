//! Complete NMOS 6502 lengths/operands, including unofficial encodings.
use anyhow::{Result, ensure};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Cpu {
    Main,
}
/// Effective physical 8 KiB PRG bank in each CPU slot $8000/$A000/$C000/$E000.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Context {
    #[serde(default)]
    pub prg_banks: [Option<u32>; 4],
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
const OPS: [(&str, &str); 256] = [
    ("brk", "imp"),
    ("ora", "inx"),
    ("kil", "imp"),
    ("slo", "inx"),
    ("nop", "zp"),
    ("ora", "zp"),
    ("asl", "zp"),
    ("slo", "zp"),
    ("php", "imp"),
    ("ora", "imm"),
    ("asl", "acc"),
    ("anc", "imm"),
    ("nop", "abs"),
    ("ora", "abs"),
    ("asl", "abs"),
    ("slo", "abs"),
    ("bpl", "rel"),
    ("ora", "iny"),
    ("kil", "imp"),
    ("slo", "iny"),
    ("nop", "zpx"),
    ("ora", "zpx"),
    ("asl", "zpx"),
    ("slo", "zpx"),
    ("clc", "imp"),
    ("ora", "aby"),
    ("nop", "imp"),
    ("slo", "aby"),
    ("nop", "abx"),
    ("ora", "abx"),
    ("asl", "abx"),
    ("slo", "abx"),
    ("jsr", "abs"),
    ("and", "inx"),
    ("kil", "imp"),
    ("rla", "inx"),
    ("bit", "zp"),
    ("and", "zp"),
    ("rol", "zp"),
    ("rla", "zp"),
    ("plp", "imp"),
    ("and", "imm"),
    ("rol", "acc"),
    ("anc", "imm"),
    ("bit", "abs"),
    ("and", "abs"),
    ("rol", "abs"),
    ("rla", "abs"),
    ("bmi", "rel"),
    ("and", "iny"),
    ("kil", "imp"),
    ("rla", "iny"),
    ("nop", "zpx"),
    ("and", "zpx"),
    ("rol", "zpx"),
    ("rla", "zpx"),
    ("sec", "imp"),
    ("and", "aby"),
    ("nop", "imp"),
    ("rla", "aby"),
    ("nop", "abx"),
    ("and", "abx"),
    ("rol", "abx"),
    ("rla", "abx"),
    ("rti", "imp"),
    ("eor", "inx"),
    ("kil", "imp"),
    ("sre", "inx"),
    ("nop", "zp"),
    ("eor", "zp"),
    ("lsr", "zp"),
    ("sre", "zp"),
    ("pha", "imp"),
    ("eor", "imm"),
    ("lsr", "acc"),
    ("alr", "imm"),
    ("jmp", "abs"),
    ("eor", "abs"),
    ("lsr", "abs"),
    ("sre", "abs"),
    ("bvc", "rel"),
    ("eor", "iny"),
    ("kil", "imp"),
    ("sre", "iny"),
    ("nop", "zpx"),
    ("eor", "zpx"),
    ("lsr", "zpx"),
    ("sre", "zpx"),
    ("cli", "imp"),
    ("eor", "aby"),
    ("nop", "imp"),
    ("sre", "aby"),
    ("nop", "abx"),
    ("eor", "abx"),
    ("lsr", "abx"),
    ("sre", "abx"),
    ("rts", "imp"),
    ("adc", "inx"),
    ("kil", "imp"),
    ("rra", "inx"),
    ("nop", "zp"),
    ("adc", "zp"),
    ("ror", "zp"),
    ("rra", "zp"),
    ("pla", "imp"),
    ("adc", "imm"),
    ("ror", "acc"),
    ("arr", "imm"),
    ("jmp", "ind"),
    ("adc", "abs"),
    ("ror", "abs"),
    ("rra", "abs"),
    ("bvs", "rel"),
    ("adc", "iny"),
    ("kil", "imp"),
    ("rra", "iny"),
    ("nop", "zpx"),
    ("adc", "zpx"),
    ("ror", "zpx"),
    ("rra", "zpx"),
    ("sei", "imp"),
    ("adc", "aby"),
    ("nop", "imp"),
    ("rra", "aby"),
    ("nop", "abx"),
    ("adc", "abx"),
    ("ror", "abx"),
    ("rra", "abx"),
    ("nop", "imm"),
    ("sta", "inx"),
    ("nop", "imm"),
    ("sax", "inx"),
    ("sty", "zp"),
    ("sta", "zp"),
    ("stx", "zp"),
    ("sax", "zp"),
    ("dey", "imp"),
    ("nop", "imm"),
    ("txa", "imp"),
    ("xaa", "imm"),
    ("sty", "abs"),
    ("sta", "abs"),
    ("stx", "abs"),
    ("sax", "abs"),
    ("bcc", "rel"),
    ("sta", "iny"),
    ("kil", "imp"),
    ("ahx", "iny"),
    ("sty", "zpx"),
    ("sta", "zpx"),
    ("stx", "zpy"),
    ("sax", "zpy"),
    ("tya", "imp"),
    ("sta", "aby"),
    ("txs", "imp"),
    ("tas", "aby"),
    ("shy", "abx"),
    ("sta", "abx"),
    ("shx", "aby"),
    ("ahx", "aby"),
    ("ldy", "imm"),
    ("lda", "inx"),
    ("ldx", "imm"),
    ("lax", "inx"),
    ("ldy", "zp"),
    ("lda", "zp"),
    ("ldx", "zp"),
    ("lax", "zp"),
    ("tay", "imp"),
    ("lda", "imm"),
    ("tax", "imp"),
    ("lax", "imm"),
    ("ldy", "abs"),
    ("lda", "abs"),
    ("ldx", "abs"),
    ("lax", "abs"),
    ("bcs", "rel"),
    ("lda", "iny"),
    ("kil", "imp"),
    ("lax", "iny"),
    ("ldy", "zpx"),
    ("lda", "zpx"),
    ("ldx", "zpy"),
    ("lax", "zpy"),
    ("clv", "imp"),
    ("lda", "aby"),
    ("tsx", "imp"),
    ("las", "aby"),
    ("ldy", "abx"),
    ("lda", "abx"),
    ("ldx", "aby"),
    ("lax", "aby"),
    ("cpy", "imm"),
    ("cmp", "inx"),
    ("nop", "imm"),
    ("dcp", "inx"),
    ("cpy", "zp"),
    ("cmp", "zp"),
    ("dec", "zp"),
    ("dcp", "zp"),
    ("iny", "imp"),
    ("cmp", "imm"),
    ("dex", "imp"),
    ("axs", "imm"),
    ("cpy", "abs"),
    ("cmp", "abs"),
    ("dec", "abs"),
    ("dcp", "abs"),
    ("bne", "rel"),
    ("cmp", "iny"),
    ("kil", "imp"),
    ("dcp", "iny"),
    ("nop", "zpx"),
    ("cmp", "zpx"),
    ("dec", "zpx"),
    ("dcp", "zpx"),
    ("cld", "imp"),
    ("cmp", "aby"),
    ("nop", "imp"),
    ("dcp", "aby"),
    ("nop", "abx"),
    ("cmp", "abx"),
    ("dec", "abx"),
    ("dcp", "abx"),
    ("cpx", "imm"),
    ("sbc", "inx"),
    ("nop", "imm"),
    ("isc", "inx"),
    ("cpx", "zp"),
    ("sbc", "zp"),
    ("inc", "zp"),
    ("isc", "zp"),
    ("inx", "imp"),
    ("sbc", "imm"),
    ("nop", "imp"),
    ("sbc", "imm"),
    ("cpx", "abs"),
    ("sbc", "abs"),
    ("inc", "abs"),
    ("isc", "abs"),
    ("beq", "rel"),
    ("sbc", "iny"),
    ("kil", "imp"),
    ("isc", "iny"),
    ("nop", "zpx"),
    ("sbc", "zpx"),
    ("inc", "zpx"),
    ("isc", "zpx"),
    ("sed", "imp"),
    ("sbc", "aby"),
    ("nop", "imp"),
    ("isc", "aby"),
    ("nop", "abx"),
    ("sbc", "abx"),
    ("inc", "abx"),
    ("isc", "abx"),
];
pub fn decode(_: Cpu, bytes: &[u8], pc: u32, context: Context) -> Result<Instruction> {
    ensure!(pc <= 65535, "6502 PC exceeds 16 bits");
    ensure!(!bytes.is_empty(), "missing opcode");
    let opcode = bytes[0];
    let (mnemonic, mode) = OPS[opcode as usize];
    let length = match mode {
        "imp" | "acc" => 1,
        "abs" | "abx" | "aby" | "ind" => 3,
        _ => 2,
    };
    ensure!(bytes.len() >= length, "truncated instruction");
    let value = if length == 3 {
        u32::from(u16::from_le_bytes([bytes[1], bytes[2]]))
    } else if length == 2 {
        u32::from(bytes[1])
    } else {
        0
    };
    let next = (pc + length as u32) & 65535;
    let target = if mode == "rel" {
        ((next as i32 + i32::from(bytes[1] as i8)) & 65535) as u32
    } else {
        value
    };
    let operand = match mode {
        "imp" => String::new(),
        "acc" => "a".into(),
        "imm" => format!("#${value:02x}"),
        "zp" => format!("z:${value:02x}"),
        "zpx" => format!("z:${value:02x},x"),
        "zpy" => format!("z:${value:02x},y"),
        "abs" => format!("a:${value:04x}"),
        "abx" => format!("a:${value:04x},x"),
        "aby" => format!("a:${value:04x},y"),
        "ind" => format!("(${value:04x})"),
        "inx" => format!("(${value:02x},x)"),
        "iny" => format!("(${value:02x}),y"),
        "rel" => format!("${target:04x}"),
        _ => unreachable!(),
    };
    let text = if operand.is_empty() {
        mnemonic.into()
    } else {
        format!("{mnemonic} {operand}")
    };
    let terminal = matches!(mnemonic, "rts" | "rti" | "brk" | "kil") || mnemonic == "jmp";
    let mut successors = Vec::new();
    if !terminal {
        successors.push(next);
    }
    if mode == "rel" || mnemonic == "jmp" && mode == "abs" || mnemonic == "jsr" {
        successors.push(target);
    }
    let mut references = Vec::new();
    if !matches!(mode, "imp" | "acc" | "imm") {
        references.push(json!({"cpu_address":target,"kind":if mode=="rel"||matches!(mnemonic,"jmp"|"jsr"){"control"}else{"memory"},"indirect":matches!(mode,"ind"|"inx"|"iny"),"indexed":matches!(mode,"abx"|"aby"|"zpx"|"zpy"|"inx"|"iny")}));
    }
    // Ambiguous unofficial aliases and wrapping branches use exact byte directives;
    // the decoded mnemonic remains readable beside them. Every encoding survives.
    let unofficial = matches!(
        mnemonic,
        "slo"
            | "rla"
            | "sre"
            | "rra"
            | "sax"
            | "lax"
            | "dcp"
            | "isc"
            | "anc"
            | "alr"
            | "arr"
            | "xaa"
            | "ahx"
            | "tas"
            | "shy"
            | "shx"
            | "las"
            | "axs"
            | "kil"
    ) || (mnemonic == "nop" && opcode != 0xea)
        || opcode == 0xeb;
    let assembly = if !unofficial
        && (mode != "rel" || (-128..=127).contains(&(i64::from(target) - i64::from(pc + 2))))
    {
        Some(text.clone())
    } else {
        None
    };
    Ok(Instruction {
        opcode,
        length,
        text,
        assembly,
        after: context,
        successors,
        references,
        terminal,
    })
}
