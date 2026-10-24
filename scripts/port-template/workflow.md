# Fixed native-port pipeline

Invoking goal.md authorizes this pipeline. Act as coordinator and launch the
following five stages in order, each in its own subagent. Wait for each handoff.
Stage 3 launches one worker subagent per behavior area, as described below.
Only one agent may edit this workspace at a time; trace workers run sequentially.
Preparation installs instructions only and launches no agents. If subagents are
unavailable, preserve work and report the concrete tooling blocker.

1. **Skeleton — prompts/skeleton.md.** Build ROM loading, settings, shared desktop/
   browser targets, named identifiers and the semantic debug/checkpoint groundwork.
   Implement no gameplay, title or game-menu logic. Require passing infrastructure
   checks and a documented debug schema before handing off.
2. **Behavior planning — prompts/plan.md.** Survey all source behavior, write
   behaviors.md with stable IDs and source references, and refine architecture.md.
   Include assets, display, sound, controls, actors, transitions, gameplay, menus,
   titles and terminal states. This is an inventory, not a substitute for tracing.
3. **Behavior contract — prompts/contract.md.** This stage's subagent coordinates
   a separate worker subagent for every inventoried behavior area. Give each
   worker the full contract prompt, its area ID and source/module context. Each
   independently rereads and traces source in detail, including cross-call side
   effects and architecture-specific branch/flag semantics, then writes executable
   unit tests and adjacent evidence in logically organized source files. Reconcile
   cross-area dependencies and any newly discovered areas; give every added area
   its own worker too. Require compiling, discovered, substantive tests; failing
   behavior assertions are expected until implementation. Do not implement game
   behavior or weaken contracts to make this stage green.
4. **Game implementation — prompts/start.md.** Implement the whole game in the
   prepared files, checking the original source as well as contracts. Make every
   behavior test pass and expose all relevant logic to the headless debug API.
   Complete ledger/uncertainty accounting, documentation and the bounded
   maintainability pass. Require python3 run.py check and python3 run.py audit
   to pass before QA. A scaffold, partial game or stub is not completion.
5. **QA — debug/goal.md.** Use the installed two-pass whole-game acceptance
   contract. Plan source-backed checks and create/extend reusable scenario scripts
   that restore validated starting states, apply inputs and observe/assert relevant
   behavior through the native debug API and original emulator. Execute each pass
   afresh, reusing scripts/checkpoints. Repair failures at the current check with
   targeted regressions; never restart a pass or return to earlier pipeline stages.
   Require the QA record audit and final implementation checks after repairs.

Read prompts/common.md for the shared contract. Record each stage's status,
subagent/area handoffs, artifact paths, blockers and exact resumption point in
progress.md. Keep design/state owners in architecture.md and area-to-source/test/
implementation links in behaviors.md. At stage 3, track pending/active/completed
workers and their test compile/run outcomes. Completion here means traced and
tested, not implemented. Never mark knowingly failing tests as passed.

This is a fixed pipeline, not a milestone loop. Resume an interrupted stage or
worker in place; preserve earlier artifacts. New discoveries update affected
inventory, contracts and implementation within the active stage. During QA,
repair code/contracts/scenarios in place with source-backed reasons, retaining
check IDs and valid results. Do not relaunch planning/development/QA cycles,
reset records or invoke begin-cycle for a newly prepared pipeline workspace.

Stop when whole-game behavior is implemented, both QA passes succeed, both
administrative audits pass and final handoffs are written. Verify behaviors.md
has no missing required areas/contracts/implementations; ledger.json and
uncertainties.json are complete; README.md, BUILDING.md, differences.md and
library-changes.md describe the actual result. Audits check records/builds, not
semantic truth. No third traversal or repeated passing checks for confidence.
Device behavior unavailable on this server remains explicitly untested.


## Optional post-QA modernization beta

Preparation also installs modernize/goal.md and empty beta records. They do not
start a sixth stage, alter completion criteria or authorize a pilot. Finish the
existing full two-pass whole-game QA and stop normally. Only a separate later
user request can invoke that workflow, after its full-QA prerequisite is met.
A partial pass or successful individual scenario is insufficient. Keep any later
modernization work in a separate commit from QA tooling and faithful-port fixes.
