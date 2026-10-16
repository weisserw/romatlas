//! Headless Mizu adapter. Never loads or writes battery saves implicitly.
use crate::Rom;
use anyhow::{Result, ensure};
use mizu_core::{GameBoy, GameBoyConfig, JoypadButton};
use serde::{Deserialize, Serialize};
use std::{
    io::Cursor,
    path::{Path, PathBuf},
    time::{Duration, Instant},
};
#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct Registers {
    pub a: u8,
    pub f: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub h: u8,
    pub l: u8,
    pub sp: u16,
    pub pc: u16,
}
#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct State {
    pub registers: Registers,
    pub ppu_cycles: u64,
    pub frames: u64,
    pub operations: u64,
    pub fixed_bank: usize,
    pub switch_bank: usize,
    pub model: String,
    pub wram_bank: u8,
    pub vram_bank: u8,
    pub double_speed: bool,
}
#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Snapshot {
    bytes: Vec<u8>,
    cycles: u64,
    operations: u64,
}
#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct SnapshotFile {
    format: String,
    image_sha256: String,
    model: String,
    snapshot: Snapshot,
}
pub struct Emulator {
    gb: GameBoy,
    path: PathBuf,
    hash: String,
    pub cycles: u64,
    pub operations: u64,
    pub trace: romatlas_analysis::Trace,
}
impl Emulator {
    pub fn open(path: &Path) -> Result<Self> {
        Self::open_model(path, "auto")
    }
    pub fn open_model(path: &Path, model: &str) -> Result<Self> {
        let rom = Rom::open(path)?;
        ensure!(
            matches!(rom.info.cartridge_type, 0 | 1..=3 | 8 | 9 | 0x0f..=0x13 | 0x19..=0x1e),
            "unsupported emulator cartridge feature: type {:#04x}; only ROM-only/MBC1/MBC3/MBC5 tested",
            rom.info.cartridge_type
        );
        let is_dmg = match model {
            "auto" => rom.info.cgb_flag & 0x80 == 0,
            "dmg" => true,
            "cgb" => false,
            _ => anyhow::bail!("model must be auto, dmg or cgb"),
        };
        ensure!(
            !(is_dmg && rom.info.cgb_flag & 0xc0 == 0xc0),
            "CGB-only cartridge cannot run in DMG mode"
        );
        ensure!(
            rom.info.header_checksum_valid,
            "emulation requires valid header checksum"
        );
        if rom.info.controller == "MBC3" {
            ensure!(
                rom.info.banks <= 128 && rom.info.ram_size <= 3,
                "unsupported MBC30 or oversized MBC3 cartridge"
            );
        }
        let gb = GameBoy::builder(path)
            .config(GameBoyConfig { is_dmg })
            .save_on_shutdown(false)
            .build()?;
        let loaded_hash: String = gb
            .cartridge_image_hash()
            .iter()
            .map(|b| format!("{b:02x}"))
            .collect();
        ensure!(
            loaded_hash == rom.info.sha256,
            "ROM changed during emulator loading"
        );
        Ok(Self {
            gb,
            path: path.to_owned(),
            hash: rom.info.sha256,
            cycles: 0,
            operations: 0,
            trace: Default::default(),
        })
    }
    pub(crate) fn storage_shape(&self) -> (u64, u64) {
        self.gb.cartridge_storage_shape()
    }
    pub fn image_sha256(&self) -> &str {
        &self.hash
    }
    pub(crate) fn mapped_address(&self, cpu: u16) -> Option<romatlas_core::Address> {
        self.gb.access_location(cpu).map(crate::tracing::location)
    }
    pub fn reset(&mut self) -> Result<()> {
        let fresh = Self::open_model(
            &self.path,
            if self.gb.hardware_state().0 {
                "dmg"
            } else {
                "cgb"
            },
        )?;
        ensure!(fresh.hash == self.hash, "ROM changed since emulator start");
        *self = fresh;
        Ok(())
    }
    pub fn state(&self) -> State {
        let r = self.gb.cpu_registers();
        let (dmg, wram_bank, vram_bank, double_speed) = self.gb.hardware_state();
        State {
            registers: Registers {
                a: r.a,
                f: r.f,
                b: r.b,
                c: r.c,
                d: r.d,
                e: r.e,
                h: r.h,
                l: r.l,
                sp: r.sp,
                pc: r.pc,
            },
            ppu_cycles: self.cycles,
            frames: self.gb.frame_count(),
            model: if dmg { "dmg".into() } else { "cgb".into() },
            wram_bank,
            vram_bank,
            double_speed,
            operations: self.operations,
            fixed_bank: self.gb.physical_rom_offset(0).unwrap() / 0x4000,
            switch_bank: self.gb.physical_rom_offset(0x4000).unwrap() / 0x4000,
        }
    }
    pub fn configure_trace(&mut self, config: romatlas_analysis::TraceConfig) -> Result<()> {
        self.trace.configure(config).map_err(anyhow::Error::msg)?;
        self.gb.configure_trace(self.trace.config.enabled);
        Ok(())
    }
    /// Hardware-suite runner: recognize the documented Mooneye register protocol.
    pub fn mooneye(&mut self, max_operations: u64, timeout_ms: u64) -> Result<serde_json::Value> {
        ensure!(
            (1..=10_000_000).contains(&max_operations),
            "test operation budget must be 1..10000000"
        );
        ensure!(
            (1..=30000).contains(&timeout_ms),
            "test timeout must be 1..30000 ms"
        );
        let start = Instant::now();
        for _ in 0..max_operations {
            if start.elapsed() >= Duration::from_millis(timeout_ms) {
                return Ok(serde_json::json!({"status":"timeout","state":self.state()}));
            }
            let outcome = self.step()?;
            if outcome.starts_with("Breakpoint") {
                let r = self.state().registers;
                let values = [r.b, r.c, r.d, r.e, r.h, r.l];
                if values == [3, 5, 8, 13, 21, 34] {
                    return Ok(serde_json::json!({"status":"passed","state":self.state()}));
                }
                if values == [0x42; 6] {
                    return Ok(serde_json::json!({"status":"failed","state":self.state()}));
                }
            }
        }
        Ok(serde_json::json!({"status":"operation_limit","state":self.state()}))
    }
    pub fn step(&mut self) -> Result<String> {
        if !self.trace.config.enabled {
            let (state, cycles) = self.gb.step_operation();
            self.cycles = self
                .cycles
                .checked_add(cycles as u64)
                .ok_or_else(|| anyhow::anyhow!("cycle counter overflow"))?;
            self.operations += 1;
            let state = state.map_err(|e| anyhow::anyhow!("CPU execution error: {e:?}"))?;
            return Ok(format!("{state:?}"));
        }
        use romatlas_core::{EventKind, TraceEvent};
        let before = self.state();
        let before_address = self
            .gb
            .access_location(before.registers.pc)
            .map(crate::tracing::location)
            .unwrap_or_else(|| crate::tracing::address(before.registers.pc, None));
        let start = self.cycles;
        let (outcome, cycles) = self.gb.step_operation();
        let events = self.gb.take_bus_events();
        let fetched: Vec<_> = events
            .iter()
            .filter(|e| e.kind == 0 || e.kind == 1)
            .map(|e| e.value)
            .take(3)
            .collect();
        let mut semantic =
            crate::edges::semantics(&fetched, before.registers.pc, &before.registers);
        // HALT bug repeats the opcode address for the first operand fetch.
        // Relative targets must use the actual operand-fetch PC.
        let fetch_addresses: Vec<_> = events
            .iter()
            .filter(|e| e.kind == 0 || e.kind == 1)
            .map(|e| e.address)
            .take(3)
            .collect();
        if fetch_addresses.len() >= 2 && fetch_addresses[0] == fetch_addresses[1] {
            for field in ["fallthrough_cpu", "encoded_cpu_target"] {
                if field == "encoded_cpu_target"
                    && !matches!(fetched[0], 0x18 | 0x20 | 0x28 | 0x30 | 0x38)
                {
                    continue;
                }
                if let Some(value) = semantic[field].as_u64() {
                    semantic[field] = serde_json::json!((value as u16).wrapping_sub(1));
                }
            }
        }
        let opcode = events.iter().find(|e| e.kind == 0).map(|e| {
            e.location
                .clone()
                .map(crate::tracing::location)
                .unwrap_or_else(|| crate::tracing::address(e.address, e.rom_offset))
        });
        for event in events {
            let mut event = crate::tracing::bus_event(event, start, &before.registers);
            if outcome.is_err() && event.kind == EventKind::Instruction {
                event.kind = EventKind::Fetch;
            }
            self.trace.capture(event);
        }

        self.cycles = self
            .cycles
            .checked_add(cycles as u64)
            .ok_or_else(|| anyhow::anyhow!("cycle counter overflow"))?;
        self.operations += 1;
        let state = outcome.map_err(|e| anyhow::anyhow!("CPU execution error: {e:?}"))?;
        if self.trace.config.enabled {
            let after = self.state();
            let target = self
                .gb
                .access_location(after.registers.pc)
                .map(crate::tracing::location)
                .unwrap_or_else(|| crate::tracing::address(after.registers.pc, None));
            if let Some(source) = opcode {
                let mut context = serde_json::json!({"state":format!("{state:?}"),"cpu_address":before.registers.pc,"cpu_target":after.registers.pc,"fixed_bank_before":before.fixed_bank,"switch_bank_before":before.switch_bank,"fixed_bank_after":after.fixed_bank,"switch_bank_after":after.switch_bank});
                context
                    .as_object_mut()
                    .unwrap()
                    .extend(semantic.as_object().unwrap().clone());
                if source.space == "cartridge-rom" {
                    context["traversal_source"] =
                        serde_json::to_value(crate::discovery::ContextDecoder::node(
                            source.clone(),
                            crate::discovery::CpuContext {
                                cpu_address: before.registers.pc,
                                fixed_bank: Some(before.fixed_bank),
                                switch_bank: Some(before.switch_bank),
                            },
                        ))?;
                    if target.space == "cartridge-rom" {
                        context["traversal_target"] =
                            serde_json::to_value(crate::discovery::ContextDecoder::node(
                                target.clone(),
                                crate::discovery::CpuContext {
                                    cpu_address: after.registers.pc,
                                    fixed_bank: Some(after.fixed_bank),
                                    switch_bank: Some(after.switch_bank),
                                },
                            ))?;
                    }
                }
                // Fetch bytes and pre-operation flags determine semantics; actual PC supplies the successor.
                self.trace.capture(TraceEvent {
                    kind: EventKind::ControlFlow,
                    cycle: self.cycles,
                    address: source,
                    target: Some(target.clone()),
                    value: None,
                    context,
                });
            } else if matches!(state, mizu_core::cpu::CpuState::RunningInterrupt(_)) {
                self.trace.capture(TraceEvent {
                    kind: EventKind::Interrupt,
                    cycle: self.cycles,
                    address: before_address,
                    target: Some(target),
                    value: None,
                    context: serde_json::json!({"state":format!("{state:?}")}),
                });
            }
        }
        Ok(format!("{state:?}"))
    }
    pub fn run(&mut self, operations: u64, timeout_ms: u64) -> Result<State> {
        ensure!(operations <= 1_000_000, "operation budget exceeds 1000000");
        ensure!(
            (1..=30_000).contains(&timeout_ms),
            "timeout must be 1..30000 ms"
        );
        let start = Instant::now();
        for _ in 0..operations {
            ensure!(
                start.elapsed() < Duration::from_millis(timeout_ms),
                "execution timeout; state retains completed operations"
            );
            self.step()?;
        }
        Ok(self.state())
    }
    /// Runs whole CPU operations until the requested PPU-cycle/frame budget is reached.
    /// May overshoot a cycle target by one operation; no frame is invented with LCD off.
    pub fn advance(
        &mut self,
        unit: &str,
        count: u64,
        max_operations: u64,
        timeout_ms: u64,
    ) -> Result<State> {
        ensure!(
            matches!(unit, "cycle" | "frame"),
            "unit must be cycle or frame"
        );
        ensure!(
            count <= if unit == "cycle" { 10_000_000 } else { 600 },
            "requested execution span exceeds budget"
        );
        ensure!(
            (1..=1_000_000).contains(&max_operations),
            "operation budget must be 1..1000000"
        );
        ensure!(
            (1..=30000).contains(&timeout_ms),
            "timeout must be 1..30000 ms"
        );
        let start = Instant::now();
        let initial = if unit == "cycle" {
            self.cycles
        } else {
            self.gb.frame_count()
        };
        let mut operations = 0;
        while (if unit == "cycle" {
            self.cycles
        } else {
            self.gb.frame_count()
        })
        .saturating_sub(initial)
            < count
        {
            ensure!(
                operations < max_operations,
                "operation budget exhausted before requested {unit} span; completed state retained"
            );
            ensure!(
                start.elapsed() < Duration::from_millis(timeout_ms),
                "execution timeout; completed state retained"
            );
            self.step()?;
            operations += 1;
        }
        Ok(self.state())
    }
    pub fn physical_memory(&self, space: &str, offset: usize, length: usize) -> Result<Vec<u8>> {
        ensure!(length <= 4096, "physical memory read exceeds 4096 bytes");
        self.gb
            .physical_memory(space, offset, length)
            .ok_or_else(|| {
                anyhow::anyhow!("unknown physical memory space or range outside storage")
            })
    }
    pub fn peek(&mut self, address: u16, length: usize) -> Result<Vec<u8>> {
        ensure!(
            length <= 4096 && address as usize + length <= 0x10000,
            "peek range exceeds address space or 4096-byte limit"
        );
        Ok(self.gb.debug_peek(address, length)?)
    }
    pub fn framebuffer(&self) -> serde_json::Value {
        serde_json::json!({"width":160,"height":144,"format":"rgb8",
            "pixels":self.gb.screen_buffer(),"frames":self.gb.frame_count(),
            "ppu_cycles":self.cycles,"operations":self.operations,"image_sha256":self.hash})
    }
    pub fn write_memory(&mut self, address: u16, bytes: &[u8]) -> Result<()> {
        ensure!(
            !bytes.is_empty() && bytes.len() <= 4096 && address as usize + bytes.len() <= 0x10000,
            "write range must contain 1..4096 bytes within CPU address space"
        );
        self.gb.debug_write(address, bytes);
        Ok(())
    }
    pub fn set_registers(
        &mut self,
        values: &serde_json::Map<String, serde_json::Value>,
    ) -> Result<State> {
        let mut r = self.gb.cpu_registers();
        ensure!(!values.is_empty(), "registers must not be empty");
        for (name, value) in values {
            let n = value
                .as_u64()
                .ok_or_else(|| anyhow::anyhow!("register {name} must be an unsigned integer"))?;
            match name.as_str() {
                "pc" | "sp" => {
                    ensure!(n <= 65535, "register {name} exceeds 16 bits");
                    if name == "pc" {
                        r.pc = n as u16;
                    } else {
                        r.sp = n as u16;
                    }
                }
                "a" | "f" | "b" | "c" | "d" | "e" | "h" | "l" => {
                    ensure!(n <= 255, "register {name} exceeds 8 bits");
                    ensure!(name != "f" || n & 15 == 0, "F low nibble must be zero");
                    let target = match name.as_str() {
                        "a" => &mut r.a,
                        "f" => &mut r.f,
                        "b" => &mut r.b,
                        "c" => &mut r.c,
                        "d" => &mut r.d,
                        "e" => &mut r.e,
                        "h" => &mut r.h,
                        _ => &mut r.l,
                    };
                    *target = n as u8;
                }
                _ => anyhow::bail!("unknown register {name}"),
            }
        }
        self.gb.debug_set_registers(r);
        Ok(self.state())
    }
    pub fn physical_rom_offset(&self, address: u16) -> Option<usize> {
        self.gb.physical_rom_offset(address)
    }
    pub fn input(&mut self, button: &str, pressed: bool) -> Result<()> {
        let key = match button {
            "a" => JoypadButton::A,
            "b" => JoypadButton::B,
            "start" => JoypadButton::Start,
            "select" => JoypadButton::Select,
            "up" => JoypadButton::Up,
            "down" => JoypadButton::Down,
            "left" => JoypadButton::Left,
            "right" => JoypadButton::Right,
            _ => anyhow::bail!("unknown button {button}"),
        };
        if pressed {
            self.gb.press_joypad(key)
        } else {
            self.gb.release_joypad(key)
        };
        Ok(())
    }
    pub fn save(&self) -> Result<Snapshot> {
        let mut bytes = Vec::new();
        self.gb.save_state(&mut bytes)?;
        Ok(Snapshot {
            bytes,
            cycles: self.cycles,
            operations: self.operations,
        })
    }
    pub fn export_snapshot(&self, path: &Path) -> Result<()> {
        use std::io::Write;
        let file = SnapshotFile {
            format: "romatlas-gb-snapshot-v1".into(),
            image_sha256: self.hash.clone(),
            model: self.state().model,
            snapshot: self.save()?,
        };
        let bytes = serde_json::to_vec(&file)?;
        let mut output = std::fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(path)?;
        let result = output.write_all(&bytes).and_then(|()| output.sync_all());
        if result.is_err() {
            let _ = std::fs::remove_file(path);
        }
        result?;
        Ok(())
    }
    pub fn import_snapshot(&mut self, path: &Path) -> Result<State> {
        let bytes = romatlas_map::read_limited(path, 64 * 1024 * 1024)?;
        let file: SnapshotFile = serde_json::from_slice(&bytes)?;
        ensure!(
            file.format == "romatlas-gb-snapshot-v1",
            "incompatible snapshot format"
        );
        ensure!(
            file.image_sha256 == self.hash,
            "snapshot ROM identity mismatch"
        );
        ensure!(file.model == self.state().model, "snapshot model mismatch");
        self.load(&file.snapshot)?;
        Ok(self.state())
    }
    pub fn load(&mut self, snapshot: &Snapshot) -> Result<()> {
        self.gb.load_state(Cursor::new(&snapshot.bytes))?;
        self.cycles = snapshot.cycles;
        self.operations = snapshot.operations;
        Ok(())
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    fn fixture(name: &str, program: &[u8], kind: u8, banks: usize) -> PathBuf {
        let path = std::env::temp_dir().join(format!("romatlas-{}-{name}.gb", std::process::id()));
        let mut bytes = vec![0; banks * 0x4000];
        bytes[0x100..0x103].copy_from_slice(&[0xc3, 0x50, 1]);
        bytes[0x150..0x150 + program.len()].copy_from_slice(program);
        bytes[0x147] = kind;
        bytes[0x148] = (banks / 2).ilog2() as u8;
        bytes[0x14d] = bytes[0x134..0x14d]
            .iter()
            .fold(0u8, |a, b| a.wrapping_sub(*b).wrapping_sub(1));
        std::fs::write(&path, bytes).unwrap();
        path
    }
    #[test]
    fn cartridge_filename_does_not_gate_execution() {
        let original = fixture("filename", &[0x18, 0xfe], 0, 2);
        let bytes = std::fs::read(&original).unwrap();
        for extension in ["rom", "", "GBC"] {
            let path = original.with_extension(extension);
            std::fs::write(&path, &bytes).unwrap();
            let mut emulator = Emulator::open(&path).unwrap();
            let initial = emulator.state();
            emulator.step().unwrap();
            emulator.reset().unwrap();
            assert_eq!(emulator.state(), initial);
            assert_eq!(std::fs::read(&path).unwrap(), bytes);
            drop(emulator);
            std::fs::remove_file(path).unwrap();
        }
        std::fs::remove_file(original).unwrap();
    }
    #[test]
    fn debugger_edits_validate_and_do_not_execute() {
        let path = fixture("debug-edits", &[0x00, 0x18, 0xfd], 0, 2);
        let mut e = Emulator::open(&path).unwrap();
        let original = e.state();
        let registers = serde_json::json!({"a": 42, "pc": 49152, "f": 176});
        e.set_registers(registers.as_object().unwrap()).unwrap();
        e.write_memory(0xc000, &[0x3e, 99]).unwrap();
        assert_eq!(e.peek(0xc000, 2).unwrap(), [0x3e, 99]);
        assert_eq!(e.state().registers.a, 42);
        assert_eq!(e.state().operations, original.operations);
        assert_eq!(e.state().ppu_cycles, original.ppu_cycles);
        let before = e.state();
        let invalid = serde_json::json!({"a": 5, "sp": 65536});
        assert!(e.set_registers(invalid.as_object().unwrap()).is_err());
        assert_eq!(e.state(), before);
        assert!(e.write_memory(0xffff, &[1, 2]).is_err());
        e.step().unwrap();
        assert_eq!(e.state().registers.a, 99);
        std::fs::remove_file(path).unwrap();
    }

    #[test]
    fn stepping_snapshot_and_inspection() {
        let path = fixture(
            "snapshot",
            &[0x3e, 0x42, 0xea, 0, 0xc0, 0x04, 0x18, 0xfd],
            0,
            2,
        );
        let mut e = Emulator::open(&path).unwrap();
        e.run(4, 1000).unwrap();
        assert_eq!(e.peek(0xc000, 1).unwrap(), vec![0x42]);
        let snapshot = e.save().unwrap();
        let before = e.state();
        let bytes = e.save().unwrap().bytes;
        e.peek(0xff00, 256).unwrap();
        assert_eq!(e.save().unwrap().bytes, bytes);
        assert_eq!(e.state(), before);
        e.run(50, 1000).unwrap();
        let after = e.state();
        let after_bytes = e.save().unwrap().bytes;
        e.load(&snapshot).unwrap();
        e.run(50, 1000).unwrap();
        assert_eq!(e.state(), after);
        assert_eq!(e.save().unwrap().bytes, after_bytes);
        assert!(e.run(1_000_001, 1000).is_err());
        assert!(e.peek(0xffff, 2).is_err());
        e.reset().unwrap();
        assert_eq!(e.state().registers.pc, 0x100);
        std::fs::remove_file(path).unwrap();
    }
    #[test]
    fn mbc1_aliases_and_translation() {
        let path = fixture(
            "mbc1",
            &[0x3e, 2, 0xea, 0, 0x20, 0x3e, 0, 0xea, 0, 0x20, 0x18, 0xfe],
            1,
            4,
        );
        let mut e = Emulator::open(&path).unwrap();
        e.run(3, 1000).unwrap();
        assert_eq!(e.state().switch_bank, 2);
        assert_eq!(e.physical_rom_offset(0x4567), Some(0x8567));
        let snapshot = e.save().unwrap();
        e.run(2, 1000).unwrap();
        assert_eq!(e.state().switch_bank, 1);
        e.load(&snapshot).unwrap();
        assert_eq!(e.state().switch_bank, 2);
        std::fs::remove_file(path).unwrap();
    }
    #[test]
    fn trace_actual_bus_and_behavior() {
        use romatlas_core::EventKind;
        let path = fixture(
            "trace",
            &[
                0x3e, 2, 0xea, 0, 0x20, 0x3e, 0x42, 0xea, 0, 0xc0, 0xfa, 0, 0xc0, 0x18, 0xfe,
            ],
            1,
            4,
        );
        let mut plain = Emulator::open(&path).unwrap();
        let mut traced = Emulator::open(&path).unwrap();
        traced
            .configure_trace(romatlas_analysis::TraceConfig {
                enabled: true,
                capacity: 100,
                ..Default::default()
            })
            .unwrap();
        plain.run(8, 1000).unwrap();
        traced.run(8, 1000).unwrap();
        assert_eq!(plain.state(), traced.state());
        assert_eq!(plain.save().unwrap().bytes, traced.save().unwrap().bytes);
        let summary = traced.trace.summary();
        assert!(summary.unique_instructions >= 6);
        let events = traced.trace.query(0, 100);
        assert!(events.iter().any(|e| e.kind == EventKind::Write
            && e.context["cpu_address"] == 0xc000
            && e.value == Some(0x42)));
        assert!(events.iter().any(|e| e.kind == EventKind::Read
            && e.context["cpu_address"] == 0xc000
            && e.value == Some(0x42)));
        assert!(events.iter().any(
            |e| e.kind == EventKind::BankSwitch && e.target.as_ref().unwrap().offset == 0x8000
        ));
        assert!(
            events
                .iter()
                .filter(|e| e.kind == EventKind::Instruction)
                .all(|e| e.address.space == "cartridge-rom")
        );
        traced.peek(0x100, 32).unwrap();
        assert_eq!(traced.trace.summary().retained, summary.retained);
        traced
            .configure_trace(romatlas_analysis::TraceConfig {
                enabled: true,
                capacity: 3,
                ..Default::default()
            })
            .unwrap();
        traced.run(10, 1000).unwrap();
        assert_eq!(traced.trace.summary().retained, 3);
        assert!(traced.trace.dropped > 0);
        std::fs::remove_file(path).unwrap();
    }
    #[test]
    fn cycle_frame_and_disabled_lcd_budgets() {
        let path = fixture("frames", &[0x18, 0xfe], 0, 2);
        let mut e = Emulator::open(&path).unwrap();
        e.configure_trace(romatlas_analysis::TraceConfig {
            enabled: true,
            capacity: 20,
            kinds: std::collections::BTreeSet::from([romatlas_core::EventKind::Frame]),
            ..Default::default()
        })
        .unwrap();
        let initial = e.state();
        let state = e.advance("cycle", 100, 1000, 1000).unwrap();
        assert!(state.ppu_cycles - initial.ppu_cycles >= 100);
        assert!(state.ppu_cycles - initial.ppu_cycles < 124);
        let snapshot = e.save().unwrap();
        let before = e.state();
        let after = e.advance("frame", 2, 100000, 10000).unwrap();
        assert_eq!(after.frames - before.frames, 2);
        assert_eq!(
            e.trace
                .query(0, 20)
                .iter()
                .filter(|r| r.kind == romatlas_core::EventKind::Frame)
                .count(),
            2
        );
        e.load(&snapshot).unwrap();
        assert_eq!(e.advance("frame", 2, 100000, 10000).unwrap(), after);
        std::fs::remove_file(path).unwrap();
        let path = fixture("lcd-off", &[0xaf, 0xe0, 0x40, 0x18, 0xfe], 0, 2);
        let mut e = Emulator::open(&path).unwrap();
        e.run(3, 1000).unwrap();
        let before = e.state();
        assert!(
            e.advance("frame", 1, 100, 1000)
                .unwrap_err()
                .to_string()
                .contains("operation budget exhausted")
        );
        assert_eq!(e.state().frames, before.frames);
        assert_eq!(e.state().operations - before.operations, 100);
        std::fs::remove_file(path).unwrap();
    }
    #[test]
    fn oam_dma_events_and_fetch_identity() {
        use romatlas_core::EventKind;
        let path = fixture("dma-trace", &[0x3e, 0xc0, 0xe0, 0x46, 0x18, 0xfe], 0, 2);
        let mut e = Emulator::open(&path).unwrap();
        let mut plain = Emulator::open(&path).unwrap();
        e.configure_trace(romatlas_analysis::TraceConfig {
            enabled: true,
            capacity: 2000,
            ..Default::default()
        })
        .unwrap();
        e.run(300, 1000).unwrap();
        plain.run(300, 1000).unwrap();
        assert_eq!(e.save().unwrap().bytes, plain.save().unwrap().bytes);
        let events = e.trace.query(0, 1000);
        let dma: Vec<_> = events.iter().filter(|r| r.kind == EventKind::Dma).collect();
        assert_eq!(dma.len(), 160);
        assert_eq!(dma[0].target.as_ref().unwrap().space, "oam");
        assert_eq!(dma[0].target.as_ref().unwrap().offset, 0);
        assert_eq!(dma[159].target.as_ref().unwrap().offset, 0x9f);
        assert!(
            events
                .iter()
                .filter(|r| r.kind == EventKind::Instruction)
                .all(|r| r.context["cpu_address"].as_u64().unwrap() < 0x8000)
        );
        std::fs::remove_file(path).unwrap();
    }
    #[test]
    fn mooneye_protocol_requires_report_breakpoint() {
        for (name, values, status) in [
            ("pass-protocol", [3, 5, 8, 13, 21, 34], "passed"),
            ("fail-protocol", [0x42; 6], "failed"),
        ] {
            let mut program = Vec::new();
            for (op, value) in [0x06, 0x0e, 0x16, 0x1e, 0x26, 0x2e].into_iter().zip(values) {
                program.extend_from_slice(&[op, value]);
            }
            program.push(0x40);
            let path = fixture(name, &program, 0, 2);
            let mut e = Emulator::open(&path).unwrap();
            assert_eq!(e.mooneye(20, 1000).unwrap()["status"], status);
            std::fs::remove_file(path).unwrap();
        }
        let path = fixture("nonpassing-loop", &[0x18, 0xfe], 0, 2);
        let mut e = Emulator::open(&path).unwrap();
        assert_eq!(e.mooneye(10, 1000).unwrap()["status"], "operation_limit");
        std::fs::remove_file(path).unwrap();
    }
    #[test]
    fn stat_coincidence_clears_at_line_boundary() {
        for lyc in [0, 1] {
            let path = fixture(
                &format!("coincidence-{lyc}"),
                &[
                    0xaf, 0xe0, 0x40, 0x3e, lyc, 0xe0, 0x45, 0x3e, 0x81, 0xe0, 0x40,
                ],
                0,
                2,
            );
            let mut e = Emulator::open(&path).unwrap();
            e.run(7, 1000).unwrap();
            let mut boundary = false;
            for _ in 0..200 {
                e.step().unwrap();
                if e.peek(0xff44, 1).unwrap()[0] == 1 {
                    boundary = true;
                    break;
                }
            }
            assert!(boundary);
            assert_eq!(
                e.peek(0xff41, 1).unwrap()[0] & 4,
                0,
                "LYC={lyc} boundary flag"
            );
            e.step().unwrap();
            assert_eq!(
                e.peek(0xff41, 1).unwrap()[0] & 4,
                if lyc == 1 { 4 } else { 0 }
            );
            std::fs::remove_file(path).unwrap();
        }
    }
    #[test]
    fn mbc5_nine_bit_rom_bank_zero_and_snapshot() {
        let path = fixture(
            "mbc5-ninebit",
            &[
                0x3e, 1, 0xea, 0, 0x30, 0x3e, 1, 0xea, 0, 0x20, 0x3e, 0, 0xea, 0, 0x30, 0xea, 0,
                0x20, 0x18, 0xfe,
            ],
            0x19,
            512,
        );
        let mut e = Emulator::open(&path).unwrap();
        e.run(3, 1000).unwrap();
        assert_eq!(e.state().switch_bank, 257);
        e.run(2, 1000).unwrap();
        assert_eq!(e.state().switch_bank, 257);
        let snapshot = e.save().unwrap();
        e.run(3, 1000).unwrap();
        assert_eq!(e.state().switch_bank, 0);
        assert_eq!(e.physical_rom_offset(0x4567), Some(0x567));
        e.load(&snapshot).unwrap();
        assert_eq!(e.state().switch_bank, 257);
        assert_eq!(e.physical_rom_offset(0x4567), Some(257 * 0x4000 + 0x567));
        std::fs::remove_file(path).unwrap();
    }
    #[test]
    fn mbc5_rumble_bit_does_not_select_ram_bank() {
        let program = [
            0x3e, 0x1a, 0xea, 0, 0, 0x3e, 0x55, 0xea, 0, 0xa0, 0x3e, 8, 0xea, 0, 0x40, 0x3e, 0x66,
            0xea, 0, 0xa0, 0x3e, 0, 0xea, 0, 0x40, 0x18, 0xfe,
        ];
        for (kind, name, expected) in [
            (0x1b, "normal-mbc5-ram", 0x55),
            (0x1e, "rumble-mbc5-ram", 0x66),
        ] {
            let path = fixture(name, &program, kind, 2);
            let mut data = std::fs::read(&path).unwrap();
            data[0x149] = 4;
            data[0x14d] = data[0x134..0x14d]
                .iter()
                .fold(0u8, |a, b| a.wrapping_sub(*b).wrapping_sub(1));
            std::fs::write(&path, data).unwrap();
            let mut e = Emulator::open(&path).unwrap();
            e.run(13, 1000).unwrap();
            assert_eq!(e.peek(0xa000, 1).unwrap(), vec![expected]);
            let snapshot = e.save().unwrap();
            let after = e.run(10, 1000).unwrap();
            e.load(&snapshot).unwrap();
            assert_eq!(e.run(10, 1000).unwrap(), after);
            std::fs::remove_file(path).unwrap();
        }
    }
    #[test]
    fn mbc3_rom_ram_and_invalid_selection() {
        let program = [
            0x3e, 0x80, 0xea, 0, 0x20, 0x3e, 0x1a, 0xea, 0, 0, 0x3e, 2, 0xea, 0, 0x40, 0x3e, 0x55,
            0xea, 0, 0xa0, 0x3e, 4, 0xea, 0, 0x40, 0xea, 0, 0xa0, 0x3e, 2, 0xea, 0, 0x40, 0x18,
            0xfe,
        ];
        for (kind, name) in [(0x13, "mbc3-banks"), (0x10, "mbc3-rtc-banks")] {
            let path = fixture(name, &program, kind, 4);
            let mut bytes = std::fs::read(&path).unwrap();
            bytes[0x149] = 3;
            bytes[0x14d] = bytes[0x134..0x14d]
                .iter()
                .fold(0u8, |a, b| a.wrapping_sub(*b).wrapping_sub(1));
            std::fs::write(&path, bytes).unwrap();
            let mut e = Emulator::open(&path).unwrap();
            assert_eq!(e.peek(0xa000, 1).unwrap(), vec![0xff]);
            e.run(9, 1000).unwrap();
            assert_eq!(e.state().switch_bank, 1);
            assert_eq!(e.peek(0xa000, 1).unwrap(), vec![0x55]);
            let snapshot = e.save().unwrap();
            e.run(3, 1000).unwrap();
            assert_eq!(e.peek(0xa000, 1).unwrap(), vec![0xff]);
            e.run(2, 1000).unwrap();
            assert_eq!(e.peek(0xa000, 1).unwrap(), vec![0x55]);
            let after = e.save().unwrap().bytes;
            e.load(&snapshot).unwrap();
            e.run(5, 1000).unwrap();
            assert_eq!(e.save().unwrap().bytes, after);
            std::fs::remove_file(path).unwrap();
        }
    }
    #[test]
    fn mbc3_rtc_rollover_latch_halt_and_snapshot() {
        fn write(program: &mut Vec<u8>, addr: u16, value: u8) {
            program.extend_from_slice(&[0x3e, value, 0xea, addr as u8, (addr >> 8) as u8]);
        }
        fn latch(program: &mut Vec<u8>) {
            write(program, 0x6000, 0);
            write(program, 0x6000, 1);
        }
        fn second(e: &mut Emulator) {
            for _ in 0..2 {
                e.advance("cycle", 2_097_152, 1_000_000, 30000).unwrap();
            }
        }
        let mut program = Vec::new();
        write(&mut program, 0xff40, 0);
        write(&mut program, 0xff00, 0x10);
        write(&mut program, 0, 0x0a);
        for (reg, value) in [(8, 58), (9, 59), (10, 23), (11, 255), (12, 1)] {
            write(&mut program, 0x4000, reg);
            write(&mut program, 0xa000, value);
        }
        write(&mut program, 0x4000, 8);
        latch(&mut program);
        program.extend_from_slice(&[0x10, 0]);
        latch(&mut program);
        for reg in 9..=12 {
            write(&mut program, 0x4000, reg);
        }
        write(&mut program, 0xa000, 0xc0);
        latch(&mut program);
        program.extend_from_slice(&[0x10, 0]);
        latch(&mut program);
        write(&mut program, 0x4000, 8);
        write(&mut program, 0x4000, 12);
        write(&mut program, 0xa000, 0);
        latch(&mut program);
        program.extend_from_slice(&[0x10, 0]);
        latch(&mut program);
        write(&mut program, 0x4000, 8);
        program.extend_from_slice(&[0x18, 0xfe]);
        let path = fixture("mbc3-rtc", &program, 0x0f, 2);
        let mut e = Emulator::open(&path).unwrap();
        let fresh = Emulator::open(&path).unwrap();
        assert_eq!(e.save().unwrap().bytes, fresh.save().unwrap().bytes);
        // JP + 3 setup writes + 10 register writes + selection + 2 latch writes + STOP.
        e.run(34, 1000).unwrap();
        assert_eq!(e.peek(0xa000, 1).unwrap(), vec![58]);
        let snapshot = e.save().unwrap();
        second(&mut e);
        second(&mut e);
        assert_eq!(
            e.peek(0xa000, 1).unwrap(),
            vec![58],
            "latched clock remains stable"
        );
        let after = e.save().unwrap().bytes;
        e.load(&snapshot).unwrap();
        second(&mut e);
        second(&mut e);
        assert_eq!(e.save().unwrap().bytes, after);
        e.input("a", true).unwrap();
        e.run(5, 1000).unwrap();
        assert_eq!(e.peek(0xa000, 1).unwrap(), vec![0]);
        e.input("a", false).unwrap();
        for expected in [0, 0, 0, 0x80] {
            e.run(2, 1000).unwrap();
            assert_eq!(e.peek(0xa000, 1).unwrap(), vec![expected]);
        }
        e.run(7, 1000).unwrap();
        assert_eq!(e.peek(0xa000, 1).unwrap(), vec![0xc0]);
        second(&mut e);
        e.input("a", true).unwrap();
        e.run(7, 1000).unwrap();
        assert_eq!(
            e.peek(0xa000, 1).unwrap(),
            vec![0],
            "halted RTC does not advance"
        );
        e.input("a", false).unwrap();
        e.run(9, 1000).unwrap();
        assert_eq!(
            e.peek(0xa000, 1).unwrap(),
            vec![0],
            "clear carry and resume"
        );
        second(&mut e);
        e.input("a", true).unwrap();
        e.run(7, 1000).unwrap();
        assert_eq!(e.peek(0xa000, 1).unwrap(), vec![1], "resumed RTC advances");
        std::fs::remove_file(path).unwrap();
    }
    #[test]
    fn cgb_banks_speed_gdma_and_replay() {
        fn write(p: &mut Vec<u8>, addr: u16, value: u8) {
            p.extend_from_slice(&[0x3e, value, 0xea, addr as u8, (addr >> 8) as u8]);
        }
        let mut p = Vec::new();
        for (addr, value) in [
            (0xff40, 0),
            (0xff70, 2),
            (0xd000, 0x55),
            (0xff70, 1),
            (0xd000, 0x66),
            (0xff70, 0),
            (0xff4f, 1),
            (0x8000, 0xaa),
            (0xff4f, 0),
            (0x8000, 0xbb),
            (0xff4f, 1),
        ] {
            write(&mut p, addr, value);
        }
        write(&mut p, 0xff4d, 1);
        p.extend_from_slice(&[0x10, 0]);
        write(&mut p, 0xc000, 0x42);
        for (addr, value) in [
            (0xff51, 0xc0),
            (0xff52, 0),
            (0xff53, 0),
            (0xff54, 0),
            (0xff55, 0),
        ] {
            write(&mut p, addr, value);
        }
        p.extend_from_slice(&[0x18, 0xfe]);
        let path = fixture("cgb-banks", &p, 0, 2);
        let mut data = std::fs::read(&path).unwrap();
        data[0x143] = 0xc0;
        data[0x14d] = data[0x134..0x14d]
            .iter()
            .fold(0u8, |a, b| a.wrapping_sub(*b).wrapping_sub(1));
        std::fs::write(&path, data).unwrap();
        assert!(Emulator::open_model(&path, "dmg").is_err());
        let mut e = Emulator::open(&path).unwrap();
        assert_eq!(e.state().model, "cgb");
        assert_eq!(e.state().registers.a, 0x11);
        e.configure_trace(romatlas_analysis::TraceConfig {
            enabled: true,
            capacity: 1000,
            ..Default::default()
        })
        .unwrap();
        e.run(23, 1000).unwrap();
        assert_eq!(e.state().wram_bank, 1);
        assert_eq!(e.state().vram_bank, 1);
        assert_eq!(e.peek(0xd000, 1).unwrap(), vec![0x66]);
        assert_eq!(e.peek(0xf000, 1).unwrap(), vec![0x66]);
        assert_eq!(e.peek(0x8000, 1).unwrap(), vec![0xaa]);
        assert_eq!(e.physical_memory("wram", 0x2000, 1).unwrap(), vec![0x55]);
        assert_eq!(e.physical_memory("wram", 0x1000, 1).unwrap(), vec![0x66]);
        assert_eq!(e.physical_memory("vram", 0, 1).unwrap(), vec![0xbb]);
        assert_eq!(e.physical_memory("vram", 0x2000, 1).unwrap(), vec![0xaa]);
        let summary = e.trace.summary().retained;
        let saved = e.save().unwrap().bytes;
        e.physical_memory("vram", 0, 4096).unwrap();
        assert_eq!(e.trace.summary().retained, summary);
        assert_eq!(e.save().unwrap().bytes, saved);
        assert!(e.physical_memory("wram", 0, 4097).is_err());
        let writes = e.trace.query(0, 1000);
        assert!(
            writes
                .iter()
                .any(|v| v.kind == romatlas_core::EventKind::Write
                    && v.address.space == "wram"
                    && v.address.offset == 0x2000
                    && v.value == Some(0x55))
        );
        assert!(
            writes
                .iter()
                .any(|v| v.kind == romatlas_core::EventKind::Write
                    && v.address.space == "vram"
                    && v.address.offset == 0
                    && v.value == Some(0xbb))
        );
        let snapshot = e.save().unwrap();
        e.run(2, 1000).unwrap();
        assert_eq!(e.peek(0xff4d, 1).unwrap()[0] & 0x81, 1);
        e.step().unwrap();
        assert!(e.state().double_speed);
        assert_eq!(e.peek(0xff4d, 1).unwrap()[0] & 0x81, 0x80);
        e.run(2050, 1000).unwrap();
        let cycle = e.cycles;
        e.step().unwrap();
        assert_eq!(
            e.cycles - cycle,
            4,
            "LD A,d8 takes 4 PPU ticks in double speed"
        );
        e.step().unwrap();
        e.run(42, 1000).unwrap();
        assert_eq!(
            e.peek(0x8000, 16).unwrap(),
            [vec![0x42], vec![0; 15]].concat()
        );
        let events = e.trace.query(0, 1000);
        let dma: Vec<_> = events
            .iter()
            .filter(|r| r.kind == romatlas_core::EventKind::Dma)
            .collect();
        assert_eq!(dma.len(), 16);
        assert_eq!(dma[0].target.as_ref().unwrap().space, "vram");
        assert_eq!(dma[0].target.as_ref().unwrap().offset, 0x2000);
        assert_eq!(dma[15].target.as_ref().unwrap().offset, 0x200f);
        let state = e.state();
        let bytes = e.save().unwrap().bytes;
        e.load(&snapshot).unwrap();
        e.run(2097, 1000).unwrap();
        assert_eq!(e.state(), state);
        assert_eq!(e.save().unwrap().bytes, bytes);
        e.reset().unwrap();
        assert_eq!(e.state().model, "cgb");
        assert!(!e.state().double_speed);
        std::fs::remove_file(path).unwrap();
    }
    #[test]
    fn cgb_hblank_dma_waits_for_active_lcd() {
        let mut p = Vec::new();
        for (addr, value) in [
            (0xff40u16, 0),
            (0xc000, 0x42),
            (0xff51, 0xc0),
            (0xff52, 0),
            (0xff53, 0),
            (0xff54, 0),
            (0xff55, 0x80),
        ] {
            p.extend_from_slice(&[0x3e, value, 0xea, addr as u8, (addr >> 8) as u8]);
        }
        p.extend_from_slice(&[0; 16]);
        p.extend_from_slice(&[0x3e, 0x80, 0xea, 0x40, 0xff, 0x18, 0xfe]);
        let path = fixture("cgb-hblank", &p, 0, 2);
        let mut data = std::fs::read(&path).unwrap();
        data[0x143] = 0x80;
        data[0x14d] = data[0x134..0x14d]
            .iter()
            .fold(0u8, |a, b| a.wrapping_sub(*b).wrapping_sub(1));
        std::fs::write(&path, data).unwrap();
        let mut e = Emulator::open_model(&path, "cgb").unwrap();
        e.configure_trace(romatlas_analysis::TraceConfig {
            enabled: true,
            capacity: 1000,
            ..Default::default()
        })
        .unwrap();
        e.run(31, 1000).unwrap();
        assert_eq!(e.peek(0x8000, 16).unwrap(), vec![0; 16]);
        assert!(
            !e.trace
                .summary()
                .counts
                .contains_key(&romatlas_core::EventKind::Dma)
        );
        let snapshot = e.save().unwrap();
        e.run(100, 1000).unwrap();
        let state = e.state();
        let bytes = e.save().unwrap().bytes;
        assert_eq!(e.trace.summary().counts[&romatlas_core::EventKind::Dma], 16);
        e.load(&snapshot).unwrap();
        e.run(100, 1000).unwrap();
        assert_eq!(e.state(), state);
        assert_eq!(e.save().unwrap().bytes, bytes);
        for _ in 0..100 {
            if e.peek(0xff41, 1).unwrap()[0] & 3 != 3 {
                break;
            }
            e.step().unwrap();
        }
        assert_eq!(
            e.peek(0x8000, 16).unwrap(),
            [vec![0x42], vec![0; 15]].concat()
        );
        e.reset().unwrap();
        assert_eq!(e.state().model, "cgb");
        std::fs::remove_file(path).unwrap();
    }
    #[test]
    fn cgb_speed_switch_pause_freezes_div_and_replays() {
        let path = fixture(
            "speed-settle",
            &[
                0xf3, 0x3e, 0, 0xe0, 0x40, 0x3e, 1, 0xe0, 0x4d, 0x10, 0, 0x3e, 1, 0xe0, 0x4d, 0x10,
                0, 0,
            ],
            0,
            2,
        );
        let mut bytes = std::fs::read(&path).unwrap();
        bytes[0x143] = 0x80;
        bytes[0x14d] = bytes[0x134..0x14d]
            .iter()
            .fold(0u8, |sum, byte| sum.wrapping_sub(*byte).wrapping_sub(1));
        std::fs::write(&path, bytes).unwrap();
        let mut e = Emulator::open(&path).unwrap();
        e.configure_trace(romatlas_analysis::TraceConfig {
            enabled: true,
            capacity: 0,
            ..Default::default()
        })
        .unwrap();
        e.run(7, 1000).unwrap();
        assert!(e.state().double_speed);
        let pc = e.state().registers.pc;
        let cycle = e.state().ppu_cycles;
        let coverage = e.trace.summary().unique_instructions;
        e.run(1025, 1000).unwrap();
        assert_eq!(e.state().registers.pc, pc);
        assert_eq!(e.peek(0xff04, 1).unwrap(), [0]);
        let midway = e.save().unwrap();
        e.run(1025, 1000).unwrap();
        assert_eq!(e.state().ppu_cycles - cycle, 4100);
        assert_eq!(e.trace.summary().unique_instructions, coverage);
        assert_eq!(e.peek(0xff04, 1).unwrap(), [0]);
        let settled = e.state();
        let settled_bytes = e.save().unwrap().bytes;
        e.load(&midway).unwrap();
        e.run(1025, 1000).unwrap();
        assert_eq!(e.state(), settled);
        assert_eq!(e.save().unwrap().bytes, settled_bytes);
        e.run(3, 1000).unwrap();
        assert!(!e.state().double_speed);
        let cycle = e.state().ppu_cycles;
        let pc = e.state().registers.pc;
        e.run(2050, 1000).unwrap();
        assert_eq!(e.state().ppu_cycles - cycle, 8200);
        assert_eq!(e.state().registers.pc, pc);
        assert_eq!(e.peek(0xff04, 1).unwrap(), [0]);
        e.step().unwrap();
        assert_eq!(e.state().registers.pc, pc + 1);
        std::fs::remove_file(path).unwrap();
    }
    #[test]
    fn snapshot_replays_pending_sprite_fetch() {
        let mut program = vec![0x3e, 0, 0xe0, 0x40];
        for (address, value) in [(0xfe00u16, 17u8), (0xfe01, 8), (0xfe02, 0)] {
            program.extend([0x3e, value, 0xea, address as u8, (address >> 8) as u8]);
        }
        program.extend([0x3e, 0x93, 0xe0, 0x40]);
        program.extend([0; 300]);
        let path = fixture("sprite-snapshot", &program, 0, 2);
        let mut e = Emulator::open(&path).unwrap();
        let mut reached = false;
        for _ in 0..250 {
            e.step().unwrap();
            if e.peek(0xff44, 1).unwrap() == [1] && e.peek(0xff41, 1).unwrap()[0] & 3 == 3 {
                reached = true;
                break;
            }
        }
        assert!(
            reached,
            "snapshot must occur at sprite fetch start on line one"
        );
        let snapshot = e.save().unwrap();
        e.run(64, 1000).unwrap();
        let state = e.state();
        let bytes = e.save().unwrap().bytes;
        e.load(&snapshot).unwrap();
        e.run(64, 1000).unwrap();
        assert_eq!(e.state(), state);
        assert_eq!(e.save().unwrap().bytes, bytes);
        std::fs::remove_file(path).unwrap();
    }
    #[test]
    fn blocked_vram_access_has_no_physical_claim() {
        let p = [
            0x3e, 0, 0xea, 0x40, 0xff, 0x3e, 0x55, 0xea, 0, 0x80, 0x3e, 0x80, 0xea, 0x40, 0xff,
            0x21, 0x41, 0xff, 0x7e, 0xe6, 3, 0xfe, 3, 0x20, 0xf9, 0xfa, 0, 0x80, 0xea, 0, 0xc0,
            0x3e, 0x42, 0xea, 0, 0x80, 0x18, 0xfe,
        ];
        let path = fixture("vram-lock", &p, 0, 2);
        let mut e = Emulator::open(&path).unwrap();
        e.configure_trace(romatlas_analysis::TraceConfig {
            enabled: true,
            capacity: 1000,
            ..Default::default()
        })
        .unwrap();
        e.run(60, 1000).unwrap();
        assert_eq!(e.physical_memory("vram", 0, 1).unwrap(), vec![0x55]);
        assert_eq!(e.physical_memory("wram", 0, 1).unwrap(), vec![0xff]);
        let events = e.trace.query(0, 1000);
        let blocked: Vec<_> = events
            .iter()
            .filter(|event| {
                event.context["cpu_address"] == 0x8000 && event.context["blocked"] == true
            })
            .collect();
        assert!(
            blocked
                .iter()
                .any(|e| e.kind == romatlas_core::EventKind::Read && e.value == Some(0xff))
        );
        assert!(
            blocked
                .iter()
                .any(|e| e.kind == romatlas_core::EventKind::Write && e.value == Some(0x42))
        );
        assert!(
            blocked
                .iter()
                .all(|e| e.address.space == "cpu" && e.context["mapped"] == false)
        );
        assert!(e.physical_memory("wram", 0x2000, 1).is_err());
        assert!(e.physical_memory("vram", 0x2000, 1).is_err());
        std::fs::remove_file(path).unwrap();
    }
    #[test]
    fn fetched_cb_and_halt_bug_edges() {
        use romatlas_core::EventKind;
        // Set a pending interrupt with IME off, then HALT. The next opcode
        // and operand come from the same address (the hardware HALT bug).
        let path = fixture(
            "edge-fetch",
            &[
                0xf3, 0x3e, 1, 0xea, 0xff, 0xff, 0xe0, 0x0f, 0xcb, 0x00, 0x76, 0x3e, 0x42,
            ],
            0,
            2,
        );
        let mut e = Emulator::open(&path).unwrap();
        e.configure_trace(romatlas_analysis::TraceConfig {
            enabled: true,
            capacity: 200,
            ..Default::default()
        })
        .unwrap();
        e.run(8, 1000).unwrap();
        assert_eq!(e.state().registers.a, 0x3e);
        let events = e.trace.query(0, 200);
        let edge = |pc| {
            events
                .iter()
                .find(|event| {
                    event.kind == EventKind::ControlFlow && event.context["cpu_address"] == pc
                })
                .unwrap()
        };
        assert_eq!(edge(0x158).context["edge_kind"], "fallthrough");
        assert_eq!(edge(0x158).context["cpu_target"], 0x15a);
        assert_eq!(edge(0x15a).context["edge_kind"], "halt");
        assert_eq!(edge(0x15a).context["traversable"], false);
        assert_eq!(edge(0x15b).context["edge_kind"], "fallthrough");
        assert_eq!(edge(0x15b).context["cpu_target"], 0x15c);
        assert_eq!(edge(0x15b).context["fallthrough_cpu"], 0x15c);
        assert!(
            !e.trace
                .edges()
                .keys()
                .any(|(source, _)| source.offset == 0x15a)
        );
        std::fs::remove_file(path).unwrap();
    }
    #[test]
    fn halt_bug_relative_target_uses_fetch_pc() {
        let path = fixture(
            "halt-jr",
            &[0xf3, 0x3e, 1, 0xea, 0xff, 0xff, 0xe0, 0x0f, 0x76, 0x18, 0],
            0,
            2,
        );
        let mut e = Emulator::open(&path).unwrap();
        e.configure_trace(romatlas_analysis::TraceConfig {
            enabled: true,
            capacity: 100,
            ..Default::default()
        })
        .unwrap();
        e.run(7, 1000).unwrap();
        let events = e.trace.query(0, 100);
        let edge = events
            .iter()
            .find(|event| {
                event.kind == romatlas_core::EventKind::ControlFlow
                    && event.context["cpu_address"] == 0x159
            })
            .unwrap();
        assert_eq!(edge.context["edge_kind"], "jump");
        assert_eq!(edge.context["cpu_target"], 0x172);
        assert_eq!(edge.context["encoded_cpu_target"], 0x172);
        std::fs::remove_file(path).unwrap();
    }
    #[test]
    fn observed_indirect_successor_is_not_executed_coverage() {
        use romatlas_core::{ImageAdapter, Provenance};
        use std::collections::BTreeSet;
        let mut p = vec![0; 17];
        p[..4].copy_from_slice(&[0x21, 0x60, 1, 0xe9]);
        p[16] = 0x76;
        let path = fixture("indirect-edge", &p, 0, 2);
        let mut e = Emulator::open(&path).unwrap();
        e.configure_trace(romatlas_analysis::TraceConfig {
            enabled: true,
            capacity: 0,
            ..Default::default()
        })
        .unwrap();
        e.run(3, 1000).unwrap();
        let rom = Rom::open(&path).unwrap();
        let source = rom.physical_address(0x153).unwrap();
        let target = rom.physical_address(0x160).unwrap();
        assert!(!e.trace.coverage().contains_key(&target));
        let observed = BTreeSet::from([source.clone()]);
        let static_only =
            romatlas_analysis::discovery::discover(&rom, vec![source.clone()], &observed, &[], 10)
                .unwrap();
        assert_eq!(static_only.candidates.len(), 1);
        let found = romatlas_analysis::discovery::discover_with_edges(
            &rom,
            vec![source.clone()],
            &observed,
            e.trace.edges(),
            &[],
            10,
        )
        .unwrap();
        assert_eq!(found.candidates.len(), 2);
        assert_eq!(
            found
                .candidates
                .iter()
                .find(|r| r.start == target)
                .unwrap()
                .provenance,
            Provenance::Static
        );
        assert!(found.references.iter().any(|r| r.source == source
            && r.target.as_ref() == Some(&target)
            && r.kind == "observed-successor"));
        let data = romatlas_core::Region {
            start: target,
            length: 1,
            kind: romatlas_core::RegionKind::Data,
            provenance: Provenance::Manual,
            confidence: 1.0,
        };
        let blocked = romatlas_analysis::discovery::discover_with_edges(
            &rom,
            vec![source],
            &observed,
            e.trace.edges(),
            &[data],
            10,
        )
        .unwrap();
        assert_eq!(blocked.candidates.len(), 1);
        assert!(
            blocked
                .diagnostics
                .iter()
                .any(|d| d.message.contains("non-code"))
        );
        std::fs::remove_file(path).unwrap();
    }
    #[test]
    fn halt_waits_advance_time_without_synthetic_reads() {
        let path = fixture("idle-trace", &[0x3e, 0, 0x76], 0, 2);
        let mut e = Emulator::open(&path).unwrap();
        e.configure_trace(romatlas_analysis::TraceConfig {
            enabled: true,
            capacity: 100,
            ..Default::default()
        })
        .unwrap();
        e.run(3, 1000).unwrap();
        let pc = e.state().registers.pc;
        let cycles = e.state().ppu_cycles;
        e.trace.clear();
        e.run(5, 1000).unwrap();
        assert_eq!(e.state().registers.pc, pc);
        assert_eq!(e.state().ppu_cycles - cycles, 20);
        assert!(e.trace.query(0, 100).is_empty());
        let saved = e.save().unwrap();
        e.run(5, 1000).unwrap();
        let state = e.state();
        let bytes = e.save().unwrap().bytes;
        e.load(&saved).unwrap();
        e.run(5, 1000).unwrap();
        assert_eq!(e.state(), state);
        assert_eq!(e.save().unwrap().bytes, bytes);
        std::fs::remove_file(path).unwrap();
    }
    #[test]
    fn cgb_palette_blocked_writes_keep_data_and_trace_denial() {
        let mut program = Vec::new();
        for (register, value) in [
            (0x40, 0),
            (0x68, 63),
            (0x69, 0x55),
            (0x6a, 63),
            (0x6b, 0x66),
            (0x68, 0xbf),
            (0x6a, 0xbf),
            (0x40, 0x91),
        ] {
            program.extend([0x3e, value, 0xe0, register]);
        }
        program.extend([0xf0, 0x41, 0xe6, 3, 0xfe, 3, 0x20, 0xf8]);
        program.extend([0x3e, 0, 0xe0, 0x69, 0xe0, 0x6b]);
        for (register, offset) in [(0x68, 0), (0x6a, 1)] {
            program.extend([0xf0, register, 0xea, offset, 0xc0]);
        }
        program.extend([
            0xf0, 0x41, 0xe6, 3, 0x20, 0xfa, 0x3e, 63, 0xe0, 0x68, 0xe0, 0x6a,
        ]);
        for (register, offset) in [(0x69, 2), (0x6b, 3)] {
            program.extend([0xf0, register, 0xea, offset, 0xc0]);
        }
        program.push(0x76);
        let path = fixture("palette-trace", &program, 0, 2);
        let mut bytes = std::fs::read(&path).unwrap();
        bytes[0x143] = 0x80;
        bytes[0x14d] = bytes[0x134..0x14d]
            .iter()
            .fold(0u8, |sum, b| sum.wrapping_sub(*b).wrapping_sub(1));
        std::fs::write(&path, bytes).unwrap();
        let mut e = Emulator::open(&path).unwrap();
        e.configure_trace(romatlas_analysis::TraceConfig {
            enabled: true,
            capacity: 1000,
            ..Default::default()
        })
        .unwrap();
        e.run(200, 1000).unwrap();
        assert_eq!(e.peek(0xc000, 4).unwrap(), [0xc0, 0xc0, 0x55, 0x66]);
        let events = e.trace.query(0, 1000);
        let blocked: Vec<_> = events
            .iter()
            .filter(|event| {
                event.kind == romatlas_core::EventKind::Write
                    && event.value == Some(0)
                    && matches!(event.context["cpu_address"].as_u64(), Some(0xff69 | 0xff6b))
            })
            .collect();
        assert_eq!(blocked.len(), 2);
        assert!(
            blocked
                .iter()
                .all(|event| event.context["blocked"] == true && event.context["mapped"] == false)
        );
        std::fs::remove_file(path).unwrap();
    }
    #[test]
    fn contextual_edges_keep_bank_state_after_controller_write() {
        use crate::discovery::{ContextDecoder, CpuContext};
        use romatlas_core::{ImageAdapter, Provenance};
        use std::collections::BTreeSet;
        let path = fixture(
            "context-bank-edge",
            &[0x3e, 2, 0xea, 0, 0x20, 0xc3, 0, 0x40],
            1,
            4,
        );
        let mut bytes = std::fs::read(&path).unwrap();
        bytes[0x8000] = 0x76;
        std::fs::write(&path, bytes).unwrap();
        let mut e = Emulator::open(&path).unwrap();
        e.configure_trace(romatlas_analysis::TraceConfig {
            enabled: true,
            capacity: 0,
            ..Default::default()
        })
        .unwrap();
        e.run(4, 1000).unwrap();
        let rom = Rom::open(&path).unwrap();
        let target = rom.physical_address(0x8000).unwrap();
        assert!(!e.trace.coverage().contains_key(&target));
        let bank_edge = e
            .trace
            .contextual_edges()
            .keys()
            .find(|(source, _)| source.offset == 0x152)
            .unwrap();
        let before: CpuContext =
            serde_json::from_str(&bank_edge.0.context["gb.cpu_context"]).unwrap();
        let after: CpuContext =
            serde_json::from_str(&bank_edge.1.context["gb.cpu_context"]).unwrap();
        assert_eq!(before.switch_bank, Some(1));
        assert_eq!(after.switch_bank, Some(2));
        let root = ContextDecoder::node(
            rom.physical_address(0x100).unwrap(),
            CpuContext {
                cpu_address: 0x100,
                fixed_bank: Some(0),
                switch_bank: Some(1),
            },
        );
        let observed: BTreeSet<_> = e.trace.coverage().keys().cloned().collect();
        let result = romatlas_analysis::discovery::discover_with_edges(
            &ContextDecoder(&rom),
            vec![root],
            &observed,
            e.trace.contextual_edges(),
            &[],
            20,
        )
        .unwrap();
        assert_eq!(
            result
                .candidates
                .iter()
                .find(|c| c.start == target)
                .unwrap()
                .provenance,
            Provenance::Static
        );
        assert!(
            result
                .references
                .iter()
                .any(|r| r.kind == "observed-successor" && r.target.as_ref() == Some(&target))
        );
        assert_eq!(e.trace.summary().retained, 0);
        e.trace.clear();
        assert!(e.trace.contextual_edges().is_empty());
        assert!(e.trace.contextual_roots().is_empty());
        std::fs::remove_file(path).unwrap();
    }
    #[test]
    fn failed_opcode_accounts_bus_time_without_code_coverage() {
        let path = fixture("illegal-time", &[0xd3], 0, 2);
        let mut plain = Emulator::open(&path).unwrap();
        let mut traced = Emulator::open(&path).unwrap();
        traced
            .configure_trace(romatlas_analysis::TraceConfig {
                enabled: true,
                capacity: 100,
                ..Default::default()
            })
            .unwrap();
        plain.step().unwrap();
        traced.step().unwrap();
        let snapshot = traced.save().unwrap();
        for _ in 0..3 {
            let before = plain.state();
            assert!(plain.step().is_err());
            assert!(traced.step().is_err());
            assert_eq!(plain.cycles - before.ppu_cycles, 4);
            assert_eq!(plain.operations - before.operations, 1);
            assert_eq!(plain.state(), traced.state());
            assert_eq!(plain.save().unwrap().bytes, traced.save().unwrap().bytes);
        }
        assert!(!traced.trace.coverage().keys().any(|a| a.offset == 0x150));
        let fetches: Vec<_> = traced
            .trace
            .query(0, 100)
            .into_iter()
            .filter(|e| e.kind == romatlas_core::EventKind::Fetch && e.address.offset == 0x150)
            .collect();
        assert_eq!(fetches.len(), 3);
        assert!(
            fetches
                .windows(2)
                .all(|pair| pair[1].cycle == pair[0].cycle + 4)
        );
        let final_state = traced.save().unwrap().bytes;
        let final_counters = traced.state();
        traced.load(&snapshot).unwrap();
        for _ in 0..3 {
            assert!(traced.step().is_err());
        }
        assert_eq!(traced.save().unwrap().bytes, final_state);
        assert_eq!(traced.state(), final_counters);
        std::fs::remove_file(path).unwrap();
    }
    #[test]
    fn typed_execution_boundary_replays_and_checks_context() {
        use romatlas_core::{Address, execution::*};
        use std::collections::BTreeMap;
        let path = fixture(
            "typed-execution",
            &[0x3e, 0x42, 0xea, 0, 0xc0, 0x18, 0xfe],
            0,
            2,
        );
        let mut e = <Emulator as ExecutionAdapter>::load(&path, Default::default()).unwrap();
        let caps = ExecutionAdapter::capabilities(&e);
        assert_eq!(caps.platform, "gb");
        assert_eq!(caps.clock, "ppu-ticks");
        assert!(
            caps.address_spaces
                .iter()
                .any(|s| s.name == "cartridge-rom" && s.size == Some(0x8000))
        );
        let before = e.state();
        assert!(
            ExecutionAdapter::execute(
                &mut e,
                StepRequest {
                    unit: StepUnit::Operation,
                    count: 2,
                    max_operations: 1,
                    timeout_ms: 1000
                }
            )
            .is_err()
        );
        assert_eq!(e.state(), before);
        ExecutionAdapter::configure_instrumentation(
            &mut e,
            romatlas_analysis::TraceConfig {
                enabled: true,
                capacity: 100,
                ..Default::default()
            },
        )
        .unwrap();
        let snapshot = ExecutionAdapter::snapshot(&e).unwrap();
        let request = StepRequest {
            unit: StepUnit::Operation,
            count: 4,
            max_operations: 4,
            timeout_ms: 1000,
        };
        let first = ExecutionAdapter::execute(&mut e, request.clone()).unwrap();
        ExecutionAdapter::restore(&mut e, &snapshot).unwrap();
        assert_eq!(ExecutionAdapter::execute(&mut e, request).unwrap(), first);
        let cpu = Address {
            space: "cpu".into(),
            offset: 0xc000,
            context: BTreeMap::new(),
        };
        let translated = ExecutionAdapter::translate(&e, &cpu).unwrap().unwrap();
        assert_eq!(translated.space, "wram");
        assert_eq!(translated.offset, 0);
        let saved = e.save().unwrap().bytes;
        assert_eq!(
            ExecutionAdapter::inspect(&mut e, &translated, 1).unwrap(),
            vec![0x42]
        );
        assert_eq!(e.save().unwrap().bytes, saved);
        let wrong = Address {
            space: "wram".into(),
            offset: 0,
            context: BTreeMap::from([("bank".into(), "1".into())]),
        };
        assert_eq!(
            ExecutionAdapter::inspect(&mut e, &wrong, 1)
                .unwrap_err()
                .code,
            -32602
        );
        assert!(ExecutionAdapter::events(&e, 0, 1000).unwrap().len() > 4);
        let bytes = std::fs::read(&path).unwrap();
        std::fs::remove_file(&path).unwrap();
        assert!(
            ExecutionAdapter::capabilities(&e)
                .address_spaces
                .iter()
                .any(|s| s.name == "cartridge-rom" && s.size == Some(0x8000))
        );
        std::fs::write(&path, bytes).unwrap();
        ExecutionAdapter::reset(&mut e).unwrap();
        assert_eq!(e.state().operations, 0);
        std::fs::remove_file(path).unwrap();
    }
    #[test]
    fn streamed_trace_is_bounded_and_state_equivalent() {
        let path = fixture(
            "stream-state",
            &[0x3e, 0x42, 0xea, 0, 0xc0, 0x18, 0xfe],
            0,
            2,
        );
        let output = path.with_extension("jsonl");
        let tiny = path.with_extension("tiny-jsonl");
        let mut plain = Emulator::open(&path).unwrap();
        let mut streamed = Emulator::open(&path).unwrap();
        streamed
            .configure_trace(romatlas_analysis::TraceConfig {
                enabled: true,
                capacity: 0,
                ..Default::default()
            })
            .unwrap();
        streamed.trace.start_stream(&output, 100000).unwrap();
        plain.run(20, 1000).unwrap();
        streamed.run(20, 1000).unwrap();
        assert_eq!(plain.state(), streamed.state());
        assert_eq!(plain.save().unwrap().bytes, streamed.save().unwrap().bytes);
        assert_eq!(streamed.trace.summary().retained, 0);
        let status = streamed.trace.stop_stream().unwrap();
        let text = std::fs::read_to_string(&output).unwrap();
        let rows: Vec<serde_json::Value> = text
            .lines()
            .map(|s| serde_json::from_str(s).unwrap())
            .collect();
        assert_eq!(status.bytes, text.len() as u64);
        assert_eq!(status.records, rows.len() as u64);
        assert!(rows.len() > 20);
        for (i, row) in rows.iter().enumerate() {
            assert_eq!(row["sequence"], i);
        }
        assert!(streamed.trace.start_stream(&output, 1000).is_err());
        streamed.trace.start_stream(&tiny, 1).unwrap();
        streamed.run(3, 1000).unwrap();
        let status = streamed.trace.stop_stream().unwrap();
        assert!(status.stopped && status.dropped > 0);
        assert_eq!(std::fs::metadata(&tiny).unwrap().len(), 0);
        for file in [path, output, tiny] {
            std::fs::remove_file(file).unwrap();
        }
    }
    #[test]
    fn unsupported_is_explicit() {
        let path = fixture("unsupported", &[0], 0x05, 2);
        let err = Emulator::open(&path).err().unwrap().to_string();
        assert!(err.contains("unsupported emulator cartridge"));
        std::fs::remove_file(path).unwrap();
    }
}
