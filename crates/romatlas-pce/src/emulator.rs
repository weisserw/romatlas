//! Bounded NTSC PC Engine execution with actual bus observations and transactional snapshots.
use crate::{
    Rom,
    decode::{Context, Cpu},
    image::address,
};
use anyhow::{Result, ensure};
use bincode::{Decode, Encode};
use jgenesis_common::{
    frontend::{
        AudioOutput, Color, ConstantInputPoller, EmulatorTrait, FrameSize, RenderFrameOptions,
        Renderer, SaveWriter, TickEffect,
    },
    romatlas_debug as probe,
};
use pce_config::PceInputs;
use pce_core::api::{PcEngineEmulator, PceEmulatorConfig};
use romatlas_analysis::{Trace, TraceConfig};
use romatlas_core::{
    Address, ApiError, EventKind, TraceEvent,
    execution::{AddressSpace, ExecutionAdapter, ExecutionCapabilities, StepRequest, StepUnit},
};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use std::{collections::BTreeMap, path::Path, time::Instant};
const MAX_TICKS: u64 = 100_000_000;
const OUTPUT_SAMPLES: usize = 48000 * 2 * 10;
fn invalid(e: impl std::fmt::Display) -> ApiError {
    ApiError::invalid(e.to_string())
}
#[derive(Default)]
pub struct Output {
    pub width: u32,
    pub height: u32,
    pub pixels: Vec<u8>,
    pub frame_hashes: Vec<String>,
    pub discard: bool,
}
impl Renderer for Output {
    type Err = std::io::Error;
    fn render_frame(
        &mut self,
        data: &[Color],
        size: FrameSize,
        _: f64,
        _: RenderFrameOptions,
    ) -> Result<(), Self::Err> {
        if self.discard {
            return Ok(());
        }
        self.width = size.width;
        self.height = size.height;
        self.pixels.clear();
        for c in &data[..(size.width * size.height) as usize] {
            self.pixels.extend_from_slice(&[c.r, c.g, c.b]);
        }
        let mut hash = Sha256::new();
        for c in &data[..(size.width * size.height) as usize] {
            hash.update([c.r, c.g, c.b, c.a]);
        }
        if self.frame_hashes.len() == 600 {
            self.frame_hashes.remove(0);
        }
        self.frame_hashes.push(format!("{:x}", hash.finalize()));
        Ok(())
    }
}
// Separate audio sink to satisfy the core's two independent mutable sinks.
#[derive(Default)]
pub struct Audio(pub Vec<i16>, pub u64);
impl AudioOutput for Audio {
    type Err = std::io::Error;
    fn push_sample(&mut self, l: f64, r: f64) -> Result<(), Self::Err> {
        if self.0.len() >= OUTPUT_SAMPLES {
            self.1 = self.1.saturating_add(1);
            return Ok(());
        }
        for s in [l, r] {
            self.0.push((s.clamp(-1.0, 1.0) * 32767.0).round() as i16);
        }
        Ok(())
    }
}
#[derive(Default)]
pub struct Saves(BTreeMap<String, Vec<u8>>);
impl SaveWriter for Saves {
    type Err = std::io::Error;
    fn load_bytes(&mut self, ext: &str) -> Result<Vec<u8>, Self::Err> {
        self.0
            .get(ext)
            .cloned()
            .ok_or_else(|| std::io::Error::from(std::io::ErrorKind::NotFound))
    }
    fn persist_bytes(&mut self, ext: &str, bytes: &[u8]) -> Result<(), Self::Err> {
        self.0.insert(ext.into(), bytes.into());
        Ok(())
    }
    fn load_serialized<D: Decode<()>>(&mut self, ext: &str) -> Result<D, Self::Err> {
        let bytes = self.load_bytes(ext)?;
        bincode::decode_from_slice(&bytes, bincode::config::standard())
            .map(|(v, _)| v)
            .map_err(std::io::Error::other)
    }
    fn persist_serialized<E: Encode>(&mut self, ext: &str, data: E) -> Result<(), Self::Err> {
        let bytes = bincode::encode_to_vec(data, bincode::config::standard())
            .map_err(std::io::Error::other)?;
        self.persist_bytes(ext, &bytes)
    }
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct CpuState {
    pub pc: u16,
    pub a: u8,
    pub x: u8,
    pub y: u8,
    pub p: u8,
    pub stack: u8,
    pub mid_instruction: bool,
    pub frozen: bool,
}
#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct State {
    pub artificial: bool,
    pub region: String,
    pub master_cycles: u64,
    pub frames: u64,
    pub main: CpuState,
    pub context: Context,
    pub hardware: (String, String, String, String),
}
#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Snapshot {
    format: String,
    version: u32,
    rom_sha256: String,
    core_sha256: String,
    core: Vec<u8>,
    input: Vec<u8>,
    cycles: u64,
    frames: u64,
    artificial: bool,
}
pub struct Emulator {
    core: PcEngineEmulator,
    pub rom: Rom,
    inputs: PceInputs,
    pub trace: Trace,
    pub output: Output,
    pub audio: Audio,
    saves: Saves,
    pub selected_cpu: Cpu,
    cycles: u64,
    frames: u64,
    previous: Option<Address>,
    artificial: bool,
}
fn config() -> PceEmulatorConfig {
    PceEmulatorConfig {
        region: Default::default(),
        aspect_ratio: Default::default(),
        palette: Default::default(),
        crop_overscan: true,
        remove_sprite_limits: false,
        audio_resampler: Default::default(),
        allow_opposing_joypad_directions: true,
        allow_simultaneous_run_select: true,
    }
}
impl Emulator {
    pub fn open(path: &Path) -> Result<Self> {
        Self::from_rom(Rom::open(path)?)
    }
    pub fn from_rom(rom: Rom) -> Result<Self> {
        ensure!(
            rom.info.mapper != "unknown",
            "unsupported HuCard mapper; CD/SuperGrafx and uncommon boards are outside baseline"
        );
        let mut saves = Saves::default();
        let mut core = PcEngineEmulator::create(
            rom.bytes[rom.info.payload_offset..].to_vec(),
            config(),
            &mut saves,
        );
        core.update_audio_output_frequency(48000);
        let cycles = core.debug_cycles();
        Ok(Self {
            core,
            rom,
            inputs: Default::default(),
            trace: Default::default(),
            output: Default::default(),
            audio: Default::default(),
            saves,
            selected_cpu: Cpu::Main,
            cycles,
            frames: 0,
            previous: None,
            artificial: false,
        })
    }
    pub fn run(&mut self, count: u64, timeout_ms: u64) -> Result<State> {
        self.execute(StepRequest {
            unit: StepUnit::Operation,
            count,
            max_operations: MAX_TICKS,
            timeout_ms,
        })
        .map_err(|e| anyhow::anyhow!(e.message))
    }
    pub fn set_region(&mut self, name: &str) -> Result<()> {
        let region = match name {
            "tg16" | "turbografx16" => pce_config::PceRegion::TurboGrafx16,
            "pce" | "pc_engine" => pce_config::PceRegion::PcEngine,
            _ => anyhow::bail!("region must be tg16 or pce"),
        };
        self.core.debug_set_region(region);
        Ok(())
    }
    pub fn bank_context(&self) -> Context {
        Context {
            mpr: self.core.debug_cpu().registers().mpr.map(Some),
            sf2_bank: self.core.debug_bank(),
        }
    }
    fn tick(&mut self) -> Result<()> {
        let before = if self.trace.config.enabled {
            Some(self.bank_context())
        } else {
            None
        };
        probe::begin(self.trace.config.enabled, self.cycles);
        probe::context("main", "cpu", self.cycles);
        let effect = self.core.tick(
            &mut self.output,
            &mut self.audio,
            &mut ConstantInputPoller(&self.inputs),
            &mut self.saves,
        )?;
        self.cycles = self.core.debug_cycles();
        let (events, dropped) = probe::drain();
        self.trace.dropped = self.trace.dropped.saturating_add(dropped);
        for e in events {
            let kind = match e.kind {
                "read" => EventKind::Read,
                "write" => EventKind::Write,
                "instruction" => EventKind::Instruction,
                "interrupt" => EventKind::Interrupt,
                "dma" => EventKind::Dma,
                _ => continue,
            };
            let mut location = e
                .physical
                .map(|(space, n)| {
                    address(
                        space,
                        n + if space == "rom" {
                            self.rom.info.payload_offset as u64
                        } else {
                            0
                        },
                    )
                })
                .unwrap_or_else(|| {
                    address(
                        if kind == EventKind::Instruction || kind == EventKind::Interrupt {
                            "cpu"
                        } else {
                            "bus"
                        },
                        u64::from(e.address),
                    )
                });
            if self.artificial {
                location.context.insert("artificial".into(), "true".into());
            }
            let mut ctx = json!({"cpu":"main","cpu_address":if kind==EventKind::Instruction||kind==EventKind::Interrupt {Some(e.address)}else{None},"bus_address":if kind==EventKind::Instruction||kind==EventKind::Interrupt {None}else{Some(e.address)},"origin":e.origin,"artificial":self.artificial,"cpu_context":before,"clock":"master_cycles"});
            if e.origin == "bus" && kind != EventKind::Instruction && kind != EventKind::Interrupt {
                let aliases: Vec<_> = before
                    .into_iter()
                    .flat_map(|c| c.mpr.into_iter().enumerate())
                    .filter_map(|(slot, page)| {
                        page.filter(|p| u32::from(*p) == e.address / 8192)
                            .map(|_| slot as u32 * 8192 + (e.address & 8191))
                    })
                    .collect();
                ctx["possible_cpu_aliases"] = json!(aliases);
            }
            if let Some(r) = e.registers {
                ctx["registers"] = json!({"a":r[0],"x":r[1],"y":r[2],"stack":r[3],"p":r[6]});
            }
            if kind == EventKind::Interrupt {
                self.previous = None;
            }
            if kind == EventKind::Instruction {
                let mut traversal = location.clone();
                traversal
                    .context
                    .insert("cpu_address".into(), e.address.to_string());
                traversal
                    .context
                    .insert("cpu_context".into(), serde_json::to_string(&before)?);
                ctx["traversal_source"] = serde_json::to_value(&traversal)?;
                if let Some(previous) = self.previous.take() {
                    let source = address(&previous.space, previous.offset);
                    self.trace.capture(TraceEvent{kind:EventKind::ControlFlow,cycle:e.cycle,address:source,target:Some(location.clone()),value:None,context:json!({"traversal_source":previous,"traversal_target":traversal,"observed":true,"artificial":self.artificial})});
                }
                self.previous = Some(traversal);
            }
            self.trace.capture(TraceEvent {
                kind,
                cycle: e.cycle,
                address: location,
                target: None,
                value: Some(e.value.into()),
                context: ctx,
            });
        }
        if let Some(before) = before {
            let after = self.bank_context();
            if before != after {
                self.trace.capture(TraceEvent {
                    kind: EventKind::BankSwitch,
                    cycle: self.cycles,
                    address: address("mapper", 0),
                    target: None,
                    value: None,
                    context: json!({"before":before,"after":after}),
                });
            }
        }
        if effect == TickEffect::FrameRendered {
            self.frames += 1;
            self.trace.capture(TraceEvent {
                kind: EventKind::Frame,
                cycle: self.cycles,
                address: address("video-frame", self.frames),
                target: None,
                value: None,
                context: json!({}),
            });
        }
        Ok(())
    }
    pub fn input_port(&mut self, port: u8, buttons: &[String]) -> Result<()> {
        ensure!(port == 1, "baseline supports one standard two-button pad");
        let mut input = PceInputs::default();
        for button in buttons {
            match button.to_ascii_lowercase().as_str() {
                "i" | "1" => input.p1.button1 = true,
                "ii" | "2" => input.p1.button2 = true,
                "run" => input.p1.run = true,
                "select" => input.p1.select = true,
                "up" => input.p1.up = true,
                "down" => input.p1.down = true,
                "left" => input.p1.left = true,
                "right" => input.p1.right = true,
                _ => anyhow::bail!("unknown PC Engine button {button}"),
            }
        }
        self.inputs = input;
        Ok(())
    }
    pub fn set_registers(&mut self, _: Cpu, values: &Value) -> Result<()> {
        ensure!(
            !self.core.debug_cpu().is_mid_block_transfer(),
            "finish instruction before register edits"
        );
        let mut r = self.core.debug_cpu().registers().clone();
        for (key, value) in values
            .as_object()
            .ok_or_else(|| anyhow::anyhow!("registers must be an object"))?
        {
            let n = value
                .as_u64()
                .ok_or_else(|| anyhow::anyhow!("register must be unsigned"))?;
            match key.as_str() {
                "pc" => r.pc = u16::try_from(n)?,
                "a" => r.a = u8::try_from(n)?,
                "x" => r.x = u8::try_from(n)?,
                "y" => r.y = u8::try_from(n)?,
                "stack" => r.s = u8::try_from(n)?,
                "p" => r.p = huc6280_emu::Flags::from(u8::try_from(n)?),
                k if k.starts_with("mpr") => {
                    let slot: usize = k[3..].parse()?;
                    ensure!(slot < 8, "MPR index");
                    r.mpr[slot] = u8::try_from(n)?;
                }
                _ => anyhow::bail!("unknown CPU register {key}"),
            }
        }
        self.core.debug_cpu_mut().set_registers(r);
        self.artificial = true;
        self.previous = None;
        Ok(())
    }
    pub fn write_memory(&mut self, a: &Address, bytes: &[u8]) -> Result<()> {
        ensure!(bytes.len() <= 4096, "write exceeds 4096 bytes");
        let start = usize::try_from(a.offset)?;
        let end = start
            .checked_add(bytes.len())
            .ok_or_else(|| anyhow::anyhow!("write overflow"))?;
        if a.space == "cpu" || a.space == "bus" {
            ensure!(
                end <= if a.space == "cpu" { 65536 } else { 0x200000 },
                "bus write range"
            );
            for (i, b) in bytes.iter().enumerate() {
                let mut bus = (start + i) as u32;
                if a.space == "cpu" {
                    bus = u32::from(self.core.debug_cpu().registers().mpr[(start + i) >> 13])
                        * 8192
                        + (bus & 8191);
                }
                self.core.debug_io_write(bus, *b);
            }
        } else {
            let size = self
                .core
                .debug_storage(&a.space)
                .ok_or_else(|| anyhow::anyhow!("unsupported backing write"))?
                .len();
            ensure!(end <= size, "backing write range");
            for (i, b) in bytes.iter().enumerate() {
                ensure!(
                    self.core.debug_write(&a.space, start + i, *b),
                    "backing write failed"
                );
            }
        }
        self.cycles = self.core.debug_cycles();
        self.artificial = true;
        self.previous = None;
        Ok(())
    }
    pub fn export_snapshot(&self) -> Result<Value> {
        Ok(serde_json::to_value(
            self.snapshot().map_err(|e| anyhow::anyhow!(e.message))?,
        )?)
    }
    pub fn import_snapshot(&mut self, v: Value) -> Result<()> {
        let s = serde_json::from_value(v)?;
        self.restore(&s).map_err(|e| anyhow::anyhow!(e.message))
    }
    pub fn write_wav(&self, path: &Path) -> Result<()> {
        use std::io::Write;
        let len = u32::try_from(self.audio.0.len() * 2)?;
        let mut f = std::fs::File::create_new(path)?;
        f.write_all(b"RIFF")?;
        f.write_all(&(36 + len).to_le_bytes())?;
        f.write_all(b"WAVEfmt ")?;
        for v in [16_u32, 0x00020001, 48000, 192000, 0x00100004] {
            f.write_all(&v.to_le_bytes())?;
        }
        f.write_all(b"data")?;
        f.write_all(&len.to_le_bytes())?;
        for s in &self.audio.0 {
            f.write_all(&s.to_le_bytes())?;
        }
        Ok(())
    }
    pub fn write_frame(&self, path: &Path) -> Result<()> {
        use std::io::Write;
        ensure!(
            !self.output.pixels.is_empty(),
            "advance through a rendered frame first"
        );
        let mut f = std::fs::File::create_new(path)?;
        write!(f, "P6\n{} {}\n255\n", self.output.width, self.output.height)?;
        f.write_all(&self.output.pixels)?;
        Ok(())
    }
    pub fn clear_trace(&mut self) {
        self.trace.clear();
        self.previous = None;
    }
    pub fn clear_audio(&mut self) {
        self.audio.0.clear();
        self.audio.1 = 0;
    }
}
impl ExecutionAdapter for Emulator {
    type LoadOptions = ();
    type State = State;
    type Snapshot = Snapshot;
    type Input = PceInputs;
    type Instrumentation = TraceConfig;
    fn load(path: &Path, _: ()) -> Result<Self, ApiError> {
        Self::open(path).map_err(invalid)
    }
    fn capabilities(&self) -> ExecutionCapabilities {
        ExecutionCapabilities {
            platform: "pce".into(),
            clock: "master_cycles".into(),
            units: vec![StepUnit::Operation, StepUnit::Cycle, StepUnit::Frame],
            address_spaces: [
                ("rom", self.rom.bytes.len() as u64),
                ("cpu", 65536),
                ("bus", 0x200000),
                ("ram", 8192),
                ("vram", 65536),
                ("cram", 1024),
                ("satb", 512),
                ("io", 8192),
                (
                    "sram",
                    self.core.debug_storage("sram").map_or(0, |v| v.len()) as u64,
                ),
            ]
            .into_iter()
            .map(|(name, size)| AddressSpace {
                name: name.into(),
                size: Some(size),
                context: json!({}),
            })
            .collect(),
            snapshots: true,
            instrumentation: true,
            inputs: true,
        }
    }
    fn state(&self) -> State {
        let cpu = self.core.debug_cpu();
        let r = cpu.registers();
        State {
            artificial: self.artificial,
            region: if self.core.debug_region() == pce_config::PceRegion::PcEngine {
                "pce"
            } else {
                "tg16"
            }
            .into(),
            master_cycles: self.cycles,
            frames: self.frames,
            main: CpuState {
                pc: r.pc,
                a: r.a,
                x: r.x,
                y: r.y,
                p: r.p.to_u8_interrupt(),
                stack: r.s,
                mid_instruction: cpu.is_mid_block_transfer(),
                frozen: false,
            },
            context: self.bank_context(),
            hardware: self.core.debug_hardware(),
        }
    }
    fn execute(&mut self, r: StepRequest) -> Result<State, ApiError> {
        if r.max_operations > MAX_TICKS || !(1..=300000).contains(&r.timeout_ms) {
            return Err(invalid("execution budget exceeds limits"));
        }
        let initial = self.state();
        let fetches = probe::fetches()[0];
        let start = Instant::now();
        for tick in 0..=r.max_operations {
            let progress = match r.unit {
                StepUnit::Cycle => self.cycles - initial.master_cycles,
                StepUnit::Frame => self.frames - initial.frames,
                StepUnit::Operation => {
                    if self.core.debug_cpu().is_mid_block_transfer() {
                        0
                    } else {
                        probe::fetches()[0] - fetches
                    }
                }
            };
            if r.count == 0 || progress >= r.count {
                return Ok(self.state());
            }
            if tick == r.max_operations {
                return Err(invalid("operation budget exhausted"));
            }
            if start.elapsed().as_millis() >= u128::from(r.timeout_ms) {
                return Err(invalid("execution timeout"));
            }

            self.tick().map_err(invalid)?;
        }
        unreachable!()
    }
    fn reset(&mut self) -> Result<(), ApiError> {
        if let Some(ram) = self.core.debug_storage("sram") {
            self.saves.0.insert("sav".into(), ram);
        }
        let mut hardware_config = config();
        hardware_config.region = self.core.debug_region();
        self.core = PcEngineEmulator::create(
            self.rom.bytes[self.rom.info.payload_offset..].to_vec(),
            hardware_config,
            &mut self.saves,
        );
        self.core.update_audio_output_frequency(48000);
        self.cycles = self.core.debug_cycles();
        self.frames = 0;
        self.inputs = Default::default();
        self.previous = None;
        self.output = Default::default();
        self.audio = Default::default();
        self.artificial = false;
        self.trace.clear();
        Ok(())
    }
    fn input(&mut self, input: PceInputs) -> Result<(), ApiError> {
        self.inputs = input;
        Ok(())
    }
    fn snapshot(&self) -> Result<Snapshot, ApiError> {
        let core =
            bincode::encode_to_vec(&self.core, bincode::config::standard()).map_err(invalid)?;
        Ok(Snapshot {
            format: "romatlas-pce-state".into(),
            version: 1,
            rom_sha256: self.rom.info.sha256.clone(),
            core_sha256: format!("{:x}", Sha256::digest(&core)),
            core,
            input: bincode::encode_to_vec(self.inputs, bincode::config::standard())
                .map_err(invalid)?,
            cycles: self.cycles,
            frames: self.frames,
            artificial: self.artificial,
        })
    }
    fn restore(&mut self, s: &Snapshot) -> Result<(), ApiError> {
        if s.format != "romatlas-pce-state"
            || s.version != 1
            || s.rom_sha256 != self.rom.info.sha256
            || s.core_sha256 != format!("{:x}", Sha256::digest(&s.core))
        {
            return Err(invalid("snapshot format/identity/checksum mismatch"));
        }
        let (mut core, n): (PcEngineEmulator, usize) = bincode::decode_from_slice(
            &s.core,
            bincode::config::standard().with_limit::<16777216>(),
        )
        .map_err(invalid)?;
        let (inputs, m): (PceInputs, usize) =
            bincode::decode_from_slice(&s.input, bincode::config::standard().with_limit::<1024>())
                .map_err(invalid)?;
        if n != s.core.len() || m != s.input.len() {
            return Err(invalid("trailing snapshot data"));
        }
        if core.debug_region() != self.core.debug_region()
            || core.debug_bank().is_some() != self.core.debug_bank().is_some()
            || core.debug_cycles() != s.cycles
        {
            return Err(invalid("snapshot core configuration/cycle mismatch"));
        }
        for space in ["ram", "sram", "vram", "cram", "satb"] {
            if core.debug_storage(space).map(|s| s.len())
                != self.core.debug_storage(space).map(|s| s.len())
            {
                return Err(invalid("snapshot storage configuration mismatch"));
            }
        }
        core.debug_take_rom(&mut self.core);
        self.core = core;
        self.inputs = inputs;
        self.cycles = s.cycles;
        self.frames = s.frames;
        self.artificial = s.artificial;
        self.previous = None;
        self.output = Default::default();
        self.audio = Default::default();
        Ok(())
    }
    fn translate(&self, a: &Address) -> Result<Option<Address>, ApiError> {
        if a.space != "cpu" || a.offset >= 65536 {
            return Err(invalid("expected CPU address 0..65535"));
        }
        let bus = u32::from(self.core.debug_cpu().registers().mpr[(a.offset as usize) >> 13])
            * 8192
            + (a.offset as u32 & 8191);
        Ok(self.rom.bus_binding(bus, self.core.debug_bank()))
    }
    fn inspect(&mut self, a: &Address, n: usize) -> Result<Vec<u8>, ApiError> {
        let end = a
            .offset
            .checked_add(n as u64)
            .ok_or_else(|| invalid("peek overflow"))?;
        if n > 65536 {
            return Err(invalid("peek limit"));
        }
        if matches!(a.space.as_str(), "cpu" | "bus" | "io") {
            if end
                > match a.space.as_str() {
                    "cpu" => 65536,
                    "bus" => 0x200000,
                    _ => 8192,
                }
            {
                return Err(invalid("mapped range"));
            }
            probe::suspend(true);
            let result = (0..n)
                .map(|i| {
                    let at = (a.offset + i as u64) as u32;
                    let bus = match a.space.as_str() {
                        "cpu" => {
                            u32::from(self.core.debug_cpu().registers().mpr[(at as usize) >> 13])
                                * 8192
                                + (at & 8191)
                        }
                        "io" => 0x1fe000 + at,
                        _ => at,
                    };
                    self.core.debug_peek_bus(bus)
                })
                .collect();
            probe::suspend(false);
            return Ok(result);
        }
        let bytes = if a.space == "rom" {
            self.rom.bytes.clone()
        } else {
            self.core
                .debug_storage(&a.space)
                .ok_or_else(|| invalid("unknown backing space"))?
        };
        bytes
            .get(a.offset as usize..end as usize)
            .map(Vec::from)
            .ok_or_else(|| invalid("backing range"))
    }
    fn configure_instrumentation(&mut self, c: TraceConfig) -> Result<(), ApiError> {
        self.previous = None;
        self.trace.configure(c).map_err(invalid)
    }
    fn events(&self, offset: usize, limit: usize) -> Result<Vec<TraceEvent>, ApiError> {
        if limit > 1000 {
            return Err(invalid("trace limit exceeds 1000"));
        }
        Ok(self.trace.query(offset, limit))
    }
}
