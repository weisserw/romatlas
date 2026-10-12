//! Translate actual core bus operations into shared trace records.
use crate::emulator::Registers;
use romatlas_core::{Address, EventKind, TraceEvent};
use std::collections::BTreeMap;
pub fn address(cpu: u16, physical: Option<usize>) -> Address {
    if let Some(offset) = physical {
        let mut context = BTreeMap::new();
        context.insert("bank".into(), (offset / 0x4000).to_string());
        Address {
            space: "cartridge-rom".into(),
            offset: offset as u64,
            context,
        }
    } else {
        Address {
            space: "cpu".into(),
            offset: cpu as u64,
            context: BTreeMap::new(),
        }
    }
}
pub fn location(location: mizu_core::memory::BusLocation) -> Address {
    let mut context = BTreeMap::new();
    if let Some(bank) = location.bank {
        context.insert("bank".into(), bank.to_string());
    }
    Address {
        space: location.space.into(),
        offset: location.offset as u64,
        context,
    }
}
pub fn bus_event(
    event: mizu_core::memory::BusEvent,
    cycle: u64,
    registers: &Registers,
) -> TraceEvent {
    let kind = match event.kind {
        0 => EventKind::Instruction,
        1 => EventKind::Fetch,
        2 => EventKind::Read,
        4 => EventKind::BankSwitch,
        5 => EventKind::Dma,
        6 => EventKind::Frame,
        _ => EventKind::Write,
    };
    let mapped = event.location.is_some();
    let mut effective = event
        .location
        .map(location)
        .unwrap_or_else(|| address(event.address, event.rom_offset));
    if mapped && event.address < 0x8000 && matches!(kind, EventKind::Write | EventKind::BankSwitch)
    {
        effective = address(event.address, None);
        effective.space = "cartridge-controller".into();
    }
    TraceEvent {
        kind,
        cycle: cycle + event.cycles as u64,
        address: effective,
        target: event
            .target_location
            .map(location)
            .or_else(|| event.target.map(|cpu| address(cpu, None)))
            .or_else(|| event.banks.map(|(_, after)| address(0x4000, Some(after)))),
        value: Some(event.value as u64),
        context: serde_json::json!({"cpu_address":event.address,"registers":registers,"bank_offsets":event.banks,"fixed_bank_offsets":event.fixed_banks,"blocked":event.blocked,"mapped":mapped,"cpu_target":event.target}),
    }
}
