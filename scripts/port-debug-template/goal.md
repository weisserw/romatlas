# Native-port QA: milestone cycle or standalone run

Execute or resume this QA subprocess when the coordinator invokes it, or as a
standalone goal when the user invokes this file. **Read debug/goal.md.** is the
standalone start/resume instruction. Preparation only installs files.
Read debug/acceptance.json before planning: a milestone cycle requires one pass
through cumulative implemented scope; a final cycle or standalone run requires
two complete passes. The coordinator configures milestone cycles with
port_debug.py begin-cycle; do not edit acceptance to evade a required pass.
No further installation or plan/acceptance approval is required.

## Start or resume here

Treat the port workspace (the parent of debug/) as the working directory. Follow
these links; the user need not name additional files:

- [Debugging guide](../DEBUGGING.md): control API, bounded Python client, original
  ROM comparison, checkpoint limits, plan/result formats and audit command.
- [Acceptance contract](acceptance.json): current scope and required pass count.
- [QA plan](plan.json) and [QA results](qa.json): create the source-informed plan
  if still planning; otherwise resume the current attempt, pass and blocking check.
  Preserve existing work rather than replacing these records on resume.
- [Workspace identity/configuration](../workspace.json): toolkit path, platform,
  original ROM identity/location and source/archive provenance. Locate original
  source and emulator documentation through this metadata and the guide.
- [Behavior ledger](../ledger.json), [known differences](../differences.md) and
  [progress](../progress.md): implemented behavior, assumptions and latest handoff.
- [Architecture](../architecture.md) and [library changes](../library-changes.md),
  where present: state ownership and shared-helper contracts/limits. Read the game
  code under [port/src/](../port/src/), plus any handoff and adopted reference
  supplement identified by the workspace records.

Use workspace-relative paths below from the workspace root, not debug/. Run
toolkit scripts from the toolkit directory in workspace.json; do not assume that
it lives at the preparation machine's absolute path if the workspace was moved.
For the native transport, run `python3 run.py debug` from this workspace. This
starts the control server only; the agent drives it to execute this QA goal.

This QA subprocess does not change best-effort
implementation completion. Preserve the starting source, unrelated work, frozen
references and original ledger IDs. Do not launch agents or publish/check in changes.

## Scope before planning

For milestone cycles read architecture.md and the latest implementation handoff.
Plan only for currently implemented milestones, including regressions in earlier
implemented behavior. Explicitly record implemented IDs and deferred future scope
in the plan/report. Do not fail the cycle because a future milestone is absent,
and do not skip defects in behavior claimed implemented. Use source-informed
expectations and the existing original-ROM comparison methods within this scope.
Early infrastructure cycles may have no playable areas or transitions: empty
coverage lists are allowed for intermediate cycles only, but at least one real
check and coverage item are required (for example an asset-loading mechanic).
For final cycles all milestones must be implemented: derive the complete game
plan from source, not merely a union of earlier partial plans. Neither final pass
may be credited from intermediate cycle results. Standalone QA retains whole-game
scope and the existing two-pass semantics.

## Plan before testing

Derive debug/plan.json from the source, behavior ledger and port. List each game
area/mode, major gameplay elements and transitions within the configured scope, including later
progression, bosses and ending/terminal screens where present. For each, define
specific applicable graphics, behavior and sound checks and a reproducible route
or setup. The ledger helps find code; it is not a complete scenario catalogue.
Use concrete expectations such as foreground occlusion or a death-to-continue
transition, not an undefined obligation to prove every pixel/frame identical.
Cover major game functionality without enumerating every collision or input
permutation. Write the ordered A-to-Z check list before executing it.

Faithfulness does not require reproducing hardware-induced lag, slowdown or
sprite flicker. Their absence in a smoother native port is not a bug and must not
produce a failing QA check or a repair task. Preserve and check intentional gameplay
timing and visual effects, such as deliberate slow motion or damage/invulnerability
flashing.

Use the installed plan/result format described in DEBUGGING.md. Preserve check
IDs and plan revisions, source/baseline identities, routes, observations, fixes
and local evidence. Correct mistaken expectations/routes in place with source
evidence and a recorded plan revision; preserve valid check IDs and prior results.
Missing source/tools or a mistaken game model must be investigated as an obstacle,
not silently treated as a successful check or an exclusion.

## Execute the configured number of passes

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

For an intermediate cycle, stop after the first A-to-Z pass succeeds: do not
start a second traversal. For final or standalone QA, when the first A-to-Z pass
finishes, start at A once for the second full pass.
Fix issues during pass two with the same continue-in-place procedure, including
targeted rechecks of earlier results invalidated by a fix. Never reset to pass
one or start a third full pass. Completion requires all configured full passes and resolved
issues; a blocked check or resource interruption leaves a resumable incomplete run.

## Tools and repair discipline

Use the native stdio API for logical input, bounded advancement, read-only state,
frame/audio capture, reset and replay checkpoints. Play reactively to discover
routes, then retain scripts/checkpoints for repeatability. Control the original
ROM through its separate emulator adapter. Compare corresponding events/states and
selected visual/audio observations as each check requires; equal frame numbers
alone do not establish matching scenarios. Explain emulator/helper limitations.
Artificial setups may help reach later areas, but must not conceal a broken
transition that the plan is supposed to test. Record exact routes and provenance.

Use bounded experiments and process deadlines to prevent a hung tool invocation.
An experiment timeout or exhausted client budget does not permit passing/skipping
the check: continue with a useful revised method or diagnose the larger problem.
Avoid unchanged retries without a new hypothesis or observation sought. Do not
use a stagnation counter, fixed repair-attempt quota or minimum bug count as an
alternative acceptance rule. Keep fixing checks, including broader code/plan
corrections, then continue from the current position without restarting QA.

Use shared platform helpers and maintain the architecture. Add focused unit
regressions for complex/error-prone repaired logic when useful; avoid trivial
arithmetic/getter tests or a huge unit suite that replaces execution of the QA
plan. Run relevant format/static/build/regression checks after changes. Follow
supported reference-correction procedures if source corrections are needed.
Keep ROMs, extracted assets and generated comparisons local; never copy GPL core
implementation into the MIT native library.

## Acceptance and resource limits

Success requires every check in the current justified plan to pass in all required full
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

Once the required passes succeed, run relevant final checks, write the QA/fix/remaining-
limitations report, and stop. This establishes completion of the declared QA
plan, not a claim of universal bug freedom or untested hardware compatibility.

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
