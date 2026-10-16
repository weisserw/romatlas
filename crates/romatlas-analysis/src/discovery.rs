//! Bounded recursive traversal; adapters own decoding, bank rules, and references.
use romatlas_core::{Address, DecodeAdapter, Provenance, Reference, Region, RegionKind};
use serde::Serialize;
use std::collections::{BTreeMap, BTreeSet, VecDeque};
#[derive(Serialize, Debug)]
pub struct Diagnostic {
    pub address: Address,
    pub message: String,
}
#[derive(Serialize)]
pub struct Discovery {
    pub candidates: Vec<Region>,
    pub references: Vec<Reference>,
    pub diagnostics: Vec<Diagnostic>,
    pub visited: usize,
    pub budget_exhausted: bool,
}
pub fn discover(
    adapter: &impl DecodeAdapter,
    entries: Vec<Address>,
    observed: &BTreeSet<Address>,
    annotations: &[Region],
    budget: usize,
) -> Result<Discovery, String> {
    discover_with_edges(
        adapter,
        entries,
        observed,
        &BTreeMap::new(),
        annotations,
        budget,
    )
}
pub fn discover_with_edges(
    adapter: &impl DecodeAdapter,
    entries: Vec<Address>,
    observed: &BTreeSet<Address>,
    edges: &BTreeMap<(Address, Address), u64>,
    annotations: &[Region],
    budget: usize,
) -> Result<Discovery, String> {
    if edges.len() > 100000 {
        return Err("observed edge budget exceeds 100000".into());
    }
    let mut successors: BTreeMap<&Address, Vec<(&Address, u64)>> = BTreeMap::new();
    for ((source, target), hits) in edges {
        if *hits > 0 {
            successors.entry(source).or_default().push((target, *hits));
        }
    }
    if budget == 0 || budget > 100000 {
        return Err("discovery budget must be 1..100000 instructions".into());
    }
    if entries.len() > 100000 {
        return Err("entry point budget exceeds 100000".into());
    }
    let mut queue: VecDeque<_> = entries.into();
    let mut seen = BTreeSet::new();
    let mut claims: BTreeMap<Address, u64> = BTreeMap::new();
    let mut result = Discovery {
        candidates: Vec::new(),
        references: Vec::new(),
        diagnostics: Vec::new(),
        visited: 0,
        budget_exhausted: false,
    };
    while let Some(address) = queue.pop_front() {
        if seen.contains(&address) {
            continue;
        }
        if seen.len() == budget {
            result.budget_exhausted = true;
            break;
        }
        seen.insert(address.clone());
        result.visited = seen.len();
        let fail = |result: &mut Discovery, message: String| {
            result.diagnostics.push(Diagnostic {
                address: address.clone(),
                message,
            })
        };
        let instruction = match adapter.decode_structure(&address) {
            Ok(d) => d,
            Err(e) => {
                fail(&mut result, e);
                continue;
            }
        };
        let physical = adapter.canonical_address(&address);
        let Some(end) = physical
            .offset
            .checked_add(instruction.length)
            .filter(|_| instruction.length > 0)
        else {
            fail(&mut result, "invalid decoded length".into());
            continue;
        };
        let overlaps = |start: &Address, length: u64| {
            start.space == physical.space
                && start.context == physical.context
                && start.offset < end
                && physical.offset < start.offset.saturating_add(length)
        };
        if claims.iter().any(|(a, length)| {
            overlaps(a, *length) && (a != &physical || *length != instruction.length)
        }) {
            fail(
                &mut result,
                "target overlaps an existing instruction boundary".into(),
            );
            continue;
        }
        if annotations.iter().any(|r| {
            r.kind != RegionKind::Code
                && r.kind != RegionKind::Unknown
                && overlaps(&r.start, r.length)
        }) {
            fail(
                &mut result,
                "instruction conflicts with a non-code annotation".into(),
            );
            continue;
        }
        if claims
            .insert(physical.clone(), instruction.length)
            .is_none()
        {
            result.candidates.push(Region {
                start: physical.clone(),
                length: instruction.length,
                kind: RegionKind::Code,
                provenance: if observed.contains(&physical) {
                    Provenance::Observed
                } else {
                    Provenance::Static
                },
                confidence: if observed.contains(&physical) {
                    1.0
                } else {
                    0.7
                },
            });
        }
        for message in instruction.diagnostics {
            fail(&mut result, message);
        }
        // Bound branching fanout as well as processed nodes; no unbounded adapter output.
        if instruction.successors.len() > 16 || instruction.references.len() > 16 {
            return Err("adapter exceeds instruction edge/reference budget".into());
        }
        if let Some(targets) = successors.get(&adapter.observation_address(&address)) {
            for (target, hits) in targets {
                result.references.push(Reference {
                    source: adapter.canonical_address(&address),
                    target: Some(adapter.canonical_address(target)),
                    kind: "observed-successor".into(),
                    context: serde_json::json!({"provenance":"Observed","hits":hits,"traversal_source":address,"traversal_target":target}),
                });
                if queue.len() < 100000 {
                    queue.push_back((*target).clone());
                } else {
                    result.budget_exhausted = true;
                }
            }
        }
        for successor in instruction.successors {
            if queue.len() < 100000 {
                queue.push_back(successor);
            } else {
                result.budget_exhausted = true;
            }
        }
        result.references.extend(instruction.references);
    }
    result.candidates.sort_by(|a, b| a.start.cmp(&b.start));
    Ok(result)
}
#[cfg(test)]
mod tests {
    use super::*;
    use romatlas_core::{ImageAdapter, StructuralInstruction};
    struct Mock;
    struct ContextMock;
    impl ImageAdapter for ContextMock {
        fn platform(&self) -> &str {
            "context-mock"
        }
        fn image_size(&self) -> u64 {
            u64::MAX
        }
        fn physical_address(&self, n: u64) -> Option<Address> {
            Some(address(n))
        }
        fn peek(&self, _: &Address, _: usize) -> Result<Vec<u8>, String> {
            Ok(vec![])
        }
    }
    impl DecodeAdapter for ContextMock {
        fn canonical_address(&self, node: &Address) -> Address {
            address(node.offset)
        }
        fn decode_structure(&self, _: &Address) -> Result<StructuralInstruction, String> {
            Ok(StructuralInstruction {
                length: 2,
                successors: vec![],
                references: vec![],
                diagnostics: vec![],
            })
        }
    }
    fn address(n: u64) -> Address {
        Address {
            space: "wide".into(),
            offset: n,
            context: BTreeMap::new(),
        }
    }
    impl ImageAdapter for Mock {
        fn platform(&self) -> &str {
            "mock"
        }
        fn image_size(&self) -> u64 {
            u64::MAX
        }
        fn physical_address(&self, n: u64) -> Option<Address> {
            Some(address(n))
        }
        fn peek(&self, _: &Address, _: usize) -> Result<Vec<u8>, String> {
            Ok(vec![])
        }
    }
    impl DecodeAdapter for Mock {
        fn decode_structure(&self, a: &Address) -> Result<StructuralInstruction, String> {
            Ok(StructuralInstruction {
                length: 2,
                successors: if a.offset == 0x1_0000_0000 {
                    vec![address(a.offset + 1), address(a.offset + 2)]
                } else {
                    vec![]
                },
                references: vec![],
                diagnostics: vec![],
            })
        }
    }
    #[test]
    fn mock_wide_boundaries() {
        let root = address(0x1_0000_0000);
        let r = discover(&Mock, vec![root.clone()], &BTreeSet::from([root]), &[], 10).unwrap();
        assert_eq!(r.candidates.len(), 2);
        assert_eq!(r.diagnostics.len(), 1);
        assert_eq!(r.candidates[0].provenance, Provenance::Observed);
    }
    #[test]
    fn noncode_annotation_stops_traversal() {
        let root = address(0x1_0000_0000);
        let data = Region {
            start: root.clone(),
            length: 4,
            kind: RegionKind::Data,
            provenance: Provenance::Manual,
            confidence: 1.0,
        };
        let result = discover(&Mock, vec![root], &BTreeSet::new(), &[data], 10).unwrap();
        assert!(result.candidates.is_empty());
        assert!(result.diagnostics[0].message.contains("non-code"));
    }
    #[test]
    fn budgets() {
        assert!(discover(&Mock, vec![], &BTreeSet::new(), &[], 0).is_err());
        let r = discover(
            &Mock,
            vec![address(0x1_0000_0000)],
            &BTreeSet::new(),
            &[],
            1,
        )
        .unwrap();
        assert!(r.budget_exhausted);
    }
    #[test]
    fn traversal_states_share_physical_claims_and_coverage() {
        let physical = address(0x1_0000_0000);
        let roots = ["one", "two"].map(|state| {
            let mut node = physical.clone();
            node.context.insert("opaque-state".into(), state.into());
            node
        });
        let result = discover(
            &ContextMock,
            roots.to_vec(),
            &BTreeSet::from([physical.clone()]),
            &[],
            10,
        )
        .unwrap();
        assert_eq!(result.visited, 2);
        assert_eq!(result.candidates.len(), 1);
        assert_eq!(result.candidates[0].start, physical);
        assert_eq!(result.candidates[0].provenance, Provenance::Observed);
        let barrier = Region {
            start: physical,
            length: 2,
            kind: RegionKind::Data,
            provenance: Provenance::Manual,
            confidence: 1.0,
        };
        assert!(
            discover(
                &ContextMock,
                roots.to_vec(),
                &BTreeSet::new(),
                &[barrier],
                10
            )
            .unwrap()
            .candidates
            .is_empty()
        );
        assert!(
            discover(&ContextMock, roots.to_vec(), &BTreeSet::new(), &[], 1)
                .unwrap()
                .budget_exhausted
        );
    }
}
