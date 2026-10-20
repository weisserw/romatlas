//! Fixed physical layout and wla-z80/wlalink reconstruction with exact whole-image checks.
use crate::{
    Rom,
    mapping::{Mapping, decode_region},
};
use anyhow::{Context, Result, bail, ensure};
use romatlas_core::RegionKind;
use romatlas_disasm::{Budget, DisassemblyBackend, first_mismatch};
use std::{
    path::{Path, PathBuf},
    process::Command,
    sync::atomic::{AtomicU64, Ordering},
};
static SERIAL: AtomicU64 = AtomicU64::new(0);
fn temporary(parent: &Path, prefix: &str) -> PathBuf {
    parent.join(format!(
        ".{prefix}-{}-{}",
        std::process::id(),
        SERIAL.fetch_add(1, Ordering::Relaxed)
    ))
}
fn tool(name: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../tools/wla-dx/build/binaries")
        .join(name)
}
pub struct Backend;
struct Stage(PathBuf);
impl Drop for Stage {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}
fn assemble(out: &Path, output: &Path, budget: &Budget) -> Result<()> {
    ensure!(
        tool("wla-z80").is_file() && tool("wlalink").is_file(),
        "wla-z80/wlalink not installed; run bash scripts/setup-sms.sh"
    );
    // Always rebuild object and output; stale partial objects cannot verify.
    budget.run(
        Command::new(tool("wla-z80"))
            .current_dir(out)
            .args(["-o", "game.o", "game.asm"]),
    )?;
    budget.run(
        Command::new(tool("wlalink"))
            .current_dir(out)
            .args(["-r", "linkfile"])
            .arg(output),
    )?;
    Ok(())
}
fn compare(image: &Path, result: &Path) -> Result<()> {
    if let Some(m) = first_mismatch(&std::fs::read(image)?, &std::fs::read(result)?) {
        bail!(
            "SMS rebuild mismatch at physical file offset {}: expected {:?}, actual {:?}; lengths {} / {}",
            m.offset,
            m.expected,
            m.actual,
            m.expected_len,
            m.actual_len
        );
    }
    Ok(())
}
impl DisassemblyBackend for Backend {
    type Mapping = Mapping;
    fn generate(&self, image: &Path, map: &Mapping, out: &Path, timeout_ms: u64) -> Result<()> {
        let budget = Budget::new(timeout_ms)?;
        let rom = Rom::open(image)?;
        map.validate(&rom)?;
        ensure!(
            !out.exists() && !out.is_symlink(),
            "output path already exists"
        );
        let parent = out
            .parent()
            .filter(|p| !p.as_os_str().is_empty())
            .unwrap_or(Path::new("."));
        std::fs::create_dir_all(parent)?;
        let stage = Stage(temporary(parent, "sms-generate"));
        std::fs::create_dir(&stage.0)?;
        std::fs::write(stage.0.join("image.sms"), &rom.bytes)?;
        let (source, diagnostics) = render(&rom, map, &budget)?;
        std::fs::write(stage.0.join("game.asm"), source)?;
        std::fs::write(stage.0.join("linkfile"), "[objects]\ngame.o\n")?;
        map.save(&stage.0.join("mapping.json"))?;
        std::fs::write(
            stage.0.join("diagnostics.json"),
            serde_json::to_vec_pretty(&diagnostics)?,
        )?;
        let rebuilt = stage.0.join("rebuilt.sms");
        let output = rebuilt
            .canonicalize()
            .unwrap_or(std::env::current_dir()?.join(&rebuilt));
        assemble(&stage.0, &output, &budget)?;
        compare(image, &rebuilt).context("generated source failed exact reconstruction")?;
        std::fs::remove_file(rebuilt)?;
        budget.check()?;
        std::fs::rename(&stage.0, out)?;
        Ok(())
    }
    fn verify(&self, image: &Path, out: &Path, timeout_ms: u64) -> Result<()> {
        let budget = Budget::new(timeout_ms)?;
        let dir = out.canonicalize()?;
        let stage = Stage(temporary(&dir, "sms-verify"));
        std::fs::create_dir(&stage.0)?;
        let result = stage.0.join("rebuilt.sms");
        assemble(&dir, &result, &budget)?;
        compare(image, &result)
    }
}

/// Render deterministic source from a validated ROM/map without executing tools.
pub fn render(rom: &Rom, map: &Mapping, budget: &Budget) -> Result<(String, Vec<String>)> {
    map.validate(rom)?;
    let prefix = rom.info.rom_offset;
    let mut source = String::from(
        "; ROMAtlas SMS / WLA-DX. Local data only; no automatic checksum repair.\n.memorymap\n defaultslot 0\n slotsize $2000\n",
    );
    for slot in 0..6 {
        source.push_str(&format!(" slot {slot} ${:04x}\n", slot * 8192));
    }
    if prefix > 0 {
        source.push_str(&format!(" slot 6 start 0 size {prefix}\n"));
    }
    source.push_str(&format!(
        ".endme\n.rombankmap\n bankstotal {}\n",
        rom.info.rom_size / 8192 + usize::from(prefix > 0)
    ));
    if prefix > 0 {
        source.push_str(&format!(" banksize {prefix}\n banks 1\n"));
    }
    source.push_str(&format!(
        " banksize 8192\n banks {}\n.endro\n",
        rom.info.rom_size / 8192
    ));
    fn position(source: &mut String, at: usize, pc: Option<u32>, prefix: usize) {
        let (bank, offset, slot) = if at < prefix {
            (0, at, 6)
        } else {
            let data = at - prefix;
            (
                data / 8192 + usize::from(prefix > 0),
                data % 8192,
                pc.map_or(0, |pc| pc as usize / 8192),
            )
        };
        source.push_str(&format!(".bank {bank} slot {slot}\n.org {offset}\n"));
    }
    fn raw(source: &mut String, mut start: usize, end: usize, prefix: usize, pc: Option<u32>) {
        let original = start;
        while start < end {
            let next = if start < prefix {
                prefix
            } else {
                prefix + ((start - prefix) / 8192 + 1) * 8192
            };
            let length = (end - start).min(next - start);
            position(
                source,
                start,
                pc.map(|pc| (pc + (start - original) as u32) & 65535),
                prefix,
            );
            source.push_str(&format!(
                ".incbin \"image.sms\" SKIP {start} READ {length}\n"
            ));
            start += length;
        }
    }
    let mut regions: Vec<_> = map.regions.iter().collect();
    regions.sort_by_key(|e| e.offset);
    let mut cursor = 0;
    let mut diagnostics = Vec::new();
    for entry in regions {
        budget.check()?;
        raw(&mut source, cursor, entry.offset, prefix, None);
        source.push_str(&format!(
            "\n; {:?}; {:?}; physical {}..{}\n",
            entry.kind,
            entry.provenance,
            entry.offset,
            entry.offset + entry.length
        ));
        for line in entry.comment.lines() {
            source.push_str(&format!("; {line}\n"));
        }
        let pc = entry.views.first().map(|v| v.cpu_address);
        position(&mut source, entry.offset, pc, prefix);
        if let Some(label) = &entry.label {
            source.push_str(&format!("{label}:\n"));
        }
        if entry.kind == RegionKind::Code {
            let view = &entry.views[0];
            let decoded = decode_region(rom, entry, view)?;
            let mut pc = view.cpu_address;
            for (at, i) in decoded {
                let contiguous = (at - prefix) % 8192 + i.length <= 8192;
                if contiguous {
                    position(&mut source, at, Some(pc), prefix);
                    if let Some(asm) = i.assembly {
                        source.push_str(&format!("    {asm}\n"));
                    } else {
                        source.push_str(&format!(
                            "    .db {} ; {}\n",
                            rom.bytes[at..at + i.length]
                                .iter()
                                .map(|b| format!("${b:02x}"))
                                .collect::<Vec<_>>()
                                .join(","),
                            i.text
                        ));
                        diagnostics.push(format!("exact encoding at {at}: {}", i.text));
                    }
                } else {
                    source.push_str(&format!(
                        "; {} (instruction crosses physical assembler banks)\n",
                        i.text
                    ));
                    raw(&mut source, at, at + i.length, prefix, Some(pc));
                    diagnostics.push(format!("bank-crossing exact encoding at {at}: {}", i.text));
                }
                pc = (pc + i.length as u32) & 65535;
            }
        } else {
            raw(
                &mut source,
                entry.offset,
                entry.offset + entry.length,
                prefix,
                None,
            );
        }
        cursor = entry.offset + entry.length;
    }
    raw(&mut source, cursor, rom.bytes.len(), prefix, None);
    Ok((source, diagnostics))
}
