# Native port debugging control and workflow

This guide supports the final QA stage of the fixed native-port pipeline and
standalone QA. Both execute two fresh whole-game passes, repairing failures at
the current check without restarting. Invoking goal.md authorizes its pipeline
subagents; preparing/installing instructions alone launches nothing. Read
debug/acceptance.json, debug/plan.json and debug/qa.json on resume and retain progress.
New pipeline workspaces do not use milestone cycles or begin-cycle. The toolkit
retains that command for existing workspaces; it is not a stage in this workflow.

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
the QA subprocess after whole-game implementation; users can also invoke it independently.

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
not inferred by the library. The skeleton stage connects a versioned game-owned
observe callback, documented in debug/api.md, to expose semantic state and bounded
events through port.state.
Implementation must maintain this schema across all game systems. Existing
ports may require an adapter extension; the generic library does not infer it.
There are no native memory/register writes or visible in-game debug menus.
State edits need a separately designed, validated port adapter.

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
hashes catch output mismatches, not arbitrary hidden-state nondeterminism;
game-owned semantic inspection and explicit scenario preconditions must validate
the relevant state.
Process restart replay is supported only for the same executable/assets; replay
logs are not a substitute for immutable source provenance. Files containing
captured original-game outputs remain private/local.

## QA pass and repair contract

The process is plan first, then two A-to-Z passes fixing as each proceeds. Build
the whole-game plan from source, native architecture and behavior accounting:
each area/mode, major mechanics and transitions, later progression and ending/
terminal states where present. Each entry has specific
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

Finish pass one, then run the full plan once more as pass two, fixing issues with
the same continue-in-place procedure. Never return to pass one or start a third
full pass. Success requires the configured completed passes, resolved issues and a report.
There is no stagnation-based completion or repair quota. A resource ceiling or
unresolved blocker leaves the current run incomplete and resumable.

### Plan and result records

Fill debug/plan.json before executing it. Lists `areas`, `mechanics`, `transitions`
contain unique nonempty descriptive strings. Whole-game plans require all three
nonempty categories. Each ordered `checks` entry contains:

```json
{
  "id": "bridge-foreground",
  "covers": ["area:bridge", "mechanic:occlusion"],
  "aspects": ["graphics", "behavior"],
  "expectation": "Player passes behind the foreground bridge rail",
  "setup": "Restore validated approach checkpoint; cross the bridge",
  "scenario": "debug/scenarios/bridge.py",
  "command": "python3 debug/scenarios/bridge.py --check bridge-foreground --pass 1 --run attempt-001",
  "source": ["reference/source/scene.asm:BridgeOcclusion"],
  "starting_state": "debug/artifacts/bridge-approach.json; validate mode, area and position"
}
```

Scenario/source/setup fields above are agent-maintained provenance in addition to
the core record schema; the administrative audit does not validate their truth.
The example assumes `bridge` and `occlusion` are listed in their respective
coverage lists. Use area:, mechanic:, transition: prefixes. Every listed item
needs a check; the agent must assess actual comprehensiveness from source. Use
status ready and a descriptive revision when planning is complete. Preserve
plan revisions and superseded results as concise history when correcting them.

Write debug/qa.json with the SHA-256 of the exact current plan file, `status`
(in_progress, incomplete, aborted or complete), passes and blockers. Each pass
has `number` 1 or 2, `status`, and ordered `checks` results. Each result has id,
status, a nonempty `build` identity, and `evidence` listing local artifact files.
Record scenario/script revision, invocation, checkpoint/setup identity and fresh
execution evidence per result; use distinct evidence directories for each pass
and repair run. Historical evidence is not a current execution result.
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

## Reusable QA scenarios

For each plan check, create or extend a versioned game-owned script in
debug/scenarios/. Shared helpers should remove repeated setup, input driving,
reference queries and observation handling. Keep source-derived expectations
readable and identify their source paths/labels/revision. Reuse the same scripts
for route discovery refinements, debugging, repair verification and both full
passes; do not repeatedly reconstruct a setup through individual tool calls.

Each script invocation must:

1. Identify ROM, assets, executable/source and script revisions; select its check
   ID and a unique pass/repair-run evidence directory.
2. Restore a known starting checkpoint or execute its retained deterministic setup
   recipe. Check import success and semantic preconditions (mode, area, actor/
   progression state, held inputs and relevant timers/flags). A successful frame/
   audio hash alone does not validate the intended scenario.
3. Apply bounded logical inputs/advancement, checking completed/remaining status;
   observe named state changes/events and selected frame/audio output. Assert
   source-backed expectations with useful expected/actual diagnostics.
4. Use the existing original-emulator workflow to obtain corresponding reference
   observations, with snapshot/patch/input provenance. Reference setup can be a
   shared script/helper; record exactly which source-backed reference each check
   uses. Do not treat the native implementation as its own oracle.
5. Write a fresh result including starting-state validation, exact inputs, relevant
   observations and local evidence. Return failure for failed expectations,
   inaccessible setup, incomplete advancement or missing observations.

Drive the native transport with PortClient below and the original ROM through its
platform adapter. Endpoint polling may miss transient effects; expose bounded,
read-only semantic event history or step at suitable deterministic boundaries.
Document field units, event sequence/retention and overflow behavior in debug/api.md.
Never silently lose an event required by a check or mutate state during inspection.

Reuse validated checkpoints within their build/ROM/asset identity. After a code
change, rebuild native checkpoints by running saved setup recipes from reset and
revalidate semantic preconditions; output identity guards must remain enforced.
Keep native replay records separate from emulator snapshots. State shortcuts
must not bypass a transition under test, and recipes must record exact edits and
initialization contracts. Validate reference snapshot/setup provenance too.

Each required pass executes all scenarios afresh, even when scripts/checkpoints
are reused. Repair rechecks use those same scripts with fresh evidence at the
current position. Preserve valid unaffected results and revise faulty scenarios
with source-backed reasons instead of weakening expectations. Neither checkpoint
reuse nor previously cached assertions counts as another pass. Keep reusable
scripts and setup metadata with the source; keep local captures/checkpoints in
debug/artifacts/ and link selected evidence rather than embedding bulk payloads.

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
actual binary name; build it before the experiment. This snippet illustrates client calls only; put real check setup, semantic
preconditions, expectations and reference observations in the reusable scenario.
Inspect selected images/audio with local tools and record actual observations.

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
