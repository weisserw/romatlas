//! Application platform selection. Shared session/control crates stay neutral.
use crate::{Cmd, MapCmd, StoreCmd};
use anyhow::{Context, Result, ensure};
use romatlas_core::CommandAdapter;
use romatlas_disasm::DisassemblyBackend;
use romatlas_genesis::{
    Rom,
    decode::{Context as DecodeContext, Cpu},
    disassembly::Backend,
    emulator::Emulator,
    mapping::Mapping,
    session::GenesisSession,
};
use serde_json::json;
use std::path::Path;
fn extension(path: &Path) -> bool {
    path.extension().and_then(|s| s.to_str()).is_some_and(|s| {
        ["md", "gen", "smd", "bin"]
            .iter()
            .any(|e| s.eq_ignore_ascii_case(e))
    })
}
fn mapping(path: &Path) -> bool {
    std::fs::File::open(path)
        .ok()
        .and_then(|f| {
            use std::io::Read;
            let mut b = String::new();
            f.take(16 * 1024 * 1024).read_to_string(&mut b).ok()?;
            serde_json::from_str::<serde_json::Value>(&b).ok()
        })
        .is_some_and(|v| {
            v.get("format")
                .and_then(|v| v.as_str())
                .is_some_and(|s| s == "romatlas-genesis-map")
        })
}
pub fn detect(c: &Cmd) -> bool {
    match c {
        Cmd::Inspect { rom }
        | Cmd::Decode { rom, .. }
        | Cmd::Run { rom, .. }
        | Cmd::TestRom { rom, .. }
        | Cmd::Generate { rom, .. }
        | Cmd::Verify { rom, .. } => extension(rom),
        Cmd::Serve { rom, .. } => rom.as_ref().is_some_and(|r| extension(r)),
        Cmd::Map { command } => match command {
            MapCmd::Init { rom, .. } => extension(rom),
            MapCmd::Validate { map, rom } => {
                rom.as_ref().is_some_and(|r| extension(r)) || mapping(map)
            }
            MapCmd::Normalize { map, .. } => mapping(map),
        },
        Cmd::Store { command } => match command {
            StoreCmd::Init { rom, .. }
            | StoreCmd::Import { rom, .. }
            | StoreCmd::Annotate { rom, .. }
            | StoreCmd::Validate { rom, .. }
            | StoreCmd::Export { rom, .. } => extension(rom),
            StoreCmd::Get { store, .. } | StoreCmd::Remove { store, .. } => mapping(store),
        },
    }
}
fn print(v: serde_json::Value) -> Result<()> {
    println!("{}", serde_json::to_string(&v)?);
    Ok(())
}
pub fn run(c: Cmd, platform: Option<&str>) -> Result<()> {
    match c {
        Cmd::Inspect { rom } => print(serde_json::to_value(Rom::open_as(&rom, platform)?.info)?),
        Cmd::Run {
            rom,
            operations,
            timeout_ms,
        } => print(serde_json::to_value(
            Emulator::from_rom(Rom::open_as(&rom, platform)?)?.run(operations, timeout_ms)?,
        )?),
        Cmd::Serve { rom, store } => {
            let mut s = GenesisSession::default();
            s.platform_override = platform.map(str::to_owned);
            if let Some(r) = rom {
                s.open(r)?;
            }
            if let Some(store) = store {
                s.command("map.load", json!({"path":store}))
                    .map_err(|e| anyhow::anyhow!(e.message))?;
            }
            Ok(romatlas_agent::Session::new(s)
                .serve(std::io::stdin().lock(), std::io::stdout().lock())?)
        }
        Cmd::Decode {
            rom,
            offset,
            count,
            cpu,
            cpu_address,
            e: _,
            m: _,
            x: _,
        } => {
            let rom = Rom::open_as(&rom, platform)?;
            let cpu = match cpu.as_str() {
                "main" | "m68k" => Cpu::Main,
                "z80" => Cpu::Z80,
                _ => anyhow::bail!("unknown CPU"),
            };
            let pc = if let Some(pc) = cpu_address {
                pc
            } else {
                rom.canonical_cpu(offset)?
            };
            print(romatlas_genesis::session::decode_range(
                &rom,
                offset,
                count,
                cpu,
                pc,
                DecodeContext::default(),
            )?)
        }
        Cmd::Map { command } => {
            match command {
                MapCmd::Init { rom, output } => {
                    crate::output::distinct_from_input(&rom, &output)?;
                    ensure!(!output.exists(), "map already exists");
                    Mapping::new(&Rom::open_as(&rom, platform)?).save(&output)?;
                }
                MapCmd::Validate { map, rom } => {
                    let m = Mapping::load(&map)?;
                    m.validate(&Rom::open_as(
                        &rom.context("Sega validation requires --rom")?,
                        platform,
                    )?)?;
                }
                MapCmd::Normalize { map, output } => {
                    ensure!(!output.is_symlink(), "output symlink unsupported");
                    let mut m = Mapping::load(&map)?;
                    m.regions.sort_by_key(|r| r.offset);
                    m.save(&output)?;
                }
            }
            print(json!({"ok":true}))
        }
        Cmd::Generate {
            rom,
            map,
            out,
            timeout_ms,
        } => {
            Backend {
                platform: platform.map(str::to_owned),
            }
            .generate(&rom, &Mapping::load(&map)?, &out, timeout_ms)?;
            print(json!({"ok":true}))
        }
        Cmd::Verify {
            rom,
            generated,
            timeout_ms,
        } => {
            Backend {
                platform: platform.map(str::to_owned),
            }
            .verify(&rom, &generated, timeout_ms)?;
            print(json!({"ok":true,"byte_identical":true}))
        }
        Cmd::Store { command } => {
            match command {
                StoreCmd::Init { rom, output } => {
                    crate::output::distinct_from_input(&rom, &output)?;
                    ensure!(!output.exists(), "store already exists");
                    Mapping::new(&Rom::open_as(&rom, platform)?).save(&output)?;
                }
                StoreCmd::Get {
                    store,
                    offset,
                    limit,
                } => {
                    ensure!(limit <= 1000, "limit exceeds 1000");
                    let m = Mapping::load(&store)?;
                    return print(
                        json!({"regions":m.regions.iter().skip(offset).take(limit).collect::<Vec<_>>(),"total":m.regions.len()}),
                    );
                }
                StoreCmd::Validate { rom, store } => {
                    Mapping::load(&store)?.validate(&Rom::open_as(&rom, platform)?)?
                }
                StoreCmd::Export { rom, store, output } => {
                    crate::output::distinct_from_input(&rom, &output)?;
                    let m = Mapping::load(&store)?;
                    m.validate(&Rom::open_as(&rom, platform)?)?;
                    m.save(&output)?;
                }
                StoreCmd::Import { rom, map, output } => {
                    crate::output::distinct_from_input(&rom, &output)?;
                    let m = Mapping::load(&map)?;
                    m.validate(&Rom::open_as(&rom, platform)?)?;
                    ensure!(!output.exists(), "store already exists");
                    m.save(&output)?;
                }
                StoreCmd::Remove { .. } | StoreCmd::Annotate { .. } => anyhow::bail!(
                    "Sega structural edits use map.annotate/map.remove through serve; code needs an explicit execution view"
                ),
            }
            print(json!({"ok":true}))
        }
        Cmd::TestRom { .. } => {
            anyhow::bail!("Game Boy Mooneye protocol does not apply to SMS/Game Gear")
        }
    }
}
