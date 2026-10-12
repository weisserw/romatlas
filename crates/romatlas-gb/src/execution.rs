//! Game Boy implementation of the shared typed execution boundary.
use crate::emulator::{Emulator, Snapshot, State};
use romatlas_core::{Address, ApiError, TraceEvent, execution::*};
use std::path::Path;
#[derive(Default)]
pub struct LoadOptions {
    pub model: Option<String>,
}
pub struct ButtonInput {
    pub button: String,
    pub pressed: bool,
}
fn error(e: anyhow::Error) -> ApiError {
    ApiError {
        code: -32000,
        message: e.to_string(),
    }
}
impl ExecutionAdapter for Emulator {
    type LoadOptions = LoadOptions;
    type State = State;
    type Snapshot = Snapshot;
    type Input = ButtonInput;
    type Instrumentation = romatlas_analysis::TraceConfig;
    fn load(image: &Path, options: LoadOptions) -> Result<Self, ApiError> {
        Emulator::open_model(image, options.model.as_deref().unwrap_or("auto")).map_err(error)
    }
    fn capabilities(&self) -> ExecutionCapabilities {
        let (rom_size, ram_size) = self.storage_shape();
        let state = self.state();
        let space = |name: &str, size: u64, context: serde_json::Value| AddressSpace {
            name: name.into(),
            size: Some(size),
            context,
        };
        ExecutionCapabilities {
            platform: "gb".into(),
            clock: "ppu-ticks".into(),
            units: vec![StepUnit::Operation, StepUnit::Cycle, StepUnit::Frame],
            address_spaces: vec![
                space("cpu", 65536, serde_json::json!({"mapped":true})),
                space(
                    "io",
                    256,
                    serde_json::json!({"cpu_base":65280,"ranges":[[0,128],[255,256]],"inspection":"mapped-register"}),
                ),
                space(
                    "cartridge-rtc",
                    if self.physical_memory("cartridge-rtc", 0, 0).is_ok() {
                        5
                    } else {
                        0
                    },
                    serde_json::json!({"registers":true}),
                ),
                space(
                    "cartridge-rom",
                    rom_size,
                    serde_json::json!({"bank_bytes":16384}),
                ),
                space(
                    "cartridge-ram",
                    ram_size,
                    serde_json::json!({"bank_bytes":8192}),
                ),
                space(
                    "wram",
                    if state.model == "dmg" { 8192 } else { 32768 },
                    serde_json::json!({"bank_bytes":4096}),
                ),
                space(
                    "vram",
                    if state.model == "dmg" { 8192 } else { 16384 },
                    serde_json::json!({"bank_bytes":8192}),
                ),
                space("oam", 160, serde_json::Value::Null),
                space("hram", 127, serde_json::Value::Null),
            ],
            snapshots: true,
            instrumentation: true,
            inputs: true,
        }
    }
    fn state(&self) -> State {
        Emulator::state(self)
    }
    fn execute(&mut self, r: StepRequest) -> Result<State, ApiError> {
        if !(1..=1000000).contains(&r.max_operations) || !(1..=30000).contains(&r.timeout_ms) {
            return Err(ApiError::invalid(
                "execution budget outside Game Boy limits",
            ));
        }
        match r.unit {
            StepUnit::Operation => {
                if r.count > r.max_operations {
                    return Err(ApiError::invalid("requested operations exceed budget"));
                }
                self.run(r.count, r.timeout_ms).map_err(error)
            }
            StepUnit::Cycle => self
                .advance("cycle", r.count, r.max_operations, r.timeout_ms)
                .map_err(error),
            StepUnit::Frame => self
                .advance("frame", r.count, r.max_operations, r.timeout_ms)
                .map_err(error),
        }
    }
    fn reset(&mut self) -> Result<(), ApiError> {
        Emulator::reset(self).map_err(error)
    }
    fn input(&mut self, input: ButtonInput) -> Result<(), ApiError> {
        Emulator::input(self, &input.button, input.pressed).map_err(error)
    }
    fn snapshot(&self) -> Result<Snapshot, ApiError> {
        self.save().map_err(error)
    }
    fn restore(&mut self, snapshot: &Snapshot) -> Result<(), ApiError> {
        self.load(snapshot).map_err(error)
    }
    fn translate(&self, address: &Address) -> Result<Option<Address>, ApiError> {
        if address.space != "cpu" {
            return Err(ApiError::unsupported("translation source space"));
        }
        if address.offset > 65535 || !address.context.is_empty() {
            return Err(ApiError::invalid(
                "CPU translation needs a plain 16-bit mapped address",
            ));
        }
        Ok(self.mapped_address(address.offset as u16))
    }
    fn inspect(&mut self, address: &Address, length: usize) -> Result<Vec<u8>, ApiError> {
        if address.space == "cpu" {
            if !address.context.is_empty() || address.offset > 65535 {
                return Err(ApiError::invalid(
                    "CPU inspection needs a plain 16-bit mapped address",
                ));
            }
            return self.peek(address.offset as u16, length).map_err(error);
        }
        if address.space == "io" {
            let end = address
                .offset
                .checked_add(length as u64)
                .ok_or_else(|| ApiError::invalid("I/O range overflow"))?;
            if !address.context.is_empty()
                || !((address.offset < 128 && end <= 128) || (address.offset == 255 && end <= 256))
            {
                return Err(ApiError::invalid("I/O range outside register space"));
            }
            return self
                .peek((0xff00 + address.offset) as u16, length)
                .map_err(error);
        }
        let bank_bytes = match address.space.as_str() {
            "cartridge-rom" => Some(16384),
            "cartridge-ram" | "vram" => Some(8192),
            "wram" => Some(4096),
            "oam" | "hram" | "cartridge-rtc" => None,
            _ => return Err(ApiError::unsupported("physical memory space")),
        };
        if address.context.keys().any(|k| k != "bank") {
            return Err(ApiError::unsupported("address context"));
        }
        if let Some(bank) = address.context.get("bank")
            && bank_bytes
                .is_none_or(|width| bank.parse::<u64>().ok() != Some(address.offset / width))
        {
            return Err(ApiError::invalid(
                "bank context disagrees with physical offset",
            ));
        }
        let offset = usize::try_from(address.offset)
            .map_err(|_| ApiError::invalid("physical offset outside host range"))?;
        self.physical_memory(&address.space, offset, length)
            .map_err(error)
    }
    fn configure_instrumentation(&mut self, config: Self::Instrumentation) -> Result<(), ApiError> {
        self.configure_trace(config).map_err(error)
    }
    fn events(&self, offset: usize, limit: usize) -> Result<Vec<TraceEvent>, ApiError> {
        if limit > 1000 {
            return Err(ApiError::invalid("event limit exceeds 1000"));
        }
        Ok(self.trace.query(offset, limit))
    }
}
