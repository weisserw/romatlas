//! Bounded Genesis execution with actual bus observations and transactional snapshots.
use crate::{
    Rom,
    decode::{Context, Cpu},
    image::address,
};
use anyhow::{Result, ensure};
use bincode::{Decode, Encode};
use genesis_config::GenesisInputs;
use genesis_core::{GenesisEmulator, GenesisEmulatorConfig};
use jgenesis_common::{
    frontend::{
        AudioOutput, Color, EmulatorTrait, FrameSize, RenderFrameOptions, Renderer, SaveWriter,
        TickEffect,
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
pub struct State {
    pub artificial: bool,
    pub cpu_cycles: u64,
    pub frames: u64,
    pub main: Value,
    pub z80: Value,
    pub context: Context,
    pub video: Value,
    pub audio: Value,
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
    core: GenesisEmulator,
    pub rom: Rom,
    inputs: GenesisInputs,
    pub trace: Trace,
    pub output: Output,
    pub audio: Audio,
    saves: Saves,
    pub selected_cpu: Cpu,
    cycles: u64,
    frames: u64,
    previous: BTreeMap<String, Address>,
    artificial: bool,
}
fn config(_: &Rom) -> GenesisEmulatorConfig {
    GenesisEmulatorConfig {
        p1_controller_type: genesis_config::GenesisControllerType::SixButton,
        p2_controller_type: genesis_config::GenesisControllerType::SixButton,
        forced_timing_mode: None,
        forced_region: None,
        aspect_ratio: Default::default(),
        force_square_pixels_in_h40: false,
        adjust_aspect_ratio_in_2x_resolution: false,
        anamorphic_widescreen: false,
        remove_sprite_limits: false,
        m68k_clock_divider: 7,
        non_linear_color_scale: false,
        deinterlace: true,
        render_vertical_border: false,
        render_horizontal_border: false,
        plane_a_enabled: true,
        plane_b_enabled: true,
        sprites_enabled: true,
        window_enabled: true,
        backdrop_enabled: true,
        quantize_ym2612_output: true,
        emulate_ym2612_ladder_effect: true,
        opn2_busy_behavior: Default::default(),
        genesis_lpf_enabled: false,
        genesis_lpf_cutoff: 3390,
        ym2612_2nd_lpf_enabled: false,
        ym2612_2nd_lpf_cutoff: 7973,
        ym2612_channels_enabled: [true; 6],
        ym2612_enabled: true,
        psg_enabled: true,
        ym2612_volume_adjustment_db: 0.0,
        psg_volume_adjustment_db: 0.0,
    }
}

impl Emulator {
    pub fn open(path: &Path) -> Result<Self> {
        Self::from_rom(Rom::open(path)?)
    }
    pub fn from_rom(rom: Rom) -> Result<Self> {
        ensure!(
            rom.info.rom_size <= 0x800000,
            "debugger supports cartridges up to 8 MiB"
        );
        if let (Some(start), Some(end), Some(kind)) =
            (rom.info.ram_start, rom.info.ram_end, rom.info.ram_type)
        {
            ensure!(
                end >= start
                    && end - start < 0x100000
                    && matches!(kind, 0xa0 | 0xb0 | 0xb8 | 0xe0 | 0xf0 | 0xf8),
                "unsupported/invalid SRAM header"
            );
            let n = (end - start) / if kind & 0x18 == 0 { 1 } else { 2 } + 1;
            ensure!(n.is_power_of_two(), "SRAM size must be a power of two");
        }
        ensure!(
            !rom.info.system.contains("32X"),
            "32X is outside the Genesis cartridge baseline"
        );
        let mut saves = Saves::default();
        let mut core = GenesisEmulator::create_exact(rom.data().to_vec(), config(&rom), &mut saves);
        ensure!(
            matches!(core.debug_mapper_name(), "Basic" | "SSF"),
            "unsupported Genesis mapper {}",
            core.debug_mapper_name()
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
            previous: BTreeMap::new(),
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
        let (banks, ram, z80) = self.core.debug_context();
        Context {
            rom_banks: banks.map(Some),
            ram_enabled: Some(ram),
            z80_bank: Some(z80),
        }
    }
    fn tick(&mut self) -> Result<()> {
        let pc = self.core.debug_z80().pc();
        ensure!(
            (0..64u16).any(|n| self
                .core
                .debug_z80_peek(pc.wrapping_add(n))
                .is_none_or(|(v, _)| !matches!(v, 0xdd | 0xfd))),
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
            if e.origin == "z80_bridge" {
                continue;
            }
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
                    if e.origin == "io" {
                        "io"
                    } else if e.cpu == "z80" {
                        "z80_cpu"
                    } else {
                        "cpu"
                    },
                    e.address.into(),
                )
            });
            if self.artificial {
                location.context.insert("artificial".into(), "true".into());
            }
            let mut ctx = json!({"cpu":e.cpu,"cpu_address":e.address,"origin":e.origin,"artificial":self.artificial,"cpu_context":e.platform_context.map(|c|Context{rom_banks:std::array::from_fn(|i|Some(c[i])),ram_enabled:Some(c[8]!=0),z80_bank:Some(c[9])}).or(before),"clock":"master_cycles"});

            if kind == EventKind::Interrupt {
                self.previous.remove(e.cpu);
            }
            if kind == EventKind::Instruction {
                let mut traversal = location.clone();
                traversal.context.insert("cpu".into(), e.cpu.into());
                traversal
                    .context
                    .insert("cpu_address".into(), e.address.to_string());
                traversal.context.insert(
                    "cpu_context".into(),
                    serde_json::to_string(&ctx["cpu_context"])?,
                );
                ctx["traversal_source"] = serde_json::to_value(&traversal)?;
                if let Some(previous) = self.previous.remove(e.cpu) {
                    let source = address(&previous.space, previous.offset);
                    self.trace.capture(TraceEvent{kind:EventKind::ControlFlow,cycle:e.cycle,address:source,target:Some(location.clone()),value:None,context:json!({"traversal_source":previous,"traversal_target":traversal,"observed":true,"artificial":self.artificial})});
                }
                self.previous.insert(e.cpu.into(), traversal);
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
        let mut input = GenesisInputs::default();
        for button in buttons {
            match button.to_ascii_lowercase().as_str() {
                "a" => input.p1.a = true,
                "b" => input.p1.b = true,
                "c" => input.p1.c = true,
                "x" => input.p1.x = true,
                "y" => input.p1.y = true,
                "z" => input.p1.z = true,
                "mode" => input.p1.mode = true,
                "start" => input.p1.start = true,
                "up" => input.p1.up = true,
                "down" => input.p1.down = true,
                "left" => input.p1.left = true,
                "right" => input.p1.right = true,
                _ => anyhow::bail!("unknown {} button {button}", self.rom.info.platform),
            }
        }
        if port == 1 {
            self.inputs.p1 = input.p1;
        } else {
            self.inputs.p2 = input.p1;
        }
        Ok(())
    }
    pub fn set_registers(&mut self, cpu: Cpu, values: &Value) -> Result<()> {
        let values = values
            .as_object()
            .ok_or_else(|| anyhow::anyhow!("registers must be object"))?;
        if cpu == Cpu::Z80 {
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
            let mut regs = self.core.debug_z80().debug_registers();
            for (k, v) in values {
                let i = names
                    .iter()
                    .position(|n| *n == k)
                    .ok_or_else(|| anyhow::anyhow!("unknown register {k}"))?;
                let v = u16::try_from(
                    v.as_u64()
                        .ok_or_else(|| anyhow::anyhow!("unsigned register required"))?,
                )?;
                ensure!(i < 12 || v <= 255, "8-bit overflow");
                ensure!(i != 14 || v <= 3, "IFF mask");
                ensure!(i != 15 || v <= 2, "interrupt mode");
                regs[i] = v;
            }
            self.core.debug_z80_mut().debug_set_registers(regs);
        } else {
            let cpu = self.core.debug_cpu();
            let mut data = cpu.data_registers();
            let mut addr = cpu.address_registers();
            let (mut usp, mut ssp, mut sr, mut pc) = (
                cpu.user_stack_pointer(),
                cpu.supervisor_stack_pointer(),
                cpu.status_register(),
                cpu.pc(),
            );
            for (k, v) in values {
                let v = u32::try_from(
                    v.as_u64()
                        .ok_or_else(|| anyhow::anyhow!("unsigned register required"))?,
                )?;
                match k.as_str() {
                    "pc" => {
                        ensure!(v <= 0xffffff && v % 2 == 0, "PC must be aligned and 24-bit");
                        pc = v
                    }
                    "sr" => sr = u16::try_from(v)?,
                    "usp" => usp = v,
                    "ssp" => ssp = v,
                    _ => {
                        let i = k
                            .get(1..)
                            .and_then(|s| s.parse::<usize>().ok())
                            .ok_or_else(|| anyhow::anyhow!("unknown register {k}"))?;
                        if k.starts_with('d') && i < 8 {
                            data[i] = v
                        } else if k.starts_with('a') && i < 7 {
                            addr[i] = v
                        } else {
                            anyhow::bail!("unknown register {k}")
                        }
                    }
                }
            }
            self.core.debug_set_registers(data, addr, usp, ssp, sr, pc);
        }
        self.artificial = true;
        self.previous.clear();
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
            ensure!(end <= 0x1000000, "I/O range");
            for (i, &value) in bytes.iter().enumerate() {
                self.core.debug_io_write((start + i) as u32, value);
            }
            self.artificial = true;
            self.previous.clear();
            return Ok(());
        }
        let (space, offset) = if a.space == "cpu" {
            ensure!(
                start >= 0xe00000 && end <= 0x1000000 && (start & 65535) + bytes.len() <= 65536,
                "CPU write must address system RAM"
            );
            ("ram", start & 65535)
        } else {
            (a.space.as_str(), start)
        };
        ensure!(
            self.core.debug_write(space, offset, bytes),
            "invalid RAM backing write"
        );
        self.artificial = true;
        self.previous.clear();
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
        self.previous.clear();
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
    type Input = GenesisInputs;
    type Instrumentation = TraceConfig;
    fn load(path: &Path, _: ()) -> Result<Self, ApiError> {
        Self::open(path).map_err(invalid)
    }
    fn capabilities(&self) -> ExecutionCapabilities {
        ExecutionCapabilities {
            platform: self.rom.info.platform.clone(),
            clock: "master_cycles".into(),
            units: vec![StepUnit::Operation, StepUnit::Cycle, StepUnit::Frame],
            address_spaces: [
                ("rom", self.rom.bytes.len() as u64),
                ("cpu", 0x1000000),
                ("z80_cpu", 65536),
                ("io", 0x1000000),
                ("ram", 65536),
                ("zram", 8192),
                ("vram", 65536),
                ("cram", 128),
                ("vsram", 80),
                (
                    "cart_ram",
                    self.core.debug_storage("cart_ram").map_or(0, |b| b.len()) as u64,
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
        let c = self.core.debug_cpu();
        let r = self.core.debug_z80().debug_registers();
        let (fm_writes, fm_status, (tones, attenuation, noise, lfsr)) =
            self.core.romatlas_sound_state();
        State {
            artificial: self.artificial,
            cpu_cycles: self.cycles,
            frames: self.frames,
            main: json!({"pc":c.pc(),"data":c.data_registers(),"address":c.address_registers(),"usp":c.user_stack_pointer(),"ssp":c.supervisor_stack_pointer(),"sr":c.status_register(),"next_opcode":c.next_opcode()}),
            z80: json!({"pc":r[7],"registers":r,"halted":self.core.debug_z80().debug_halted()}),
            context: self.bank_context(),
            audio: json!({"ym2612":{"last_writes":fm_writes,"status":fm_status},"psg":{"tones":tones,"attenuation":attenuation,"noise":noise,"lfsr":lfsr}}),
            video: json!({"timing_mode":format!("{:?}",self.core.timing_mode()),"nominal_fps":self.core.target_fps(),"scanline":self.core.debug_position().0,"master_clock":self.core.debug_position().1,"registers":self.core.romatlas_video_registers()}),
        }
    }
    fn execute(&mut self, r: StepRequest) -> Result<State, ApiError> {
        if r.max_operations > MAX_TICKS || !(1..=300000).contains(&r.timeout_ms) {
            return Err(invalid("execution budget exceeds limits"));
        }
        let initial = self.state();
        let fetch_index = usize::from(self.selected_cpu == Cpu::Z80);
        let initial_fetches = probe::fetches()[fetch_index];
        let start = Instant::now();
        for tick in 0..=r.max_operations {
            let progress = match r.unit {
                StepUnit::Cycle => self.cycles - initial.cpu_cycles,
                StepUnit::Frame => self.frames - initial.frames,
                StepUnit::Operation => {
                    probe::fetches()[fetch_index].saturating_sub(initial_fetches)
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
        if let Some(ram) = self.core.debug_storage("cart_ram") {
            self.saves.0.insert("sav".into(), ram.to_vec());
        }
        self.core = GenesisEmulator::create_exact(
            self.rom.data().to_vec(),
            config(&self.rom),
            &mut self.saves,
        );
        self.core.update_audio_output_frequency(48000);
        self.cycles = 0;
        self.frames = 0;
        self.inputs = Default::default();
        self.previous.clear();
        self.output = Default::default();
        self.audio = Default::default();
        Ok(())
    }
    fn input(&mut self, input: GenesisInputs) -> Result<(), ApiError> {
        self.inputs = input;
        Ok(())
    }
    fn snapshot(&self) -> Result<Snapshot, ApiError> {
        let core =
            bincode::encode_to_vec(&self.core, bincode::config::standard()).map_err(invalid)?;
        Ok(Snapshot {
            format: format!("romatlas-{}-state", self.rom.info.platform),
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
        if s.format != format!("romatlas-{}-state", self.rom.info.platform)
            || s.version != 1
            || s.rom_sha256 != self.rom.info.sha256
            || s.core_sha256 != format!("{:x}", Sha256::digest(&s.core))
        {
            return Err(invalid("snapshot format/identity/checksum mismatch"));
        }
        let (mut core, n): (GenesisEmulator, usize) = bincode::decode_from_slice(
            &s.core,
            bincode::config::standard().with_limit::<16777216>(),
        )
        .map_err(invalid)?;
        let (inputs, m): (GenesisInputs, usize) =
            bincode::decode_from_slice(&s.input, bincode::config::standard().with_limit::<1024>())
                .map_err(invalid)?;
        if n != s.core.len() || m != s.input.len() {
            return Err(invalid("trailing snapshot data"));
        }
        if core.debug_mapper_name() != self.core.debug_mapper_name() {
            return Err(invalid("snapshot mapper configuration mismatch"));
        }
        // Check structural storage sizes before moving immutable ROM into candidate.
        for space in ["ram", "zram", "cart_ram", "vram", "cram", "vsram"] {
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
        self.previous.clear();
        self.output = Default::default();
        self.audio = Default::default();
        Ok(())
    }
    fn translate(&self, a: &Address) -> Result<Option<Address>, ApiError> {
        let binding = if a.space == "cpu" && a.offset < 0x1000000 {
            self.core.debug_peek(a.offset as u32).and_then(|(_, b)| b)
        } else if a.space == "z80_cpu" && a.offset < 65536 {
            self.core
                .debug_z80_peek(a.offset as u16)
                .and_then(|(_, b)| b)
        } else {
            return Err(invalid("expected 24-bit cpu or 16-bit z80_cpu address"));
        };
        Ok(binding.map(|b| self.rom.binding(b)))
    }
    fn inspect(&mut self, a: &Address, n: usize) -> Result<Vec<u8>, ApiError> {
        if n > 65536 || a.offset.checked_add(n as u64).is_none() {
            return Err(invalid("peek range exceeds limits"));
        }
        if matches!(a.space.as_str(), "cpu" | "z80_cpu" | "io") {
            let end = if a.space == "z80_cpu" {
                65536
            } else {
                0x1000000
            };
            if a.offset + n as u64 > end {
                return Err(invalid("CPU range"));
            }
            return (0..n)
                .map(|i| {
                    let at = a.offset + i as u64;
                    if a.space == "io" {
                        Ok(self.core.debug_io_read(at as u32))
                    } else {
                        let peek = if a.space == "cpu" {
                            self.core.debug_peek(at as u32)
                        } else {
                            self.core.debug_z80_peek(at as u16)
                        };
                        peek.map(|p| p.0)
                            .ok_or_else(|| invalid("unmapped or I/O address; use detached io peek"))
                    }
                })
                .collect();
        }
        let bytes = if a.space == "rom" {
            self.rom.bytes.clone()
        } else {
            self.core
                .debug_storage(&a.space)
                .ok_or_else(|| invalid("unsupported backing storage"))?
        };
        bytes
            .get(a.offset as usize..a.offset as usize + n)
            .map(Vec::from)
            .ok_or_else(|| invalid("backing range outside storage"))
    }
    fn configure_instrumentation(&mut self, c: TraceConfig) -> Result<(), ApiError> {
        self.previous.clear();
        self.trace.configure(c).map_err(invalid)
    }
    fn events(&self, offset: usize, limit: usize) -> Result<Vec<TraceEvent>, ApiError> {
        if limit > 1000 {
            return Err(invalid("trace limit exceeds 1000"));
        }
        Ok(self.trace.query(offset, limit))
    }
}
