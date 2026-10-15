//! Game Boy application adapter. The generic agent crate never imports this module.
use anyhow::{Context, Result, ensure};
use romatlas_core::{ApiError, CommandAdapter, ImageAdapter, Provenance, Region, RegionKind};
use romatlas_gb::{
    Rom,
    emulator::{Emulator, Snapshot},
    mapping,
};
use romatlas_map::store::Store;
use serde_json::{Value, json};
use std::{collections::BTreeMap, path::PathBuf};
#[derive(Default)]
pub struct GbSession {
    path: Option<PathBuf>,
    rom: Option<Rom>,
    store: Option<Store>,
    emulator: Option<Emulator>,
    snapshots: BTreeMap<String, Snapshot>,
}
impl CommandAdapter for GbSession {
    fn platform(&self) -> &str {
        "gb"
    }
    fn methods(&self) -> Vec<String> {
        [
            "api.describe",
            "rom.open",
            "rom.info",
            "rom.read",
            "rom.regions",
            "emulator.start",
            "emulator.reset",
            "emulator.step",
            "emulator.run",
            "emulator.input",
            "emulator.state",
            "emulator.capabilities",
            "emulator.test",
            "emulator.snapshot.save",
            "emulator.snapshot.load",
            "emulator.memory",
            "emulator.physical_memory",
            "trace.configure",
            "trace.summary",
            "trace.query",
            "trace.clear",
            "trace.stream.start",
            "trace.stream.stop",
            "analysis.coverage",
            "analysis.edges",
            "analysis.discover",
            "analysis.discover_context",
            "analysis.references",
            "analysis.pointers",
            "analysis.decode",
            "analysis.structure",
            "analysis.conflicts",
            "analysis.unknown",
            "map.get",
            "map.annotate",
            "map.remove",
            "map.validate",
            "map.export",
            "map.import",
            "map.save",
            "map.load",
            "disasm.generate",
            "disasm.verify",
        ]
        .into_iter()
        .map(str::to_owned)
        .collect()
    }
    fn command(&mut self, method: &str, params: Value) -> std::result::Result<Value, ApiError> {
        if !self.methods().iter().any(|m| m == method) {
            return Err(ApiError::unsupported(method));
        }
        self.dispatch(method, &params).map_err(|e| ApiError {
            code: -32000,
            message: format!("{e:#}"),
        })
    }
}
impl GbSession {
    fn rom(&self) -> Result<&Rom> {
        self.rom.as_ref().context("open a ROM first")
    }
    fn store(&self) -> Result<&Store> {
        self.store.as_ref().context("open a ROM first")
    }
    fn emulator(&mut self) -> Result<&mut Emulator> {
        self.emulator.as_mut().context("start the emulator first")
    }
    fn path(&self) -> Result<&PathBuf> {
        self.path.as_ref().context("open a ROM first")
    }
    fn validate_source_identity(&self) -> Result<()> {
        ensure!(
            Rom::open(self.path()?)?.info.sha256 == self.rom()?.info.sha256,
            "ROM changed since rom.open; reopen the ROM before this operation"
        );
        Ok(())
    }
    pub fn open(&mut self, path: PathBuf) -> Result<Value> {
        let rom = Rom::open(&path)?;
        let store = Store::new("gb", &rom.info.sha256, rom.info.size as u64)?;
        let info = serde_json::to_value(&rom.info)?;
        self.path = Some(path);
        self.rom = Some(rom);
        self.store = Some(store);
        self.emulator = None;
        self.snapshots.clear();
        Ok(info)
    }
    fn dispatch(&mut self, m: &str, p: &Value) -> Result<Value> {
        Ok(match m {
            "rom.open" => self.open(path(p, "path")?)?,
            "rom.info" => serde_json::to_value(&self.rom()?.info)?,
            "rom.read" => {
                let offset = num(p, "offset", 0)?;
                let length = num(p, "length", 16)?;
                ensure!(length <= 4096, "read length exceeds 4096");
                let rom = self.rom()?;
                let a = rom.physical_address(offset).context("offset outside ROM")?;
                json!({"address":a,"bytes":rom.peek(&a,length as usize).map_err(anyhow::Error::msg)?})
            }
            "rom.regions" | "analysis.unknown" => {
                let store = self.store()?;
                let mut ranges: Vec<_> = store
                    .regions
                    .values()
                    .filter(|r| r.kind != RegionKind::Unknown)
                    .map(|r| (r.start.offset, r.start.offset + r.length))
                    .collect();
                ranges.sort();
                let mut cursor = 0;
                let mut gaps = Vec::new();
                for (start, end) in ranges {
                    if cursor < start {
                        gaps.push((cursor, start - cursor));
                    }
                    cursor = cursor.max(end);
                }
                if cursor < store.image_size {
                    gaps.push((cursor, store.image_size - cursor));
                }
                page_records(gaps.into_iter(), p, |(offset, length)| {
                    Ok(json!({"offset":offset,"length":length}))
                })?
            }
            "emulator.start" => {
                let model = p
                    .get("model")
                    .map(Value::as_str)
                    .unwrap_or(Some("auto"))
                    .context("model must be a string")?;
                let e = Emulator::open_model(self.path()?, model)?;
                ensure!(
                    e.image_sha256() == self.rom()?.info.sha256,
                    "ROM changed since rom.open; reopen the ROM before emulator.start"
                );
                let value = serde_json::to_value(e.state())?;
                self.emulator = Some(e);
                self.snapshots.clear();
                value
            }
            "emulator.reset" => {
                self.emulator()?.reset()?;
                self.snapshots.clear();
                serde_json::to_value(self.emulator()?.state())?
            }
            "emulator.test" => {
                let max = num(p, "max_operations", 10_000_000)?;
                let timeout = num(p, "timeout_ms", 10000)?;
                self.emulator()?.mooneye(max, timeout)?
            }
            "emulator.capabilities" => serde_json::to_value(
                romatlas_core::execution::ExecutionAdapter::capabilities(self.emulator()?),
            )?,
            "emulator.state" => serde_json::to_value(self.emulator()?.state())?,
            "emulator.run" | "emulator.step" => {
                let timeout = num(p, "timeout_ms", 1000)?;
                let unit = p
                    .get("unit")
                    .map(Value::as_str)
                    .unwrap_or(Some("instruction"))
                    .context("unit must be string")?;
                let state = if unit == "instruction" || unit == "operation" {
                    ensure!(
                        unit != "operation" || p.get("operations").is_none(),
                        "use count for typed operation stepping"
                    );
                    let count = num(
                        p,
                        if unit == "operation" {
                            "count"
                        } else {
                            "operations"
                        },
                        if m == "emulator.step" { 1 } else { 1000 },
                    )?;
                    romatlas_core::execution::ExecutionAdapter::execute(
                        self.emulator()?,
                        romatlas_core::execution::StepRequest {
                            unit: romatlas_core::execution::StepUnit::Operation,
                            count,
                            max_operations: num(p, "max_operations", 1000000)?,
                            timeout_ms: timeout,
                        },
                    )
                    .map_err(|e| anyhow::anyhow!(e.message))?
                } else {
                    ensure!(
                        p.get("operations").is_none(),
                        "use count and max_operations for cycle/frame stepping"
                    );
                    let count = num(p, "count", 1)?;
                    let max = num(p, "max_operations", 1_000_000)?;
                    let typed = match unit {
                        "cycle" => romatlas_core::execution::StepUnit::Cycle,
                        "frame" => romatlas_core::execution::StepUnit::Frame,
                        _ => anyhow::bail!("unit must be instruction, operation, cycle or frame"),
                    };
                    romatlas_core::execution::ExecutionAdapter::execute(
                        self.emulator()?,
                        romatlas_core::execution::StepRequest {
                            unit: typed,
                            count,
                            max_operations: max,
                            timeout_ms: timeout,
                        },
                    )
                    .map_err(|e| anyhow::anyhow!(e.message))?
                };
                serde_json::to_value(state)?
            }
            "emulator.physical_memory" => {
                let space = string(p, "space")?;
                let offset = usize::try_from(num(p, "offset", 0)?)?;
                let length = usize::try_from(num(p, "length", 16)?)?;
                json!({"bytes":self.emulator()?.physical_memory(space,offset,length)?})
            }
            "emulator.memory" => {
                let address = num(p, "address", 0)?;
                ensure!(address <= 0xffff, "CPU address outside range");
                let length = num(p, "length", 16)?;
                ensure!(length <= 4096, "memory length exceeds 4096");
                json!({"bytes":self.emulator()?.peek(address as u16,length as usize)?})
            }
            "emulator.input" => {
                let pressed = p
                    .get("pressed")
                    .and_then(Value::as_bool)
                    .context("pressed must be boolean")?;
                self.emulator()?.input(string(p, "button")?, pressed)?;
                json!({"ok":true})
            }
            "emulator.snapshot.save" => {
                let name = string(p, "name")?.to_owned();
                ensure!(
                    name.len() <= 128 && !name.is_empty(),
                    "snapshot name must be 1..128 bytes"
                );
                ensure!(
                    self.snapshots.len() < 16 || self.snapshots.contains_key(&name),
                    "snapshot budget exhausted (16)"
                );
                let state = self.emulator()?.save()?;
                self.snapshots.insert(name.clone(), state);
                json!({"name":name})
            }
            "emulator.snapshot.load" => {
                let snapshot = self
                    .snapshots
                    .get(string(p, "name")?)
                    .context("unknown snapshot")?
                    .clone();
                self.emulator()?.load(&snapshot)?;
                serde_json::to_value(self.emulator()?.state())?
            }
            "trace.configure" => {
                let mut config = self.emulator()?.trace.config.clone();
                if let Some(v) = p.get("enabled") {
                    config.enabled = v.as_bool().context("enabled must be boolean")?;
                }
                if p.get("capacity").is_some() {
                    config.capacity = usize::try_from(num_required(p, "capacity")?)?;
                }
                if let Some(kinds) = p.get("kinds") {
                    config.kinds = kinds
                        .as_array()
                        .context("kinds must be an array")?
                        .iter()
                        .map(|v| serde_json::from_value(v.clone()))
                        .collect::<std::result::Result<_, _>>()?;
                }
                if let Some(v) = p.get("space") {
                    config.space = if v.is_null() {
                        None
                    } else {
                        Some(
                            v.as_str()
                                .context("space must be string or null")?
                                .to_owned(),
                        )
                    };
                }
                if let Some(v) = p.get("range") {
                    config.range = if v.is_null() {
                        None
                    } else {
                        let a = v.as_array().context("range must be [start,end]")?;
                        ensure!(a.len() == 2, "range must have two integers");
                        Some((
                            a[0].as_u64().context("range start")?,
                            a[1].as_u64().context("range end")?,
                        ))
                    };
                }
                self.emulator()?.configure_trace(config)?;
                json!({"ok":true})
            }
            "trace.stream.start" => {
                let file = path(p, "path")?;
                let max = num(p, "max_bytes", 16777216)?;
                self.emulator()?
                    .trace
                    .start_stream(&file, max)
                    .map_err(anyhow::Error::msg)?;
                json!({"ok":true})
            }
            "trace.stream.stop" => json!({"stream":self.emulator()?.trace.stop_stream()}),
            "trace.summary" => serde_json::to_value(self.emulator()?.trace.summary())?,
            "trace.query" => {
                let offset = usize::try_from(num(p, "offset", 0)?)?;
                let limit = usize::try_from(num(p, "limit", 100)?)?;
                ensure!(limit <= 1000, "trace query limit exceeds 1000");
                let trace = &self.emulator()?.trace;
                let total = trace.summary().retained;
                json!({"items":trace.query(offset,limit),"total":total,"next_offset":offset.saturating_add(limit).min(total)})
            }
            "trace.clear" => {
                self.emulator()?.trace.clear();
                json!({"ok":true})
            }
            "analysis.coverage" => page_records(
                self.emulator()?.trace.coverage().iter(),
                p,
                |(address, hits)| Ok(json!({"address":address,"hits":hits})),
            )?,
            "analysis.edges" => page_records(
                self.emulator()?.trace.edges().iter(),
                p,
                |((source, target), hits)| {
                    Ok(json!({"source":source,"target":target,"hits":hits,"provenance":"Observed"}))
                },
            )?,
            "analysis.discover_context" => {
                use romatlas_gb::discovery::{ContextDecoder, CpuContext};
                let rom = self.rom()?;
                let roots = p
                    .get("entrypoints")
                    .and_then(Value::as_array)
                    .context("entrypoints must contain offset and cpu_context objects")?;
                ensure!(
                    !roots.is_empty() && roots.len() <= 1000,
                    "entrypoint count must be 1..1000"
                );
                let mut entries = roots
                    .iter()
                    .map(|root| {
                        let address = rom
                            .physical_address(num(root, "offset", 256)?)
                            .context("entrypoint outside ROM")?;
                        let context: CpuContext = serde_json::from_value(
                            root.get("cpu_context")
                                .context("cpu_context is required")?
                                .clone(),
                        )?;
                        Ok(ContextDecoder::node(address, context))
                    })
                    .collect::<Result<Vec<_>>>()?;
                let use_observed = p
                    .get("use_observed")
                    .map(Value::as_bool)
                    .unwrap_or(Some(true))
                    .context("use_observed must be boolean")?;
                let mut observed = Default::default();
                let mut edges = BTreeMap::new();
                if use_observed && let Some(e) = &self.emulator {
                    observed = e.trace.coverage().keys().cloned().collect();
                    edges = e.trace.contextual_edges().clone();
                    entries.extend(e.trace.contextual_roots().keys().cloned());
                }
                let annotations = self.store()?.regions.values().cloned().collect::<Vec<_>>();
                let profiles = p
                    .get("profiles")
                    .map(|value| serde_json::from_value(value.clone()))
                    .transpose()?;
                let decoder =
                    romatlas_gb::discovery_profiles::ProfiledContextDecoder::new(rom, profiles)?;
                let result = romatlas_analysis::discovery::discover_with_edges(
                    &decoder,
                    entries,
                    &observed,
                    &edges,
                    &annotations,
                    usize::try_from(num(p, "budget", 1000)?)?,
                )
                .map_err(anyhow::Error::msg)?;
                let offset = usize::try_from(num(p, "offset", 0)?)?;
                let limit = usize::try_from(num(p, "limit", 100)?)?;
                ensure!(limit <= 1000, "discovery page limit exceeds 1000");
                let total = result.candidates.len();
                let total_references = result.references.len();
                let total_diagnostics = result.diagnostics.len();
                let reference_offset = usize::try_from(num(p, "reference_offset", 0)?)?;
                let diagnostic_offset = usize::try_from(num(p, "diagnostic_offset", 0)?)?;
                json!({"candidates":result.candidates.into_iter().skip(offset).take(limit).collect::<Vec<_>>(),"total":total,"next_offset":offset.saturating_add(limit).min(total),"references":result.references.into_iter().skip(reference_offset).take(limit).collect::<Vec<_>>(),"total_references":total_references,"next_reference_offset":reference_offset.saturating_add(limit).min(total_references),"diagnostics":result.diagnostics.into_iter().skip(diagnostic_offset).take(limit).collect::<Vec<_>>(),"total_diagnostics":total_diagnostics,"next_diagnostic_offset":diagnostic_offset.saturating_add(limit).min(total_diagnostics),"visited":result.visited,"budget_exhausted":result.budget_exhausted})
            }
            "analysis.discover" | "analysis.references" => {
                use std::collections::BTreeSet;
                let budget = usize::try_from(num(p, "budget", 1000)?)?;
                let offsets = if let Some(v) = p.get("entrypoints") {
                    v.as_array()
                        .context("entrypoints must be physical offset array")?
                        .iter()
                        .map(|v| v.as_u64().context("entrypoint must be unsigned integer"))
                        .collect::<Result<Vec<_>>>()?
                } else {
                    vec![256]
                };
                ensure!(offsets.len() <= 1000, "entrypoint count exceeds 1000");
                let mut observed = BTreeSet::new();
                let mut edges = BTreeMap::new();
                if p.get("use_observed")
                    .map(Value::as_bool)
                    .unwrap_or(Some(true))
                    .context("use_observed must be boolean")?
                    && let Some(e) = &self.emulator
                {
                    edges = e.trace.edges().clone();
                    for address in e.trace.coverage().keys() {
                        if address.space == "cartridge-rom" {
                            observed.insert(address.clone());
                        }
                    }
                }
                let rom = self.rom()?;
                let mut entries = offsets
                    .into_iter()
                    .map(|offset| {
                        rom.physical_address(offset)
                            .context("entrypoint outside ROM")
                    })
                    .collect::<Result<Vec<_>>>()?;
                entries.extend(observed.iter().cloned());
                let annotations = self.store()?.regions.values().cloned().collect::<Vec<_>>();
                let result = romatlas_analysis::discovery::discover_with_edges(
                    rom,
                    entries,
                    &observed,
                    &edges,
                    &annotations,
                    budget,
                )
                .map_err(anyhow::Error::msg)?;
                let offset = usize::try_from(num(p, "offset", 0)?)?;
                let limit = usize::try_from(num(p, "limit", 100)?)?;
                ensure!(limit <= 1000, "discovery page limit exceeds 1000");
                if m == "analysis.references" {
                    page_records(result.references.into_iter(), p, |reference| {
                        Ok(serde_json::to_value(reference)?)
                    })?
                } else {
                    let total = result.candidates.len();
                    let total_diagnostics = result.diagnostics.len();
                    let total_references = result.references.len();
                    json!({"candidates":result.candidates.into_iter().skip(offset).take(limit).collect::<Vec<_>>(),"total":total,"next_offset":offset.saturating_add(limit).min(total),"diagnostics":result.diagnostics.into_iter().take(limit).collect::<Vec<_>>(),"total_diagnostics":total_diagnostics,"references":result.references.into_iter().take(limit).collect::<Vec<_>>(),"total_references":total_references,"visited":result.visited,"budget_exhausted":result.budget_exhausted})
                }
            }
            "analysis.pointers" => {
                let offset = usize::try_from(num(p, "offset", 0)?)?;
                let count = usize::try_from(num(p, "count", 16)?)?;
                let bank = |key: &str| -> Result<Option<usize>> {
                    p.get(key)
                        .filter(|v| !v.is_null())
                        .map(|v| {
                            v.as_u64()
                                .context("bank must be unsigned integer")
                                .and_then(|n| usize::try_from(n).map_err(Into::into))
                        })
                        .transpose()
                };
                let layout: romatlas_gb::pointers::Layout = p
                    .get("layout")
                    .map(|value| serde_json::from_value(value.clone()))
                    .transpose()?
                    .unwrap_or_default();
                let result = romatlas_gb::pointers::analyze_layout(
                    self.rom()?,
                    offset,
                    count,
                    romatlas_gb::pointers::BankContext {
                        fixed_bank: bank("fixed_bank")?,
                        switch_bank: bank("switch_bank")?,
                    },
                    layout,
                )?;
                let limit = usize::try_from(num(p, "limit", 100)?)?;
                ensure!(limit <= 1000, "pointer page limit exceeds 1000");
                let page_offset = usize::try_from(num(p, "page_offset", 0)?)?;
                let end = offset + (count * layout.width());
                let conflicts: Vec<_> = self
                    .store()?
                    .regions
                    .iter()
                    .filter(|(_, r)| {
                        r.start.offset < (end as u64)
                            && (offset as u64) < r.start.offset + r.length
                            && r.kind != RegionKind::Unknown
                            && r.kind != RegionKind::Data
                    })
                    .map(|(id, _)| *id)
                    .collect();
                json!({"candidate":result.candidate,"references":result.references.into_iter().skip(page_offset).take(limit).collect::<Vec<_>>(),"total":result.words,"layout":layout,"entry_width":layout.width(),"resolved_rom":result.resolved_rom,"ambiguous_rom":result.ambiguous_rom,"invalid_rom":result.invalid_rom,"non_rom":result.non_rom,"unique_targets":result.unique_targets,"conflicts":conflicts,"diagnostics":result.diagnostics})
            }
            "analysis.structure" => {
                let rom = self.rom()?;
                let address = rom
                    .physical_address(num(p, "offset", 256)?)
                    .context("offset outside ROM")?;
                let context: romatlas_gb::discovery::CpuContext = serde_json::from_value(
                    p.get("cpu_context")
                        .context("cpu_context is required")?
                        .clone(),
                )?;
                serde_json::to_value(
                    rom.decode_structure_with_context(&address, context)
                        .map_err(anyhow::Error::msg)?,
                )?
            }
            "analysis.decode" => {
                let offset = num(p, "offset", 256)?;
                let count = num(p, "count", 16)?;
                ensure!(count <= 4096, "decode count exceeds 4096");
                let rom = self.rom()?;
                ensure!(offset < rom.info.size as u64, "offset outside ROM");
                let mut cursor = offset as usize;
                let mut rows = Vec::new();
                for _ in 0..count {
                    if cursor >= rom.info.size {
                        break;
                    }
                    let (bank, address) = Rom::sym_location(cursor);
                    let end = ((cursor / 0x4000 + 1) * 0x4000).min(rom.info.size);
                    let d = romatlas_gb::decode::decode(&rom.bytes[cursor..end], address as u16)?;
                    let len = d.length as usize;
                    rows.push(json!({"offset":cursor,"bank":bank,"instruction":d}));
                    cursor += len;
                }
                json!({"items":rows,"next_offset":cursor})
            }
            "analysis.conflicts" => json!({"conflicts":self.store()?.conflicts()}),
            "map.get" => page_records(self.store()?.regions.iter(), p, |(id, r)| {
                Ok(json!({"id":id,"region":r,"metadata":self.store()?.region_metadata.get(id)}))
            })?,
            "map.annotate" => {
                let offset = num_required(p, "offset")?;
                let length = num_required(p, "length")?;
                let kind = match string(p, "kind")? {
                    "code" => RegionKind::Code,
                    "data" => RegionKind::Data,
                    "text" => RegionKind::Text,
                    "padding" => RegionKind::Padding,
                    "image" => RegionKind::Image,
                    "unknown" => RegionKind::Unknown,
                    _ => anyhow::bail!("unknown region kind"),
                };
                let provenance = match p
                    .get("provenance")
                    .map(Value::as_str)
                    .unwrap_or(Some("manual"))
                    .context("provenance must be string")?
                {
                    "manual" => Provenance::Manual,
                    "static" => Provenance::Static,
                    "observed" => Provenance::Observed,
                    _ => anyhow::bail!("unknown provenance"),
                };
                let confidence = p
                    .get("confidence")
                    .map(Value::as_f64)
                    .unwrap_or(Some(1.0))
                    .context("confidence must be numeric")? as f32;
                let rom = self.rom()?;
                let region = Region {
                    start: rom.physical_address(offset).context("offset outside ROM")?,
                    length,
                    kind,
                    provenance,
                    confidence,
                };
                // Stage metadata and structural changes together so rejected edits
                // cannot discard imported directives or explanatory evidence.
                let mut store = self.store()?.clone();
                let id = if p.get("id").is_some() {
                    let id = num_required(p, "id")?;
                    store.edit(id, region)?;
                    id
                } else {
                    store.annotate(region)?
                };
                for field in ["name", "role", "evidence", "comment"] {
                    if let Some(value) = p.get(field) {
                        ensure!(
                            value.is_string() || value.is_null(),
                            "{field} must be string or null"
                        );
                        let metadata = store.region_metadata.entry(id).or_insert_with(|| json!({}));
                        let object = metadata
                            .as_object_mut()
                            .context("region metadata must be object")?;
                        if value.is_null() {
                            object.remove(field);
                        } else {
                            object.insert(field.into(), value.clone());
                        }
                    }
                }
                mapping::validate_store(rom, &store)?;
                self.store = Some(store);
                json!({"id":id})
            }
            "map.remove" => {
                let id = num_required(p, "id")?;
                self.store()?;
                self.store.as_mut().unwrap().remove(id)?;
                json!({"ok":true})
            }
            "map.import" => {
                let text = if p.get("path").is_some() {
                    String::from_utf8(romatlas_map::read_limited(&path(p, "path")?, 1024 * 1024)?)?
                } else {
                    string(p, "sym")?.to_owned()
                };
                ensure!(text.len() <= 1024 * 1024, "sym import exceeds 1 MiB");
                let map = romatlas_map::Map::parse(&text)?;
                let store = mapping::import(self.rom()?, &map)?;
                let total = store.regions.len();
                self.store = Some(store);
                json!({"ok":true,"regions":total})
            }
            "map.validate" => {
                mapping::validate_store(self.rom()?, self.store()?)?;
                json!({"ok":true})
            }
            "map.save" => {
                let output = path(p, "path")?;
                crate::output::distinct_from_input(self.path()?, &output)?;
                self.store()?.save(&output)?;
                json!({"ok":true})
            }
            "map.load" => {
                let s = Store::load(&path(p, "path")?)?;
                mapping::validate_store(self.rom()?, &s)?;
                self.store = Some(s);
                json!({"ok":true})
            }
            "map.export" => {
                let include_sym = p
                    .get("include_sym")
                    .map(Value::as_bool)
                    .unwrap_or(Some(true))
                    .context("include_sym must be boolean")?;
                ensure!(
                    include_sym || p.get("path").is_some(),
                    "file-only export requires path"
                );
                let text = mapping::export(self.rom()?, self.store()?)?.export()?;
                if p.get("path").is_some() {
                    let output = path(p, "path")?;
                    crate::output::distinct_from_input(self.path()?, &output)?;
                    std::fs::write(output, &text)?;
                }
                if include_sym {
                    json!({"sym":text})
                } else {
                    json!({"ok":true,"path":path(p,"path")?,"bytes":text.len()})
                }
            }
            "disasm.generate" => {
                self.validate_source_identity()?;
                let map = mapping::export(self.rom()?, self.store()?)?;
                romatlas_gb::disassembly::generate_with_timeout(
                    self.path()?,
                    &map,
                    &path(p, "out")?,
                    num(p, "timeout_ms", 120000)?,
                )?;
                json!({"ok":true})
            }
            "disasm.verify" => {
                self.validate_source_identity()?;
                romatlas_gb::disassembly::verify_with_timeout(
                    self.path()?,
                    &path(p, "generated")?,
                    num(p, "timeout_ms", 120000)?,
                )?;
                json!({"ok":true,"byte_identical":true})
            }
            _ => anyhow::bail!("unsupported capability: {m}"),
        })
    }
}
fn string<'a>(p: &'a Value, key: &str) -> Result<&'a str> {
    p.get(key)
        .and_then(Value::as_str)
        .with_context(|| format!("{key} must be a string"))
}
fn path(p: &Value, key: &str) -> Result<PathBuf> {
    Ok(PathBuf::from(string(p, key)?))
}
fn num_required(p: &Value, key: &str) -> Result<u64> {
    p.get(key)
        .and_then(Value::as_u64)
        .with_context(|| format!("{key} must be an unsigned integer"))
}
fn num(p: &Value, key: &str, default: u64) -> Result<u64> {
    if p.get(key).is_some() {
        num_required(p, key)
    } else {
        Ok(default)
    }
}
fn page_records<T>(
    rows: impl ExactSizeIterator<Item = T>,
    p: &Value,
    format: impl FnMut(T) -> Result<Value>,
) -> Result<Value> {
    let offset = usize::try_from(num(p, "offset", 0)?)?;
    let limit = usize::try_from(num(p, "limit", 100)?)?;
    ensure!(limit <= 1000, "page limit exceeds 1000");
    let total = rows.len();
    let items: Vec<_> = rows
        .skip(offset)
        .take(limit)
        .map(format)
        .collect::<Result<_>>()?;
    Ok(json!({"items":items,"total":total,"next_offset":offset.saturating_add(limit).min(total)}))
}

#[cfg(test)]
mod pagination_tests {
    use super::*;
    #[test]
    fn large_pages_convert_only_returned_records() {
        let converted = std::cell::Cell::new(0);
        let result = page_records(0..100000usize, &json!({"offset":50000,"limit":2}), |n| {
            converted.set(converted.get() + 1);
            Ok(json!({"id":n}))
        })
        .unwrap();
        assert_eq!(converted.get(), 2);
        assert_eq!(result["total"], 100000);
        assert_eq!(result["next_offset"], 50002);
        assert_eq!(result["items"][0]["id"], 50000);
        for (offset, limit) in [(50000, 0), (100001, 2)] {
            let result = page_records(
                0..100000usize,
                &json!({"offset":offset,"limit":limit}),
                |n| {
                    converted.set(converted.get() + 1);
                    Ok(json!(n))
                },
            )
            .unwrap();
            assert!(result["items"].as_array().unwrap().is_empty());
            assert_eq!(converted.get(), 2);
        }
        assert!(
            page_records(0..100000usize, &json!({"limit":1001}), |n| {
                converted.set(converted.get() + 1);
                Ok(json!(n))
            })
            .is_err()
        );
        assert_eq!(converted.get(), 2);
    }
}
