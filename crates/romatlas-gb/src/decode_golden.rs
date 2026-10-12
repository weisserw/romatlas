//! Hand-derived SM83 base matrix, independent of the bundled opcode JSON.
use crate::decode::{decode, spec};
const LOW: &str = r#"
NOP - ----
LD BC,n16 ----
LD (BC),A ----
INC BC ----
INC B Z0H-
DEC B Z1H-
LD B,n8 ----
RLCA - 000C
LD (a16),SP ----
ADD HL,BC -0HC
LD A,(BC) ----
DEC BC ----
INC C Z0H-
DEC C Z1H-
LD C,n8 ----
RRCA - 000C
STOP n8 ----
LD DE,n16 ----
LD (DE),A ----
INC DE ----
INC D Z0H-
DEC D Z1H-
LD D,n8 ----
RLA - 000C
JR e8 ----
ADD HL,DE -0HC
LD A,(DE) ----
DEC DE ----
INC E Z0H-
DEC E Z1H-
LD E,n8 ----
RRA - 000C
JR NZ,e8 ----
LD HL,n16 ----
LD (HL+),A ----
INC HL ----
INC H Z0H-
DEC H Z1H-
LD H,n8 ----
DAA - Z-0C
JR Z,e8 ----
ADD HL,HL -0HC
LD A,(HL+) ----
DEC HL ----
INC L Z0H-
DEC L Z1H-
LD L,n8 ----
CPL - -11-
JR NC,e8 ----
LD SP,n16 ----
LD (HL-),A ----
INC SP ----
INC (HL) Z0H-
DEC (HL) Z1H-
LD (HL),n8 ----
SCF - -001
JR C,e8 ----
ADD HL,SP -0HC
LD A,(HL-) ----
DEC SP ----
INC A Z0H-
DEC A Z1H-
LD A,n8 ----
CCF - -00C
"#;
const HIGH: &str = r#"
RET NZ ----
POP BC ----
JP NZ,a16 ----
JP a16 ----
CALL NZ,a16 ----
PUSH BC ----
ADD A,n8 Z0HC
RST $00 ----
RET Z ----
RET - ----
JP Z,a16 ----
PREFIX - ----
CALL Z,a16 ----
CALL a16 ----
ADC A,n8 Z0HC
RST $08 ----
RET NC ----
POP DE ----
JP NC,a16 ----
X - ----
CALL NC,a16 ----
PUSH DE ----
SUB A,n8 Z1HC
RST $10 ----
RET C ----
RETI - ----
JP C,a16 ----
X - ----
CALL C,a16 ----
X - ----
SBC A,n8 Z1HC
RST $18 ----
LDH (a8),A ----
POP HL ----
LDH (C),A ----
X - ----
X - ----
PUSH HL ----
AND A,n8 Z010
RST $20 ----
ADD SP,e8 00HC
JP HL ----
LD (a16),A ----
X - ----
X - ----
X - ----
XOR A,n8 Z000
RST $28 ----
LDH A,(a8) ----
POP AF ZNHC
LDH A,(C) ----
DI - ----
X - ----
PUSH AF ----
OR A,n8 Z000
RST $30 ----
LD HL,SP+,e8 00HC
LD SP,HL ----
LD A,(a16) ----
EI - ----
X - ----
X - ----
CP A,n8 Z1HC
RST $38 ----
"#;
fn check(op: u8, mnemonic: &str, operands: &str, flags: &str) {
    let s = spec(op, false);
    let mnemonic = if mnemonic == "X" {
        format!("ILLEGAL_{op:02X}")
    } else {
        mnemonic.to_owned()
    };
    assert_eq!(s.mnemonic, mnemonic, "opcode {op:02x}");
    let operands: Vec<_> = if operands == "-" {
        vec![]
    } else {
        operands.split(',').collect()
    };
    assert_eq!(s.operands.len(), operands.len(), "opcode {op:02x}");
    for (actual, token) in s.operands.iter().zip(&operands) {
        let indirect = token.starts_with('(');
        let raw = token.trim_matches(['(', ')']);
        let increment = raw.ends_with('+');
        let decrement = raw.ends_with('-');
        let name = raw.trim_end_matches(['+', '-']);
        assert_eq!(actual.name, name, "opcode {op:02x}");
        assert_eq!(actual.immediate, !indirect, "opcode {op:02x}");
        assert_eq!(actual.increment, increment, "opcode {op:02x}");
        assert_eq!(actual.decrement, decrement, "opcode {op:02x}");
        let width = match name {
            "a16" | "n16" => 2,
            "a8" | "n8" | "e8" => 1,
            _ => 0,
        };
        assert_eq!(actual.bytes, width, "opcode {op:02x}");
    }
    for (name, value) in ["Z", "N", "H", "C"].iter().zip(flags.chars()) {
        assert_eq!(
            s.flags[*name],
            value.to_string(),
            "opcode {op:02x} flag {name}"
        );
    }
    if op != 0xcb {
        let d = decode(&[op, 0x80, 0x34], 0x100).unwrap();
        assert_eq!(d.mnemonic, mnemonic);
        for (name, value) in ["Z", "N", "H", "C"].iter().zip(flags.chars()) {
            assert_eq!(
                d.flags[*name],
                value.to_string(),
                "decoded opcode {op:02x} flag {name}"
            );
        }
        let decoded_specs: Vec<_> = d.operands.iter().map(|o| &o.spec).collect();
        assert_eq!(
            serde_json::to_value(decoded_specs).unwrap(),
            serde_json::to_value(&s.operands).unwrap(),
            "decoded operand descriptors {op:02x}"
        );
        let mut cursor = 1;
        for operand in d.operands {
            let expected = match operand.spec.name.as_str() {
                "a16" | "n16" => {
                    assert_eq!(cursor, 1);
                    cursor += 2;
                    Some(0x3480)
                }
                "a8" | "n8" => {
                    cursor += 1;
                    Some(0x80)
                }
                "e8" => {
                    cursor += 1;
                    Some(-128)
                }
                _ => None,
            };
            assert_eq!(operand.value, expected, "opcode {op:02x}");
        }
    }
}
#[test]
fn all_base_operands_and_flags() {
    for (start, text) in [(0u8, LOW), (0xc0, HIGH)] {
        let rows: Vec<_> = text.lines().filter(|r| !r.trim().is_empty()).collect();
        assert_eq!(rows.len(), 64);
        for (index, line) in rows.iter().enumerate() {
            let cells: Vec<_> = line.split_whitespace().collect();
            assert_eq!(cells.len(), 3);
            check(start + index as u8, cells[0], cells[1], cells[2]);
        }
    }
    let regs = ["B", "C", "D", "E", "H", "L", "(HL)", "A"];
    for op in 0x40..=0x7f {
        if op == 0x76 {
            check(op, "HALT", "-", "----");
        } else {
            check(
                op,
                "LD",
                &format!(
                    "{},{}",
                    regs[((op >> 3) & 7) as usize],
                    regs[(op & 7) as usize]
                ),
                "----",
            );
        }
    }
    for op in 0x80..=0xbf {
        let group = ((op >> 3) & 7) as usize;
        check(
            op,
            ["ADD", "ADC", "SUB", "SBC", "AND", "XOR", "OR", "CP"][group],
            &format!("A,{}", regs[(op & 7) as usize]),
            match op {
                0x97 | 0xbf => "1100",
                0xaf => "1000",
                0x9f => "Z1H-",
                _ => [
                    "Z0HC", "Z0HC", "Z1HC", "Z1HC", "Z010", "Z000", "Z000", "Z1HC",
                ][group],
            },
        );
    }
}
