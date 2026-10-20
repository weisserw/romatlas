//! Bounded NTSC SMS execution with actual bus observations and transactional snapshots.
use crate::{
    Rom,
    decode::{Context, Cpu},
    image::address,
};
use anyhow::{Result, ensure};
use bincode::{Decode, Encode};
use jgenesis_common::{
    frontend::{
        AudioOutput, Color, EmulatorTrait, FrameSize, RenderFrameOptions, Renderer, SaveWriter,
        TickEffect, TimingMode,
    },
    romatlas_debug as probe,
};
use romatlas_analysis::{Trace, TraceConfig};
use romatlas_core::{
    Address, ApiError, EventKind, TraceEvent,
    execution::{AddressSpace, ExecutionAdapter, ExecutionCapabilities, StepRequest, StepUnit},
};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use smsgg_config::SmsGgInputs as SmsInputs;
use smsgg_core::{
    SmsGgEmulator as SmsEmulator, SmsGgEmulatorConfig as SmsEmulatorConfig, SmsGgHardware,
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
    pub discard: bool,
}
impl Renderer for Output {
    type Err = std::io::Error;
    fn render_frame(
        &mut self,
        data: &[Color],
        size: FrameSize,
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
    pub af: u16,
    pub bc: u16,
    pub de: u16,
    pub hl: u16,
    pub ix: u16,
    pub iy: u16,
    pub sp: u16,
    pub af_shadow: u16,
    pub bc_shadow: u16,
    pub de_shadow: u16,
    pub hl_shadow: u16,
    pub i: u16,
    pub r: u16,
    pub iff: u16,
    pub im: u16,
    pub halted: bool,
}
#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct State {
    pub artificial: bool,
    pub cpu_cycles: u64,
    pub frames: u64,
    pub main: CpuState,
    pub context: Context,
    pub video: Value,
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
    core: SmsEmulator,
    pub rom: Rom,
    inputs: SmsInputs,
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
fn config() -> SmsEmulatorConfig {
    SmsEmulatorConfig {
        sms_timing_mode: TimingMode::Ntsc,
        sms_model: smsgg_config::SmsModel::Sms2,
        forced_psg_version: None,
        sms_aspect_ratio: Default::default(),
        gg_aspect_ratio: Default::default(),
        remove_sprite_limit: false,
        forced_region: None,
        sms_crop_vertical_border: true,
        sms_crop_left_border: false,
        gg_frame_blending: false,
        gg_use_sms_resolution: false,
        fm_sound_unit_enabled: true,
        z80_divider: std::num::NonZeroU32::new(15).unwrap(),
    }
}

impl Emulator {
    pub fn open(path: &Path) -> Result<Self> {
        Self::from_rom(Rom::open(path)?)
    }
    pub fn from_rom(rom: Rom) -> Result<Self> {
        ensure!(
            !rom.info.region_code.is_some_and(|r| matches!(r, 5..=7)),
            "Game Gear cartridge is outside SMS baseline"
        );
        // Restrict runtime to unambiguous backing sizes; source rebuilding accepts all valid image sizes.
        ensure!(
            rom.info.rom_size.is_power_of_two(),
            "debugger currently requires power-of-two ROM size"
        );
        let mut saves = Saves::default();
        let mut core = SmsEmulator::create(
            Some(rom.data().to_vec()),
            None,
            SmsGgHardware::MasterSystem,
            config(),
            &mut saves,
        );
        core.update_audio_output_frequency(48000);
        Ok(Self {
            core,
            rom,
            inputs: Default::default(),
            trace: Default::default(),
            output: Default::default(),
            audio: Default::default(),
            saves,
            selected_cpu: Cpu::Main,
            cycles: 0,
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
    pub fn bank_context(&self) -> Context {
        let mut context = Context::default();
        for (slot, bank) in context.rom_banks.iter_mut().enumerate() {
            if let Some((_, Some(("rom", offset)))) =
                self.core.debug_peek((slot as u16) * 16384 + 1024)
            {
                *bank = Some((offset / 16384) as u32);
            }
        }
        context.ram_enabled = Some(
            self.core
                .debug_peek(0xa000)
                .is_some_and(|(_, b)| b.is_some_and(|b| b.0 == "cart_ram")),
        );
        context.ram_bank = self
            .core
            .debug_peek(0xa000)
            .and_then(|(_, b)| b)
            .filter(|b| b.0 == "cart_ram")
            .map(|b| (b.1 / 16384) as u32);
        context.cartridge_enabled = Some(self.core.debug_enabled());
        context
    }
    fn tick(&mut self) -> Result<()> {
        let pc = self.core.debug_cpu().pc();
        ensure!(
            (0..64u16).any(|n| self
                .core
                .debug_peek(pc.wrapping_add(n))
                .is_some_and(|(v, _)| !matches!(v, 0xdd | 0xfd))),
            "Z80 prefix chain exceeds bounded instruction limit"
        );
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
            &self.inputs,
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
            let mut location = e.physical.map(|b| self.rom.binding(b)).unwrap_or_else(|| {
                address(
                    if e.origin == "io" { "io" } else { "cpu" },
                    e.address.into(),
                )
            });
            if self.artificial {
                location.context.insert("artificial".into(), "true".into());
            }
            let mut ctx = json!({"cpu":"main","cpu_address":e.address,"origin":e.origin,"artificial":self.artificial,"cpu_context":before,"clock":"cpu_cycles"});
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
        ensure!(matches!(port, 1 | 2), "controller port must be 1 or 2");
        let mut input = SmsInputs::default();
        for button in buttons {
            match button.to_ascii_lowercase().as_str() {
                "1" | "button1" => input.p1.button1 = true,
                "2" | "button2" => input.p1.button2 = true,
                "pause" => input.pause = true,
                "up" => input.p1.up = true,
                "down" => input.p1.down = true,
                "left" => input.p1.left = true,
                "right" => input.p1.right = true,
                _ => anyhow::bail!("unknown SMS button {button}"),
            }
        }
        self.inputs.pause = input.pause;
        if port == 1 {
            self.inputs.p1 = input.p1;
        } else {
            self.inputs.p2 = input.p1;
        }
        Ok(())
    }
    pub fn set_registers(&mut self, _: Cpu, values: &Value) -> Result<()> {
        let mut r = self.core.debug_cpu().debug_registers();
        let names = [
            "af",
            "bc",
            "de",
            "hl",
            "ix",
            "iy",
            "sp",
            "pc",
            "af_shadow",
            "bc_shadow",
            "de_shadow",
            "hl_shadow",
            "i",
            "r",
            "iff",
            "im",
        ];
        for (key, value) in values
            .as_object()
            .ok_or_else(|| anyhow::anyhow!("registers must be object"))?
        {
            let index = names
                .iter()
                .position(|n| *n == key)
                .ok_or_else(|| anyhow::anyhow!("unknown register {key}"))?;
            let v = u16::try_from(
                value
                    .as_u64()
                    .ok_or_else(|| anyhow::anyhow!("unsigned register required"))?,
            )?;
            ensure!(index < 12 || v <= 255, "8-bit register overflow");
            ensure!(index != 14 || v <= 3, "IFF mask exceeds 3");
            ensure!(index != 15 || v <= 2, "interrupt mode exceeds 2");
            r[index] = v;
        }
        self.core.debug_cpu_mut().debug_set_registers(r);
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
        ensure!(bytes.len() <= 4096, "write range exceeds limit");
        if a.space == "io" {
            ensure!(end <= 65536, "I/O range");
            for (i, &value) in bytes.iter().enumerate() {
                self.core.debug_io_write((start + i) as u16, value);
            }
            self.artificial = true;
            self.previous = None;
            return Ok(());
        }
        let (space, offset) = if a.space == "cpu" {
            ensure!(
                start >= 0xc000 && end <= 65536,
                "CPU write must address system RAM"
            );
            ("ram", start & 8191)
        } else {
            (a.space.as_str(), start)
        };
        ensure!(
            self.core.debug_write(space, offset, bytes),
            "invalid RAM backing write"
        );
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
    type Input = SmsInputs;
    type Instrumentation = TraceConfig;
    fn load(path: &Path, _: ()) -> Result<Self, ApiError> {
        Self::open(path).map_err(invalid)
    }
    fn capabilities(&self) -> ExecutionCapabilities {
        ExecutionCapabilities {
            platform: "sms".into(),
            clock: "cpu_cycles".into(),
            units: vec![StepUnit::Operation, StepUnit::Cycle, StepUnit::Frame],
            address_spaces: [
                ("rom", self.rom.bytes.len() as u64),
                ("cpu", 65536),
                ("io", 65536),
                ("ram", 8192),
                ("vram", 16384),
                ("cram", 32),
                ("cart_ram", 32768),
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
        let r = cpu.debug_registers();
        let mut video_registers = Vec::new();
        self.core.dump_vdp_registers(|group, fields| {
            video_registers.push(json!({"group":group,"fields":fields}))
        });
        State {
            artificial: self.artificial,
            cpu_cycles: self.cycles,
            frames: self.frames,
            main: CpuState {
                pc: r[7],
                af: r[0],
                bc: r[1],
                de: r[2],
                hl: r[3],
                ix: r[4],
                iy: r[5],
                sp: r[6],
                af_shadow: r[8],
                bc_shadow: r[9],
                de_shadow: r[10],
                hl_shadow: r[11],
                i: r[12],
                r: r[13],
                iff: r[14],
                im: r[15],
                halted: cpu.debug_halted(),
            },
            context: self.bank_context(),
            video: json!({"scanline":self.core.debug_position().0,"dot":self.core.debug_position().1,"registers":video_registers}),
        }
    }
    fn execute(&mut self, r: StepRequest) -> Result<State, ApiError> {
        if r.max_operations > MAX_TICKS || !(1..=300000).contains(&r.timeout_ms) {
            return Err(invalid("execution budget exceeds limits"));
        }
        let initial = self.state();
        let start = Instant::now();
        for tick in 0..=r.max_operations {
            let progress = match r.unit {
                StepUnit::Cycle => self.cycles - initial.cpu_cycles,
                StepUnit::Frame => self.frames - initial.frames,
                StepUnit::Operation => tick,
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
        if let Some(ram) = self.core.debug_storage("cart_ram") {
            self.saves.0.insert("sav".into(), ram.to_vec());
        }
        self.core = SmsEmulator::create(
            Some(self.rom.data().to_vec()),
            None,
            SmsGgHardware::MasterSystem,
            config(),
            &mut self.saves,
        );
        self.core.update_audio_output_frequency(48000);
        self.cycles = 0;
        self.frames = 0;
        self.inputs = Default::default();
        self.previous = None;
        self.output = Default::default();
        self.audio = Default::default();
        Ok(())
    }
    fn input(&mut self, input: SmsInputs) -> Result<(), ApiError> {
        self.inputs = input;
        Ok(())
    }
    fn snapshot(&self) -> Result<Snapshot, ApiError> {
        let core =
            bincode::encode_to_vec(&self.core, bincode::config::standard()).map_err(invalid)?;
        Ok(Snapshot {
            format: "romatlas-sms-state".into(),
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
        if s.format != "romatlas-sms-state"
            || s.version != 1
            || s.rom_sha256 != self.rom.info.sha256
            || s.core_sha256 != format!("{:x}", Sha256::digest(&s.core))
        {
            return Err(invalid("snapshot format/identity/checksum mismatch"));
        }
        let (mut core, n): (SmsEmulator, usize) = bincode::decode_from_slice(
            &s.core,
            bincode::config::standard().with_limit::<16777216>(),
        )
        .map_err(invalid)?;
        let (inputs, m): (SmsInputs, usize) =
            bincode::decode_from_slice(&s.input, bincode::config::standard().with_limit::<1024>())
                .map_err(invalid)?;
        if n != s.core.len() || m != s.input.len() {
            return Err(invalid("trailing snapshot data"));
        }
        if core.debug_mapper_name() != self.core.debug_mapper_name() {
            return Err(invalid("snapshot mapper configuration mismatch"));
        }
        // Check structural storage sizes before moving immutable ROM into candidate.
        for space in ["ram", "cart_ram", "vram", "cram"] {
            if core.debug_storage(space).map(|s| s.len())
                != self.core.debug_storage(space).map(|s| s.len())
            {
                return Err(invalid("snapshot storage configuration mismatch"));
            }
        }
        core.take_rom_from(&mut self.core);
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
        Ok(self
            .core
            .debug_peek(a.offset as u16)
            .and_then(|(_, b)| b)
            .map(|b| self.rom.binding(b)))
    }
    fn inspect(&mut self, a: &Address, n: usize) -> Result<Vec<u8>, ApiError> {
        if a.space == "io" {
            if n > 4096 || a.offset.checked_add(n as u64).is_none_or(|end| end > 65536) {
                return Err(invalid("I/O range"));
            }
            return Ok((0..n)
                .map(|i| self.core.debug_io_read((a.offset + i as u64) as u16))
                .collect());
        }
        if n > 65536 || a.offset.checked_add(n as u64).is_none() {
            return Err(invalid("peek range exceeds limits"));
        }
        if a.space == "cpu" {
            if a.offset + n as u64 > 65536 {
                return Err(invalid("CPU range"));
            }
            return (0..n)
                .map(|i| {
                    self.core
                        .debug_peek((a.offset + i as u64) as u16)
                        .map(|v| v.0)
                        .ok_or_else(|| invalid("side-effect-free I/O peek unsupported"))
                })
                .collect();
        }
        let bytes = if a.space == "rom" {
            Some(self.rom.bytes.as_slice())
        } else {
            self.core.debug_storage(&a.space)
        }
        .ok_or_else(|| invalid("unsupported backing storage"))?;
        bytes
            .get(a.offset as usize..a.offset as usize + n)
            .map(Vec::from)
            .ok_or_else(|| invalid("backing range outside storage"))
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
