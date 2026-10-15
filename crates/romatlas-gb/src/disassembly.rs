//! External Game Boy backend. Stock source emission; lossless linking skips rgbfix.
use crate::Rom;
use anyhow::{Context, Result, bail};
use romatlas_disasm::Budget;
use romatlas_map::Map;
use std::{path::Path, process::Command};
pub const MGBDIS_REVISION: &str = "95d951a9881a0d7ff1a42a94d1be175da92b1777";
pub fn generate(rom: &Path, map: &Map, out: &Path) -> Result<()> {
    generate_with_timeout(rom, map, out, 120000)
}
pub fn generate_with_timeout(rom: &Path, map: &Map, out: &Path, timeout_ms: u64) -> Result<()> {
    let budget = Budget::new(timeout_ms)?;
    let image = Rom::open(rom)?;
    crate::mapping::import(&image, map).context("map is unsafe for this ROM")?;
    if out.exists() {
        bail!("output directory already exists; choose a new directory");
    }
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let tool = root.join("tools/mgbdis");
    let revision = budget
        .run(
            Command::new("git")
                .arg("-C")
                .arg(&tool)
                .args(["rev-parse", "HEAD"]),
        )
        .context(
            "could not check pinned mgbdis revision (installation, git failure, or command budget)",
        )?;
    if revision.trim() != MGBDIS_REVISION {
        bail!("mgbdis revision mismatch; run scripts/setup.sh");
    }
    let parent = out
        .parent()
        .filter(|p| !p.as_os_str().is_empty())
        .unwrap_or(Path::new("."));
    std::fs::create_dir_all(parent)?;
    static NEXT_STAGE: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
    let stage = parent.join(format!(
        ".romatlas-input-{}-{}",
        std::process::id(),
        NEXT_STAGE.fetch_add(1, std::sync::atomic::Ordering::Relaxed)
    ));
    let generated = stage.join("source");
    std::fs::create_dir(&stage)?;
    let result = (|| {
        let input_dir = stage.join("input");
        std::fs::create_dir(&input_dir)?;
        let input = input_dir.join(rom.file_name().context("ROM path has no filename")?);
        std::fs::write(&input, &image.bytes)?;
        std::fs::write(input.with_extension("sym"), map.export()?)?;
        budget.run(
            Command::new("python3")
                .arg(tool.join("mgbdis.py"))
                .arg(&input)
                .arg("--output-dir")
                .arg(&generated),
        )?;
        let mut symbols: Vec<_> = map
            .entries
            .iter()
            .filter(|e| e.address >= 0x8000 && !e.annotation.contains(':'))
            .collect();
        symbols.sort();
        if !symbols.is_empty() {
            let mut definitions =
                String::from("; RAM/I/O constants from the portable .sym; no ROM content\n");
            for e in symbols {
                anyhow::ensure!(
                    !e.annotation.starts_with('.'),
                    "local RAM label needs a global name: {}",
                    e.annotation
                );
                definitions.push_str(&format!("IF DEF({0})\n    ASSERT {0} == ${1:04x}\nELSE\n    DEF {0} EQU ${1:04x}\nENDC\n", e.annotation, e.address));
            }
            std::fs::write(generated.join("romatlas-symbols.inc"), definitions)?;
            let game = generated.join("game.asm");
            let source = std::fs::read_to_string(&game)?;
            std::fs::write(
                game,
                source.replace(
                    "INCLUDE \"hardware.inc\"",
                    "INCLUDE \"hardware.inc\"\nINCLUDE \"romatlas-symbols.inc\"",
                ),
            )?;
        }
        verify_with_budget(&input, &generated, &budget).context(
            "generated assembly failed byte-fidelity checks; revise the map or use raw data",
        )?;
        std::fs::remove_file(generated.join("game.o"))?;
        std::fs::remove_file(generated.join("romatlas-rebuilt.gb"))?;
        budget.check()?;
        std::fs::rename(&generated, out)?;
        Ok(())
    })();
    std::fs::remove_dir_all(stage)?;
    result
}
pub fn verify(rom: &Path, out: &Path) -> Result<()> {
    verify_with_timeout(rom, out, 120000)
}
pub fn verify_with_timeout(rom: &Path, out: &Path, timeout_ms: u64) -> Result<()> {
    let budget = Budget::new(timeout_ms)?;
    verify_with_budget(rom, out, &budget)
}
fn verify_with_budget(rom: &Path, out: &Path, budget: &Budget) -> Result<()> {
    check_verify_outputs(rom, out)?;
    // Keep the comparison input before any build command can write files.
    let expected = std::fs::read(rom)?;
    for tool in ["rgbasm", "rgblink"] {
        let version = budget
            .run(Command::new(tool).arg("--version"))
            .with_context(|| format!("missing {tool}; run scripts/setup.sh"))?;
        if !version.contains("v0.9.4") {
            bail!("{tool} version must be v0.9.4");
        }
    }
    // Never trust a partial/cached object from an interrupted prior build.
    match std::fs::remove_file(out.join("game.o")) {
        Ok(()) => {}
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {}
        Err(e) => return Err(e.into()),
    }
    // The stock Makefile runs rgbfix, which changes headers. Build assets and object only.
    budget.run(Command::new("make").arg("game.o").current_dir(out))?;
    budget.run(
        Command::new("rgblink")
            .args(["-o", "romatlas-rebuilt.gb", "game.o"])
            .current_dir(out),
    )?;
    let actual = std::fs::read(out.join("romatlas-rebuilt.gb"))?;
    compare(&expected, &actual)
}
fn check_verify_outputs(rom: &Path, out: &Path) -> Result<()> {
    let input = std::fs::canonicalize(rom)?;
    let directory = std::fs::canonicalize(out)?;
    for name in ["game.o", "romatlas-rebuilt.gb"] {
        let output = directory.join(name);
        anyhow::ensure!(
            output != input,
            "verification output would overwrite the input ROM: {name}"
        );
        match std::fs::symlink_metadata(&output) {
            Ok(metadata) => {
                anyhow::ensure!(
                    !metadata.file_type().is_symlink(),
                    "verification output must not be a symlink: {name}"
                );
                #[cfg(unix)]
                {
                    use std::os::unix::fs::MetadataExt;
                    let original = std::fs::metadata(&input)?;
                    anyhow::ensure!(
                        metadata.dev() != original.dev() || metadata.ino() != original.ino(),
                        "verification output aliases the input ROM: {name}"
                    );
                }
            }
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => {}
            Err(e) => return Err(e.into()),
        }
    }
    Ok(())
}
pub fn compare(expected: &[u8], actual: &[u8]) -> Result<()> {
    if let Some(m) = romatlas_disasm::first_mismatch(expected, actual) {
        let (bank, address) = Rom::sym_location(m.offset as usize);
        bail!(
            "first mismatch: offset {:#x}, bank {bank:02x}, address {address:04x}, expected {:?}, actual {:?}; sizes {} / {}",
            m.offset,
            m.expected,
            m.actual,
            m.expected_len,
            m.actual_len
        );
    }
    Ok(())
}

pub struct GameBoyBackend;
impl romatlas_disasm::DisassemblyBackend for GameBoyBackend {
    type Mapping = Map;
    fn generate(&self, rom: &Path, map: &Map, out: &Path, timeout_ms: u64) -> Result<()> {
        generate_with_timeout(rom, map, out, timeout_ms)
    }
    fn verify(&self, rom: &Path, out: &Path, timeout_ms: u64) -> Result<()> {
        verify_with_timeout(rom, out, timeout_ms)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn mismatch_context() {
        let e = compare(&[0; 0x4001], &[0; 0x4000]).unwrap_err().to_string();
        assert!(e.contains("bank 01, address 4000"));
    }
    #[cfg(unix)]
    #[test]
    fn verification_rejects_input_aliases_before_building() {
        let dir =
            std::env::temp_dir().join(format!("romatlas-verify-alias-{}", std::process::id()));
        std::fs::create_dir(&dir).unwrap();
        let rom = dir.join("owned.gb");
        let output = dir.join("romatlas-rebuilt.gb");
        std::fs::write(&rom, b"original ROM").unwrap();
        std::os::unix::fs::symlink(&rom, &output).unwrap();
        assert!(check_verify_outputs(&rom, &dir).is_err());
        std::fs::remove_file(&output).unwrap();
        std::fs::hard_link(&rom, &output).unwrap();
        assert!(check_verify_outputs(&rom, &dir).is_err());
        std::fs::remove_file(&output).unwrap();
        std::fs::rename(&rom, &output).unwrap();
        assert!(check_verify_outputs(&output, &dir).is_err());
        assert_eq!(std::fs::read(&output).unwrap(), b"original ROM");
        std::fs::remove_dir_all(dir).unwrap();
    }
}
