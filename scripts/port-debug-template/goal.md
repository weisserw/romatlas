# Stage 5: whole-game QA

Execute or resume this QA subprocess when the fixed port pipeline invokes it,
or as a standalone goal when the user invokes this file. **Read debug/goal.md.**
is the standalone start/resume instruction. Preparation only installs files.
Complete two fresh A-to-Z passes over a source-informed whole-game plan, fixing
failures in place without restarting either pass. No further installation or
plan/acceptance approval is required.

## Start or resume here

Treat the port workspace (the parent of debug/) as the working directory. Read:

- [Debugging guide](../DEBUGGING.md): native API, bounded Python client, original
  emulator comparison, reusable scenarios, checkpoint limits and record formats.
- [Acceptance contract](acceptance.json), [QA plan](plan.json) and
  [QA results](qa.json): preserve the current plan, pass and blocking check.
- [Workspace configuration](../workspace.json): toolkit path, platform, ROM
  identity/location and source/archive provenance.
- [Behavior ledger](../ledger.json), [known differences](../differences.md),
  [progress](../progress.md), [architecture](../architecture.md) and
  [library changes](../library-changes.md).

Read prompts/common.md, behaviors.md, debug/api.md and source-backed unit
tests/traces where present.
Inspect port/src/, original source and any adopted reference supplement. The
inventory/tests help navigation; independently check source-backed expectations.
For older standalone ports without semantic inspection, add the necessary
read-only game-owned observations before attempting checks that depend on them.
Keep inspection deterministic and all relevant logic accessible headlessly.

Use workspace-relative paths from the workspace root. Run toolkit scripts from
the toolkit path in workspace.json. python3 run.py debug starts the transport;
the agent drives it to execute QA. Preserve frozen references, original ledger
IDs, previous evidence and unrelated work. Do not launch other agents or publish/
check in changes. The fixed pipeline uses the installed two-pass acceptance;
do not call begin-cycle, reset records or lower acceptance to evade a pass.

## Plan and reusable scenarios before testing

Derive debug/plan.json from the source and actual port. Cover every area/mode,
major gameplay mechanic and transition, later progression, bosses, menus/title
and ending/terminal states where present. Define applicable graphics, behavior
and sound checks with specific expectations and source references. Cover major
functionality without enumerating every collision/input permutation or demanding
undefined pixel/frame equivalence. Write the ordered A-to-Z plan before running it.

For each QA check, create or extend a reusable scenario script under
debug/scenarios/ (shared helpers are encouraged). It must restore a validated
starting state, apply bounded inputs, observe relevant semantic events/state and
assert source-backed expectations. Use native debug/api.md observations and the
original ROM emulator workflow for corresponding reference evidence. Retain
matching source/build/ROM/asset identity, exact inputs, starting preconditions,
expected events and selected frame/audio evidence. Share setup helpers/checkpoints
across checks when useful; avoid repeatedly reconstructing routes and issuing
individual tool calls. Link script, invocation, source expectations and setup/
checkpoint validation from each plan entry as described in DEBUGGING.md.

Use the same scenario script for diagnosis, repair verification and each required
QA pass. Evolve it with source-backed corrections as understanding improves.
Reuse scripts and valid checkpoints, but execute every check in each pass afresh
and write distinct result/evidence records. Cached pass results or unit-test
success cannot substitute for fresh scenario execution. A script's assertion is
not proof by itself: inspect the relevant source and audiovisual evidence too.

Check checkpoint identity and semantic starting preconditions on every restore.
After code/asset changes, regenerate checkpoints using the retained setup recipe
and validate again; never bypass transport identity checks. Original emulator
snapshots and native replay checkpoints are separate formats. Artificial setups
may reach later areas but must not bypass the transition a check is testing.

Hardware-induced lag, slowdown or sprite flicker need not be reproduced. Preserve
and check intentional timing/effects such as slow motion or invulnerability
flashing. Correct mistaken expectations/routes in place with source evidence and
recorded plan revisions, preserving stable check IDs and previous valid results.
Missing source/tools or a mistaken model is an obstacle to resolve, not a pass.

## Execute two passes and repair in place

First pass: work through the entire plan in order, fixing failures as encountered.
A failed or inaccessible check blocks advancement. Keep diagnosing and correcting
it until it passes; do not skip it, defer it to a backlog or call an inconclusive
result acceptable completion. An inaccessible state is a defect in the route,
implementation or understanding that must be resolved.

Fix broader implementation problems in the current run too. Do not restart the
attempt, reset the pass number, discard completed checks or return to A because
of a code fix, its scope, or a plan/route correction. Preserve the current pass
and next-check position. Do not weaken expectations or remove valid failing
checks merely to obtain a pass. If no sound repair is available, leave an
incomplete handoff at the blocked check and resume there when resolved.

After a fix, recheck the failed entry and specifically affected earlier checks,
recording the impact assessment and fresh evidence. Retain unaffected results.
These targeted regressions are repairs within the current pass, not another
A-to-Z traversal. A broad fix alone is not grounds to repeat every earlier check.
Correct mistaken plans/routes in place, reconcile changed checks and evidence,
and update the plan hash; newly added checks need results in every required pass record.
Do not fabricate results or clear progress merely because a plan hash changed.

When the first A-to-Z pass finishes, start at A once for the second full pass.
Fix issues during pass two with the same continue-in-place procedure, including
targeted rechecks of earlier results invalidated by a fix. Never reset to pass
one or start a third full pass. Completion requires both full passes and resolved
issues; a blocked check or resource interruption leaves a resumable incomplete run.

## Tools and repair discipline

Use the native stdio API for logical input, bounded advancement, read-only
semantic state/events, frame/audio capture, reset and replay checkpoints. Control
the original ROM with its separate emulator adapter. Corresponding events/states,
not equal frame numbers alone, establish matching scenarios. Record provenance
and emulator/helper limits. Play reactively only while discovering a route, then
preserve it in the reusable scenario instead of repeating manual setup.

Use process deadlines and explicit client budgets. A timeout does not permit
passing/skipping a check: diagnose it with a revised hypothesis/method. Do not
use stagnation counters, repair quotas or minimum bug counts as acceptance.
If blocked, preserve the exact check, script invocation, investigation and
resumption point without restarting the pipeline.

Maintain the architecture and coding standards in prompts/coding-standards.md
where present (otherwise use the port's established standards). Repair meaningful
unit contracts alongside complex/error-prone fixes, retaining adjacent source
reasoning. Unit tests do not replace execution of QA scenarios. Run relevant
format/static/build/regression checks after changes; use supported verified
reference corrections where needed. Keep ROMs/assets/comparisons local and never
copy GPL core implementation into the MIT native library.

## Acceptance and resource limits

Success requires every check in the current justified plan to pass in both full
passes, with current valid results, no unresolved blockers and a final report.
Record debug/qa.json and run the administrative `port_debug.py audit` check.
That command validates records; it cannot judge game coverage or evidence truth.
Do not report success from a partial pass, skipped/inaccessible check, logged
failure, source assumption or merely reaching the end of a budget.

A configured external resource ceiling may interrupt work. Stop cleanly and leave
an incomplete handoff with the blocking check, investigation, needed holistic
changes/plan correction and exact resumption point. Resource exhaustion is not
successful QA. No numerical goal-wide ceiling has been chosen by the user; do not
invent one or confuse technical per-command limits with acceptance. Preserve a
resumable record at useful points and before an external ceiling is exhausted.

Once both passes succeed, run the relevant final checks and python3 run.py audit
after any QA repairs, write the QA/fix/remaining-limitations report, and run the
QA record audit. Stop after both audits pass. This establishes completion of the
declared QA plan, not a claim of universal bug freedom or untested hardware compatibility.

Follow DEBUGGING.md's disk/evidence retention discipline. Keep reusable scripts,
validated checkpoint/setup provenance and cited evidence, with bulk captures and
checkpoint payloads outside transport logs. Preserve superseded plans/results
and concise correction history rather than every disposable probe.
