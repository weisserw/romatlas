//! GNU binutils supplies the complete 68000 decoder; DeZ80 supplies the Z80 decoder.
use anyhow::{Context as _, Result, ensure};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use std::{
    path::PathBuf,
    process::Command,
    sync::atomic::{AtomicU64, Ordering},
};
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Cpu {
    Main,
    Z80,
}
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Context {
    #[serde(default)]
    pub rom_banks: [Option<u32>; 8],
    #[serde(default)]
    pub ram_enabled: Option<bool>,
    #[serde(default)]
    pub z80_bank: Option<u32>,
}
#[derive(Clone, Debug, Serialize)]
pub struct Instruction {
    pub opcode: u16,
    pub length: usize,
    pub text: String,
    pub assembly: Option<String>,
    pub after: Context,
    pub successors: Vec<u32>,
    pub references: Vec<Value>,
    pub terminal: bool,
}
static SERIAL: AtomicU64 = AtomicU64::new(0);
struct Temp(PathBuf);
impl Drop for Temp {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}
pub fn decode(cpu: Cpu, bytes: &[u8], pc: u32, context: Context) -> Result<Instruction> {
    decode_with_budget(
        cpu,
        bytes,
        pc,
        context,
        &romatlas_disasm::Budget::new(5000)?,
    )
}
pub fn decode_with_budget(
    cpu: Cpu,
    bytes: &[u8],
    pc: u32,
    context: Context,
    budget: &romatlas_disasm::Budget,
) -> Result<Instruction> {
    ensure!(
        context.rom_banks.iter().flatten().all(|&b| b <= 255)
            && context.z80_bank.is_none_or(|b| b <= 511),
        "bank context exceeds hardware register width"
    );
    if cpu == Cpu::Z80 {
        let i = romatlas_sms::decode::decode(
            romatlas_sms::decode::Cpu::Main,
            bytes,
            pc,
            Default::default(),
        )?;
        return Ok(Instruction {
            opcode: u16::from(i.opcode),
            length: i.length,
            text: i.text,
            assembly: i.assembly,
            after: context,
            successors: i.successors,
            references: i.references,
            terminal: i.terminal,
        });
    }
    ensure!(
        pc <= 0xffffff && pc.is_multiple_of(2) && bytes.len() >= 2,
        "68000 needs an aligned 24-bit PC and complete word"
    );
    let trap_word = u16::from_be_bytes([bytes[0], bytes[1]]);
    if matches!(trap_word >> 12, 0xa | 0xf) {
        return Ok(Instruction {
            opcode: trap_word,
            length: 2,
            text: format!(
                "line-{} trap 0x{trap_word:04x}",
                if trap_word >> 12 == 0xa { "a" } else { "f" }
            ),
            assembly: None,
            after: context,
            successors: vec![],
            references: vec![],
            terminal: true,
        });
    }
    type Cache = std::collections::BTreeMap<(u32, Vec<u8>), Instruction>;
    static CACHE: std::sync::OnceLock<std::sync::Mutex<Cache>> = std::sync::OnceLock::new();
    let key = (pc, bytes[..bytes.len().min(22)].to_vec());
    budget.check()?;
    if let Some(mut i) = CACHE
        .get_or_init(Default::default)
        .lock()
        .unwrap()
        .get(&key)
        .cloned()
    {
        i.after = context;
        return Ok(i);
    }
    let temp = Temp(std::env::temp_dir().join(format!(
        "romatlas-m68k-{}-{}",
        std::process::id(),
        SERIAL.fetch_add(1, Ordering::Relaxed)
    )));
    std::fs::create_dir(&temp.0)?;
    let file = temp.0.join("input.bin");
    std::fs::write(&file, &bytes[..bytes.len().min(22)])?;
    let text = budget
        .run(
            Command::new(crate::tool("objdump"))
                .args([
                    "-D",
                    "-z",
                    "-b",
                    "binary",
                    "-m",
                    "m68k:68000",
                    "--insn-width=16",
                ])
                .arg(format!("--adjust-vma={pc}"))
                .arg(&file),
        )
        .context("GNU m68k binutils required: bash scripts/setup-genesis.sh")?;
    let line = text
        .lines()
        .find_map(|line| {
            let mut parts = line.trim().split('\t');
            let addr = parts.next()?.strip_suffix(':')?;
            if u32::from_str_radix(addr, 16).ok()? != pc {
                return None;
            }
            let raw = parts.next()?;
            let asm = parts.next()?;
            Some((raw, asm))
        })
        .context("no complete 68000 instruction decoded")?;
    let length = line
        .0
        .split_whitespace()
        .map(|w| w.len() / 2)
        .sum::<usize>();
    let opcode = u16::from_be_bytes([bytes[0], bytes[1]]);
    let line_trap = matches!(opcode >> 12, 0xa | 0xf);
    ensure!(
        length >= 2
            && length <= bytes.len()
            && (line_trap || !line.1.starts_with(".short"))
            && !line.1.contains("<UNDEFINED>")
            && !line.1.starts_with(".word"),
        "invalid/truncated 68000 instruction"
    );
    let mut text = line.1.trim().to_string();
    // GNU objdump prints address-register displacements in bare hexadecimal,
    // while GNU as interprets bare numbers as decimal and bare a-f as symbols.
    let mut scan = 0;
    while let Some(relative) = text[scan..].find("@(") {
        let start = scan + relative + 2;
        let end = text[start..]
            .find([',', ')'])
            .map(|n| start + n)
            .unwrap_or(text.len());
        let value = &text[start..end];
        let digits = value.strip_prefix('-').unwrap_or(value);
        if !digits.is_empty() && digits.chars().all(|c| c.is_ascii_hexdigit()) {
            let normalized = format!(
                "{}0x{}",
                if value.starts_with('-') { "-" } else { "" },
                digits
            );
            text.replace_range(start..end, &normalized);
            scan = start + normalized.len();
        } else {
            scan = end;
        }
        if scan >= text.len() {
            break;
        }
    }

    let next = (pc + length as u32) & 0xffffff;
    let mut successors = vec![next];
    let mut references = Vec::new();
    let mut terminal = false;
    let branch = opcode & 0xf000 == 0x6000;
    let dbcc = opcode & 0xf0f8 == 0x50c8;
    if branch || dbcc {
        let disp = if dbcc || opcode & 255 == 0 {
            ensure!(bytes.len() >= 4, "truncated displacement");
            i32::from(i16::from_be_bytes([bytes[2], bytes[3]]))
        } else {
            i32::from(opcode as u8 as i8)
        };
        let target = (pc.wrapping_add(2).wrapping_add_signed(disp)) & 0xffffff;
        references.push(json!({"cpu_address":target,"kind":"control","indirect":false}));
        if branch && opcode & 0x0f00 == 0 {
            successors.clear();
            terminal = true
        }
        successors.push(target);
    }
    let jump = opcode & 0xffc0 == 0x4ec0 || opcode & 0xffc0 == 0x4e80;
    if jump {
        let mode = opcode & 63;
        let target = match mode {
            0x38 if length >= 4 => {
                Some(i32::from(i16::from_be_bytes([bytes[2], bytes[3]])) as u32 & 0xffffff)
            }
            0x39 if length >= 6 => {
                Some(u32::from_be_bytes(bytes[2..6].try_into().unwrap()) & 0xffffff)
            }
            0x3a if length >= 4 => Some(
                pc.wrapping_add(2)
                    .wrapping_add_signed(i32::from(i16::from_be_bytes([bytes[2], bytes[3]])))
                    & 0xffffff,
            ),
            _ => None,
        };
        if opcode & 0xffc0 == 0x4ec0 {
            successors.clear();
            terminal = true
        }
        if let Some(t) = target {
            successors.push(t)
        }
        references.push(json!({"cpu_address":target,"kind":"control","indirect":target.is_none()}));
    }
    if line_trap {
        successors.clear();
        terminal = true;
    }
    if matches!(opcode, 0x4e73 | 0x4e75 | 0x4e77 | 0x4e72 | 0x4afc) || opcode & 0xfff0 == 0x4e40 {
        successors.clear();
        terminal = true;
    }
    // GNU prints absolute/PC-relative effective addresses explicitly. Registers and
    // indexed addresses stay unresolved; immediates are values, never implicit pointers.
    if !jump && !branch && !dbcc {
        let operands = text.split_once(' ').map(|x| x.1).unwrap_or("");
        for operand in operands.split(',') {
            let operand = operand.trim();
            let write = operands.ends_with(operand)
                && !["cmp", "btst", "tst", "lea", "pea"]
                    .iter()
                    .any(|m| text.starts_with(m));
            if operand.contains('@') {
                let pc_relative = operand.starts_with("%pc@(");
                let value = if pc_relative {
                    operand
                        .find("0x")
                        .and_then(|i| {
                            let digits = operand[i + 2..]
                                .chars()
                                .take_while(|c| c.is_ascii_hexdigit())
                                .collect::<String>();
                            u32::from_str_radix(&digits, 16).ok()
                        })
                        .map(|v| v & 0xffffff)
                } else {
                    None
                };
                let indexed = pc_relative
                    && operands.find("%pc@(").is_some_and(|start| {
                        let tail = &operands[start + 5..];
                        let end = tail.find(')').unwrap_or(tail.len());
                        tail[..end].contains(',')
                    });
                references.push(json!({"cpu_address":value,"kind":"memory","indirect":!pc_relative,"indexed":indexed,"write":write}));
            }
            if operand.starts_with("0x") && !operand.contains(['%', '(', ')']) {
                let end = operand
                    .find(|c: char| !c.is_ascii_hexdigit() && c != 'x')
                    .unwrap_or(operand.len());
                if let Ok(v) = u32::from_str_radix(&operand[2..end], 16) {
                    references.push(json!({"cpu_address":v&0xffffff,"kind":"memory","indirect":false,"write":write}));
                }
            }
        }
    }
    // Literal branch destinations must be local assembler expressions, never
    // relocations whose value changes with the file's physical placement.
    let assembly =
        if line_trap || branch || dbcc || jump || text.contains('%') && text.contains("pc") {
            None
        } else {
            Some(text.clone())
        };
    let result = Instruction {
        opcode,
        length,
        text,
        assembly,
        after: context,
        successors,
        references,
        terminal,
    };
    let mut cache = CACHE.get().unwrap().lock().unwrap();
    if cache.len() >= 4096 {
        cache.clear();
    }
    cache.insert(key, result.clone());
    Ok(result)
}
