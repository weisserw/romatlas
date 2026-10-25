# Stage 5: player-facing whole-game QA

Execute or resume when the fixed port pipeline invokes QA or the user invokes
this file. Read debug/goal.md. Use python3 run.py debug for the native transport.
Preparation alone starts nothing. Work in the port workspace. Read DEBUGGING.md,
debug/acceptance.json, debug/qa-policy.json, debug/plan.json, debug/qa.json,
workspace.json, behaviors.md, differences.md and the current handoff/progress.
Consult source-backed behavior contracts and source when needed to understand an
expectation. Preserve original evidence, valid results and unrelated work. Do not
publish, commit or start other agents. Native runtime must remain headless.

## What QA means

Test the port as a knowledgeable human game tester would: perform the checklist's
action, observe the consequences, and judge whether the game behaves correctly.
The primary oracle is an independently stated player-visible expectation based
on game knowledge and source-backed contracts, not continuous emulator equality.
Use read-only native state/events for objective outcomes and selected image/audio
captures for presentation judgment. Record what was expected, what happened and
why a visual/audio result is acceptable or faulty. Do not invent an expectation
from the port's observed output or claim to have watched/listened without review.

The emulator is an adjunct for a specific uncertainty or defect investigation.
Ordinary navigation and gameplay tests launch the native port only. Do not drive
both games, synchronize scheduler owners, compare every actor/byte, or demand
identical inputs/ticks/pixels across versions as routine QA. Hardware lag/slowdown
and sprite limits need not be reproduced; deliberate gameplay timing/effects do.

## Small, separate initial sanity suite

Only during original-fidelity port production, perform a bounded initial sanity
suite of representative renderer/scrolling/animation/audio checks against the
original. Existing reviewed equivalent evidence can satisfy this initial check;
do not repeat it during every gameplay check or in the second gameplay pass.
Select meaningful captures/events and justify any precision/alignment used. Do
not expand sanity checks into a differential playthrough or private-state audit.

Keep these tests in a separate manifest/suite with kind original-fidelity-sanity
and explicit original_fidelity_sanity enabled/reason configuration. The runner
requires --include-original-fidelity-sanity. Gameplay tests must not import,
launch or depend on sanity adapters, emulator binaries, source snapshots or pixel
baselines. Future feature projects can explicitly disable this suite (for example
widescreen) without disabling gameplay regressions. Never disable it silently
because a comparison fails. Keep intentional gameplay changes in independently
authored expectation profiles. Modernization is a separate manually invoked task.

## Gameplay checklist and durable tests

Cover all major areas/modes, player actions, interactions, progression, bosses,
menus, death/reload and endings. Preserve stable IDs and coverage when revising
an existing plan. Replace internal equality requirements with observable rules;
archive old plans and record this methodology change explicitly. Retain existing
accepted evidence that supports the revised, no-stronger expectation. Preserve
known defects and their focused regressions. A failed valid behavior remains a
failure; changing method does not grant new passes or erase issues.

For each check write: starting conditions, action, expected consequences, bounded
completion conditions and selected presentation review. Example: falling in a pit
plays the death sequence, consumes one life, reloads the expected level start and
restores controls/health. Check those facts; do not compare every intervening frame.
Use semantic events to catch brief effects; use short precise stepping only for a
named timing contract or actual failure. Presentation judgment can tolerate normal
render/timing differences while rejecting missing, corrupt or inappropriate output.

Use scripts/qa_harness.py with a game-owned manifest and native-only adapter.
Maintain durable scenarios in debug/scenarios/ and use the same scenario script
for diagnosis and repair; execute every check in each pass afresh. Validate semantic starting preconditions
at each restore. Objective checks should fail automatically; agent presentation
reviews record capture paths, criteria, judgment and rationale separately. A machine
pass with missing required visual/audio review is not full QA acceptance. Never
substitute cached reviews/results for a required fresh scenario execution/review.
Keep comparison sanity separate from gameplay expectation profiles.

## Native navigation and setup

Reuse existing native checkpoints/recipes after validating executable/ROM/assets
identity and relevant semantic preconditions. Reference halves of old paired
checkpoints are historical evidence, not mandatory setup dependencies. After code
changes regenerate only affected native setups via retained inputs; never bypass
identity checks or write state to grant prerequisites. Preserve natural-play origin.

Prefer one persistent native session with bookmarks and bounded semantic run-until
commands. Native disk checkpoints replay history. Advertised port.snapshot.save/load
provide fast named rewind within a persistent session; save before experimenting
and retry from that bookmark. Older ports may support only replay fallback.
Avoid process restarts/revalidation for each jump. Advance known-safe spans in
batches; inspect compact state at decisions and save useful milestones. There is
no source clock to synchronize during navigation. Do not require damage-free or
optimal routes unless that is the test; normal life loss/recovery can be valid.
An endpoint or event bound that fails is setup failure, not evidence of success.

Once a bounded operation is running, wait for completion in waits <=60 seconds.
Do not fill the wait with speculation, repeated source reads or file polling.
Keep progress updates concise. Log setup/restore, discovery and test execution
costs separately. Build tools only to resolve a demonstrated blocker; avoid
perfecting navigation tooling instead of executing a test already in reach.

## Two gameplay passes, repairs and completion

Keep the installed two-pass whole-game acceptance. Resume the current check;
do not restart a pass or discard accepted checks. Complete pass one, then one
fresh full gameplay pass two, repairing in place with targeted affected-check
regressions. No third confidence pass. Scope corrections preserve previous
plans/results and reconcile plan hashes. A blocked check stays incomplete with
an exact resumption point; never fabricate results or weaken a valid failure.

On failure inspect the smallest useful evidence. Bring in the original and precise
tracing only when needed to resolve uncertainty or identify a cause. Fix the code
and meaningful regression; do not turn every diagnostic byte/frame into permanent
whole-game acceptance. Follow architecture/coding standards and relevant pinned
Rust 1.90.0 locked checks after repairs. Never copy GPL emulator code into the MIT
runtime or launch GUI/audio devices.

Completion requires both gameplay passes, the applicable initial sanity record,
resolved issues, selected presentation reviews and a report of tested scope and
limitations. Run the QA record audit and implementation audit/final relevant checks.
Audits validate accounting, not semantic truth. An external resource interruption
leaves a resumable incomplete handoff. Stop after QA; no automatic modernization.
