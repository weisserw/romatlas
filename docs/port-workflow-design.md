# Native-port pipeline design

This is the current workflow design beyond the original M1–M4 mapping scope.
The [porting guide](porting.md) owns commands and runtime details; the
[coordinator template](../scripts/port-template/workflow.md) owns per-game stage
handoffs. This design does not claim a real game has completed the pipeline.

Preparation accepts a reviewed archive and matching locally supplied ROM,
restores/verifies its source, and creates a separate Rust workspace. It launches
no agents and never migrates an existing game workspace implicitly. Unknown
source regions are tracked by consumers and port impact without requiring the
original disassembly goal to be completed first. Frozen source/provenance and
original accounting identities remain intact; verified corrections supplement
them through the reference-update procedure.

Invoking goal.md authorizes a fixed sequence of five subagent stages:

1. Skeleton: toolkit ROM loading/settings, desktop/browser wiring, all source
   actor/state/level/audio/table/asset identifiers, native module/state interfaces
   and semantic headless debug/checkpoint groundwork. No game/menu/title logic.
2. Behavior planning: inventory all behavior with source references and dependencies
   in behaviors.md; refine state ownership, modules and scheduling in architecture.md.
3. Behavior contract: a stage coordinator starts one separate worker per behavior
   area, sequentially. Each rereads source independently, traces detailed branches,
   low-level flag/width/carry semantics and cross-call side effects, then writes
   executable unit tests with adjacent source evidence in logical source files.
   Tests compile and are discovered; unimplemented behavior may fail assertions.
4. Game implementation: implement the whole game in the prepared files against
   original source and contracts until all tests pass. Maintain meaningful names,
   comments, semantic debug observations and complete source accounting. Finish
   documentation, build/static checks and a bounded maintainability pass before
   the implementation audit and QA handoff.
5. QA: run only the short initial smoke test and selected exact-frame comparisons
   against the original, including representative rendering/scrolling/animation/audio
   checks. Retain reusable scripts and validated setup, repair affected checks in
   place and audit one bounded suite execution. No whole-game QA passes follow.

The naming/comment standards from the existing implementation prompt are retained
verbatim in both generated contract and implementation prompts, with one owning
coding-standards template. All authored Rust, fixtures and debug adapters follow
those standards. Planning is a navigation aid, never an authority for test
expectations; source must be reread and cited alongside the tests. Valid contracts
cannot be weakened merely to make implementations pass. New discoveries update
the active stage's inventory, traces, tests and code without a milestone loop.

The implementation is native game behavior, not an embedded emulator or literal
assembly translation. Use independent MIT runtime helpers and source-informed
modern algorithms. Original assets/data load from the user's ROM at startup and
stay out of distributions. Validated overrides and added named slots coexist
with original content. Game rules, asset preparation and original battery saves
are shared between desktop and browser; controls/settings persist through the
frontends. Player save states are outside scope. Headless debug replay checkpoints
are a separate test facility and must preserve deterministic initialization/time.
No GPL emulator implementation enters the MIT runtime.

The semantic debug API exposes the state/events necessary for contracts and QA,
with stable identifiers, units, retention and read-only inspection documented in
debug/api.md. Existing replay checkpoints validate identity and output hashes;
scenarios additionally assert semantic preconditions. Changed builds/assets
require regenerated checkpoints from retained setup recipes. Original emulator
snapshots remain separate. Artificial setups cannot bypass transitions under test.
Every required QA pass executes its scenarios afresh; cached successes do not count.

Whole-game completion requires implemented behavior, complete behavior/uncertainty
accounting, passing source-backed unit contracts, format/static/desktop/Wasm
checks, documentation, the short smoke/comparison suite and both administrative audits.
Audits validate records/builds, not semantic truth. Failures block advancement;
targeted repairs retain unaffected results and pass position. Resource exhaustion
leaves a resumable incomplete handoff. There is no third pass, restart rule,
intermediate smoke route or invented goal-wide repair budget.

Headless Linux checks never launch SDL or claim untested desktop/device/other-OS
compatibility. Hardware-induced slowdown/flicker may be omitted; intentional
gameplay timing/effects remain. Shared-library work is authorized in the toolkit,
uncommitted, with initial-file snapshots and a change log for a separately invoked
review. Check-in, packaging, immutable dependency pinning and publication remain
later scope. Toolkit checks do not add game-fidelity acceptance requirements.
