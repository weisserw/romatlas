//! Bounded adapter for pinned MIT SingleStepTests corpora (downloaded locally).
use anyhow::{Result, ensure};
use serde_json::{Value, json};
use std::collections::BTreeMap;
use wdc65816_emu::core::{Registers, Wdc65816};

#[derive(Default)]
struct Bus {
    ram: BTreeMap<u32, u8>,
    cycles: usize,
}
impl Bus {
    fn new(state: &Value) -> Self {
        Self {
            ram: state["ram"]
                .as_array()
                .unwrap()
                .iter()
                .map(|v| (v[0].as_u64().unwrap() as u32, v[1].as_u64().unwrap() as u8))
                .collect(),
            cycles: 0,
        }
    }
    fn read(&mut self, a: u32) -> u8 {
        self.cycles += 1;
        self.ram.get(&a).copied().unwrap_or(0)
    }
    fn write(&mut self, a: u32, v: u8) {
        self.cycles += 1;
        self.ram.insert(a, v);
    }
    fn memory_matches(&self, state: &Value) -> bool {
        state["ram"].as_array().unwrap().iter().all(|v| {
            self.ram.get(&(v[0].as_u64().unwrap() as u32)) == Some(&(v[1].as_u64().unwrap() as u8))
        })
    }
}
impl wdc65816_emu::traits::BusInterface for Bus {
    fn read(&mut self, a: u32) -> u8 {
        self.read(a)
    }
    fn write(&mut self, a: u32, v: u8) {
        self.write(a, v)
    }
    fn idle(&mut self) {
        self.cycles += 1;
    }
    fn nmi(&self) -> bool {
        false
    }
    fn acknowledge_nmi(&mut self) {}
    fn irq(&self) -> bool {
        false
    }
    fn halt(&self) -> bool {
        false
    }
    fn reset(&self) -> bool {
        false
    }
}
impl spc700_emu::traits::BusInterface for Bus {
    fn read(&mut self, a: u16) -> u8 {
        self.read(u32::from(a))
    }
    fn write(&mut self, a: u16, v: u8) {
        self.write(u32::from(a), v)
    }
    fn idle(&mut self) {
        self.cycles += 1;
    }
}
fn n(v: &Value, k: &str) -> u16 {
    v[k].as_u64().unwrap() as u16
}
pub fn run(cpu: &str, path: &std::path::Path) -> Result<Value> {
    let cases: Vec<Value> = serde_json::from_slice(&std::fs::read(path)?)?;
    let mut tested = 0;
    for case in cases.iter().take(64) {
        let initial = &case["initial"];
        let expected = &case["final"];
        let mut bus = Bus::new(initial);
        let (actual, cycles) = if cpu == "65816" {
            let mut chip = Wdc65816::new();
            chip.set_registers(Registers {
                a: n(initial, "a"),
                x: n(initial, "x"),
                y: n(initial, "y"),
                s: n(initial, "s"),
                d: n(initial, "d"),
                pbr: n(initial, "pbr") as u8,
                dbr: n(initial, "dbr") as u8,
                pc: n(initial, "pc"),
                p: (n(initial, "p") as u8).into(),
                emulation_mode: n(initial, "e") != 0,
            });
            let mut ticks = 0;
            loop {
                chip.tick(&mut bus);
                ticks += 1;
                ensure!(ticks < 64, "CPU did not finish");
                if !chip.is_mid_instruction() {
                    break;
                }
            }
            let r = chip.registers();
            (
                json!({"a":r.a,"x":r.x,"y":r.y,"s":r.s,"d":r.d,"pbr":r.pbr,"dbr":r.dbr,"pc":r.pc,"p":u8::from(r.p),"e":u8::from(r.emulation_mode)}),
                ticks,
            )
        } else {
            ensure!(cpu == "spc700", "unknown CPU");
            let mut chip = spc700_emu::Spc700::new();
            chip.debug_set_registers(spc700_emu::Registers {
                a: n(initial, "a") as u8,
                x: n(initial, "x") as u8,
                y: n(initial, "y") as u8,
                sp: n(initial, "sp") as u8,
                pc: n(initial, "pc"),
                psw: (n(initial, "psw") as u8).into(),
            });
            let mut ticks = 0;
            loop {
                chip.tick(&mut bus);
                ticks += 1;
                ensure!(ticks < 64, "SPC did not finish");
                if !chip.debug_mid_instruction() {
                    break;
                }
            }
            let r = chip.debug_registers();
            (
                json!({"a":r.a,"x":r.x,"y":r.y,"sp":r.sp,"pc":r.pc,"psw":u8::from(r.psw)}),
                ticks,
            )
        };
        for (key, value) in actual.as_object().unwrap() {
            ensure!(
                *value == expected[key],
                "{} {}: {key}: got {value}, expected {}",
                cpu,
                case["name"],
                expected[key]
            );
        }
        ensure!(
            bus.memory_matches(expected),
            "{} {}: memory mismatch",
            cpu,
            case["name"]
        );
        ensure!(
            cycles == case["cycles"].as_array().unwrap().len(),
            "{} {}: cycle count {} != {}",
            cpu,
            case["name"],
            cycles,
            case["cycles"].as_array().unwrap().len()
        );
        tested += 1;
    }
    Ok(
        json!({"cpu":cpu,"file":path.file_name().unwrap().to_string_lossy(),"passed":tested,"checks":["registers","final_memory","cycle_count"],"bus_sequence_compared":false}),
    )
}

#[test]
fn pinned_cpu_vectors() {
    let Some(path) = std::env::var_os("ROMATLAS_SNES_CPU_TESTS") else {
        eprintln!(
            "CPU corpus not requested; run scripts/test-snes-cpu.sh for pinned external vectors"
        );
        return;
    };
    let root = std::path::Path::new(&path);
    let mut total = 0;
    for cpu in ["65816", "spc700"] {
        for file in std::fs::read_dir(root.join(cpu)).unwrap() {
            let file = file.unwrap().path();
            if file.extension().is_some_and(|s| s == "json")
                && file.file_name().unwrap() != "source.json"
            {
                total += run(cpu, &file).unwrap()["passed"].as_u64().unwrap();
            }
        }
    }
    assert_eq!(total, 832);
}
