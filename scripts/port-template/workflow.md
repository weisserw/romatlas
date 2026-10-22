# Iterative native port workflow

Invoking goal.md authorizes this complete loop. Act as coordinator and execute
three subprocesses sequentially, each with a subagent: planning, porting, debug.
Wait for each handoff; never run agents editing this workspace concurrently.
Preparation installs instructions only and launches no agents. If subagents are
unavailable, preserve a resumable handoff and report the concrete tooling blocker.

1. Launch a planning subagent with prompts/plan.md. It analyzes the disassembled
   source and writes architecture.md: game architecture, modes, systems, special
   concerns and a finite ordered series of milestones covering the entire game.
2. Launch a porting subagent with prompts/start.md and exactly one selected
   milestone. It implements that milestone, updates accounting and progress, runs
   relevant checks and applicable smoke-test.md steps, and hands off. Future
   behavior remains pending, never hidden behind exclusions or stubs.
3. Before debug, initialize its cycle from the toolkit:
   `python3 scripts/port_debug.py begin-cycle /path/to/workspace --milestone ID`
   Add `--final` only when all milestones are implemented, whole-game accounting
   is complete and `python3 run.py audit` passes. Use a unique ID for each cycle.
   Launch a debug subagent with debug/goal.md and the architecture.md milestone
   status. It derives a cumulative test plan covering currently implemented
   milestones, runs it ONE TIME, fixes failures in place and continues. It does
   not restart the plan after fixes. The final cycle instead derives a complete
   whole-game plan and runs it twice. Prior cycle results cannot substitute for
   either final pass. Run the QA record audit before accepting its handoff.
4. Launch planning again after each successful debug cycle. It incorporates the
   QA report, updates status/design and may revise future milestones based on
   discoveries. Repeat steps 2–4 until the finite plan is complete. After final
   QA, planning reconciles final status without creating a new test cycle merely
   for confidence. Missing required behavior requires more implementation;
   unassessed device behavior does not.

Keep stage, cycle ID, selected milestone, implementation/QA status, report paths
and exact resumption point in architecture.md and progress.md before each handoff.
A blocked stage stays incomplete; resume it rather than advancing or recreating
records. begin-cycle with the same ID resumes existing records; a new cycle may
begin only after successful audited QA of the previous cycle. It archives the
previous plan/results/acceptance before initializing fresh records. Retain cited
artifacts and distinct cycle reports; never overwrite previous evidence.

On the final handoff require every milestone implemented and QA complete,
whole-game ledger and uncertainties complete, coherent architecture, README.md,
BUILDING.md, differences.md, library-changes.md and implementation/QA reports.
Run the implementation and final QA audits; stop when both pass. Audits establish
record/build accounting, not semantic truth. No third final traversal or repeated
passing checks for confidence. The final QA uses the existing continue-in-place
repair rules, including targeted rechecks of affected earlier checks.

Implementation keeps the existing prompts/start.md source-informed architecture,
useful Rust comments, descriptive enums/constants, named ROM addresses/offsets
isolated in loaders or asset definitions, ROM-backed assets, extensions,
settings, saves, shared desktop/browser logic and library-change requirements.
Use bounded debugger experiments with a named question and stopping condition.
Broader runtime fidelity validation is outside the porting subprocess; the debug
subprocess owns comparisons and comprehensive coverage within implemented scope.
No desktop graphics or unavailable devices on this headless server.

Preserve reference/, original ledger identities and existing game workspaces.
Use prompts/reference-updates.md for optional evidence-backed corrections; inspect
research/current.json for adopted supplements. Source incompleteness is not a new
disassembly completion requirement. Implement reasonable source-informed assumptions
and document uncertainty honestly. Keep shared-library edits uncommitted for the
separate reviewer using prompts/review.md; no check-in or publication is authorized.
Toolkit tests do not add game-fidelity acceptance requirements. Keep generated ROMs,
source, assets and captures local and follow the existing disk/evidence discipline.
