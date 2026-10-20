//! Bounded bank-aware hypotheses, separate from classification and execution coverage.
use crate::{
    Rom,
    decode::{self, Context, Cpu},
    mapping::{Mapping, View, resolve_cpu},
};
use anyhow::{Context as _, Result, ensure};
use romatlas_core::{Address, RegionKind};
use serde_json::{Value, json};
use std::collections::{BTreeMap, BTreeSet, VecDeque};
fn observed(a: &Address) -> Option<View> {
    Some(View {
        cpu: if a.context.get("cpu").is_some_and(|v| v == "z80") {
            Cpu::Z80
        } else {
            Cpu::Main
        },
        cpu_address: a.context.get("cpu_address")?.parse().ok()?,
        context: serde_json::from_str(a.context.get("cpu_context")?).ok()?,
        uploaded: false,
    })
}
fn compatible(a: Context, b: Context) -> bool {
    (a.ram_enabled.is_none() || b.ram_enabled.is_none() || a.ram_enabled == b.ram_enabled)
        && (a.z80_bank.is_none() || b.z80_bank.is_none() || a.z80_bank == b.z80_bank)
        && a.rom_banks
            .iter()
            .zip(b.rom_banks)
            .all(|(a, b)| a.is_none() || b.is_none() || *a == b)
}
pub fn discover(
    rom: &Rom,
    map: &Mapping,
    p: &Value,
    roots: &BTreeMap<Address, u64>,
    edges: &BTreeMap<(Address, Address), u64>,
) -> Result<Value> {
    let num = |k: &str, d: u64| -> Result<usize> {
        Ok(usize::try_from(
            p.get(k)
                .map(|v| v.as_u64().with_context(|| format!("{k} must be unsigned")))
                .unwrap_or(Ok(d))?,
        )?)
    };
    let tool_budget = romatlas_disasm::Budget::new(num("timeout_ms", 1000)? as u64)?;
    let budget = num("max_instructions", 1000)?;
    let limit = num("limit", 100)?;
    let offset = num("offset", 0)?;
    ensure!(
        (1..=100000).contains(&budget) && limit <= 1000,
        "analysis budget/page limit"
    );
    let use_observed = p
        .get("use_observed")
        .map(|v| v.as_bool().context("use_observed must be boolean"))
        .transpose()?
        .unwrap_or(true);
    let entries = p["entrypoints"]
        .as_array()
        .context("entrypoints array required")?;
    ensure!(entries.len() <= 1000, "entrypoint limit");
    let mut queue = VecDeque::new();
    for e in entries {
        let at = e["offset"].as_u64().context("entry offset required")? as usize;
        let view: View = if let Some(v) = e.get("view") {
            serde_json::from_value(v.clone())?
        } else {
            View {
                cpu: Cpu::Main,
                cpu_address: match e.get("cpu_address") {
                    Some(v) => u32::try_from(v.as_u64().context("cpu_address unsigned")?)?,
                    None => rom.canonical_cpu(at)?,
                },
                context: serde_json::from_value(
                    e.get("context")
                        .or_else(|| e.get("cpu_context"))
                        .cloned()
                        .unwrap_or(json!({})),
                )?,
                uploaded: false,
            }
        };
        queue.push_back((at, view));
    }
    let mut indexed_edges = BTreeMap::<u64, Vec<_>>::new();
    if use_observed {
        for (source, target) in edges.keys() {
            if source.space == "rom" {
                indexed_edges
                    .entry(source.offset)
                    .or_default()
                    .push((source, target));
            }
        }
    }
    let mut seen = BTreeSet::new();
    let mut budget_exhausted = false;
    let mut candidates = Vec::new();
    let mut diagnostics = Vec::new();
    let mut references = Vec::new();
    while let Some((at, view)) = queue.pop_front() {
        if !seen.insert(serde_json::to_string(&(at, &view))?) {
            continue;
        }
        if seen.len() > budget {
            budget_exhausted = true;
            break;
        }
        if use_observed {
            for (a, _) in roots
                .range(crate::image::address("rom", at as u64)..)
                .take(16)
            {
                if a.space != "rom" || a.offset != at as u64 {
                    break;
                }
                if let Some(v) = observed(a)
                    && v.cpu == view.cpu
                    && v.cpu_address == view.cpu_address
                    && v.context != view.context
                    && compatible(view.context, v.context)
                {
                    queue.push_back((at, v));
                }
            }
        }
        if view.uploaded
            || !resolve_cpu(rom, view.cpu, view.cpu_address, view.context)
                .is_some_and(|a| a.space == "rom" && a.offset == at as u64)
        {
            diagnostics.push(
                json!({"offset":at,"message":"unresolved/conflicting CPU-window bank context"}),
            );
            continue;
        }
        if at >= rom.bytes.len() {
            continue;
        }
        tool_budget.check()?;
        let i = match decode::decode_with_budget(
            view.cpu,
            &rom.bytes[at..rom.bytes.len()],
            view.cpu_address,
            view.context,
            &tool_budget,
        ) {
            Ok(i) => i,
            Err(e) => {
                diagnostics.push(json!({"offset":at,"message":e.to_string()}));
                continue;
            }
        };
        if (0..i.length).any(|byte| {
            !resolve_cpu(
                rom,
                view.cpu,
                (view.cpu_address + byte as u32) & 0xffffff,
                view.context,
            )
            .is_some_and(|a| a.space == "rom" && a.offset == (at + byte) as u64)
        }) {
            diagnostics.push(
                json!({"offset":at,"message":"instruction crosses a noncontiguous CPU window"}),
            );
            continue;
        }
        if map.regions.iter().any(|r| {
            !matches!(r.kind, RegionKind::Code | RegionKind::Unknown)
                && r.offset < at + i.length
                && at < r.offset + r.length
        }) {
            diagnostics.push(json!({"offset":at,"message":"non-code annotation barrier"}));
            continue;
        }
        candidates.push(json!({"offset":at,"length":i.length,"view":view,"provenance":"Static","classification":"candidate","execution_coverage":false}));
        if i.references
            .iter()
            .any(|r| r["indirect"] == true && r["kind"] == "control")
        {
            diagnostics
                .push(json!({"offset":at,"message":"unresolved indirect CPU-window target"}));
        }
        for r in &i.references {
            let target = r["cpu_address"]
                .as_u64()
                .filter(|_| r["kind"] != "io" && r["indexed"] != true && r["indirect"] != true)
                .and_then(|pc| resolve_cpu(rom, view.cpu, pc as u32, view.context));
            references.push(json!({"offset":at,"reference":r,"target":target,"code_proven":false}));
        }
        // A store may change the mapper; a call may return with changed banks. Stop
        // static propagation across these unknown effects; observed edges can resume.
        let changes_banks = i.text.starts_with("jsr")
            || i.text.starts_with("bsr")
            || i.text.starts_with("call")
            || i.references.iter().any(|r| {
                r["write"] == true
                    && (r["indirect"] == true
                        || r["cpu_address"].as_u64().is_some_and(|a| {
                            if view.cpu == Cpu::Z80 {
                                (0x6000..=0x60ff).contains(&a) || a >= 0x8000
                            } else {
                                (0xa13000..=0xa130ff).contains(&a)
                            }
                        }))
            });
        if !changes_banks {
            for pc in i.successors {
                if let Some(a) = resolve_cpu(rom, view.cpu, pc, view.context) {
                    if a.space == "rom" {
                        queue.push_back((
                            a.offset as usize,
                            View {
                                cpu_address: pc,
                                ..view.clone()
                            },
                        ));
                    }
                } else {
                    diagnostics.push(json!({"offset":at,"cpu_address":pc,"message":"unresolved CPU-window target"}));
                }
            }
        }
        if use_observed {
            for (source, target) in indexed_edges
                .get(&(at as u64))
                .into_iter()
                .flatten()
                .take(16)
            {
                if source.space == "rom"
                    && source.offset == at as u64
                    && target.space == "rom"
                    && let (Some(s), Some(t)) = (observed(source), observed(target))
                    && s.cpu == view.cpu
                    && s.cpu_address == view.cpu_address
                    && s.context == view.context
                {
                    queue.push_back((target.offset as usize, t));
                }
            }
        }
        if queue.len() > 100000 {
            break;
        }
    }
    let total = candidates.len();
    let total_references = references.len();
    let total_diagnostics = diagnostics.len();
    Ok(
        json!({"candidates":candidates.into_iter().skip(offset).take(limit).collect::<Vec<_>>(),"total":total,"next_offset":offset.saturating_add(limit).min(total),"references":references.into_iter().skip(num("reference_offset",0)?).take(limit).collect::<Vec<_>>(),"total_references":total_references,"diagnostics":diagnostics.into_iter().skip(num("diagnostic_offset",0)?).take(limit).collect::<Vec<_>>(),"total_diagnostics":total_diagnostics,"truncated":budget_exhausted||!queue.is_empty(),"budget_exhausted":budget_exhausted,"visited":seen.len().min(budget),"map_changed":false}),
    )
}
