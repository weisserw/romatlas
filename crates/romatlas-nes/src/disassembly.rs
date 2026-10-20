//! Fixed physical layout and ca65/ld65 reconstruction with exact whole-image checks.
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
        .join("../../tools/cc65/bin")
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
        tool("ca65").is_file() && tool("ld65").is_file(),
        "ca65/ld65 not installed; run bash scripts/setup-nes.sh"
    );
    // Always rebuild object and output; stale partial objects cannot verify.
    budget.run(
        Command::new(tool("ca65"))
            .current_dir(out)
            .args(["--cpu", "6502", "game.asm", "-o", "game.o"]),
    )?;
    budget.run(
        Command::new(tool("ld65"))
            .current_dir(out)
            .args(["-C", "layout.cfg", "game.o", "-o"])
            .arg(output),
    )?;
    Ok(())
}
fn compare(image: &Path, result: &Path) -> Result<()> {
    if let Some(m) = first_mismatch(&std::fs::read(image)?, &std::fs::read(result)?) {
        bail!(
            "NES rebuild mismatch at physical file offset {}: expected {:?}, actual {:?}; lengths {} / {}",
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
        let stage = Stage(temporary(parent, "nes-generate"));
        std::fs::create_dir(&stage.0)?;
        std::fs::write(stage.0.join("image.nes"), &rom.bytes)?;
        let (source, diagnostics) = render(&rom, map, &budget)?;
        std::fs::write(stage.0.join("game.asm"), source)?;
        std::fs::write(
            stage.0.join("layout.cfg"),
            format!(
                "MEMORY {{ IMAGE: start = 0, size = {}, file = %O; }}\nSEGMENTS {{ IMAGE: load = IMAGE, type = ro; }}\n",
                rom.bytes.len()
            ),
        )?;
        map.save(&stage.0.join("mapping.json"))?;
        std::fs::write(
            stage.0.join("diagnostics.json"),
            serde_json::to_vec_pretty(&diagnostics)?,
        )?;
        let rebuilt = stage.0.join("rebuilt.nes");
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
        let stage = Stage(temporary(&dir, "nes-verify"));
        std::fs::create_dir(&stage.0)?;
        let result = stage.0.join("rebuilt.nes");
        assemble(&dir, &result, &budget)?;
        compare(image, &result)
    }
}

/// Render deterministic source from a validated ROM/map without executing tools.
pub fn render(rom: &Rom, map: &Mapping, budget: &Budget) -> Result<(String, Vec<String>)> {
    map.validate(rom)?;
    let mut source = String::from(
        "; ROMAtlas NES source; original bytes are local-only.\n.setcpu \"6502\"\n.segment \"IMAGE\"\n",
    );
    let mut regions: Vec<_> = map.regions.iter().collect();
    regions.sort_by_key(|e| e.offset);
    let mut cursor = 0;
    let mut diagnostics = Vec::new();
    let raw = |source: &mut String, start: usize, end: usize| {
        if start < end {
            source.push_str(&format!(
                ".incbin \"image.nes\", {start}, {}\n",
                end - start
            ));
        }
    };
    for entry in regions {
        budget.check()?;
        raw(&mut source, cursor, entry.offset);
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

        if entry.kind == RegionKind::Code {
            let view = &entry.views[0];
            let decoded = decode_region(rom, entry, view)?;
            source.push_str(&format!(".org ${:04x}\n", view.cpu_address));
            source.push_str(&format!(
                "{}:\n",
                entry
                    .label
                    .clone()
                    .unwrap_or_else(|| format!("region_{:06x}", entry.offset))
            ));
            for (offset, i) in decoded {
                if let Some(asm) = i.assembly {
                    source.push_str(&format!("    {asm}\n"));
                } else {
                    source.push_str(&format!(
                        "    .byte {} ; {}\n",
                        rom.bytes[offset..offset + i.length]
                            .iter()
                            .map(|b| format!("${b:02x}"))
                            .collect::<Vec<_>>()
                            .join(","),
                        i.text
                    ));
                    diagnostics.push(format!("raw instruction encoding at {offset}: {}", i.text));
                }
            }
            source.push_str(".reloc\n");
        } else {
            if let Some(label) = &entry.label {
                source.push_str(&format!("{label}:\n"));
            }
            raw(&mut source, entry.offset, entry.offset + entry.length);
        }
        cursor = entry.offset + entry.length;
    }
    raw(&mut source, cursor, rom.bytes.len());

    Ok((source, diagnostics))
}
