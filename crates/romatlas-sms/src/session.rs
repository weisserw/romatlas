//! SMS CommandAdapter for the shared persistent JSON-RPC transport.
use crate::{
    Rom,
    decode::{self, Context, Cpu},
    disassembly::Backend,
    emulator::{Emulator, Snapshot},
    image::address,
    mapping::{Entry, Mapping},
};
use anyhow::{Context as _, Result, ensure};
use romatlas_core::{
    ApiError, CommandAdapter,
    execution::{ExecutionAdapter, StepRequest, StepUnit},
};
use romatlas_disasm::DisassemblyBackend;
use serde_json::{Value, json};
use std::{
    collections::BTreeMap,
    io::Write,
    path::{Path, PathBuf},
};
#[derive(Default)]
pub struct SmsSession {
    pub platform_override: Option<String>,
    path: Option<PathBuf>,
    rom: Option<Rom>,
    map: Option<Mapping>,
    emulator: Option<Emulator>,
    snapshots: BTreeMap<String, Snapshot>,
}
fn num(p: &Value, k: &str, d: u64) -> Result<u64> {
    p.get(k)
        .map(|v| v.as_u64().with_context(|| format!("{k} must be unsigned")))
        .unwrap_or(Ok(d))
}
fn string<'a>(p: &'a Value, k: &str) -> Result<&'a str> {
    p[k].as_str().with_context(|| format!("missing string {k}"))
}
fn path(p: &Value) -> Result<PathBuf> {
    Ok(PathBuf::from(string(p, "path")?))
}
fn cpu(p: &Value) -> Result<Cpu> {
    match p.get("cpu").and_then(Value::as_str).unwrap_or("main") {
        "main" | "Z80" | "z80" => Ok(Cpu::Main),
        _ => anyhow::bail!("unknown CPU"),
    }
}
fn page<T: serde::Serialize, I: IntoIterator<Item = T>>(items: I, p: &Value) -> Result<Value>
where
    I::IntoIter: ExactSizeIterator,
{
    let offset = num(p, "offset", 0)? as usize;
    let limit = num(p, "limit", 100)? as usize;
    ensure!(limit <= 1000, "limit exceeds 1000");
    let iter = items.into_iter();
    let total = iter.len();
    let values: Vec<_> = iter.skip(offset).take(limit).collect();
    Ok(json!({"items":values,"total":total,"next_offset":offset.saturating_add(limit).min(total)}))
}
impl CommandAdapter for SmsSession {
    fn platform(&self) -> &str {
        self.rom
            .as_ref()
            .map(|r| r.info.platform.as_str())
            .or(self.platform_override.as_deref())
            .unwrap_or("sms")
    }
    fn methods(&self) -> Vec<String> {
        [
            "api.describe",
            "rom.open",
            "rom.info",
            "rom.read",
            "rom.regions",
            "emulator.start",
            "emulator.capabilities",
            "emulator.state",
            "emulator.step",
            "emulator.run",
            "emulator.reset",
            "emulator.input",
            "emulator.memory.write",
            "emulator.registers.set",
            "emulator.memory",
            "emulator.translate",
            "emulator.physical_memory",
            "emulator.snapshot.save",
            "emulator.snapshot.load",
            "emulator.snapshot.export",
            "emulator.snapshot.import",
            "emulator.audio.clear",
            "emulator.audio.export",
            "emulator.framebuffer",
            "trace.configure",
            "trace.summary",
            "trace.query",
            "trace.clear",
            "trace.stream.start",
            "trace.stream.stop",
            "analysis.coverage",
            "analysis.contexts",
            "analysis.edges",
            "analysis.decode",
            "analysis.discover",
            "analysis.discover_context",
            "analysis.references",
            "analysis.pointers",
            "analysis.unknown",
            "map.get",
            "map.annotate",
            "map.remove",
            "map.validate",
            "map.save",
            "map.load",
            "map.export",
            "map.import",
            "disasm.generate",
            "disasm.verify",
        ]
        .into_iter()
        .map(str::to_owned)
        .collect()
    }
    fn command(&mut self, m: &str, p: Value) -> Result<Value, ApiError> {
        if !self.methods().iter().any(|s| s == m) {
            return Err(ApiError::unsupported(m));
        }
        self.dispatch(m, &p).map_err(|e| ApiError {
            code: -32000,
            message: format!("{e:#}"),
        })
    }
}
impl SmsSession {
    fn rom(&self) -> Result<&Rom> {
        self.rom.as_ref().context("open a ROM first")
    }
    fn map(&self) -> Result<&Mapping> {
        self.map.as_ref().context("open a ROM first")
    }
    fn emu(&mut self) -> Result<&mut Emulator> {
        self.emulator.as_mut().context("start emulator first")
    }
    fn identity(&self) -> Result<()> {
        ensure!(
            Rom::open(self.path.as_ref().context("open ROM first")?)?
                .info
                .sha256
                == self.rom()?.info.sha256,
            "ROM changed since open; reopen it"
        );
        Ok(())
    }
    fn distinct(&self, path: &Path) -> Result<()> {
        if path.exists() {
            ensure!(
                path.canonicalize()?
                    != self
                        .path
                        .as_ref()
                        .context("open ROM first")?
                        .canonicalize()?,
                "output aliases input ROM"
            );
        }
        #[cfg(unix)]
        if path.exists() {
            use std::os::unix::fs::MetadataExt;
            let original = std::fs::metadata(self.path.as_ref().context("open ROM first")?)?;
            let output = std::fs::metadata(path)?;
            ensure!(
                original.dev() != output.dev() || original.ino() != output.ino(),
                "output aliases input ROM"
            );
        }
        ensure!(!path.is_symlink(), "output symlink unsupported");
        Ok(())
    }
    fn save_new(&self, path: &Path, v: &Value) -> Result<()> {
        self.distinct(path)?;
        let data = serde_json::to_vec_pretty(v)?;
        std::fs::File::create_new(path)?.write_all(&data)?;
        Ok(())
    }
    pub fn open(&mut self, path: PathBuf) -> Result<Value> {
        let rom = Rom::open_as(&path, self.platform_override.as_deref())?;
        let info = serde_json::to_value(&rom.info)?;
        let map = Mapping::new(&rom);
        self.path = Some(path);
        self.rom = Some(rom);
        self.map = Some(map);
        self.emulator = None;
        self.snapshots.clear();
        Ok(info)
    }
    fn dispatch(&mut self, m: &str, p: &Value) -> Result<Value> {
        Ok(match m {
            "api.describe" => json!({"platform":self.platform(),"methods":self.methods()}),
            "rom.open" => self.open(path(p)?)?,
            "rom.info" => serde_json::to_value(&self.rom()?.info)?,
            "rom.read" => {
                let offset = num(p, "offset", 0)? as usize;
                let length = num(p, "length", 16)? as usize;
                ensure!(length <= 4096, "read length exceeds 4096");
                json!({"offset":offset,"bytes":self.rom()?.check_range(offset,length)?})
            }
            "rom.regions" | "analysis.unknown" => page(
                self.map()?
                    .unknown()
                    .into_iter()
                    .map(|(offset, length)| json!({"offset":offset,"length":length})),
                p,
            )?,
            "emulator.start" => {
                self.identity()?;
                let emu = Emulator::from_rom(self.rom()?.clone())?;
                let value = serde_json::to_value(emu.state())?;
                self.emulator = Some(emu);
                self.snapshots.clear();
                value
            }
            "emulator.capabilities" => serde_json::to_value(self.emu()?.capabilities())?,
            "emulator.state" => serde_json::to_value(self.emu()?.state())?,
            "emulator.reset" => {
                self.emu()?
                    .reset()
                    .map_err(|e| anyhow::anyhow!(e.message))?;
                self.snapshots.clear();
                serde_json::to_value(self.emu()?.state())?
            }
            "emulator.step" | "emulator.run" => {
                let cpu = cpu(p)?;
                let unit = match p.get("unit").and_then(Value::as_str).unwrap_or("operation") {
                    "operation" => StepUnit::Operation,
                    "cycle" => StepUnit::Cycle,
                    "frame" => StepUnit::Frame,
                    _ => anyhow::bail!("unknown step unit"),
                };
                let count = num(p, "count", num(p, "operations", 1)?)?;
                let request = StepRequest {
                    unit,
                    count,
                    max_operations: num(p, "max_operations", 10_000_000)?,
                    timeout_ms: num(p, "timeout_ms", 1000)?,
                };
                self.emu()?.selected_cpu = cpu;
                serde_json::to_value(
                    self.emu()?
                        .execute(request)
                        .map_err(|e| anyhow::anyhow!(e.message))?,
                )?
            }
            "emulator.input" => {
                let buttons: Vec<String> =
                    serde_json::from_value(p.get("buttons").cloned().unwrap_or(json!([])))?;
                self.emu()?
                    .input_port(u8::try_from(num(p, "port", 1)?)?, &buttons)?;
                json!({"ok":true})
            }
            "emulator.registers.set" => {
                self.emu()?.set_registers(
                    cpu(p)?,
                    p.get("registers").context("missing registers object")?,
                )?;
                serde_json::to_value(self.emu()?.state())?
            }
            "emulator.memory.write" => {
                let a = address(
                    p.get("space").and_then(Value::as_str).unwrap_or("cpu"),
                    num(p, "address", 0)?,
                );
                let bytes: Vec<u8> = serde_json::from_value(p["bytes"].clone())?;
                self.emu()?.write_memory(&a, &bytes)?;
                json!({"ok":true,"artificial":true})
            }
            "emulator.translate" => {
                let a = address("cpu", num(p, "address", 0)?);
                json!({"address":a,"physical":self.emu()?.translate(&a).map_err(|e|anyhow::anyhow!(e.message))?})
            }
            "emulator.memory" | "emulator.physical_memory" => {
                let space = p
                    .get("space")
                    .and_then(Value::as_str)
                    .unwrap_or(if m == "emulator.memory" { "cpu" } else { "rom" });
                if m == "emulator.physical_memory" {
                    ensure!(
                        !matches!(space, "cpu" | "io"),
                        "physical_memory requires backing-storage space"
                    );
                }
                let a = address(space, num(p, "address", num(p, "offset", 0)?)?);
                let length = num(p, "length", 16)? as usize;
                ensure!(length <= 4096, "peek length exceeds 4096");
                json!({"address":a,"bytes":self.emu()?.inspect(&a,length).map_err(|e|anyhow::anyhow!(e.message))?})
            }
            "emulator.snapshot.save" => {
                let name = string(p, "name")?.to_owned();
                ensure!(
                    self.snapshots.len() < 64 || self.snapshots.contains_key(&name),
                    "snapshot count exceeds 64"
                );
                let snap = self
                    .emu()?
                    .snapshot()
                    .map_err(|e| anyhow::anyhow!(e.message))?;
                self.snapshots.insert(name, snap);
                json!({"ok":true})
            }
            "emulator.snapshot.load" => {
                let snap = self
                    .snapshots
                    .get(string(p, "name")?)
                    .context("unknown snapshot")?
                    .clone();
                self.emu()?
                    .restore(&snap)
                    .map_err(|e| anyhow::anyhow!(e.message))?;
                serde_json::to_value(self.emu()?.state())?
            }
            "emulator.snapshot.export" => {
                let v = self.emu()?.export_snapshot()?;
                if p.get("path").is_some() {
                    self.save_new(&path(p)?, &v)?;
                    json!({"ok":true})
                } else {
                    v
                }
            }
            "emulator.snapshot.import" => {
                let value = if p.get("path").is_some() {
                    let path = path(p)?;
                    ensure!(
                        std::fs::metadata(&path)?.len() <= 32 * 1024 * 1024,
                        "snapshot exceeds 32 MiB"
                    );
                    serde_json::from_slice(&std::fs::read(path)?)?
                } else {
                    p.get("snapshot").context("missing snapshot")?.clone()
                };
                self.emu()?.import_snapshot(value)?;
                serde_json::to_value(self.emu()?.state())?
            }
            "emulator.audio.clear" => {
                self.emu()?.clear_audio();
                json!({"ok":true})
            }
            "emulator.audio.export" => {
                let path = path(p)?;
                self.distinct(&path)?;
                self.emu()?.write_wav(&path)?;
                json!({"ok":true,"sample_rate":48000,"channels":2,"stereo_frames":self.emu()?.audio.0.len()/2,"dropped_stereo_frames":self.emu()?.audio.1})
            }
            "emulator.framebuffer" => {
                if p.get("path").is_some() {
                    let target = path(p)?;
                    self.distinct(&target)?;
                    self.emu()?.write_frame(&target)?;
                    json!({"ok":true,"format":"ppm"})
                } else {
                    let e = self.emu()?;
                    json!({"width":e.output.width,"height":e.output.height,"hashes":e.output.frame_hashes,"pixels":e.output.pixels,"format":"rgb8"})
                }
            }
            "trace.configure" => {
                let mut c = self.emu()?.trace.config.clone();
                c.enabled = p
                    .get("enabled")
                    .map(|v| v.as_bool().context("enabled must be boolean"))
                    .transpose()?
                    .unwrap_or(c.enabled);
                c.capacity = num(p, "capacity", c.capacity as u64)? as usize;
                if let Some(k) = p.get("kinds") {
                    c.kinds = serde_json::from_value(k.clone())?;
                }
                if let Some(s) = p.get("space") {
                    c.space = if s.is_null() {
                        None
                    } else {
                        Some(s.as_str().context("space must be string")?.into())
                    };
                }
                if let Some(r) = p.get("range") {
                    c.range = if r.is_null() {
                        None
                    } else {
                        Some(serde_json::from_value(r.clone())?)
                    };
                }
                self.emu()?
                    .configure_instrumentation(c)
                    .map_err(|e| anyhow::anyhow!(e.message))?;
                json!({"ok":true})
            }
            "trace.summary" => serde_json::to_value(self.emu()?.trace.summary())?,
            "trace.query" => {
                let offset = num(p, "offset", 0)? as usize;
                let limit = num(p, "limit", 100)? as usize;
                ensure!(limit <= 1000, "limit exceeds 1000");
                json!({"events":self.emu()?.trace.query(offset,limit)})
            }
            "trace.clear" => {
                self.emu()?.clear_trace();
                json!({"ok":true})
            }
            "trace.stream.start" => {
                let path = path(p)?;
                self.distinct(&path)?;
                self.emu()?
                    .trace
                    .start_stream(&path, num(p, "max_bytes", 1048576)?)
                    .map_err(anyhow::Error::msg)?;
                json!({"ok":true})
            }
            "trace.stream.stop" => json!({"ok":true,"stream":self.emu()?.trace.stop_stream()}),
            "analysis.contexts" => page(
                self.emu()?
                    .trace
                    .contextual_roots()
                    .iter()
                    .map(|(address, hits)| json!({"address":address,"hits":hits})),
                p,
            )?,
            "analysis.coverage" => page(
                self.emu()?
                    .trace
                    .coverage()
                    .iter()
                    .map(|(address, hits)| json!({"address":address,"hits":hits})),
                p,
            )?,
            "analysis.edges" => page(
                self.emu()?.trace.edges().iter().map(
                    |((source, target), hits)| json!({"source":source,"target":target,"hits":hits}),
                ),
                p,
            )?,
            "analysis.pointers" => {
                let offset = num(p, "offset", 0)? as usize;
                let count = num(p, "count", 1)? as usize;
                let width = num(p, "width", 2)? as usize;
                ensure!(
                    count <= 4096 && matches!(width, 2 | 3),
                    "pointer count/width unsupported"
                );
                let stride = num(p, "stride", width as u64)? as usize;
                ensure!(stride >= width && stride <= 256, "invalid pointer stride");
                let mut hypotheses = Vec::new();
                for index in 0..count {
                    let at = offset
                        .checked_add(
                            index
                                .checked_mul(stride)
                                .context("pointer range overflow")?,
                        )
                        .context("pointer range overflow")?;
                    let bytes = self.rom()?.check_range(at, width)?;
                    let value = bytes
                        .iter()
                        .enumerate()
                        .fold(0u32, |n, (i, b)| n | (u32::from(*b) << (8 * i)));
                    ensure!(
                        width == 2,
                        "SMS pointers are 16-bit little-endian; explicit bank layout required for bank fields"
                    );
                    let cpu_address = Some(value);
                    let context: Context =
                        serde_json::from_value(p.get("context").cloned().unwrap_or(json!({})))?;
                    let target = crate::mapping::resolve(self.rom()?, value, context);
                    hypotheses.push(json!({"offset":at,"encoded_value":value,"cpu_address":cpu_address,"target":target,"code_proven":false,"status":"hypothesis"}));
                }
                json!({"hypotheses":hypotheses,"map_changed":false})
            }
            "analysis.decode" | "analysis.references" => {
                let offset = num(p, "offset", 0)? as usize;
                let count = num(p, "count", 16)? as usize;
                let cpu = cpu(p)?;
                let context: Context =
                    serde_json::from_value(p.get("context").cloned().unwrap_or(json!({})))?;
                let pc = if p.get("cpu_address").is_some() {
                    u32::try_from(num(p, "cpu_address", 0)?)?
                } else {
                    self.rom()?.canonical_cpu(offset)?
                };
                decode_range(self.rom()?, offset, count, cpu, pc, context)?
            }
            "analysis.discover" | "analysis.discover_context" => {
                let empty_roots = BTreeMap::new();
                let empty_edges = BTreeMap::new();
                let roots = self
                    .emulator
                    .as_ref()
                    .map(|e| e.trace.contextual_roots())
                    .unwrap_or(&empty_roots);
                let edges = self
                    .emulator
                    .as_ref()
                    .map(|e| e.trace.contextual_edges())
                    .unwrap_or(&empty_edges);
                crate::analysis::discover(self.rom()?, self.map()?, p, roots, edges)?
            }
            "map.get" => page(self.map()?.regions.iter(), p)?,
            "map.annotate" => {
                let entry: Entry =
                    serde_json::from_value(p.get("region").cloned().unwrap_or_else(|| p.clone()))?;
                let mut candidate = self.map()?.clone();
                let id = candidate.annotate(self.rom()?, entry)?;
                self.map = Some(candidate);
                json!({"ok":true,"id":id})
            }
            "map.remove" => {
                let id = num(p, "id", 0)?;
                let mut candidate = self.map()?.clone();
                let n = candidate.regions.len();
                candidate.regions.retain(|r| r.id != id);
                ensure!(candidate.regions.len() != n, "unknown region id");
                self.map = Some(candidate);
                json!({"ok":true})
            }
            "map.validate" => {
                self.map()?.validate(self.rom()?)?;
                json!({"ok":true})
            }
            "map.save" | "map.export" => {
                self.identity()?;
                let dest = path(p)?;
                self.distinct(&dest)?;
                self.map()?.validate(self.rom()?)?;
                self.map()?.save(&dest)?;
                json!({"ok":true})
            }
            "map.load" | "map.import" => {
                self.identity()?;
                let map = Mapping::load(&path(p)?)?;
                map.validate(self.rom()?)?;
                self.map = Some(map);
                json!({"ok":true})
            }
            "disasm.generate" => {
                self.identity()?;
                let out = PathBuf::from(string(p, "out")?);
                Backend {
                    platform: Some(self.rom()?.info.platform.clone()),
                }
                .generate(
                    self.path.as_ref().unwrap(),
                    self.map()?,
                    &out,
                    num(p, "timeout_ms", 120000)?,
                )?;
                json!({"ok":true})
            }
            "disasm.verify" => {
                self.identity()?;
                Backend {
                    platform: Some(self.rom()?.info.platform.clone()),
                }
                .verify(
                    self.path.as_ref().unwrap(),
                    &PathBuf::from(string(p, "generated")?),
                    num(p, "timeout_ms", 120000)?,
                )?;
                json!({"ok":true,"byte_identical":true})
            }
            _ => unreachable!(),
        })
    }
}
pub fn decode_range(
    rom: &Rom,
    mut offset: usize,
    count: usize,
    cpu: Cpu,
    mut pc: u32,
    mut context: Context,
) -> Result<Value> {
    ensure!(count <= 4096, "decode count exceeds 4096");
    let mut instructions = Vec::new();
    let mut diagnostics = Vec::new();
    let mut references = Vec::new();
    for _ in 0..count {
        if offset >= rom.bytes.len() {
            break;
        }
        match decode::decode(cpu, &rom.bytes[offset..], pc, context) {
            Ok(i) => {
                for r in &i.references {
                    let mut r = r.clone();
                    let target = r
                        .get("cpu_address")
                        .filter(|_| {
                            r["kind"] != "io" && r["indexed"] != true && r["indirect"] != true
                        })
                        .and_then(Value::as_u64)
                        .and_then(|a| {
                            u32::try_from(a)
                                .ok()
                                .and_then(|n| crate::mapping::resolve(rom, n, context))
                        });
                    r["target"] = serde_json::to_value(target)?;
                    references.push(json!({"source_offset":offset,"reference":r}));
                }
                let length = i.length;
                context = i.after;
                pc = (pc & 0xff0000) | ((pc + length as u32) & 0xffff);
                instructions.push(json!({"offset":offset,"instruction":i}));
                offset += length;
            }
            Err(e) => {
                diagnostics.push(json!({"offset":offset,"message":e.to_string()}));
                break;
            }
        }
    }
    Ok(
        json!({"instructions":instructions,"next_offset":offset,"context":context,"diagnostics":diagnostics,"references":references}),
    )
}
