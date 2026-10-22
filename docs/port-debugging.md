# Native port debugging control and workflow

This guide supports the iterative port coordinator's debug subprocess and
standalone QA. Intermediate milestone cycles execute one full pass over currently
implemented scope; final and standalone QA execute two full whole-game passes.
Read debug/acceptance.json for the active scope and pass count. Invoking the port
goal authorizes its QA subagents; installation alone launches nothing.

The coordinator uses `python3 scripts/port_debug.py begin-cycle WORKSPACE
--milestone ID` (add `--final` for the final whole-game cycle). Same-ID invocation
resumes records unchanged; a new ID requires the previous cycle's successful audit
and preserves its acceptance/plan/results in debug/cycles/ID/. Retain all cited
artifacts and distinct reports. Never weaken acceptance or reset an incomplete
cycle. Read architecture.md for implemented milestones and explicitly deferred
future behavior. Final QA derives a complete plan and runs both passes afresh.

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
Preparation installs the files without launching QA. The port coordinator invokes
the QA subprocess after each milestone; users can also invoke it independently.

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

## QA pass and repair contract

The process is plan first, then A-to-Z QA fixing as it goes, with a second complete
pass only for final/standalone QA. Build the plan from source, native architecture and
behavior accounting within the implemented scope: each area/mode, major mechanics
and transitions, later
progression and ending/terminal states where present. Each entry has specific
applicable graphics, behavior and sound checks and a reproducible route/setup.
The scope is major game functionality, not exhaustive frame/input permutations.

A failure or inaccessible check blocks advancement until corrected. Fix every
issue as it is encountered, including broader implementation problems. Do not
restart the run, reset pass numbers or discard completed checks for code changes,
plan corrections or route corrections. If genuinely blocked, leave an incomplete
handoff and resume at that check when resolved. Never skip a valid failing check.

After any fix, recheck the failed check and specifically affected earlier checks,
recording impact and fresh evidence. Retain unaffected results. Targeted regression
checks stay within the current pass; the scope of a fix alone does not require
another A-to-Z traversal. Correct plans/routes in place with source evidence,
preserve check IDs and results, and reconcile changed/new checks and plan hashes.
New checks need results in every required pass record; do not invent successful results.

For intermediate cycles, finish after pass one. For final/standalone QA, finish
pass one, then run the full plan once more as pass two, fixing issues with
the same continue-in-place procedure. Never return to pass one or start a third
full pass. Success requires the configured completed passes, resolved issues and a report.
There is no stagnation-based completion or repair quota. A resource ceiling or
unresolved blocker leaves the current run incomplete and resumable.

### Plan and result records

Fill debug/plan.json before executing it. Lists `areas`, `mechanics`, `transitions`
contain unique nonempty descriptive strings. Intermediate cycles may leave a
category empty when no behavior in it is implemented; at least one covered item
and one real ordered check are still required. Whole-game plans require all three
nonempty categories. Record implemented milestone IDs and deferred future scope
in the plan/report. Each ordered `checks` entry contains:

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
plan revisions and superseded results as concise history when correcting them.

Write debug/qa.json with the SHA-256 of the exact current plan file, `status`
(in_progress, incomplete, aborted or complete), passes and blockers. Each pass
has `number` 1 (and 2 when required), `status`, and ordered `checks` results. Each result has id,
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
the second pass. For corrections, retain valid results, replace only invalidated
results with actual rechecks and update the plan hash after reconciling records.
A hash mismatch requires reconciliation, not a fresh run or reset to pass one.

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

## Disk and evidence retention

Keep durable evidence separate from disposable scratch. Retain maps, readable
source, current state, plan/result history, concise observations, reproducible
routes and the selected captures/checkpoints actually cited by those records.
Preserving superseded attempts means retaining their plans/results and explanation;
it does not require retaining every intermediate dump or duplicate capture.

Use one reusable scratch directory for exploratory output. Before each batch and
at handoff, inspect free disk space and scratch size; remove superseded scratch
files after recording useful conclusions and promoting cited evidence. Never prune
cited evidence, source, ROMs, frozen references, saves or active-process output.
Save selected frames/audio intervals rather than continuous captures. Keep bulk
pixel/sample arrays and checkpoint payloads out of transport logs; explicitly save
needed captures/checkpoints once and link them. Bound trace/log bytes and stderr,
not just execution time. Reuse build targets; avoid a fresh Cargo target directory
for each probe and remove owned temporary test workspaces when finished.

When disk is tight, compact disposable output before another experiment. If safe
cleanup is insufficient, leave an incomplete resumable handoff rather than filling
the filesystem. This is storage hygiene, not a weaker acceptance criterion or a
limit on required game coverage. Do not delete another task's files.

The Python client logs compact transport metadata: lists longer than 64 entries
and strings longer than 2048 characters become count/size/SHA-256 summaries.
Returned API results are unchanged. Transport logs are limited to 8 MiB per
experiment; stderr retains its first 1 MiB while further bytes are drained.
Logs are diagnostic summaries, not replay checkpoint storage. Explicitly persist
any checkpoint needed for replay, and save selected frame/audio files separately.
These per-client limits do not bound accumulated workspace artifacts; use the
scratch/promotion discipline above across experiments.
