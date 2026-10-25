# Native port debugging control and workflow

This guide supports the final QA stage of the fixed native-port pipeline and
standalone QA. New pipeline workspaces use only the short smoke test and initial
exact-frame comparisons specified in their debug/goal.md and version-5 acceptance
contract. The whole-game two-pass instructions below apply to separately installed
standalone QA and existing full-QA contracts. Invoking goal.md authorizes its pipeline
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
| port.reset | Fresh deterministic game, cleared input, clocks, journal and snapshot bank |
| port.snapshot.save/load/delete | name; optional process-local fast branch snapshots when advertised (max16), no disk payload |
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

## Player-facing QA policy

`debug/qa-policy.json` states the active methodology. Main QA runs the native
port as a game tester would: perform a checklist action and evaluate meaningful
consequences against independently stated game rules and source-backed contracts.
Read-only semantic observations support objective assertions; selected image/audio
captures support agent judgment. Record criteria, actual observations, capture
paths and rationale. Missing required presentation review leaves acceptance pending
regardless of machine assertions. Do not use observed output to invent the oracle.

The emulator is an adjunct for a named uncertainty or defect. Do not launch or
synchronize both games during routine gameplay/navigation. Different physical
frame counts, hardware lag and internal scheduling are not automatic failures.
Use bounded semantic completion conditions and selected events rather than exact
frame/byte equality. Detailed stepping is for a specific transient contract or
failure diagnosis. Preserve natural setup; no arbitrary state grants.

Keep a small original-fidelity sanity suite separate from gameplay: representative
rendering, animation/scrolling and audio checks near the beginning of faithful
port QA. Reuse adequate reviewed early evidence on migration. Do not run a
full differential playthrough, or repeat sanity for every gameplay check/pass.
Keep its manifest, configuration and record independent. A future widescreen or
other feature project can explicitly disable it with a documented reason while
continuing every applicable gameplay regression. Never disable it automatically
to hide a failure. Changing gameplay rules requires explicit authored profiles.

For existing workspaces archive the old plan, keep IDs/coverage, rewrite internal
comparison obligations as player-visible behavior, reconcile the plan hash, and
retain accepted evidence supporting the revised expectation. Record the policy
change; preserve known defects and focused regressions. This is a method change,
not a reset or declaration that untested checks passed.

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
native navigation and observation handling. Keep source-derived expectations
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
4. Evaluate independently stated game rules and source-backed expectations. Run
   native-only by default. Consult the original only for a named uncertainty or
   defect; retain the question, observations and conclusion if used. Review selected
   frame/audio captures against player-visible criteria, not universal pixel hashes.
5. Write a fresh result including starting-state validation, exact inputs, relevant
   observations and local evidence. Return failure for failed expectations,
   inaccessible setup, incomplete advancement or missing observations.

Drive the native transport with PortClient below. The original adapter is optional
for diagnosis and separate initial sanity checks; it is not a gameplay dependency. Endpoint polling may miss transient effects; expose bounded,
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

## Interactive scenario authoring

Use `scripts/scenario_client.py` with a persistent PortClient. ScenarioSession
provides input, up to16 named bookmarks, restore and bounded `until` operations.
Reference bookmarks use full emulator snapshots. Native bookmarks use
port.snapshot.save/load when advertised; older ports fall back to identity-checked
replay checkpoints with history-dependent restore cost. No game-state writes or checkpoint identity bypasses are introduced.

`port.run_until` takes `ticks`/`timeout_ms` with the same bounds as port.advance and
1–32 `conditions` against JSON pointers relative to the game-owned observation:
`{"pointer":"/mode","value":"gameplay"}`. Operators are eq(default), ne,
lt, le, gt and ge; ordered operators require numbers. Conditions are checked before
execution and after every tick. Missing fields are errors. Results include matched,
reason(matched/tick_limit/timeout), completed, remaining and game. Successful ticks
are recorded in replay history. Prefer integer semantic values for exact comparisons.

Discover natural input routes in the native port, save useful branches and retain
recipes, then run durable assertions from validated setups. Prefer persistent
sessions and native port.run_until, not per-frame host polling. Reference
run_until/observe are diagnostic tools, not mandatory navigation synchronization. A failed setup,
missing observation or exhausted budget is a setup failure, not a game mismatch.
The ScenarioSession helper raises SetupFailure on an unmatched bounded until.
Keep independently authored gameplay expectation profiles distinct from the optional
original-fidelity comparison suite; never bless observed output as an expectation.

## Minimal integration harness

`scripts/qa_harness.py` runs a game-owned manifest/adapter without starting or
updating the whole QA workflow. Select `--test ID` or `--suite NAME`, an explicit
`--expectation PROFILE`, and a new `--evidence DIRECTORY`. Each test retains a
versioned result, named assertion results, phase wall times, per-transport request
counts and failure diagnostics. A suite continues after a failed test and exits
nonzero if any test fails. Existing evidence is never overwritten.

The manifest lists `tests` (IDs, documented `expectations`, and optional `kind`),
`suites` (ordered ID lists), and an `adapter` Python file relative to the manifest.
Kind defaults to `gameplay`; direct comparisons must use `original-fidelity-sanity`.
These tests are excluded by default and reported as excluded, never as passed.
To run them, set manifest `original_fidelity_sanity` to an enabled boolean and
nonempty reason, then pass `--include-original-fidelity-sanity`. An all-sanity
selection without opt-in is an error. Setting enabled false prevents opt-in;
this explicit switch does not disable gameplay tests. Use separate manifests and
lazy imports so gameplay does not even require the comparison environment. The adapter's
`run(context, entry)` uses existing clients, registers them in `context.clients`,
registers cleanup with `context.stack`, and separates `context.phase('setup')`
from `context.phase('execution')`. Use `context.check(name, callable)` to record
assertion groups. Exceptions during setup, transport failures and `SetupFailure`
are `setup_failure`; execution assertions are `game_mismatch`. Unexpected errors
retain their traceback and count as setup/tooling failure. Preserve the failed
observations and last transport requests; a passing subset never makes a test pass.

Keep original-fidelity and intentionally modified behavior in separately authored,
explicitly selected expectation profiles, with source/design justification.
Unsupported profiles fail before execution. The runner never creates expectations
from observations. Run Python without `-O`, since existing scenarios use asserts.

These historical paired examples are disabled by default and are not the small
initial sanity suite or the main gameplay process. They require a deliberate
manifest configuration change plus the opt-in flag to execute.

The historical DuckTales `debug/tooling/integration-legacy-differential.json`
showcase suite executes accepted J-climb-vertical through the existing paired
scenario, independently of accepted QA records. Its adapter regenerates stale
paired gap checkpoints using the retained natural-input recipe, then imports
with all identity guards and validates canonical state, source yield, player and
terrain before climbing. Native restore still replays from reset.

```sh
python3 scripts/qa_harness.py --manifest /root/ducktales/debug/tooling/integration-legacy-differential.json --test J-climb-vertical --include-original-fidelity-sanity --expectation original-fidelity --evidence /root/ducktales/debug/artifacts/tooling-j-$(date -u +%Y%m%dT%H%M%S)
```

Harness-only regression: `python3 scripts/test-qa-harness.py`. This uses synthetic
adapters and requires no commercial ROM or game build.

### Historical differential caching (diagnostic only)

The J integration adapter caches the paired natural climbing-entry checkpoint in
`debug/cache/integration/J-climb-vertical/`. Cache keys include native/reference
builds, ROM/assets/native/frozen source identities, expectation profile and exact
setup recipe dependencies. A cache hit still imports through the ordinary guarded
APIs and checks canonical native state, original owner, player, terrain, camera,
room and active actors before running assertions. Cold setups follow the retained
natural-input route; staged pairs are validated before atomic publication. Cache
entries record file hashes and the originating evidence directory. Changed
identities/recipes select a fresh entry; corrupt content is never accepted.

`setup-cache-selection.json` records hit/miss, selected identity and promotion.
Execution remains fresh on every invocation. Setup caching is not assertion-result
caching, and native restore still replays its input history. The vertical source
observer uses server-side equality/inequality conditions when advertised, retaining
its source caller/composition/fresh-IRQ contract and bounded legacy fallback.

## QA readiness and scope

Use qa_harness.py as the common runner for new checks. Keep game-owned adapters and
explicit expectation contracts, retaining source-backed scenario assertions. A
suite result is execution evidence, not an automatic edit to QA acceptance records.
Accepted legacy checks can remain valid while later checks move to the harness;
author native player-visible replacements for their second-pass run; retain exact
old comparisons only when a specific regression needs them.
There is no prerequisite to replay all old checks before continuing the current one.

For each check document a minimum valid setup, bounded action, semantic observations,
source-backed success criteria, exclusions and known setup obstacles. Test distinct
major rules/transitions and representative normal/boundary outcomes. Avoid exhaustive
input/placement permutations and incidental CPU/RAM/driver equality without a
specific behavioral or regression reason. Retain deliberate timing and effects.
Do not remove distinct bosses/endings solely because they are expensive.

Natural-play routes establish setup provenance. Reuse validated native checkpoints,
but execute the action/assertions afresh. Setup may bypass unrelated earlier checks
by restoring a validated state; it must never bypass the transition under test.
After repeated guesses without new evidence, inspect source/topology and improve
observation before another input probe. Every probe needs a target, bounds and
expected/actual diagnostics. Exhaustion remains incomplete, never passed/skipped.
Scope corrections require source-backed plan/history reconciliation; tooling work
alone must not reset plan IDs or silently invalidate accepted evidence.

## Discover, record and replay a branch

ScenarioSession.reset establishes a recipe origin. input, advance and until record
successful deterministic inputs/advancement. bookmark stores a branch point;
restore discards the abandoned branch from the recorded recipe. export_recipe
returns a JSON recipe; replay validates it and native transport identity before
reset/execution. Keep expected observations separate and source-derived. Check
semantic final state after replay. Reference recipes additionally require the
caller's ROM/build/source provenance checks: api.describe alone does not identify
a reference ROM/build. Calls issued outside ScenarioSession are not recorded;
use the session for all state-changing authoring calls. An indeterminate transport
failure or partial native advance requires reset/restore before recipe export.
Native fast bookmarks clone current state without replay when advertised; fallback
bookmarks on older ports remain replay-based.

DuckTales debug/tooling/author_branch_recipe.py validates a newly authored title
choice branch: try Right, restore, choose Left, export only the retained branch,
and reproduce the full final semantic state from reset. It is an authoring/tooling
check, not a new fidelity pass or a claim of automated discovery of arbitrary routes.

## Intentional divergence without disabling regression coverage

The owned example examples/qa/fixture/integration.json has original-fidelity and
double-speed contracts. Its executable changes movement from one to two pixels
per tick; the adapter's expectations are independently authored constants. Pause
must still prevent movement in both variants. Run scripts/test-qa-variants.py to
verify: original passes original; modified movement fails original; modified passes
its explicit profile; a broken pause still fails the modified profile. No expected
value is learned from output, and no blanket modified-game skip is available.
This validates harness policy without changing DuckTales gameplay. Future intentional
DuckTales changes need their own documented domain expectations and source/design
provenance; retain original-fidelity contracts separately.

Example command from the toolkit root:

```sh
python3 scripts/qa_harness.py --manifest examples/qa/fixture/integration.json \
  --suite all --expectation original-fidelity --evidence /tmp/fresh-fixture-run
```

The manifest/adapter pattern is general-purpose. Native setup caching, full reference
comparison and audiovisual review remain game-owned responsibilities. Do not equate
passing a synthetic fixture with validating an intentional change to a real game.

### Versioned DuckTales example

The ROM-free example in examples/qa/ducktales preserves the validated J adapter,
paired setup cache, source-backed scenario scripts, authoring probe and32-check
scope review. It requires a prepared DuckTales workspace containing the user's own
ROM, frozen source and native executable; none of those inputs, snapshots, captured
assets or QA acceptance records is distributed in the example. Set
ROMATLAS_GAME_WORKSPACE to that existing workspace. The toolkit path is resolved
from the example's checkout. Example (new evidence directory each time):

```sh
ROMATLAS_GAME_WORKSPACE=/path/to/ducktales python3 scripts/qa_harness.py \
  --manifest examples/qa/ducktales/tooling/integration.json --test J-climb-vertical --include-original-fidelity-sanity \
  --expectation original-fidelity --evidence /path/to/ducktales/debug/artifacts/fresh-j
```

The versioned copy can have a different recipe identity from local legacy helpers;
its first run may rebuild its cache. The portable authoring probe and adapter/cache
unit tests were exercised against a local prepared workspace. Full J fidelity was
validated using the game-owned original copies; do not claim a full run of the
portable copy merely because imports/unit tests pass. The coverage review records
a reviewed plan hash, not new accepted gameplay results. ROM-free helpers belong
in version control; original-state caches/captures remain local and ignored.

## Efficient native navigation and bounded execution

Use persistent native sessions, input batches and semantic run-until conditions.
Validate identity/preconditions at retained milestones. Native replay checkpoints
are not instant rewind; avoid restarting/replaying the entire route for each jump.
Do not synchronize reference scheduler owners or compare every actor during setup.
Use compact waypoint diagnostics; append traces rather than rewriting growing files.
Ordinary damage/recovery is permitted unless the test specifically forbids it.

Once a bounded script runs, wait for completion in <=60-second waits. Do not fill
waiting time with speculative reasoning, repeated source reads or file polling.
Separate restore/setup/discovery/assertion times. Invalid/missing observations are
setup failures; reaching a bound is not a pass. Use focused detailed investigation
only after an actual uncertainty/failure, then retain a useful regression.

## Native-only DuckTales gameplay example

`examples/qa/ducktales/tooling/gameplay.json` and its adapter demonstrate native
choice rejection from an existing legitimate local checkpoint, semantic completion
bounds, explicit independent expectations and selected presentation capture. Set
ROMATLAS_GAME_WORKSPACE to the prepared game workspace. It requires the named
private choice-front checkpoint; none is distributed. The example imports
`scripts/native_gameplay.py`, which starts only the native executable, records
inputs/origin checkpoint hash, and offers bounded advance/until and captures.
No reference binary, symbols or source snapshots are needed to execute it.

The subcase is not full L or whole-game QA acceptance. Its objective assertions
can pass while required presentation review remains pending. Record an explicit
review.json with expected/observed criteria, capture paths and judgments before
counting that presentation obligation. Native disk checkpoints still replay and can be costly. NativeGameplay.bookmark
and rewind use session-local fast snapshots on capable servers.

## Fast native branch snapshots

Ports opt into `debug::serve_with_snapshots` (or injectable
`serve_io_with_snapshots`) with a deeply isolated Clone implementation. The legacy
serve API remains supported without snapshot capabilities. Frame/sample clock,
held and previous inputs, journal, audio hash/capture/drop counts, health and the
complete game are cloned. Shared immutable content is fine; mutable shared handles
must be forked and callbacks rebound by the game. A shallow clone is incorrect.

`port.snapshot.save`, `.load` and `.delete` take a name (1–128 bytes). Up to16
snapshots stay in server memory; overwrite or delete unused slots. Load does no
factory reset, tick replay or disk IO. Reset clears the bank. Snapshots cannot be
exported, transferred between processes or loaded across builds; use guarded
replay checkpoints for durable setup. Save before a jump/branch, try it, then load
to retry immediately. This is named rewind, not an automatic every-frame timeline.
ScenarioSession picks the advertised fast path; NativeGameplay exposes bookmark
and rewind and truncates abandoned branch inputs. Stay in one process to retain
the bank. Do not restart/re-import the long setup for each experiment.
