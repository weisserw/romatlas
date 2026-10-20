//! Fixed physical layout and GNU 68000 binutils reconstruction with exact whole-image checks.
use crate::{
    Rom,
    decode::Cpu,
    mapping::{Mapping, decode_region_with_budget},
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
#[derive(Default)]
pub struct Backend {
    pub platform: Option<String>,
}
struct Stage(PathBuf);
impl Drop for Stage {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}
fn assemble(out: &Path, output: &Path, budget: &Budget) -> Result<()> {
    let jobs: Vec<(String, u32, usize)> =
        serde_json::from_slice(&std::fs::read(out.join("z80-build.json"))?)?;
    let tools = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../tools/wla-dx/build/binaries");
    for (name, _, _) in jobs {
        ensure!(
            name.starts_with("z80-") && name[4..].bytes().all(|b| b.is_ascii_digit()),
            "invalid Z80 source name"
        );
        budget.run(
            Command::new(tools.join("wla-z80"))
                .current_dir(out)
                .arg("-o")
                .arg(format!("{name}.o"))
                .arg(format!("{name}.asm")),
        )?;
        budget.run(
            Command::new(tools.join("wlalink"))
                .current_dir(out)
                .arg("-r")
                .arg(format!("{name}.link"))
                .arg(format!("{name}.bin")),
        )?;
    }
    budget.run(Command::new(crate::tool("as")).current_dir(out).args([
        "-m68000",
        "--register-prefix-optional",
        "game.asm",
        "-o",
        "game.o",
    ]))?;
    budget.run(Command::new(crate::tool("ld")).current_dir(out).args([
        "--no-check-sections",
        "-T",
        "layout.ld",
        "game.o",
        "-o",
        "game.elf",
    ]))?;
    budget.run(
        Command::new(crate::tool("objcopy"))
            .current_dir(out)
            .args(["-O", "binary", "game.elf"])
            .arg(output),
    )?;
    let (layout, prefix): (String, usize) =
        serde_json::from_slice(&std::fs::read(out.join("encoding.json"))?)?;
    let mut bytes = std::fs::read(output)?;
    ensure!(prefix <= bytes.len(), "encoding prefix outside image");
    let payload = &mut bytes[prefix..];
    match layout.as_str() {
        "word_swapped" => {
            ensure!(payload.len() % 2 == 0, "word-swap alignment");
            for pair in payload.chunks_exact_mut(2) {
                pair.swap(0, 1);
            }
        }
        "smd_interleaved" => {
            ensure!(payload.len() % 16384 == 0, "SMD block alignment");
            let canonical = payload.to_vec();
            for (src, dst) in canonical
                .chunks_exact(16384)
                .zip(payload.chunks_exact_mut(16384))
            {
                for i in 0..8192 {
                    dst[i] = src[2 * i + 1];
                    dst[8192 + i] = src[2 * i];
                }
            }
        }
        "big_endian" => {}
        _ => anyhow::bail!("invalid source encoding"),
    }
    std::fs::write(output, bytes)?;
    Ok(())
}
fn compare(image: &Path, result: &Path) -> Result<()> {
    if let Some(m) = first_mismatch(&std::fs::read(image)?, &std::fs::read(result)?) {
        bail!(
            "Genesis rebuild mismatch at physical file offset {}: expected {:?}, actual {:?}; lengths {} / {}",
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
        map.validate_with_budget(&rom, &budget)?;
        ensure!(
            !out.exists() && !out.is_symlink(),
            "output path already exists"
        );
        let parent = out
            .parent()
            .filter(|p| !p.as_os_str().is_empty())
            .unwrap_or(Path::new("."));
        std::fs::create_dir_all(parent)?;
        let stage = Stage(temporary(parent, "genesis-generate"));
        std::fs::create_dir(&stage.0)?;
        std::fs::write(stage.0.join("image.bin"), &rom.bytes)?;
        std::fs::write(
            stage.0.join("encoding.json"),
            serde_json::to_vec_pretty(&(rom.info.layout.clone(), rom.info.rom_offset))?,
        )?;
        let mut z80_jobs = Vec::new();
        for entry in &map.regions {
            if entry.kind != RegionKind::Code || entry.views[0].cpu != Cpu::Z80 {
                continue;
            }
            let view = &entry.views[0];
            let name = format!("z80-{}", entry.id);
            let mut source = String::from(
                "; ROMAtlas Z80 sound CPU; local original bytes.\n.memorymap\n defaultslot 0\n slotsize 65536\n slot 0 $0000\n.endme\n.rombankmap\n bankstotal 1\n banksize 65536\n banks 1\n.endro\n.bank 0 slot 0\n",
            );
            source.push_str(&format!(".org {}\n", view.cpu_address));
            for line in entry.comment.lines() {
                source.push_str(&format!("; {line}\n"));
            }
            source.push_str(&format!(
                "{}:\n",
                entry
                    .label
                    .clone()
                    .unwrap_or_else(|| format!("Sound_{}", entry.id))
            ));
            for (at, i) in decode_region_with_budget(&rom, entry, view, &budget)? {
                if let Some(asm) = i.assembly {
                    source.push_str(&format!(" {asm}\n"));
                } else {
                    source.push_str(&format!(
                        " .db {} ; {}\n",
                        rom.bytes[at..at + i.length]
                            .iter()
                            .map(|b| format!("${b:02x}"))
                            .collect::<Vec<_>>()
                            .join(","),
                        i.text
                    ));
                }
            }
            std::fs::write(stage.0.join(format!("{name}.asm")), source)?;
            std::fs::write(
                stage.0.join(format!("{name}.link")),
                format!("[objects]\n{name}.o\n"),
            )?;
            z80_jobs.push((name, view.cpu_address, entry.length));
        }
        std::fs::write(
            stage.0.join("z80-build.json"),
            serde_json::to_vec_pretty(&z80_jobs)?,
        )?;
        let (source, diagnostics) = render(&rom, map, &budget)?;
        std::fs::write(stage.0.join("game.asm"), source)?;
        let mut layout = String::from("SECTIONS {\n");
        let mut regions: Vec<_> = map.regions.iter().collect();
        regions.sort_by_key(|r| r.offset);
        let mut cursor = 0;
        let mut section = 0;
        for region in &regions {
            if cursor < region.offset {
                layout.push_str(&format!(
                    ".r{section} {cursor} : AT({cursor}) {{ *(.r{section}) }}\n"
                ));
                section += 1;
            }
            let vma = region
                .views
                .first()
                .map_or(region.offset as u32, |v| v.cpu_address);
            layout.push_str(&format!(
                ".r{section} {vma} : AT({}) {{ *(.r{section}) }}\n",
                region.offset
            ));
            section += 1;
            cursor = region.offset + region.length;
        }
        if cursor < rom.bytes.len() {
            layout.push_str(&format!(
                ".r{section} {cursor} : AT({cursor}) {{ *(.r{section}) }}\n"
            ));
        }
        layout.push_str("/DISCARD/ : { *(.text) *(.data) *(.bss) }\n}\n");
        std::fs::write(stage.0.join("layout.ld"), layout)?;
        map.save(&stage.0.join("mapping.json"))?;
        std::fs::write(
            stage.0.join("diagnostics.json"),
            serde_json::to_vec_pretty(&diagnostics)?,
        )?;
        let rebuilt = stage.0.join("rebuilt.bin");
        let output = rebuilt
            .canonicalize()
            .unwrap_or(std::env::current_dir()?.join(&rebuilt));
        assemble(&stage.0, &output, &budget)?;
        if std::fs::read(&rebuilt)? != rom.original() {
            // GAS can canonicalize ignored extension bits or absolute widths.
            // Retain affected instructions as exact encodings with decoded comments.
            let candidate = Rom::from_bytes(std::fs::read(&rebuilt)?).ok();
            let mut raw = std::collections::BTreeSet::new();
            for entry in &map.regions {
                if entry.kind == RegionKind::Code && entry.views[0].cpu == Cpu::Main {
                    for (offset, i) in
                        decode_region_with_budget(&rom, entry, &entry.views[0], &budget)?
                    {
                        if candidate.as_ref().is_none_or(|r| {
                            r.bytes.get(offset..offset + i.length)
                                != Some(&rom.bytes[offset..offset + i.length])
                        }) {
                            raw.insert(offset as u64);
                        }
                    }
                }
            }
            if !raw.is_empty() {
                let (source, mut diagnostics) = render_with_raw(&rom, map, &budget, &raw)?;
                diagnostics.push(
                    "Assembler normalized an encoding; affected instructions retain exact bytes"
                        .into(),
                );
                std::fs::write(stage.0.join("game.asm"), source)?;
                std::fs::write(
                    stage.0.join("diagnostics.json"),
                    serde_json::to_vec_pretty(&diagnostics)?,
                )?;
                assemble(&stage.0, &output, &budget)?;
            }
        }
        compare(image, &rebuilt).context("generated source failed exact reconstruction")?;
        std::fs::remove_file(rebuilt)?;
        budget.check()?;
        std::fs::rename(&stage.0, out)?;
        Ok(())
    }
    fn verify(&self, image: &Path, out: &Path, timeout_ms: u64) -> Result<()> {
        let budget = Budget::new(timeout_ms)?;
        let dir = out.canonicalize()?;
        let stage = Stage(temporary(&dir, "genesis-verify"));
        std::fs::create_dir(&stage.0)?;
        let result = stage.0.join("rebuilt.bin");
        assemble(&dir, &result, &budget)?;
        compare(image, &result)
    }
}

/// Render deterministic source from a validated ROM/map without executing tools.
pub fn render(rom: &Rom, map: &Mapping, budget: &Budget) -> Result<(String, Vec<String>)> {
    render_with_raw(rom, map, budget, &Default::default())
}
fn render_with_raw(
    rom: &Rom,
    map: &Mapping,
    budget: &Budget,
    raw_encodings: &std::collections::BTreeSet<u64>,
) -> Result<(String, Vec<String>)> {
    map.validate_with_budget(rom, budget)?;
    let mut source = String::from("| ROMAtlas Genesis source; original bytes stay local.\n");
    let mut regions: Vec<_> = map.regions.iter().collect();
    regions.sort_by_key(|e| e.offset);
    let mut cursor = 0;
    let mut diagnostics = Vec::new();
    let mut section = 0;
    let raw = |source: &mut String, start: usize, end: usize, section: &mut usize| {
        if start < end {
            source.push_str(&format!(".section .r{},\"a\"\n", *section));
            *section += 1;
            source.push_str(&format!(
                ".incbin \"image.bin\", {start}, {}\n",
                end - start
            ));
        }
    };
    for entry in regions {
        budget.check()?;
        raw(&mut source, cursor, entry.offset, &mut section);
        source.push_str(&format!(".section .r{},\"a\"\n", section));
        section += 1;
        source.push_str(&format!(
            "\n| {:?}; {:?}; physical {}..{}\n",
            entry.kind,
            entry.provenance,
            entry.offset,
            entry.offset + entry.length
        ));
        for line in entry.comment.lines() {
            source.push_str(&format!("| {line}\n"));
        }

        if entry.kind == RegionKind::Code {
            let view = &entry.views[0];
            if view.cpu == Cpu::Z80 {
                source.push_str(&format!(
                    ".incbin \"z80-{}.bin\", {}, {}\n",
                    entry.id, view.cpu_address, entry.length
                ));
                cursor = entry.offset + entry.length;
                continue;
            }
            let decoded = decode_region_with_budget(rom, entry, view, budget)?;

            source.push_str(&format!(
                "{}:\n",
                entry
                    .label
                    .clone()
                    .unwrap_or_else(|| format!("region_{:06x}", entry.offset))
            ));
            for (offset, i) in decoded {
                if let Some(asm) = i
                    .assembly
                    .filter(|_| !raw_encodings.contains(&(offset as u64)))
                {
                    source.push_str(&format!("    {asm}\n"));
                } else {
                    source.push_str(&format!(
                        "    .byte {} | {}\n",
                        rom.bytes[offset..offset + i.length]
                            .iter()
                            .map(|b| format!("0x{b:02x}"))
                            .collect::<Vec<_>>()
                            .join(","),
                        i.text
                    ));
                    diagnostics.push(format!("raw instruction encoding at {offset}: {}", i.text));
                }
            }
        } else {
            if let Some(label) = &entry.label {
                source.push_str(&format!("{label}:\n"));
            }
            source.push_str(&format!(
                ".incbin \"image.bin\", {}, {}\n",
                entry.offset, entry.length
            ));
        }
        cursor = entry.offset + entry.length;
    }
    raw(&mut source, cursor, rom.bytes.len(), &mut section);

    Ok((source, diagnostics))
}
