//! Deterministic bounded baseline SNES execution and immutable inspection.
use crate::decode::Cpu;
use crate::{
    Rom,
    image::{Layout, address},
};
use anyhow::{Result, ensure};
use bincode::{Decode, Encode};
use jgenesis_common::{
    frontend::{
        AudioOutput, Color, EmulatorTrait, FrameSize, RenderFrameOptions, Renderer, SaveWriter,
        TimingMode,
    },
    romatlas_debug as probe,
};
use romatlas_analysis::{Trace, TraceConfig};
use romatlas_core::execution::{
    AddressSpace, ExecutionAdapter, ExecutionCapabilities, StepRequest, StepUnit,
};
use romatlas_core::{Address, ApiError, EventKind, TraceEvent};
use serde::{Deserialize, Serialize};
use serde_json::json;
use sha2::{Digest, Sha256};
use snes_core::{
    api::{CoprocessorRoms, SnesEmulator, SnesEmulatorConfig},
    input::SnesInputs,
};
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
}
impl Renderer for Output {
    type Err = std::io::Error;
    fn render_frame(
        &mut self,
        data: &[Color],
        size: FrameSize,
        _: RenderFrameOptions,
    ) -> Result<(), Self::Err> {
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
#[derive(Clone, Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct CpuState {
    pub pc: u32,
    pub a: u16,
    pub x: u16,
    pub y: u16,
    pub p: u8,
    pub e: bool,
    pub db: u8,
    pub direct_page: u16,
    pub stack: u16,
    pub mid_instruction: bool,
}
#[derive(Clone, Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct SpcState {
    pub pc: u16,
    pub a: u8,
    pub x: u8,
    pub y: u8,
    pub p: u8,
    pub stack: u8,
    pub cycles: u64,
    pub mid_instruction: bool,
    pub boot_rom_mapped: bool,
}
#[derive(Clone, Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct State {
    pub artificial: bool,
    pub artificial_sram: bool,
    pub master_cycles: u64,
    pub frames: u64,
    pub main: CpuState,
    pub spc700: SpcState,
}
#[derive(Clone)]
pub struct Snapshot {
    artificial: bool,
    artificial_sram: bool,
    hash: String,
    core: Vec<u8>,
    inputs: SnesInputs,
}
#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Envelope {
    artificial: bool,
    artificial_sram: bool,
    format: String,
    version: u32,
    rom_sha256: String,
    core_sha256: String,
    core: Vec<u8>,
    input: Vec<u8>,
}
pub struct Emulator {
    pub artificial: bool,
    pub artificial_sram: bool,
    core: SnesEmulator,
    pub rom: Rom,
    inputs: SnesInputs,
    pub output: Output,
    pub audio: Audio,
    saves: Saves,
    pub trace: Trace,
    pub selected_cpu: Cpu,
    previous: [Option<Address>; 2],
    pub sequence: u64,
}
impl Emulator {
    pub fn open(path: &Path) -> Result<Self> {
        Self::from_rom(Rom::open(path)?)
    }
    pub fn from_rom(rom: Rom) -> Result<Self> {
        let mut saves = Saves::default();
        let core = SnesEmulator::create(
            rom.payload().to_vec(),
            SnesEmulatorConfig {
                forced_timing_mode: Some(TimingMode::Ntsc),
                ..Default::default()
            },
            CoprocessorRoms::none(),
            &mut saves,
        )?;
        ensure!(
            core.debug_mapper() == Some(rom.info.layout == Layout::HiRom),
            "upstream core selected an unsupported or conflicting cartridge layout"
        );
        Ok(Self {
            artificial: false,
            artificial_sram: false,
            core,
            rom,
            inputs: Default::default(),
            output: Default::default(),
            audio: Default::default(),
            saves,
            trace: Default::default(),
            selected_cpu: Cpu::Main,
            previous: [None, None],
            sequence: 0,
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
    pub fn input_buttons(&mut self, buttons: &[String]) -> Result<()> {
        self.input_port(1, buttons)
    }
    pub fn input_port(&mut self, port: u8, buttons: &[String]) -> Result<()> {
        ensure!(matches!(port, 1 | 2), "controller port must be 1 or 2");
        let mut input = SnesInputs::default();
        for button in buttons {
            match button.to_ascii_lowercase().as_str() {
                "a" => input.p1.a = true,
                "b" => input.p1.b = true,
                "x" => input.p1.x = true,
                "y" => input.p1.y = true,
                "l" => input.p1.l = true,
                "r" => input.p1.r = true,
                "start" => input.p1.start = true,
                "select" => input.p1.select = true,
                "up" => input.p1.up = true,
                "down" => input.p1.down = true,
                "left" => input.p1.left = true,
                "right" => input.p1.right = true,
                _ => anyhow::bail!("unknown SNES button {button}"),
            }
        }
        if port == 1 {
            self.inputs.p1 = input.p1;
        } else {
            self.inputs.p2 = snes_core::input::SnesInputDevice::Controller(input.p1);
        }
        Ok(())
    }
    pub fn export_snapshot(&self) -> Result<serde_json::Value> {
        let s = self.snapshot().map_err(|e| anyhow::anyhow!(e.message))?;
        Ok(serde_json::to_value(Envelope {
            format: "romatlas-snes-state".into(),
            version: 2,
            artificial: self.artificial,
            artificial_sram: self.artificial_sram,
            rom_sha256: s.hash,
            core_sha256: format!("{:x}", Sha256::digest(&s.core)),
            core: s.core,
            input: bincode::encode_to_vec(s.inputs, bincode::config::standard())?,
        })?)
    }
    pub fn import_snapshot(&mut self, v: serde_json::Value) -> Result<()> {
        let e: Envelope = serde_json::from_value(v)?;
        ensure!(
            e.format == "romatlas-snes-state" && e.version == 2,
            "unsupported SNES snapshot version"
        );
        ensure!(
            e.core_sha256 == format!("{:x}", Sha256::digest(&e.core)),
            "snapshot core checksum mismatch"
        );
        let (inputs, n) = bincode::decode_from_slice(&e.input, bincode::config::standard())?;
        ensure!(n == e.input.len(), "trailing snapshot input");
        self.restore(&Snapshot {
            artificial: e.artificial,
            artificial_sram: e.artificial_sram,
            hash: e.rom_sha256,
            core: e.core,
            inputs,
        })
        .map_err(|e| anyhow::anyhow!(e.message))
    }
    fn tick(&mut self) -> Result<()> {
        let master = self.state().master_cycles;
        probe::begin(self.trace.config.enabled, master);
        self.core.tick(
            &mut self.output,
            &mut self.audio,
            &self.inputs,
            &mut self.saves,
        )?;
        let (events, dropped) = probe::drain();
        self.trace.dropped = self.trace.dropped.saturating_add(dropped);
        for e in events {
            self.sequence = self.sequence.saturating_add(1);
            let cpu = if e.cpu == "spc700" {
                Cpu::Spc700
            } else {
                Cpu::Main
            };
            let index = usize::from(cpu == Cpu::Spc700);
            let mapped = if cpu == Cpu::Main {
                self.rom.translate(e.address)
            } else if e.space == "aram" {
                Some(address("aram", u64::from(e.address)))
            } else if e.space == "ipl" {
                Some(address("ipl", u64::from(e.address & 63)))
            } else {
                None
            };
            let mut location = match (&mapped, e.kind) {
                (Some(a), "write") if a.space == "rom" => address("cpu", u64::from(e.address)),
                (Some(a), _) => a.clone(),
                _ => address(
                    if cpu == Cpu::Main { "cpu" } else { "spc_cpu" },
                    u64::from(e.address),
                ),
            };
            if e.kind == "frame" {
                location = address("video-frame", self.state().frames);
            }
            if self.artificial {
                location.context.insert("artificial".into(), "true".into());
            }
            if self.artificial && e.kind == "write" && location.space == "sram" {
                self.artificial_sram = true;
            }
            let kind = match e.kind {
                "instruction" => EventKind::Instruction,
                "read" => EventKind::Read,
                "write" => EventKind::Write,
                "interrupt" => EventKind::Interrupt,
                "frame" => EventKind::Frame,
                _ => continue,
            };
            let mut context = json!({"cpu":e.cpu,"cpu_address":e.address,"origin":e.origin,"sequence":self.sequence,"clock":if cpu==Cpu::Main{"master_cycles"}else{"spc_cycles"},"clock_cycle":e.cycle,"master_tick_start":master,"artificial":self.artificial});
            if let Some(r) = e.registers {
                context["registers"] = json!({"a":r[0],"x":r[1],"y":r[2],"stack":r[3],"direct_page":r[4],"db":r[5],"p":r[6],"e":r[7]!=0});
            }
            // Edges join two actual observed opcode fetches for the same CPU.
            // Interrupts split the chain so an interrupt handler is not a guessed successor.
            if kind == EventKind::Interrupt {
                self.previous[index] = None;
            }
            if kind == EventKind::Instruction {
                let mut traversal = location.clone();
                traversal.context.insert("cpu".into(), e.cpu.into());
                traversal
                    .context
                    .insert("cpu_address".into(), e.address.to_string());
                if let Some(r) = e.registers {
                    let cpu_context = if cpu == Cpu::Main {
                        crate::decode::Context {
                            e: Some(r[7] != 0),
                            m: Some(r[6] & 0x20 != 0),
                            x: Some(r[6] & 0x10 != 0),
                            carry: Some(r[6] & 1 != 0),
                            db: Some(r[5] as u8),
                            direct_page: Some(r[4] as u16),
                        }
                    } else {
                        crate::decode::Context {
                            direct_page: Some(if r[6] & 0x20 != 0 { 0x100 } else { 0 }),
                            ..Default::default()
                        }
                    };
                    traversal
                        .context
                        .insert("cpu_context".into(), serde_json::to_string(&cpu_context)?);
                }
                if let Some(source_view) = self.previous[index].replace(traversal.clone()) {
                    let mut source = source_view.clone();
                    source.context.retain(|k, _| k == "artificial");
                    let mut edge_context = context.clone();
                    edge_context["traversal_source"] = serde_json::to_value(source_view)?;
                    edge_context["traversal_target"] = serde_json::to_value(&traversal)?;
                    self.trace.capture(TraceEvent {
                        kind: EventKind::ControlFlow,
                        cycle: master,
                        address: source,
                        target: Some(location.clone()),
                        value: None,
                        context: edge_context,
                    });
                }
                context["traversal_source"] = serde_json::to_value(traversal)?;
                self.trace.capture(TraceEvent {
                    kind: EventKind::Fetch,
                    cycle: master,
                    address: location.clone(),
                    target: None,
                    value: Some(u64::from(e.value)),
                    context: context.clone(),
                });
            }
            if e.origin == "dma" {
                self.trace.capture(TraceEvent {
                    kind: EventKind::Dma,
                    cycle: master,
                    address: location.clone(),
                    target: None,
                    value: Some(u64::from(e.value)),
                    context: context.clone(),
                });
            }
            self.trace.capture(TraceEvent {
                kind,
                cycle: master,
                address: location,
                target: None,
                value: Some(u64::from(e.value)),
                context,
            });
        }
        Ok(())
    }
    pub fn write_memory(&mut self, at: &Address, bytes: &[u8]) -> Result<()> {
        ensure!(bytes.len() <= 4096, "write exceeds 4096 bytes");
        let mut targets = Vec::new();
        for i in 0..bytes.len() {
            let mut a = address(
                &at.space,
                at.offset
                    .checked_add(i as u64)
                    .ok_or_else(|| anyhow::anyhow!("address overflow"))?,
            );
            if matches!(a.space.as_str(), "cpu" | "spc_cpu") {
                a = self
                    .translate(&a)
                    .map_err(|e| anyhow::anyhow!(e.message))?
                    .ok_or_else(|| anyhow::anyhow!("mapped I/O writes unsupported"))?;
            }
            let size = match a.space.as_str() {
                "wram" => 1 << 17,
                "aram" => 1 << 16,
                "sram" => self.rom.info.sram_size,
                _ => anyhow::bail!("only WRAM/ARAM/SRAM can be patched"),
            };
            ensure!(a.offset < size as u64, "write outside backing storage");
            targets.push(a);
        }
        if targets.iter().any(|a| a.space == "sram") {
            self.artificial_sram = true;
        }
        for (a, b) in targets.iter().zip(bytes) {
            self.core.debug_write(&a.space, a.offset as usize, *b);
        }
        if !bytes.is_empty() {
            self.artificial = true;
            self.previous = [None, None];
        }
        Ok(())
    }
    pub fn set_registers(&mut self, cpu: Cpu, patch: &serde_json::Value) -> Result<()> {
        let values = patch
            .as_object()
            .ok_or_else(|| anyhow::anyhow!("registers must be an object"))?;
        let state = self.state();
        if cpu == Cpu::Main {
            ensure!(
                !state.main.mid_instruction,
                "step main CPU to an instruction boundary before register edits"
            );
            let mut value = serde_json::to_value(state.main)?;
            for (key, v) in values {
                ensure!(
                    ["pc", "a", "x", "y", "p", "e", "db", "direct_page", "stack"]
                        .contains(&key.as_str()),
                    "unsupported main CPU register {key}"
                );
                value[key] = v.clone();
            }
            let mut r: CpuState = serde_json::from_value(value)?;
            ensure!(r.pc <= 0xffffff, "PC exceeds 24 bits");
            if r.e {
                r.p |= 0x30;
                r.stack = 0x100 | (r.stack & 255);
            }
            if r.p & 0x10 != 0 {
                r.x &= 255;
                r.y &= 255;
            }
            self.core.debug_set_main(wdc65816_emu::core::Registers {
                a: r.a,
                x: r.x,
                y: r.y,
                s: r.stack,
                d: r.direct_page,
                dbr: r.db,
                pbr: (r.pc >> 16) as u8,
                pc: r.pc as u16,
                p: r.p.into(),
                emulation_mode: r.e,
            });
        } else {
            ensure!(
                !state.spc700.mid_instruction,
                "step SPC700 to an instruction boundary before register edits"
            );
            let mut value = serde_json::to_value(state.spc700)?;
            for (key, v) in values {
                ensure!(
                    ["pc", "a", "x", "y", "p", "stack"].contains(&key.as_str()),
                    "unsupported SPC register {key}"
                );
                value[key] = v.clone();
            }
            let r: SpcState = serde_json::from_value(value)?;
            self.core.debug_set_spc(spc700_emu::Registers {
                a: r.a,
                x: r.x,
                y: r.y,
                sp: r.stack,
                pc: r.pc,
                psw: r.p.into(),
            });
        }
        self.artificial = true;
        self.previous = [None, None];
        Ok(())
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
        self.previous = [None, None];
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
    type Input = SnesInputs;
    type Instrumentation = TraceConfig;
    fn load(path: &Path, _: ()) -> Result<Self, ApiError> {
        Self::open(path).map_err(invalid)
    }
    fn capabilities(&self) -> ExecutionCapabilities {
        ExecutionCapabilities {
            platform: "snes".into(),
            clock: "master_cycles".into(),
            units: vec![StepUnit::Operation, StepUnit::Cycle, StepUnit::Frame],
            address_spaces: [
                ("rom", self.rom.bytes.len() as u64),
                ("cpu", 1 << 24),
                ("spc_cpu", 1 << 16),
                ("wram", 1 << 17),
                ("aram", 1 << 16),
                ("ipl", 64),
                ("vram", 65536),
                ("cgram", 512),
                ("oam", 544),
                ("sram", self.rom.info.sram_size as u64),
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
        let (master_cycles, frames, r, mid) = self.core.debug_state();
        let (spc, cycles, spc_mid) = self.core.debug_spc_state();
        State {
            artificial: self.artificial,
            artificial_sram: self.artificial_sram,
            master_cycles,
            frames,
            main: CpuState {
                pc: (u32::from(r.pbr) << 16) | u32::from(r.pc),
                a: r.a,
                x: r.x,
                y: r.y,
                p: r.p.into(),
                e: r.emulation_mode,
                db: r.dbr,
                direct_page: r.d,
                stack: r.s,
                mid_instruction: mid,
            },
            spc700: SpcState {
                pc: spc.pc,
                a: spc.a,
                x: spc.x,
                y: spc.y,
                p: spc.psw.into(),
                stack: spc.sp,
                cycles,
                mid_instruction: spc_mid,
                boot_rom_mapped: self.core.debug_boot_rom_mapped(),
            },
        }
    }
    fn execute(&mut self, r: StepRequest) -> Result<State, ApiError> {
        if r.max_operations > MAX_TICKS || !(1..=300000).contains(&r.timeout_ms) {
            return Err(invalid("execution budget exceeds limits"));
        }
        let initial = self.state();
        let index = usize::from(self.selected_cpu == Cpu::Spc700);
        let fetches = probe::fetches()[index];
        let start = Instant::now();
        for tick in 0..=r.max_operations {
            let now = self.state();
            let progress = match r.unit {
                StepUnit::Frame => now.frames - initial.frames,
                StepUnit::Cycle => now.master_cycles - initial.master_cycles,
                StepUnit::Operation => {
                    let mid = if index == 0 {
                        now.main.mid_instruction
                    } else {
                        now.spc700.mid_instruction
                    };
                    if mid && r.count != 0 {
                        0
                    } else {
                        probe::fetches()[index] - fetches
                    }
                }
            };
            if progress >= r.count {
                return Ok(now);
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
        if let Some(s) = self.core.debug_sram() {
            self.saves.persist_bytes("sav", s).map_err(invalid)?;
        }
        self.core.hard_reset(&mut self.saves);
        self.inputs = Default::default();
        self.artificial = self.artificial_sram;
        self.previous = [None, None];
        self.output = Default::default();
        self.audio = Default::default();
        Ok(())
    }
    fn input(&mut self, input: SnesInputs) -> Result<(), ApiError> {
        self.inputs = input;
        Ok(())
    }
    fn snapshot(&self) -> Result<Snapshot, ApiError> {
        Ok(Snapshot {
            artificial: self.artificial,
            artificial_sram: self.artificial_sram,
            hash: self.rom.info.sha256.clone(),
            core: bincode::encode_to_vec(&self.core, bincode::config::standard())
                .map_err(invalid)?,
            inputs: self.inputs,
        })
    }
    fn restore(&mut self, s: &Snapshot) -> Result<(), ApiError> {
        if s.hash != self.rom.info.sha256 {
            return Err(invalid("snapshot ROM identity mismatch"));
        }
        let (mut core, n): (SnesEmulator, usize) = bincode::decode_from_slice(
            &s.core,
            bincode::config::standard().with_limit::<16777216>(),
        )
        .map_err(invalid)?;
        if n != s.core.len() {
            return Err(invalid("trailing core snapshot data"));
        }
        if core.debug_mapper() != Some(self.rom.info.layout == Layout::HiRom)
            || core.debug_sram().map_or(0, |s| s.len()) != self.rom.info.sram_size
        {
            return Err(invalid("snapshot cartridge configuration mismatch"));
        }
        core.take_rom_from(&mut self.core);
        self.core = core;
        self.inputs = s.inputs;
        self.artificial = s.artificial;
        self.artificial_sram = s.artificial_sram;
        self.previous = [None, None];
        self.output = Default::default();
        self.audio = Default::default();
        Ok(())
    }
    fn translate(&self, a: &Address) -> Result<Option<Address>, ApiError> {
        match a.space.as_str() {
            "cpu" => Ok(u32::try_from(a.offset)
                .ok()
                .and_then(|n| self.rom.translate(n))),
            "spc_cpu" if a.offset < 65536 => {
                let n = a.offset as u16;
                Ok(if (0xf0..=0xff).contains(&n) {
                    None
                } else if n >= 0xffc0 && self.core.debug_boot_rom_mapped() {
                    Some(address("ipl", u64::from(n & 63)))
                } else {
                    Some(address("aram", a.offset))
                })
            }
            _ => Err(ApiError::unsupported("mapped CPU address space")),
        }
    }
    fn inspect(&mut self, a: &Address, n: usize) -> Result<Vec<u8>, ApiError> {
        if n > 65536 || a.offset.checked_add(n as u64).is_none() {
            return Err(invalid("peek range exceeds limits"));
        }
        (0..n)
            .map(|i| {
                let mut addr = address(&a.space, a.offset + i as u64);
                if matches!(addr.space.as_str(), "cpu" | "spc_cpu") {
                    addr = self.translate(&addr)?.ok_or_else(|| {
                        ApiError::unsupported("side-effect-free I/O/unmapped peek")
                    })?;
                }
                let o = usize::try_from(addr.offset).map_err(invalid)?;
                match addr.space.as_str() {
                    "vram" | "cgram" | "oam" => self
                        .core
                        .debug_ppu_storage(&addr.space, o)
                        .ok_or_else(|| invalid("PPU storage range")),
                    "rom" => self
                        .rom
                        .bytes
                        .get(o)
                        .copied()
                        .ok_or_else(|| invalid("ROM offset")),
                    "wram" if o < 1 << 17 => Ok(self.core.debug_wram(o as u32)),
                    "sram" => self
                        .core
                        .debug_sram()
                        .and_then(|s| s.get(o))
                        .copied()
                        .ok_or_else(|| invalid("SRAM offset")),
                    "aram" if o < 65536 => Ok(self.core.debug_spc_peek(o as u16, true).unwrap()),
                    "ipl" if o < 64 => self.core.debug_ipl(o).ok_or_else(|| invalid("IPL offset")),
                    _ => Err(ApiError::unsupported("memory space or range")),
                }
            })
            .collect()
    }
    fn configure_instrumentation(&mut self, c: TraceConfig) -> Result<(), ApiError> {
        self.trace.configure(c).map_err(invalid)
    }
    fn events(&self, offset: usize, limit: usize) -> Result<Vec<TraceEvent>, ApiError> {
        if limit > 1000 {
            return Err(invalid("trace limit exceeds 1000"));
        }
        Ok(self.trace.query(offset, limit))
    }
}
