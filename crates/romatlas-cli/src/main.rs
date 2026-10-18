mod output;
mod session;
mod snes;
use anyhow::Result;
use clap::{Parser, Subcommand};
use romatlas_gb::Rom;
use romatlas_map::Map;
use std::path::PathBuf;
#[derive(Parser)]
#[command(
    version,
    about = "ROM-free structural maps, instrumented debugging and exact reconstruction"
)]
struct Cli {
    #[command(subcommand)]
    command: Cmd,
    #[arg(long, global = true, default_value = "auto")]
    platform: String,
}
#[derive(Subcommand)]
enum Cmd {
    TestRom {
        rom: PathBuf,
        #[arg(long, default_value = "auto")]
        model: String,
        #[arg(long, default_value_t = 10000000)]
        operations: u64,
        #[arg(long, default_value_t = 10000)]
        timeout_ms: u64,
    },
    Serve {
        #[arg(long)]
        rom: Option<PathBuf>,
        #[arg(long)]
        store: Option<PathBuf>,
    },
    Run {
        rom: PathBuf,
        #[arg(long, default_value_t = 1000)]
        operations: u64,
        #[arg(long, default_value_t = 1000)]
        timeout_ms: u64,
    },
    Inspect {
        rom: PathBuf,
    },
    Decode {
        rom: PathBuf,
        #[arg(long, default_value_t = 0)]
        offset: usize,
        #[arg(long, default_value_t = 16)]
        count: usize,
        #[arg(long, default_value = "main")]
        cpu: String,
        #[arg(long)]
        cpu_address: Option<u32>,
        #[arg(long)]
        e: Option<bool>,
        #[arg(long)]
        m: Option<bool>,
        #[arg(long)]
        x: Option<bool>,
    },
    Store {
        #[command(subcommand)]
        command: StoreCmd,
    },
    Map {
        #[command(subcommand)]
        command: MapCmd,
    },
    Generate {
        rom: PathBuf,
        #[arg(long)]
        map: PathBuf,
        #[arg(long)]
        out: PathBuf,
        #[arg(long, default_value_t = 120000)]
        timeout_ms: u64,
    },
    Verify {
        rom: PathBuf,
        #[arg(long)]
        generated: PathBuf,
        #[arg(long, default_value_t = 120000)]
        timeout_ms: u64,
    },
}
#[derive(Subcommand)]
enum MapCmd {
    Init {
        rom: PathBuf,
        #[arg(long)]
        output: PathBuf,
    },
    Validate {
        map: PathBuf,
        #[arg(long)]
        rom: Option<PathBuf>,
    },
    Normalize {
        map: PathBuf,
        #[arg(long)]
        output: PathBuf,
    },
}
#[derive(Subcommand)]
enum StoreCmd {
    Import {
        rom: PathBuf,
        #[arg(long)]
        map: PathBuf,
        #[arg(long)]
        output: PathBuf,
    },
    Init {
        rom: PathBuf,
        #[arg(long)]
        output: PathBuf,
    },
    Get {
        store: PathBuf,
        #[arg(long, default_value_t = 0)]
        offset: usize,
        #[arg(long, default_value_t = 100)]
        limit: usize,
    },
    Annotate {
        rom: PathBuf,
        #[arg(long)]
        store: PathBuf,
        #[arg(long)]
        offset: u64,
        #[arg(long)]
        length: u64,
        #[arg(long)]
        kind: String,
        #[arg(long, default_value = "manual")]
        provenance: String,
        #[arg(long, default_value_t = 1.0)]
        confidence: f32,
        #[arg(long)]
        id: Option<u64>,
    },
    Remove {
        store: PathBuf,
        #[arg(long)]
        id: u64,
    },
    Validate {
        rom: PathBuf,
        #[arg(long)]
        store: PathBuf,
    },
    Export {
        rom: PathBuf,
        #[arg(long)]
        store: PathBuf,
        #[arg(long)]
        output: PathBuf,
    },
}
fn store_command(command: StoreCmd) -> Result<()> {
    use romatlas_core::{ImageAdapter, Provenance, Region, RegionKind};
    use romatlas_map::store::Store;
    let value = match command {
        StoreCmd::Import { rom, map, output } => {
            anyhow::ensure!(!output.exists(), "store already exists");
            let rom = Rom::open(&rom)?;
            romatlas_gb::mapping::import(&rom, &read_map(&map)?)?.save(&output)?;
            serde_json::json!({"ok":true})
        }
        StoreCmd::Init { rom, output } => {
            anyhow::ensure!(!output.exists(), "store already exists");
            let rom = Rom::open(&rom)?;
            Store::new("gb", &rom.info.sha256, rom.info.size as u64)?.save(&output)?;
            serde_json::json!({"ok":true})
        }
        StoreCmd::Get {
            store,
            offset,
            limit,
        } => {
            anyhow::ensure!(limit <= 1000, "limit exceeds 1000");
            let s = Store::load(&store)?;
            let regions: Vec<_> = s
                .regions
                .iter()
                .skip(offset)
                .take(limit)
                .map(|(id, r)| serde_json::json!({"id":id,"region":r}))
                .collect();
            serde_json::json!({"regions":regions,"total":s.regions.len(),"next_offset":offset.saturating_add(limit).min(s.regions.len())})
        }
        StoreCmd::Annotate {
            rom,
            store,
            offset,
            length,
            kind,
            provenance,
            confidence,
            id,
        } => {
            let rom = Rom::open(&rom)?;
            let mut s = Store::load(&store)?;
            romatlas_gb::mapping::validate_store(&rom, &s)?;
            let kind = match kind.as_str() {
                "code" => RegionKind::Code,
                "data" => RegionKind::Data,
                "text" => RegionKind::Text,
                "padding" => RegionKind::Padding,
                "image" => RegionKind::Image,
                "unknown" => RegionKind::Unknown,
                _ => anyhow::bail!("unknown region kind"),
            };
            let provenance = match provenance.as_str() {
                "manual" => Provenance::Manual,
                "static" => Provenance::Static,
                "observed" => Provenance::Observed,
                _ => anyhow::bail!("unknown provenance"),
            };
            let region = Region {
                start: rom
                    .physical_address(offset)
                    .ok_or_else(|| anyhow::anyhow!("offset outside ROM"))?,
                length,
                kind,
                provenance,
                confidence,
            };
            romatlas_gb::mapping::validate_region(&rom, &region)?;
            let id = if let Some(id) = id {
                s.edit(id, region)?;
                id
            } else {
                s.annotate(region)?
            };
            s.save(&store)?;
            serde_json::json!({"ok":true,"id":id})
        }
        StoreCmd::Remove { store, id } => {
            let mut s = Store::load(&store)?;
            s.remove(id)?;
            s.save(&store)?;
            serde_json::json!({"ok":true})
        }
        StoreCmd::Validate { rom, store } => {
            romatlas_gb::mapping::validate_store(&Rom::open(&rom)?, &Store::load(&store)?)?;
            serde_json::json!({"ok":true})
        }
        StoreCmd::Export { rom, store, output } => {
            crate::output::distinct_from_input(&rom, &output)?;
            let m = romatlas_gb::mapping::export(&Rom::open(&rom)?, &Store::load(&store)?)?;
            std::fs::write(output, m.export()?)?;
            serde_json::json!({"ok":true})
        }
    };
    println!("{value}");
    Ok(())
}
fn read_map(path: &std::path::Path) -> Result<Map> {
    Map::parse(&String::from_utf8(romatlas_map::read_limited(
        path,
        1024 * 1024,
    )?)?)
}
fn main() {
    if let Err(e) = run() {
        eprintln!(
            "{}",
            serde_json::json!({"ok":false,"error":format!("{e:#}")})
        );
        std::process::exit(1);
    }
}
fn run() -> Result<()> {
    let cli = Cli::parse();
    anyhow::ensure!(
        matches!(cli.platform.as_str(), "auto" | "gb" | "snes"),
        "unknown platform"
    );
    if cli.platform == "snes" || cli.platform == "auto" && snes::detect(&cli.command) {
        return snes::run(cli.command);
    }
    match cli.command {
        Cmd::TestRom {
            rom,
            model,
            operations,
            timeout_ms,
        } => {
            let mut e = romatlas_gb::emulator::Emulator::open_model(&rom, &model)?;
            println!("{}", e.mooneye(operations, timeout_ms)?);
        }
        Cmd::Serve { rom, store } => {
            use romatlas_core::CommandAdapter;
            let mut adapter = session::GbSession::default();
            if let Some(rom) = rom {
                adapter.open(rom)?;
            }
            if let Some(store) = store {
                adapter
                    .command("map.load", serde_json::json!({"path":store}))
                    .map_err(|e| anyhow::anyhow!(e.message))?;
            }
            romatlas_agent::Session::new(adapter)
                .serve(std::io::stdin().lock(), std::io::stdout().lock())?;
        }
        Cmd::Run {
            rom,
            operations,
            timeout_ms,
        } => {
            let mut emulator = romatlas_gb::emulator::Emulator::open(&rom)?;
            println!(
                "{}",
                serde_json::to_string(&emulator.run(operations, timeout_ms)?)?
            );
        }
        Cmd::Store { command } => store_command(command)?,
        Cmd::Inspect { rom } => {
            println!("{}", serde_json::to_string_pretty(&Rom::open(&rom)?.info)?)
        }
        Cmd::Decode {
            rom, offset, count, ..
        } => {
            anyhow::ensure!(count <= 4096, "decode count exceeds 4096");
            let image = Rom::open(&rom)?;
            anyhow::ensure!(offset < image.bytes.len(), "offset outside ROM");
            let mut cursor = offset;
            let mut result = Vec::new();
            for _ in 0..count {
                if cursor >= image.bytes.len() {
                    break;
                }
                let (bank, pc) = Rom::sym_location(cursor);
                let end = ((cursor / 0x4000 + 1) * 0x4000).min(image.bytes.len());
                let instruction =
                    romatlas_gb::decode::decode(&image.bytes[cursor..end], pc as u16)?;
                let length = instruction.length as usize;
                result.push(
                    serde_json::json!({"offset":cursor,"bank":bank,"instruction":instruction}),
                );
                cursor += length;
            }
            println!(
                "{}",
                serde_json::json!({"instructions":result,"next_offset":cursor})
            );
        }
        Cmd::Map { command } => match command {
            MapCmd::Init { rom, output } => {
                crate::output::distinct_from_input(&rom, &output)?;
                let image = Rom::open(&rom)?;
                std::fs::write(output, Map::raw(image.info.banks).export()?)?;
                println!("{{\"ok\":true}}");
            }
            MapCmd::Validate { map, rom } => {
                let map = read_map(&map)?;
                if let Some(path) = rom {
                    romatlas_gb::mapping::import(&Rom::open(&path)?, &map)?;
                } else {
                    map.validate(None)?;
                }
                println!("{{\"ok\":true}}");
            }
            MapCmd::Normalize { map, output } => {
                std::fs::write(output, read_map(&map)?.export()?)?;
                println!("{{\"ok\":true}}");
            }
        },
        Cmd::Generate {
            rom,
            map,
            out,
            timeout_ms,
        } => {
            romatlas_gb::disassembly::generate_with_timeout(
                &rom,
                &read_map(&map)?,
                &out,
                timeout_ms,
            )?;
            println!("{{\"ok\":true}}");
        }
        Cmd::Verify {
            rom,
            generated,
            timeout_ms,
        } => {
            romatlas_gb::disassembly::verify_with_timeout(&rom, &generated, timeout_ms)?;
            println!("{{\"ok\":true,\"byte_identical\":true}}");
        }
    }
    Ok(())
}
