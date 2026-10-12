//! Interpret actual opcode/operand fetches. Never substitute debugger reads.
use crate::{
    decode::{Flow, decode},
    emulator::Registers,
};
pub fn semantics(bytes: &[u8], cpu_pc: u16, before: &Registers) -> serde_json::Value {
    let Ok(instruction) = decode(bytes, cpu_pc) else {
        return serde_json::json!({"edge_kind":"unknown","traversable":false});
    };
    let taken = if instruction.conditional {
        instruction
            .operands
            .first()
            .map(|operand| match operand.spec.name.as_str() {
                "NZ" => before.f & 0x80 == 0,
                "Z" => before.f & 0x80 != 0,
                "NC" => before.f & 0x10 == 0,
                "C" => before.f & 0x10 != 0,
                _ => false,
            })
    } else {
        None
    };
    let kind = if taken == Some(false) {
        "fallthrough"
    } else {
        match instruction.flow {
            Flow::Next => "fallthrough",
            Flow::Jump => "jump",
            Flow::Call => "call",
            Flow::Return if instruction.mnemonic == "RETI" => "interrupt-return",
            Flow::Return => "return",
            Flow::Restart => "restart",
            Flow::IndirectJump => "indirect-jump",
            Flow::Halt => "halt",
            Flow::Stop => "stop",
            Flow::Invalid => "invalid",
        }
    };
    serde_json::json!({"edge_kind":kind,"conditional":instruction.conditional,"taken":taken,"traversable":!matches!(instruction.flow,Flow::Halt|Flow::Stop|Flow::Invalid),"encoded_cpu_target":instruction.target,"fallthrough_cpu":instruction.fallthrough})
}
#[cfg(test)]
mod tests {
    use super::*;
    fn registers(f: u8) -> Registers {
        Registers {
            a: 0,
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            h: 0,
            l: 0,
            f,
            pc: 0x100,
            sp: 0xfffe,
        }
    }
    #[test]
    fn equal_target_does_not_hide_taken_branch() {
        let yes = semantics(&[0x28, 0], 0x100, &registers(0x80));
        let no = semantics(&[0x28, 0], 0x100, &registers(0));
        assert_eq!(yes["taken"], true);
        assert_eq!(no["taken"], false);
        assert_eq!(yes["edge_kind"], "jump");
        assert_eq!(no["edge_kind"], "fallthrough");
    }
    #[test]
    fn calls_returns_indirection_and_suspend() {
        for (bytes, kind, traverse) in [
            (&[0xcd, 0, 2][..], "call", true),
            (&[0xc9][..], "return", true),
            (&[0xd9][..], "interrupt-return", true),
            (&[0xe9][..], "indirect-jump", true),
            (&[0x76][..], "halt", false),
            (&[0x10, 0][..], "stop", false),
        ] {
            let result = semantics(bytes, 0x100, &registers(0));
            assert_eq!(result["edge_kind"], kind);
            assert_eq!(result["traversable"], traverse);
        }
    }
}
