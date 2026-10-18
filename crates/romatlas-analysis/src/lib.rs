//! Platform-neutral bounded event capture and execution summaries.
use romatlas_core::{Address, EventKind, TraceEvent};
use serde::Serialize;
pub mod stream;
use std::collections::{BTreeMap, BTreeSet, VecDeque};
#[derive(Clone, Debug)]
pub struct TraceConfig {
    pub enabled: bool,
    pub capacity: usize,
    pub kinds: BTreeSet<EventKind>,
    pub space: Option<String>,
    pub range: Option<(u64, u64)>,
}
impl Default for TraceConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            capacity: 10000,
            kinds: BTreeSet::new(),
            space: None,
            range: None,
        }
    }
}
#[derive(Default)]
pub struct Trace {
    pub config: TraceConfig,
    events: VecDeque<TraceEvent>,
    counts: BTreeMap<EventKind, u64>,
    coverage: BTreeMap<Address, u64>,
    edges: BTreeMap<(Address, Address), u64>,
    contextual_roots: BTreeMap<Address, u64>,
    contextual_edges: BTreeMap<(Address, Address), u64>,
    pub dropped: u64,
    pub aggregate_dropped: u64,
    stream: Option<stream::Stream>,
}
#[derive(Serialize)]
pub struct Summary {
    pub retained: usize,
    pub dropped: u64,
    pub counts: BTreeMap<EventKind, u64>,
    pub unique_instructions: usize,
    pub unique_edges: usize,
    pub unique_contextual_roots: usize,
    pub unique_contextual_edges: usize,
    pub aggregate_dropped: u64,
    pub stream: Option<stream::StreamStatus>,
}
impl Trace {
    pub fn start_stream(&mut self, path: &std::path::Path, max_bytes: u64) -> Result<(), String> {
        if self.stream.is_some() {
            return Err("stop the existing stream before opening another".into());
        }
        self.stream = Some(stream::Stream::open(path, max_bytes)?);
        Ok(())
    }
    pub fn stop_stream(&mut self) -> Option<stream::StreamStatus> {
        self.stream.take().map(|s| {
            let mut status = s.status;
            status.stopped = true;
            status
        })
    }
    pub fn configure(&mut self, config: TraceConfig) -> Result<(), String> {
        if config.capacity > 100000 {
            return Err("trace capacity exceeds 100000".into());
        }
        if config.range.is_some_and(|(a, b)| a >= b) {
            return Err("trace range must be nonempty".into());
        }
        self.config = config;
        while self.events.len() > self.config.capacity {
            self.events.pop_front();
            self.dropped += 1;
        }
        Ok(())
    }
    pub fn capture(&mut self, event: TraceEvent) {
        if !self.config.enabled {
            return;
        }
        *self.counts.entry(event.kind.clone()).or_default() += 1;
        if matches!(event.kind, EventKind::ControlFlow | EventKind::Instruction) {
            let source = event
                .context
                .get("traversal_source")
                .and_then(|v| serde_json::from_value::<Address>(v.clone()).ok())
                .filter(|a| a.space == event.address.space && a.offset == event.address.offset);
            if let Some(source) = source {
                if self.contextual_roots.len() < 100000
                    || self.contextual_roots.contains_key(&source)
                {
                    *self.contextual_roots.entry(source.clone()).or_default() += 1;
                } else {
                    self.aggregate_dropped += 1;
                }
                if event
                    .context
                    .get("traversable")
                    .and_then(serde_json::Value::as_bool)
                    != Some(false)
                    && let Some(target) = event
                        .context
                        .get("traversal_target")
                        .and_then(|v| serde_json::from_value::<Address>(v.clone()).ok())
                        .filter(|a| {
                            event
                                .target
                                .as_ref()
                                .is_some_and(|t| t.space == a.space && t.offset == a.offset)
                        })
                {
                    let key = (source, target);
                    if self.contextual_edges.len() < 100000
                        || self.contextual_edges.contains_key(&key)
                    {
                        *self.contextual_edges.entry(key).or_default() += 1;
                    } else {
                        self.aggregate_dropped += 1;
                    }
                }
            }
        }
        if event.kind == EventKind::Instruction {
            if self.coverage.len() < 100000 || self.coverage.contains_key(&event.address) {
                *self.coverage.entry(event.address.clone()).or_default() += 1;
            } else {
                self.aggregate_dropped += 1;
            }
        }
        if event.kind == EventKind::ControlFlow
            && event
                .context
                .get("traversable")
                .and_then(serde_json::Value::as_bool)
                != Some(false)
            && let Some(target) = &event.target
        {
            let key = (event.address.clone(), target.clone());
            if self.edges.len() < 100000 || self.edges.contains_key(&key) {
                *self.edges.entry(key).or_default() += 1;
            } else {
                self.aggregate_dropped += 1;
            }
        }
        if (!self.config.kinds.is_empty() && !self.config.kinds.contains(&event.kind))
            || self
                .config
                .space
                .as_ref()
                .is_some_and(|s| s != &event.address.space)
            || self
                .config
                .range
                .is_some_and(|(a, b)| event.address.offset < a || event.address.offset >= b)
        {
            return;
        }
        if let Some(stream) = &mut self.stream {
            stream.capture(&event);
        }
        if self.config.capacity == 0 {
            self.dropped += 1;
            return;
        }
        if self.events.len() == self.config.capacity {
            self.events.pop_front();
            self.dropped += 1;
        }
        self.events.push_back(event);
    }
    pub fn summary(&self) -> Summary {
        Summary {
            retained: self.events.len(),
            dropped: self.dropped,
            counts: self.counts.clone(),
            unique_instructions: self.coverage.len(),
            unique_edges: self.edges.len(),
            unique_contextual_roots: self.contextual_roots.len(),
            unique_contextual_edges: self.contextual_edges.len(),
            aggregate_dropped: self.aggregate_dropped,
            stream: self.stream.as_ref().map(|s| s.status.clone()),
        }
    }
    pub fn query(&self, offset: usize, limit: usize) -> Vec<TraceEvent> {
        self.events
            .iter()
            .skip(offset)
            .take(limit.min(1000))
            .cloned()
            .collect()
    }
    pub fn coverage(&self) -> &BTreeMap<Address, u64> {
        &self.coverage
    }
    pub fn edges(&self) -> &BTreeMap<(Address, Address), u64> {
        &self.edges
    }
    pub fn contextual_roots(&self) -> &BTreeMap<Address, u64> {
        &self.contextual_roots
    }
    pub fn contextual_edges(&self) -> &BTreeMap<(Address, Address), u64> {
        &self.contextual_edges
    }
    pub fn clear(&mut self) {
        self.events.clear();
        self.counts.clear();
        self.coverage.clear();
        self.edges.clear();
        self.contextual_roots.clear();
        self.contextual_edges.clear();
        self.dropped = 0;
        self.aggregate_dropped = 0;
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    fn event(offset: u64) -> TraceEvent {
        TraceEvent {
            kind: EventKind::Instruction,
            cycle: 0,
            address: Address {
                space: "mock".into(),
                offset,
                context: BTreeMap::new(),
            },
            target: None,
            value: None,
            context: Default::default(),
        }
    }
    #[test]
    fn budget_filters_and_disable() {
        let mut t = Trace::default();
        t.capture(event(1));
        assert_eq!(t.summary().unique_instructions, 0);
        t.configure(TraceConfig {
            enabled: true,
            capacity: 2,
            range: Some((2, 9)),
            ..Default::default()
        })
        .unwrap();
        for i in 0..5 {
            t.capture(event(i));
        }
        assert_eq!(t.summary().unique_instructions, 5);
        assert_eq!(t.query(0, 10)[0].address.offset, 3);
        assert_eq!(t.dropped, 1);
        t.clear();
        assert_eq!(t.summary().retained, 0);
    }
    #[test]
    fn opaque_context_aggregates_survive_detail_filters() {
        let mut trace = Trace::default();
        trace
            .configure(TraceConfig {
                enabled: true,
                capacity: 0,
                kinds: BTreeSet::from([EventKind::Frame]),
                ..Default::default()
            })
            .unwrap();
        let mut e = event(0x1_0000_0000);
        e.kind = EventKind::ControlFlow;
        let mut source = e.address.clone();
        source
            .context
            .insert("opaque-state".into(), "before".into());
        let mut target = source.clone();
        target.offset += 2;
        target.context.insert("opaque-state".into(), "after".into());
        e.target = Some(Address {
            context: BTreeMap::new(),
            ..target.clone()
        });
        e.context = serde_json::json!({"traversable":true,"traversal_source":source,"traversal_target":target});
        trace.capture(e.clone());
        assert_eq!(trace.contextual_edges().len(), 1);
        assert_eq!(trace.contextual_roots().len(), 1);
        assert_eq!(trace.summary().retained, 0);
        e.context["traversable"] = false.into();
        trace.capture(e);
        assert_eq!(trace.contextual_edges().values().next(), Some(&1));
        assert_eq!(trace.contextual_roots().values().next(), Some(&2));
        trace.clear();
        assert!(trace.contextual_edges().is_empty() && trace.contextual_roots().is_empty());
    }
}

pub mod discovery;
