//! CPU-view-aware static hypotheses. No inference is promoted to observed coverage.
use crate::{
    Rom,
    decode::{self, Context, Cpu},
    mapping::{Mapping, View},
};
use anyhow::{Context as _, Result, ensure};
use serde_json::{Value, json};
use std::collections::{BTreeMap, BTreeSet, VecDeque};
pub fn discover(
    rom: &Rom,
    map: &Mapping,
    p: &Value,
    roots: &BTreeMap<romatlas_core::Address, u64>,
    edges: &BTreeMap<(romatlas_core::Address, romatlas_core::Address), u64>,
) -> Result<Value> {
    let use_observed = p
        .get("use_observed")
        .and_then(Value::as_bool)
        .unwrap_or(true);
    let budget = p
        .get("max_instructions")
        .or_else(|| p.get("budget"))
        .and_then(Value::as_u64)
        .unwrap_or(1000) as usize;
    ensure!(
        (1..=100000).contains(&budget),
        "instruction budget must be 1..100000"
    );
    let entries = p
        .get("entrypoints")
        .and_then(Value::as_array)
        .context("entrypoints must be an array")?;
    ensure!(entries.len() <= 1000, "entrypoints exceeds 1000");
    let mut observed_edges = BTreeMap::<usize, Vec<_>>::new();
    for ((source, target), hits) in edges {
        if source.space == "rom" {
            observed_edges
                .entry(source.offset as usize)
                .or_default()
                .push((source, target, hits));
        }
    }
    let mut truncated = false;
    let mut queue = VecDeque::new();
    for e in entries {
        let offset = e["offset"].as_u64().context("entry offset required")? as usize;
        let view: View = if let Some(v) = e.get("view") {
            serde_json::from_value(v.clone())?
        } else {
            View {
                cpu: Cpu::Main,
                cpu_address: e
                    .get("cpu_address")
                    .and_then(Value::as_u64)
                    .map(u32::try_from)
                    .transpose()?
                    .unwrap_or(rom.canonical_cpu(offset)?),
                context: serde_json::from_value(e.get("context").cloned().unwrap_or(json!({})))?,
                uploaded: false,
            }
        };
        let bounds = if view.uploaded {
            let length = e["length"]
                .as_u64()
                .context("uploaded code discovery requires confirmed storage length")?
                as usize;
            let end = offset
                .checked_add(length)
                .context("upload range overflow")?;
            ensure!(
                length > 0 && end <= rom.bytes.len(),
                "invalid upload bounds"
            );
            (offset, end)
        } else {
            (rom.info.copier_header_size, rom.bytes.len())
        };
        queue.push_back((offset, view, bounds));
    }
    let mut seen = BTreeSet::new();
    let mut candidates = Vec::new();
    let mut diagnostics = Vec::new();
    let mut references = Vec::new();
    while let Some((offset, view, bounds)) = queue.pop_front() {
        let key = serde_json::to_string(&(offset, &view, bounds))?;
        if !seen.insert(key) {
            continue;
        }
        if seen.len() > budget {
            break;
        }
        if offset < bounds.0 || offset >= bounds.1 {
            diagnostics.push(json!({"offset":offset,"message":"outside image"}));
            continue;
        }
        if use_observed {
            for (root_index, (root, _)) in roots
                .range(crate::image::address("rom", offset as u64)..)
                .enumerate()
            {
                if root.space != "rom" || root.offset != offset as u64 {
                    break;
                }
                if root_index == 16 {
                    truncated = true;
                    diagnostics.push(json!({"offset":offset,"message":"observed context fanout truncated at 16"}));
                    break;
                }
                if let Some(observed) = observed_view(root)
                    && observed.cpu == view.cpu
                    && observed.cpu_address == view.cpu_address
                    && compatible(view.context, observed.context)
                {
                    let merged = merge(view.context, observed.context);
                    if merged != view.context && queue.len() < 100000 {
                        queue.push_back((
                            offset,
                            View {
                                context: merged,
                                ..view.clone()
                            },
                            bounds,
                        ));
                    }
                }
            }
        }
        let instruction = match decode::decode(
            view.cpu,
            &rom.bytes[offset..bounds.1],
            view.cpu_address,
            view.context,
        ) {
            Ok(i) => i,
            Err(e) => {
                diagnostics.push(json!({"offset":offset,"view":view,"message":e.to_string()}));
                continue;
            }
        };
        if map.regions.iter().any(|r| {
            !matches!(
                r.kind,
                romatlas_core::RegionKind::Code | romatlas_core::RegionKind::Unknown
            ) && r.offset < offset + instruction.length
                && offset < r.offset + r.length
        }) {
            diagnostics.push(json!({"offset":offset,"message":"non-code annotation barrier"}));
            continue;
        }
        if view.cpu == Cpu::Main
            && !view.uploaded
            && rom.rom_offset(view.cpu_address).ok() != Some(offset)
        {
            diagnostics.push(
                json!({"offset":offset,"message":"CPU alias conflicts with physical ROM identity"}),
            );
            continue;
        }
        candidates.push(json!({"offset":offset,"length":instruction.length,"view":view,"provenance":"Static","classification":"candidate","execution_coverage":false}));
        let resolve = |pc: u32| -> Option<usize> {
            if view.cpu == Cpu::Main && !view.uploaded {
                rom.rom_offset(pc).ok()
            } else {
                let delta = i64::from(pc) - i64::from(view.cpu_address);
                usize::try_from(offset as i64 + delta)
                    .ok()
                    .filter(|o| *o >= bounds.0 && *o < bounds.1)
            }
        };
        let mut successors = Vec::new();
        if let Some(target) = instruction.target {
            let target_offset = resolve(target);
            references.push(json!({"offset":offset,"cpu_address":target,"target_offset":target_offset,"kind":if instruction.call{"call"}else{"branch"},"provenance":"Static"}));
            if let Some(o) = target_offset {
                successors.push((
                    o,
                    View {
                        cpu_address: target,
                        context: instruction.after,
                        ..view.clone()
                    },
                ));
            }
        }
        if !instruction.terminal {
            let next = (view.cpu_address & 0xff0000)
                | ((view.cpu_address + instruction.length as u32) & 0xffff);
            let ctx = if instruction.call {
                Context {
                    e: None,
                    m: None,
                    x: None,
                    carry: None,
                    db: None,
                    direct_page: None,
                }
            } else {
                instruction.after
            };
            if let Some(o) = resolve(next) {
                successors.push((
                    o,
                    View {
                        cpu_address: next,
                        context: ctx,
                        ..view.clone()
                    },
                ));
            }
        }
        if use_observed {
            for (edge_index, (source, target, hits)) in observed_edges
                .get(&offset)
                .into_iter()
                .flatten()
                .enumerate()
            {
                if edge_index == 16 {
                    truncated = true;
                    diagnostics.push(
                        json!({"offset":offset,"message":"observed edge fanout truncated at 16"}),
                    );
                    break;
                }
                if source.space != "rom" || source.offset != offset as u64 {
                    continue;
                }
                if let (Some(s), Some(t)) = (observed_view(source), observed_view(target))
                    && s.cpu == view.cpu
                    && s.cpu_address == view.cpu_address
                    && compatible(view.context, s.context)
                    && target.space == "rom"
                {
                    references.push(json!({"offset":offset,"target_offset":target.offset,"cpu_address":t.cpu_address,"kind":"observed-successor","hits":hits,"provenance":"Observed","artificial":source.context.get("artificial").is_some_and(|v|v=="true")}));
                    successors.push((target.offset as usize, t));
                }
            }
        }
        if instruction.indirect {
            diagnostics
                .push(json!({"offset":offset,"message":"unresolved indirect CPU-window target"}));
        }
        for successor in successors {
            if queue.len() < 100000 {
                queue.push_back((successor.0, successor.1, bounds));
            }
        }
    }
    Ok(
        json!({"candidates":candidates,"references":references,"diagnostics":diagnostics,"visited":seen.len().min(budget),"budget_exhausted":truncated||seen.len()>budget||!queue.is_empty(),"observed_edges_merged":use_observed}),
    )
}

fn observed_view(a: &romatlas_core::Address) -> Option<View> {
    Some(View {
        cpu: match a.context.get("cpu")?.as_str() {
            "main" => Cpu::Main,
            "spc700" => Cpu::Spc700,
            _ => return None,
        },
        cpu_address: a.context.get("cpu_address")?.parse().ok()?,
        context: serde_json::from_str(a.context.get("cpu_context")?).ok()?,
        uploaded: false,
    })
}
fn compatible(a: Context, b: Context) -> bool {
    fn agree<T: PartialEq>(a: Option<T>, b: Option<T>) -> bool {
        a.is_none() || b.is_none() || a == b
    }
    agree(a.e, b.e)
        && agree(a.m, b.m)
        && agree(a.x, b.x)
        && agree(a.carry, b.carry)
        && agree(a.db, b.db)
        && agree(a.direct_page, b.direct_page)
}
fn merge(a: Context, b: Context) -> Context {
    Context {
        e: a.e.or(b.e),
        m: a.m.or(b.m),
        x: a.x.or(b.x),
        carry: a.carry.or(b.carry),
        db: a.db.or(b.db),
        direct_page: a.direct_page.or(b.direct_page),
    }
}

/// Shared typed decoding entry point. The SNES discovery API additionally keeps
/// competing execution views separate rather than merging physical byte claims.
impl romatlas_core::DecodeAdapter for Rom {
    fn canonical_address(&self, a: &romatlas_core::Address) -> romatlas_core::Address {
        crate::image::address(&a.space, a.offset)
    }
    fn decode_structure(
        &self,
        a: &romatlas_core::Address,
    ) -> Result<romatlas_core::StructuralInstruction, String> {
        let decode = || -> Result<romatlas_core::StructuralInstruction> {
            ensure!(
                a.space == "rom",
                "structural decoding needs immutable ROM storage"
            );
            let offset = usize::try_from(a.offset)?;
            let view: View = if let Some(v) = a.context.get("view") {
                serde_json::from_str(v)?
            } else {
                View {
                    cpu: Cpu::Main,
                    cpu_address: self.canonical_cpu(offset)?,
                    context: serde_json::from_str(
                        a.context
                            .get("cpu_context")
                            .map(String::as_str)
                            .unwrap_or("{}"),
                    )?,
                    uploaded: false,
                }
            };
            let data =
                self.check_range(offset, (self.bytes.len().saturating_sub(offset)).min(4))?;
            let i = decode::decode(view.cpu, data, view.cpu_address, view.context)?;
            let mut successors = Vec::new();
            let mut references = Vec::new();
            let target = |pc: u32, context: Context| -> Result<Option<romatlas_core::Address>> {
                let target = if view.cpu == Cpu::Main && !view.uploaded {
                    self.rom_offset(pc).ok()
                } else {
                    let start = a
                        .context
                        .get("storage_start")
                        .context("uploaded traversal requires storage_start/storage_end")?
                        .parse::<usize>()?;
                    let end = a
                        .context
                        .get("storage_end")
                        .context("uploaded traversal requires storage_end")?
                        .parse::<usize>()?;
                    usize::try_from(offset as i64 + i64::from(pc) - i64::from(view.cpu_address))
                        .ok()
                        .filter(|n| *n >= start && *n < end && end <= self.bytes.len())
                };
                target
                    .map(|offset| {
                        let mut next = a.clone();
                        next.offset = offset as u64;
                        next.context.insert(
                            "view".into(),
                            serde_json::to_string(&View {
                                cpu_address: pc,
                                context,
                                ..view.clone()
                            })?,
                        );
                        Ok(next)
                    })
                    .transpose()
            };
            if let Some(pc) = i.target {
                let to = target(pc, i.after)?;
                references.push(romatlas_core::Reference {
                    source: self.canonical_address(a),
                    target: to.as_ref().map(|a| self.canonical_address(a)),
                    kind: if i.call { "call" } else { "branch" }.into(),
                    context: json!({"cpu_address":pc,"provenance":"Static"}),
                });
                if let Some(to) = to {
                    successors.push(to);
                }
            }
            if !i.terminal {
                let context = if i.call { Context::default() } else { i.after };
                let pc =
                    (view.cpu_address & 0xff0000) | ((view.cpu_address + i.length as u32) & 0xffff);
                if let Some(to) = target(pc, context)? {
                    successors.push(to);
                }
            }
            Ok(romatlas_core::StructuralInstruction {
                length: i.length as u64,
                successors,
                references,
                diagnostics: i.diagnostics,
            })
        };
        decode().map_err(|e| e.to_string())
    }
}
