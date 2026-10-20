//! ROMAtlas fork: optional, bounded, thread-local observation of actual core calls.
//! No callbacks or inspector reads execute emulated bus operations.
use std::cell::{Cell, RefCell};
#[derive(Clone, Debug)]
pub struct Event {
    pub physical: Option<(&'static str, u64)>,
    pub kind: &'static str,
    pub cpu: &'static str,
    pub origin: &'static str,
    pub cycle: u64,
    pub address: u32,
    pub value: u8,
    pub space: &'static str,
    pub registers: Option<[u32; 10]>,
}
#[derive(Default)]
struct Probe {
    enabled: bool,
    cpu: &'static str,
    origin: &'static str,
    cycle: u64,
    space: &'static str,
    events: Vec<Event>,
    dropped: u64,
    fetches: [u64; 2],
}
thread_local! {
    static PROBE: RefCell<Probe> = RefCell::default();
    static PHYSICAL: Cell<Option<(&'static str, u64)>> = const { Cell::new(None) };
    static SUSPENDED: Cell<bool> = const { Cell::new(false) };
}
/// Fast untraced execution deliberately records neither fetch counts nor evidence.
pub fn suspend(value: bool) {
    SUSPENDED.set(value);
}

pub fn begin(enabled: bool, cycle: u64) {
    PROBE.with_borrow_mut(|p| {
        p.enabled = enabled;
        p.cpu = "main";
        p.origin = "cpu";
        p.cycle = cycle;
        p.events.clear();
        p.dropped = 0;
    });
}
pub fn context(cpu: &'static str, origin: &'static str, cycle: u64) {
    if SUSPENDED.get() {
        return;
    }
    PROBE.with_borrow_mut(|p| {
        p.cpu = cpu;
        p.origin = origin;
        p.cycle = cycle;
        p.space = if cpu == "main" { "cpu" } else { "spc_cpu" };
    });
}
pub fn emit(kind: &'static str, address: u32, value: u8, registers: Option<[u32; 10]>) {
    if SUSPENDED.get() {
        return;
    }
    PROBE.with_borrow_mut(|p| {
        if kind == "instruction" {
            let i = usize::from(p.cpu == "spc700");
            p.fetches[i] = p.fetches[i].saturating_add(1);
        }
        // Fetch observations also feed bounded coverage/edges with detail disabled.
        if !p.enabled {
            return;
        }
        if p.events.len() == 1024 {
            p.dropped += 1;
            return;
        }
        p.events.push(Event {
            physical: PHYSICAL.get(),
            kind,
            cpu: p.cpu,
            origin: p.origin,
            cycle: p.cycle,
            address,
            value,
            registers,
            space: p.space,
        });
    });
}
pub fn fetches() -> [u64; 2] {
    PROBE.with_borrow(|p| p.fetches)
}
pub fn drain() -> (Vec<Event>, u64) {
    PROBE.with_borrow_mut(|p| (std::mem::take(&mut p.events), p.dropped))
}

pub fn space(space: &'static str) {
    if SUSPENDED.get() {
        return;
    }
    PROBE.with_borrow_mut(|p| p.space = space);
}

/// NES: binding captured by actual backing-storage access, never byte matching.
pub fn binding(value: Option<(&'static str, u64)>) {
    PHYSICAL.set(value);
}
pub fn last_binding() -> Option<(&'static str, u64)> {
    PHYSICAL.get()
}

/// Emit a bus event with a local origin, preserving the caller's probe context.
pub fn bus_event(kind: &'static str, origin: &'static str, address: u32, value: u8) {
    let previous = PROBE.with_borrow_mut(|p| {
        let old = p.origin;
        p.origin = origin;
        old
    });
    emit(kind, address, value, None);
    PROBE.with_borrow_mut(|p| p.origin = previous);
}

pub fn with_origin<T>(origin: &'static str, f: impl FnOnce() -> T) -> T {
    let previous = PROBE.with_borrow_mut(|p| {
        let old = p.origin;
        p.origin = origin;
        old
    });
    let result = f();
    PROBE.with_borrow_mut(|p| p.origin = previous);
    result
}
