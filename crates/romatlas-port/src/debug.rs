//! Device-free native port control. Replay checkpoints rebuild a fresh game;
//! they are not battery saves or emulator snapshots.
use crate::{
    Game,
    audio::SampleClock,
    graphics::Frame,
    input::{InputState, MultiInputState, PlayerInput},
    timing::TickRate,
};
use anyhow::{Result, ensure};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use std::{
    collections::{BTreeMap, BTreeSet},
    io::{BufRead, Read, Write},
    time::Instant,
};

const MAX_TICKS: u64 = 100_000;
const MAX_RECORDS: usize = 100_000;
const MAX_LINE: usize = 16 * 1024 * 1024;
#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Segment {
    ticks: u64,
    held: [BTreeSet<String>; 4],
}
#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Checkpoint {
    version: u32,
    identity: String,
    segments: Vec<Segment>,
    held: [BTreeSet<String>; 4],
    frame_sha256: String,
    audio_sha256: String,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct StateCondition {
    pointer: String,
    value: Value,
    #[serde(default = "equal_operator")]
    op: String,
}
fn equal_operator() -> String {
    "eq".into()
}
fn state_matches(state: &Value, conditions: &[StateCondition]) -> Result<bool> {
    let mut matched = true;
    for condition in conditions {
        let actual = state.pointer(&condition.pointer).ok_or_else(|| {
            anyhow::anyhow!("missing semantic observation: {}", condition.pointer)
        })?;
        let result = match condition.op.as_str() {
            "eq" => actual == &condition.value,
            "ne" => actual != &condition.value,
            "lt" | "le" | "gt" | "ge" => {
                let a = actual
                    .as_f64()
                    .ok_or_else(|| anyhow::anyhow!("numeric observation required"))?;
                let b = condition
                    .value
                    .as_f64()
                    .ok_or_else(|| anyhow::anyhow!("numeric expectation required"))?;
                match condition.op.as_str() {
                    "lt" => a < b,
                    "le" => a <= b,
                    "gt" => a > b,
                    _ => a >= b,
                }
            }
            _ => anyhow::bail!("unknown condition operator"),
        };
        matched &= result;
    }
    Ok(matched)
}
#[derive(Clone)]
struct Machine<G> {
    game: G,
    frame: Frame,
    clock: SampleClock,
    ticks: u64,
    held: [BTreeSet<String>; 4],
    previous: [BTreeSet<String>; 4],
    segments: Vec<Segment>,
    audio_hash: Sha256,
    healthy: bool,
    capture: Vec<f32>,
    dropped: u64,
}
impl<G: Game> Machine<G> {
    fn new(game: G, size: (u32, u32), rate: TickRate) -> Result<Self> {
        Ok(Self {
            game,
            frame: Frame::new(size.0, size.1)?,
            clock: SampleClock::new(rate, 48000)?,
            ticks: 0,
            held: Default::default(),
            previous: Default::default(),
            segments: Vec::new(),
            audio_hash: Sha256::new(),
            healthy: true,
            capture: Vec::new(),
            dropped: 0,
        })
    }
    fn step(&mut self) -> Result<()> {
        let players = std::array::from_fn(|i| PlayerInput {
            held: self.held[i].clone(),
            pressed: self.held[i]
                .difference(&self.previous[i])
                .cloned()
                .collect(),
            released: self.previous[i]
                .difference(&self.held[i])
                .cloned()
                .collect(),
        });
        let primary = InputState {
            held: players[0].held.clone(),
            pressed: players[0].pressed.clone(),
            released: players[0].released.clone(),
        };
        self.game
            .tick_players(&MultiInputState { primary, players });
        self.previous = self.held.clone();
        let count = self.clock.next_tick();
        let mut samples = Vec::with_capacity(count * 2);
        self.game.audio(count, 48000, &mut samples);
        self.ticks += 1;
        if samples.len() != count * 2 || !samples.iter().all(|v| v.is_finite()) {
            self.healthy = false;
            anyhow::bail!("invalid game audio output; reset required");
        }
        let capacity = 48000 * 2 * 10;
        let keep = samples.len().min(capacity - self.capture.len());
        self.capture.extend_from_slice(&samples[..keep]);
        self.dropped += ((samples.len() - keep) / 2) as u64;
        for s in samples {
            self.audio_hash.update(s.to_le_bytes());
        }
        Ok(())
    }
    fn render(&mut self) -> String {
        self.frame.clear([0, 0, 0, 255]);
        self.game.render(&mut self.frame);
        format!("{:x}", Sha256::digest(&self.frame.pixels))
    }
    fn record(&mut self, ticks: u64) {
        if ticks > 0 {
            self.segments.push(Segment {
                ticks,
                held: self.held.clone(),
            });
        }
    }
}
/// Persistent JSON-lines protocol. Actions are game logical actions, not host keys.
/// The identity must bind ROM, assets and port build. Factory reset must be deterministic.
/// Render must be observational; checkpoints replay tick/audio history and verify output.
pub fn serve<G: Game>(
    mut factory: impl FnMut() -> Result<G>,
    size: (u32, u32),
    rate: TickRate,
    actions: BTreeSet<String>,
    identity: String,
    observe: impl Fn(&G) -> Value,
) -> Result<()> {
    let stdin = std::io::stdin();
    let stdout = std::io::stdout();
    serve_io(
        &mut stdin.lock(),
        &mut stdout.lock(),
        &mut factory,
        size,
        rate,
        actions,
        identity,
        observe,
    )
}
/// Injectable transport for headless tests and embedding.
#[allow(clippy::too_many_arguments)]
pub fn serve_io<G: Game>(
    reader: &mut impl BufRead,
    writer: &mut impl Write,
    factory: impl FnMut() -> Result<G>,
    size: (u32, u32),
    rate: TickRate,
    actions: BTreeSet<String>,
    identity: String,
    observe: impl Fn(&G) -> Value,
) -> Result<()> {
    serve_io_impl(
        reader, writer, factory, size, rate, actions, identity, observe, None,
    )
}

/// Opt-in in-memory snapshots. Clone must isolate all mutable simulation state;
/// shared immutable assets are fine. Snapshots never serialize arbitrary objects.
pub fn serve_with_snapshots<G: Game + Clone>(
    factory: impl FnMut() -> Result<G>,
    size: (u32, u32),
    rate: TickRate,
    actions: BTreeSet<String>,
    identity: String,
    observe: impl Fn(&G) -> Value,
) -> Result<()> {
    serve_io_with_snapshots(
        &mut std::io::stdin().lock(),
        &mut std::io::stdout().lock(),
        factory,
        size,
        rate,
        actions,
        identity,
        observe,
    )
}

/// Injectable snapshot-capable transport for tests and headless embedding.
#[allow(clippy::too_many_arguments)]
pub fn serve_io_with_snapshots<G: Game + Clone>(
    reader: &mut impl BufRead,
    writer: &mut impl Write,
    factory: impl FnMut() -> Result<G>,
    size: (u32, u32),
    rate: TickRate,
    actions: BTreeSet<String>,
    identity: String,
    observe: impl Fn(&G) -> Value,
) -> Result<()> {
    serve_io_impl(
        reader,
        writer,
        factory,
        size,
        rate,
        actions,
        identity,
        observe,
        Some(Machine::<G>::clone),
    )
}

type CopyMachine<G> = fn(&Machine<G>) -> Machine<G>;

#[allow(clippy::too_many_arguments)]
fn serve_io_impl<G: Game>(
    reader: &mut impl BufRead,
    writer: &mut impl Write,
    mut factory: impl FnMut() -> Result<G>,
    size: (u32, u32),
    rate: TickRate,
    actions: BTreeSet<String>,
    identity: String,
    observe: impl Fn(&G) -> Value,
    copy_machine: Option<CopyMachine<G>>,
) -> Result<()> {
    ensure!(
        u64::from(size.0) * u64::from(size.1) * 4 <= 1024 * 1024,
        "debug framebuffer exceeds 1 MiB"
    );
    let mut machine = Machine::new(factory()?, size, rate)?;
    let mut snapshots: BTreeMap<String, Machine<G>> = BTreeMap::new();
    loop {
        let mut line = Vec::new();
        let read = (&mut *reader)
            .take((MAX_LINE + 1) as u64)
            .read_until(b'\n', &mut line)?;
        if read == 0 {
            return Ok(());
        }
        if line.len() > MAX_LINE {
            anyhow::bail!("debug request exceeds 16 MiB");
        }
        let request = serde_json::from_slice::<Value>(&line);
        let id = request
            .as_ref()
            .ok()
            .and_then(|v| v.get("id"))
            .cloned()
            .unwrap_or(Value::Null);
        let result = (|| -> Result<Value> {
            let request = request?;
            ensure!(request["jsonrpc"] == "2.0", "expected JSON-RPC 2.0");
            let method = request["method"]
                .as_str()
                .ok_or_else(|| anyhow::anyhow!("missing method"))?;
            let p = request.get("params").cloned().unwrap_or(json!({}));
            ensure!(p.is_object(), "params must be an object");
            let number = |key: &str, default: u64| -> Result<u64> {
                p.get(key).map_or(Ok(default), |v| {
                    v.as_u64()
                        .ok_or_else(|| anyhow::anyhow!("{key} must be unsigned integer"))
                })
            };
            let validate = |held: &[BTreeSet<String>; 4]| -> Result<()> {
                ensure!(
                    held.iter().flatten().all(|a| actions.contains(a)),
                    "unknown action"
                );
                Ok(())
            };
            match method {
                "api.describe" => {
                    let mut methods = vec![
                        "api.describe",
                        "port.state",
                        "port.input",
                        "port.advance",
                        "port.run_until",
                        "port.framebuffer",
                        "port.reset",
                        "port.audio",
                        "port.audio.clear",
                        "port.checkpoint.export",
                        "port.checkpoint.import",
                    ];
                    if copy_machine.is_some() {
                        methods.extend([
                            "port.snapshot.save",
                            "port.snapshot.load",
                            "port.snapshot.delete",
                        ]);
                    }
                    Ok(
                        json!({"protocol":"romatlas-native-debug-v1","identity":identity,"actions":actions,
                        "methods":methods,"checkpoint":"deterministic_replay",
                        "snapshot":if copy_machine.is_some() { "in_memory_clone" } else { "unsupported" },
                        "max_snapshots":16,"audio_rate":48000,"max_ticks":MAX_TICKS,"state_edits":false}),
                    )
                }
                "port.snapshot.save" | "port.snapshot.load" | "port.snapshot.delete" => {
                    let copy = copy_machine
                        .ok_or_else(|| anyhow::anyhow!("fast snapshots unsupported"))?;
                    let name = p["name"]
                        .as_str()
                        .ok_or_else(|| anyhow::anyhow!("snapshot name required"))?;
                    ensure!(
                        !name.is_empty() && name.len() <= 128,
                        "invalid snapshot name"
                    );
                    match method {
                        "port.snapshot.save" => {
                            ensure!(machine.healthy, "reset required after invalid game output");
                            ensure!(
                                snapshots.contains_key(name) || snapshots.len() < 16,
                                "snapshot limit is 16; delete an unused snapshot"
                            );
                            snapshots.insert(name.to_owned(), copy(&machine));
                        }
                        "port.snapshot.load" => {
                            let saved = snapshots
                                .get(name)
                                .ok_or_else(|| anyhow::anyhow!("unknown snapshot"))?;
                            machine = copy(saved);
                        }
                        _ => {
                            ensure!(snapshots.remove(name).is_some(), "unknown snapshot");
                        }
                    }
                    Ok(json!({"ok":true,"ticks":machine.ticks}))
                }
                "port.state" => Ok(
                    json!({"ticks":machine.ticks,"held":machine.held,"healthy":machine.healthy,"game":observe(&machine.game)}),
                ),
                "port.input" => {
                    let player = number("player", 0)?;
                    ensure!(player < 4, "player must be 0..3");
                    let held: BTreeSet<String> =
                        serde_json::from_value(p.get("actions").cloned().unwrap_or(json!([])))?;
                    ensure!(held.iter().all(|a| actions.contains(a)), "unknown action");
                    machine.held[player as usize] = held;
                    Ok(json!({"ok":true}))
                }
                "port.advance" | "port.run_until" => {
                    let conditional = method == "port.run_until";
                    let predicates: Vec<StateCondition> = if conditional {
                        serde_json::from_value(p.get("conditions").cloned().unwrap_or(Value::Null))?
                    } else {
                        Vec::new()
                    };
                    ensure!(
                        !conditional || (1..=32).contains(&predicates.len()),
                        "run_until requires 1..32 conditions"
                    );
                    if conditional {
                        state_matches(&observe(&machine.game), &predicates)?;
                    }
                    ensure!(machine.healthy, "reset required after invalid game output");
                    let count = number("ticks", 1)?;
                    let timeout = number("timeout_ms", 1000)?;
                    ensure!(
                        count <= MAX_TICKS.saturating_sub(machine.ticks)
                            && (1..=300000).contains(&timeout),
                        "advance budget exceeds limits"
                    );
                    ensure!(
                        machine.segments.len() < MAX_RECORDS,
                        "checkpoint journal full; reset or start another run"
                    );
                    let start = Instant::now();
                    let mut done = 0;
                    let mut matched =
                        conditional && state_matches(&observe(&machine.game), &predicates)?;
                    while !matched
                        && done < count
                        && start.elapsed().as_millis() < u128::from(timeout)
                    {
                        machine.step()?;
                        done += 1;
                        if conditional {
                            match state_matches(&observe(&machine.game), &predicates) {
                                Ok(value) => matched = value,
                                Err(error) => {
                                    machine.record(done);
                                    return Err(error);
                                }
                            }
                        }
                    }
                    machine.record(done);
                    if conditional {
                        Ok(
                            json!({"ticks":machine.ticks,"completed":done,"remaining":count-done,"matched":matched,
                            "reason":if matched {"matched"} else if done==count {"tick_limit"} else {"timeout"},"game":observe(&machine.game)}),
                        )
                    } else {
                        Ok(
                            json!({"ticks":machine.ticks,"completed":done,"remaining":count-done,"complete":done==count,"game":observe(&machine.game)}),
                        )
                    }
                }
                "port.framebuffer" => {
                    let hash = machine.render();
                    Ok(
                        json!({"width":machine.frame.width,"height":machine.frame.height,"format":"rgba8","sha256":hash,"pixels":machine.frame.pixels}),
                    )
                }
                "port.reset" => {
                    machine = Machine::new(factory()?, size, rate)?;
                    snapshots.clear();
                    Ok(json!({"ok":true}))
                }
                "port.audio" => {
                    let offset = number("offset", 0)? as usize;
                    let limit = number("limit", 4096)? as usize;
                    ensure!(limit <= 4096, "audio page limit is 4096 stereo frames");
                    let total = machine.capture.len() / 2;
                    let start = offset.min(total) * 2;
                    let end = (offset.min(total) + limit).min(total) * 2;
                    Ok(
                        json!({"sample_rate":48000,"channels":2,"frames":total,"dropped_frames":machine.dropped,"samples":machine.capture[start..end]}),
                    )
                }
                "port.audio.clear" => {
                    machine.capture.clear();
                    machine.dropped = 0;
                    Ok(json!({"ok":true}))
                }
                "port.checkpoint.export" => {
                    ensure!(machine.healthy, "reset required after invalid game output");
                    let frame_sha256 = machine.render();
                    Ok(serde_json::to_value(Checkpoint {
                        version: 1,
                        identity: identity.clone(),
                        segments: machine.segments.clone(),
                        held: machine.held.clone(),
                        frame_sha256,
                        audio_sha256: format!("{:x}", machine.audio_hash.clone().finalize()),
                    })?)
                }
                "port.checkpoint.import" => {
                    let checkpoint: Checkpoint = serde_json::from_value(p["checkpoint"].clone())?;
                    let timeout = number("timeout_ms", 1000)?;
                    ensure!((1..=300000).contains(&timeout), "invalid timeout");
                    ensure!(
                        checkpoint.version == 1 && checkpoint.identity == identity,
                        "checkpoint identity/version mismatch"
                    );
                    ensure!(
                        checkpoint.segments.len() <= MAX_RECORDS,
                        "too many checkpoint segments"
                    );
                    let mut total = 0u64;
                    for s in &checkpoint.segments {
                        validate(&s.held)?;
                        total = total
                            .checked_add(s.ticks)
                            .ok_or_else(|| anyhow::anyhow!("checkpoint tick overflow"))?;
                    }
                    ensure!(total <= MAX_TICKS, "checkpoint replay exceeds tick budget");
                    validate(&checkpoint.held)?;
                    let start = Instant::now();
                    let mut candidate = Machine::new(factory()?, size, rate)?;
                    for segment in &checkpoint.segments {
                        candidate.held = segment.held.clone();
                        for _ in 0..segment.ticks {
                            ensure!(
                                start.elapsed().as_millis() < u128::from(timeout),
                                "checkpoint replay timeout; live state preserved"
                            );
                            candidate.step()?;
                        }
                    }
                    ensure!(
                        candidate.render() == checkpoint.frame_sha256
                            && format!("{:x}", candidate.audio_hash.clone().finalize())
                                == checkpoint.audio_sha256,
                        "checkpoint replay output mismatch; live state preserved"
                    );
                    candidate.held = checkpoint.held;
                    candidate.segments = checkpoint.segments;
                    machine = candidate;
                    Ok(json!({"ok":true,"ticks":machine.ticks,"game":observe(&machine.game)}))
                }
                _ => anyhow::bail!("unknown native debug method"),
            }
        })();
        let reply = match result {
            Ok(value) => json!({"jsonrpc":"2.0","id":id,"result":value}),
            Err(error) => {
                json!({"jsonrpc":"2.0","id":id,"error":{"code":-32000,"message":format!("{error:#}")}})
            }
        };
        let encoded = serde_json::to_vec(&reply)?;
        if encoded.len() > MAX_LINE {
            serde_json::to_writer(
                &mut *writer,
                &json!({"jsonrpc":"2.0","id":id,"error":{"code":-32000,"message":"debug response exceeds 16 MiB"}}),
            )?;
        } else {
            writer.write_all(&encoded)?;
        }
        writeln!(writer)?;
        writer.flush()?;
    }
}

/// Bind replay checkpoints to this executable, ROM identity and override contents.
/// Inputs are read locally; no ROM bytes enter the checkpoint.
pub fn identity(rom_sha256: &str, assets: Option<&std::path::Path>) -> Result<String> {
    fn file(hash: &mut Sha256, path: &std::path::Path) -> Result<()> {
        let mut input = std::fs::File::open(path)?;
        let mut buffer = [0; 65536];
        loop {
            let n = input.read(&mut buffer)?;
            if n == 0 {
                break;
            }
            hash.update(&buffer[..n]);
        }
        Ok(())
    }
    fn directory(
        hash: &mut Sha256,
        root: &std::path::Path,
        path: &std::path::Path,
        budget: &mut (usize, u64),
    ) -> Result<()> {
        let mut entries = std::fs::read_dir(path)?.collect::<std::io::Result<Vec<_>>>()?;
        entries.sort_by_key(|e| e.file_name());
        for entry in entries {
            let meta = entry.file_type()?;
            ensure!(!meta.is_symlink(), "debug asset identity rejects symlinks");
            if meta.is_dir() {
                ensure!(
                    entry.path().components().count() - root.components().count() < 32,
                    "asset directory depth exceeds limit"
                );
                directory(hash, root, &entry.path(), budget)?;
            } else if meta.is_file() {
                budget.0 += 1;
                budget.1 += entry.metadata()?.len();
                ensure!(
                    budget.0 <= 10000 && budget.1 <= 64 * 1024 * 1024,
                    "asset identity exceeds limits"
                );
                hash.update(
                    entry
                        .path()
                        .strip_prefix(root)?
                        .to_string_lossy()
                        .as_bytes(),
                );
                hash.update(entry.metadata()?.len().to_le_bytes());
                file(hash, &entry.path())?;
            } else {
                anyhow::bail!("unsupported asset file type");
            }
        }
        Ok(())
    }
    let mut hash = Sha256::new();
    hash.update(b"romatlas-native-debug-v1");
    hash.update(rom_sha256.as_bytes());
    file(&mut hash, &std::env::current_exe()?)?;
    if let Some(root) = assets {
        directory(&mut hash, root, root, &mut (0, 0))?;
    }
    Ok(format!("{:x}", hash.finalize()))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[derive(Clone, Default)]
    struct Counter {
        position: i32,
        presses: u64,
        phase: u64,
    }
    impl Game for Counter {
        fn tick(&mut self, input: &InputState) {
            if input.held("right") {
                self.position += 1;
            }
            if input.pressed("right") {
                self.presses += 1;
            }
        }
        fn render(&self, frame: &mut Frame) {
            frame.pixel(0, 0, [self.position as u8, 0, 0, 255]);
        }
        fn audio(&mut self, n: usize, _: u32, out: &mut Vec<f32>) {
            for _ in 0..n {
                self.phase += 1;
                let value = (self.phase % 16) as f32 / 16.;
                out.extend([value, -value]);
            }
        }
    }
    fn run(requests: Vec<(&str, Value)>) -> Vec<Value> {
        let input = requests
            .into_iter()
            .enumerate()
            .map(|(id, (method, params))| {
                json!({"jsonrpc":"2.0","id":id,"method":method,"params":params}).to_string() + "\n"
            })
            .collect::<String>();
        let mut output = Vec::new();
        serve_io(
            &mut input.as_bytes(),
            &mut output,
            || Ok(Counter::default()),
            (2, 2),
            TickRate::DMG,
            BTreeSet::from(["right".into()]),
            "test-build".into(),
            |g| json!({"position":g.position,"presses":g.presses,"phase":g.phase}),
        )
        .unwrap();
        String::from_utf8(output)
            .unwrap()
            .lines()
            .map(|s| serde_json::from_str(s).unwrap())
            .collect()
    }
    fn run_snapshots(requests: Vec<(&str, Value)>) -> Vec<Value> {
        let input = requests
            .into_iter()
            .enumerate()
            .map(|(id, (method, params))| {
                json!({"jsonrpc":"2.0","id":id,"method":method,"params":params}).to_string() + "\n"
            })
            .collect::<String>();
        let mut output = Vec::new();
        serve_io_with_snapshots(
            &mut input.as_bytes(),
            &mut output,
            || Ok(Counter::default()),
            (2, 2),
            TickRate::DMG,
            BTreeSet::from(["right".into()]),
            "test-build".into(),
            |g| json!({"position":g.position,"presses":g.presses,"phase":g.phase}),
        )
        .unwrap();
        String::from_utf8(output)
            .unwrap()
            .lines()
            .map(|s| serde_json::from_str(s).unwrap())
            .collect()
    }
    #[test]
    fn snapshots_restore_clock_audio_edges_and_branch_journal_without_replay() {
        let replies = run_snapshots(vec![
            ("port.input", json!({"actions":["right"]})),
            ("port.advance", json!({"ticks":3})),
            ("port.snapshot.save", json!({"name":"branch"})),
            ("port.state", json!({})),
            ("port.audio", json!({})),
            ("port.input", json!({"actions":[]})),
            ("port.advance", json!({"ticks":7})),
            ("port.audio.clear", json!({})),
            ("port.snapshot.load", json!({"name":"branch"})),
            ("port.state", json!({})),
            ("port.audio", json!({})),
            ("port.advance", json!({"ticks":2})),
            ("port.checkpoint.export", json!({})),
            ("port.snapshot.load", json!({"name":"branch"})),
            ("port.advance", json!({"ticks":2})),
            ("port.checkpoint.export", json!({})),
            ("port.snapshot.delete", json!({"name":"branch"})),
            ("port.snapshot.load", json!({"name":"branch"})),
            ("port.state", json!({})),
        ]);
        assert_eq!(replies[3]["result"], replies[9]["result"]);
        assert_eq!(replies[4]["result"], replies[10]["result"]);
        assert_eq!(replies[12]["result"], replies[15]["result"]);
        assert_eq!(replies[14]["result"]["game"]["presses"], 1);
        assert_eq!(replies[14]["result"]["game"]["position"], 5);
        assert!(replies[17].get("error").is_some());
        assert_eq!(replies[18]["result"]["ticks"], 5);
    }

    #[test]
    fn snapshots_are_opt_in_bounded_and_cleared_on_reset() {
        assert!(
            run(vec![("port.snapshot.save", json!({"name":"x"}))])[0]
                .get("error")
                .is_some()
        );
        let mut requests = (0..17)
            .map(|n| ("port.snapshot.save", json!({"name":n.to_string()})))
            .collect::<Vec<_>>();
        requests.extend([
            ("port.snapshot.save", json!({"name":"0"})),
            ("port.snapshot.delete", json!({"name":"0"})),
            ("port.snapshot.save", json!({"name":"replacement"})),
            ("port.reset", json!({})),
            ("port.snapshot.load", json!({"name":"replacement"})),
        ]);
        let replies = run_snapshots(requests);
        assert!(replies[16].get("error").is_some());
        for n in [17, 18, 19, 20] {
            assert!(replies[n].get("result").is_some());
        }
        assert!(replies[21].get("error").is_some());
    }

    #[test]
    fn conditional_steps_stop_exactly_and_replay() {
        let replies = run(vec![
            ("port.input", json!({"actions":["right"]})),
            (
                "port.run_until",
                json!({"ticks":20,"conditions":[{"pointer":"/position","value":3,"op":"ge"}]}),
            ),
            ("port.checkpoint.export", json!({})),
            (
                "port.run_until",
                json!({"ticks":2,"conditions":[{"pointer":"/position","value":99}]}),
            ),
            (
                "port.run_until",
                json!({"ticks":2,"conditions":[{"pointer":"/missing","value":0}]}),
            ),
            ("port.state", json!({})),
        ]);
        assert_eq!(replies[1]["result"]["completed"], 3);
        assert_eq!(replies[1]["result"]["reason"], "matched");
        assert_eq!(replies[3]["result"]["reason"], "tick_limit");
        assert!(replies[4].get("error").is_some());
        assert_eq!(replies[5]["result"]["game"]["position"], 5);
        let replay = run(vec![
            (
                "port.checkpoint.import",
                json!({"checkpoint":replies[2]["result"]}),
            ),
            ("port.state", json!({})),
        ]);
        assert_eq!(replay[1]["result"]["game"]["position"], 3);
        assert_eq!(replay[1]["result"]["game"]["presses"], 1);
    }
    #[test]
    fn replay_preserves_input_edges_audio_and_rejects_transactionally() {
        let replies = run(vec![
            ("port.input", json!({"actions":["right"]})),
            ("port.advance", json!({"ticks":3})),
            ("port.checkpoint.export", json!({})),
            ("port.advance", json!({"ticks":2})),
            ("port.state", json!({})),
        ]);
        let checkpoint = replies[2]["result"].clone();
        let mut bad = checkpoint.clone();
        bad["audio_sha256"] = json!("tampered");
        let restored = run(vec![
            ("port.checkpoint.import", json!({"checkpoint":checkpoint})),
            ("port.advance", json!({"ticks":2})),
            ("port.state", json!({})),
            ("port.checkpoint.import", json!({"checkpoint":bad})),
            ("port.state", json!({})),
            ("port.input", json!({"actions":["unknown"]})),
            ("port.state", json!({})),
            ("port.input", json!({"actions":[]})),
            ("port.advance", json!({"ticks":1})),
            ("port.input", json!({"actions":["right"]})),
            ("port.advance", json!({"ticks":1})),
            ("port.state", json!({})),
        ]);
        assert_eq!(restored[2]["result"], replies[4]["result"]);
        assert!(restored[3].get("error").is_some());
        assert_eq!(restored[2]["result"], restored[4]["result"]);
        assert!(restored[5].get("error").is_some());
        assert_eq!(restored[4]["result"], restored[6]["result"]);
        assert_eq!(restored[11]["result"]["game"]["presses"], 2);
    }
    #[test]
    fn budget_and_checkpoint_validation_do_not_advance_live_state() {
        let replies = run(vec![
            ("port.advance", json!({"ticks":100001})),
            ("port.advance", json!({"ticks":1,"timeout_ms":0})),
            (
                "port.checkpoint.import",
                json!({"checkpoint":{"version":1,"identity":"foreign","segments":[],"held":[[],[],[],[]],"frame_sha256":"","audio_sha256":""}}),
            ),
            ("port.state", json!({})),
            ("port.advance", json!({"ticks":0})),
            ("port.audio", json!({"limit":4097})),
        ]);
        for i in [0, 1, 2, 5] {
            assert!(replies[i].get("error").is_some());
        }
        assert_eq!(replies[3]["result"]["ticks"], 0);
        assert_eq!(replies[4]["result"]["completed"], 0);
    }
}
