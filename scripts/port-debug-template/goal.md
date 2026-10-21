# Autonomous two-pass native-port QA

Execute or resume the autonomous QA goal below when the user invokes this file.
The only start/resume instruction the user needs is: **Read debug/goal.md.**
Preparing a workspace merely installs these files; it does not invoke this goal.
The two-pass acceptance contract is already agreed. Derive the game-specific plan
autonomously; no further installation or plan/acceptance approval is required.

## Start or resume here

Treat the port workspace (the parent of debug/) as the working directory. Follow
these links; the user need not name additional files:

- [Debugging guide](../DEBUGGING.md): control API, bounded Python client, original
  ROM comparison, checkpoint limits, plan/result formats and audit command.
- [Acceptance contract](acceptance.json): the agreed two-pass success criterion.
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

This separately requested QA goal does not change initial best-effort
implementation completion. Preserve the starting source, unrelated work, frozen
references and original ledger IDs. Do not launch agents or publish/check in changes.

## Plan before testing

Derive debug/plan.json from the source, behavior ledger and port. List each game
area/mode, all major gameplay elements and major transitions, including later
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
and local evidence. Once a pass begins, the plan is fixed for that QA attempt.
Missing source/tools or a mistaken game model must be investigated as an obstacle,
not silently treated as a successful check or an exclusion.

## Execute the plan twice

First pass: work through the entire plan in order, fixing failures as encountered.
A failed or inaccessible check blocks advancement. Keep diagnosing and correcting
it until it passes; do not skip it, defer it to a backlog or call an inconclusive
result acceptable completion. An inaccessible state is a defect in the route,
implementation or understanding that must be resolved.

If the failure reveals a broader implementation problem, halt the current QA
attempt, make the necessary holistic game-code changes and restart from the
beginning. If the plan rests on a mistaken understanding, correct it from source
and evidence and restart QA. Preserve superseded plans/results and explain why
restart was necessary. Do not weaken expectations or remove a valid failing check
merely to obtain a pass. If no sound repair or plan correction is available,
abort and report the fundamental blocker; do not continue A-to-Z past it.

Only after the full first pass succeeds, start at A again and re-check every
entry in a complete second pass against the updated port. A second-pass failure
also blocks advancement until fixed. For a local fix, recheck the failed entry
and any previously passed checks affected by the change before continuing. Record
that impact assessment and fresh evidence; results invalidated by a fix do not
count as current passes. A holistic change or plan change restarts QA at pass one.
There is no automatic third full pass after two valid successful passes.

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
alternative acceptance rule. Keep fixing checks; escalate fundamental problems
into code/plan correction and restart instead of endless identical experiments.

Use shared platform helpers and maintain the architecture. Add focused unit
regressions for complex/error-prone repaired logic when useful; avoid trivial
arithmetic/getter tests or a huge unit suite that replaces execution of the QA
plan. Run relevant format/static/build/regression checks after changes. Follow
supported reference-correction procedures if source corrections are needed.
Keep ROMs, extracted assets and generated comparisons local; never copy GPL core
implementation into the MIT native library.

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

Once both passes succeed, run relevant final checks, write the QA/fix/remaining-
limitations report, and stop. This establishes completion of the declared QA
plan, not a claim of universal bug freedom or untested hardware compatibility.
