# Native port debugging control and workflow

This is a separately requested workflow, not an additional implementation gate.
The initial port goal still ends after its best-effort implementation/accounting
and required checks. The agreed two-pass QA acceptance contract is preinstalled;
invoking debug/goal.md selects this separate workflow without another acceptance
approval. Installation alone does not start QA or claim that a game is correct.

Faithfulness does not require reproducing hardware-induced lag, slowdown or
sprite flicker. Their absence in a smoother native port is not a bug and must not
produce a failing QA check or a repair task. Preserve and check intentional gameplay
timing and visual effects, such as deliberate slow motion or damage/invulnerability
flashing.

## Install and configure

Run toolkit commands from the toolkit directory recorded in workspace.json.

New workspaces prepared with `scripts/port.py prepare` already contain the debug
API launcher, this guide, debug/goal.md and acceptance/plan/result templates.
To start or resume QA, open an agent in the port workspace and tell it:
**Read debug/goal.md.** That file links everything else and directs execution.
Preparation installs the files without launching QA or adding it to the initial
implementation completion criteria.

The install command below is for older workspaces that lack these files:

`python3 scripts/port_debug.py install WORKSPACE` installs DEBUGGING.md, the
separate debug/goal.md, acceptance.json, and empty plan.json/qa.json templates.
It preserves existing files and does not start an agent. The agreed acceptance
policy is two complete successful passes over a source-informed QA plan. A
numerical goal-wide resource ceiling is optional and has not been chosen; any
external resource interruption leaves QA incomplete, never successful.

The installed CLI needs `--debug-stdio` wired to `romatlas_port::debug::serve`.
Future port scaffolds include this wiring. Alleyway and Rygar have been adapted,
with Alleyway's existing summary and a read-only Rygar summary. Other existing
ports need the same small launcher adaptation; arbitrary private game state is
not inferred by the library. There are no native memory/register writes or
visible in-game debug menus. State edits need a separately designed port adapter.

## Start the transport

From an adapted workspace, `python3 run.py debug` starts a persistent headless
JSON-RPC 2.0 server on stdin/stdout. Cargo diagnostics use stderr. Alternatively:

```sh
cargo run --locked --manifest-path port/Cargo.toml -- --rom input/game.rom --debug-stdio
```

No window/audio device or on-disk battery save is initialized. The session starts
from fresh ROM-backed defaults. --assets is supported, while --headless/--settings
cannot be combined with --debug-stdio. Commands consume logical action names
reported by api.describe, not keyboard names. Player slots are zero based, 0..3.

| Method | Params / result |
|---|---|
| api.describe | Protocol/build identity, actions, limits and methods |
| port.state | Total ticks, held actions, health flag and optional game-owned summary |
| port.input | player (default 0), complete actions array; [] releases that player |
| port.advance | ticks (default 1), timeout_ms (default 1000, max 300000); returns completed/remaining and state |
| port.framebuffer | width/height, rgba8 bytes and SHA-256; caller can save a local image |
| port.audio | offset and limit (stereo frames, max 4096); paged 48-kHz stereo float capture, total/drop counts |
| port.audio.clear | Clear captured output without changing synthesis state |
| port.reset | Fresh deterministic game, cleared input, clocks and journal |
| port.checkpoint.export | Return a ROM-free replay record; save it locally with build/source context |
| port.checkpoint.import | checkpoint and timeout_ms; replay into a fresh candidate, validate output hashes, then adopt |

```json
{"jsonrpc":"2.0","id":1,"method":"port.input","params":{"actions":["start"]}}
{"jsonrpc":"2.0","id":2,"method":"port.advance","params":{"ticks":1}}
{"jsonrpc":"2.0","id":3,"method":"port.input","params":{"actions":[]}}
{"jsonrpc":"2.0","id":4,"method":"port.framebuffer"}
```

The driver ticks simulation and generates audio at rational console cadence,
using 48-kHz sample budgets. Inputs produce press/release edges on tick boundaries.
Rendering is on demand into a fresh cleared buffer. Games must make rendering
observational. This driver defines one deterministic audio schedule; desktop/browser
callback schedules need not be byte-identical to it. Captured audio retains ten
seconds; export/clear pages as needed and inspect drops.

Advancement is capped at 100000 total ticks per reset and 100000 journal segments.
Wall time is checked between ticks, not inside game functions; a single game tick,
render or constructor may overrun or panic. Supervising clients must enforce a
process-level deadline and record the last request so crashes/hangs become issues.
Requests/responses are limited to 16 MiB; oversized requests terminate transport,
whereas recoverable command errors return JSON-RPC errors. Framebuffers are capped
at 1 MiB RGBA. A failed game audio contract requires reset before further stepping
or checkpoint export. These technical limits are not the debugging goal's budget.

## Replay checkpoint limits

A checkpoint replays logical inputs and tick/audio calls from deterministic reset;
it does not serialize arbitrary game objects. Restore cost grows with elapsed
ticks and is capped by tick/time budgets. The candidate replaces live state only
after successful replay and frame/audio hash checks; failed restores preserve it.
The executable/ROM/override identity prevents reuse across builds or changed assets.
After code changes, reuse scenario recipes from fresh reset instead of importing
old-build checkpoints. Launchers recheck override/build identity before reset/load.
No ROM bytes or extracted originals belong in checkpoint metadata.

The port must have deterministic initialization and pure rendering. Frame/audio
hashes catch output mismatches, not arbitrary hidden-state nondeterminism; useful
port-owned inspection and future typed snapshots can strengthen this later.
Process restart replay is supported only for the same executable/assets; replay
logs are not a substitute for immutable source provenance. Files containing
captured original-game outputs remain private/local.

## Autonomous two-pass QA contract

The user-approved process is plan first, then A-to-Z QA fixing as it goes, then a
complete second A-to-Z pass. Build the plan from source, native architecture and
behavior accounting: each area/mode, all major mechanics and transitions, later
progression and ending/terminal states where present. Each entry has specific
applicable graphics, behavior and sound checks and a reproducible route/setup.
The scope is major game functionality, not exhaustive frame/input permutations.

A failure or inaccessible check blocks advancement until corrected. Do not skip,
defer, mark unsupported/inconclusive as passed, or finish merely by recording a
backlog. Keep investigating/fixing it. A fundamental code problem requires halting
the attempt, holistic correction and restarting QA at pass one. A mistaken plan
requires an evidence-supported plan correction and the same restart. Preserve
superseded attempts and reasons; do not remove a valid check to escape a failure.
If no sound correction is available, abort with the fundamental blocker.

After pass one fully succeeds, recheck every entry in pass two. For local fixes,
rerun the failed check and affected earlier checks before continuing; invalidate
stale results. Holistic code changes or plan changes restart at pass one. Two
complete valid passes, no blockers and a final report constitute success. There
is no stagnation-based completion, repair quota, successful budget-limited result
or automatic third pass. A resource ceiling can interrupt the run as incomplete.
Per-experiment timeouts protect tools; they do not excuse skipping a check.

### Plan and result records

Fill debug/plan.json before executing it. Lists `areas`, `mechanics`, `transitions`
contain unique nonempty descriptive strings. Each ordered `checks` entry contains:

```json
{
  "id": "bridge-foreground",
  "covers": ["area:bridge", "mechanic:occlusion"],
  "aspects": ["graphics", "behavior"],
  "expectation": "Player passes behind the foreground bridge rail",
  "setup": "Recorded route/checkpoint and inputs leading to the crossing"
}
```

The example assumes `bridge` and `occlusion` are listed in their respective
coverage lists. Use area:, mechanic:, transition: prefixes. Every listed item
needs a check; the agent must assess actual comprehensiveness from source. Use
status ready and a descriptive revision when planning is complete. Preserve
superseded plans/results in local artifacts when restarting.

Write debug/qa.json with the SHA-256 of the exact current plan file, `status`
(in_progress, incomplete, aborted or complete), passes and blockers. Each pass
has `number` 1 or 2, `status`, and ordered `checks` results. Each result has id,
status, a nonempty `build` identity, and `evidence` listing local artifact files.
Only status passed counts. Inaccessible, failed and pending entries remain
blockers during execution. Each pass must cover the full ordered plan. The
agent must keep evidence valid after fixes and record affected-check rechecks;
the record audit cannot infer code impact or verify actual audiovisual truth.

For completed QA, `blockers` must be empty and `report` names an existing local
handoff file. Run from the toolkit:

```sh
python3 scripts/port_debug.py audit /path/to/port-workspace
```

The audit rejects partial/failing/inaccessible or stale-plan records. It validates
accounting, not semantic correctness. Preserve first-pass records while executing
the second pass. Restarting after holistic changes creates a fresh attempt; old
passing records cannot be reused to evade the required two passes.

## Agent scenario client

`scripts/native_debug_client.py` supplies PortClient on the Unix debugging host.
It supervises the native process with a hard deadline and request count, records
requests/responses/stderr locally, limits message/evidence sizes and kills its own
process group if the child hangs. These are per-experiment controls, not a
goal-wide acceptance budget or an alternative QA completion rule. Each experiment must declare them explicitly:

```python
from native_debug_client import PortClient
with PortClient(["/path/to/port/target/debug/game-port", "--rom",
                 "/path/to/input/game.rom", "--debug-stdio"],
                seconds=30, max_requests=100,
                evidence="/path/to/workspace/debug/artifacts/probe-001") as port:
    port.request("port.input", {"actions": ["start"]})
    port.request("port.advance", {"ticks": 1})
    port.request("port.input", {"actions": []})
    port.request("port.advance", {"ticks": 60})
    state = port.request("port.state")
    image = port.frame("screen.ppm")
```

Add the toolkit scripts directory to Python's import path. Evidence directories
must be new, preventing accidental overwrites. Use the generated game package's
actual binary name; build it before the experiment. Inspect captured images with
local image tools and retain useful scenarios rather than regenerating every route.
