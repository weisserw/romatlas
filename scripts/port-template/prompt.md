# Implement one milestone of the native port

Read architecture.md and the coordinator-selected milestone first. Implement only
that milestone, preserving earlier work. The whole game remains the overall goal;
future milestones stay pending. Hand off to the coordinator for scoped QA.

Read workflow.md, uncertainties.json and
prompts/reference-updates.md. Use reference/reading.asm and relevant archive
documents while surveying and implementing the source; reading the entire source
before starting is not required.
On resume read architecture.md, progress.md, differences.md, library-changes.md,
ledger.json and existing code. Inspect research/current.json and its adopted
reading/source supplement when present. Preserve work; do not reinitialize.
The scaffold is an arbitrary ROM-backed tile demonstration. Replace it incrementally
according to the milestone plan; a scaffold, first level or playable demo does not
complete the overall goal.

## Write readable native code from the first implementation

Implement behavior in terms of the current game's concepts, not a transcription
of assembly operations with the same unexplained numbers. Understand the relevant
source and consumers, define meaningful types and names, then use them while
writing the logic. Do not postpone naming or comments to a later cleanup pass.

Do not use bare numeric literals for values whose meaning comes from the game
or original hardware. The following examples illustrate categories, not mechanics
or names that every game must have:

| Avoid in executable game logic | Express the meaning instead |
| --- | --- |
| `rom[0xf4db]` or repeated numeric ROM ranges | A named ROM location or asset-loader definition, followed by decoded data |
| `memory[0xc120]` for game state | A named field in a native state structure |
| `flags |= 1 << 14` or `flags & 0x40` | A named flag/mask or typed flag operation |
| `state == 3` or `set_state(0x2a)` | An enum variant, or a named ID when an integer representation is needed |
| `distance < 16` or `position >= 176` | A constant identifying the range or boundary, with units/coordinate space |
| `speed -= 4` or `timer = 30` | A named rate, increment or duration in the appropriate units |

Small decimal integers need meaningful names just as hexadecimal addresses do.
Names must explain the role: `STATE_3`, `VALUE_16` and `DATA_F4DB` only repeat the
encoding and are insufficient. Reuse existing names instead of creating aliases.
Use enums for finite semantic choices and typed interfaces where practical; use
scoped named constants when numeric representation is appropriate. Keep original
encodings and numeric locations in definitions, loaders or serialization code,
not scattered at call sites. Distinguish banked CPU addresses from physical ROM
offsets. Original RAM addresses should become native state fields, not a second
memory map. Preserve exact data encodings and deterministic behavior.

Write useful comments alongside the implementation. Document major modules and
important type/interface contracts. For a condition, calculation or transition
whose purpose is not apparent from its names, explain what its inputs represent,
what rule it implements and what effect it has. Also explain relevant invariants,
update ordering, source quirks and intentional differences. Naming each constant
alone is insufficient if the combined expression still obscures the behavior.
Derive names and explanations from this game's source; distinguish facts from
assumptions. If a meaning is unresolved, use an honest provisional name and state
what is known without inventing intent. Cite source labels or addresses when
useful for traceability. Avoid comments that only paraphrase the Rust statements.

Ordinary literals whose meaning is evident, such as zero initialization, and
opaque ROM table contents need no artificial constants. Executable conditions,
call arguments and game parameters are not opaque data merely because they came
from assembly. No abstraction or comment-count quota applies. These are coding
requirements, not an additional review phase, report or gameplay traversal.

## Source-informed architecture before substantial implementation

Read representative source paths for startup, gameplay, rendering and sound.
Identify major systems, state ownership, update order and dependencies on original
data or hardware. This is a bounded initial survey, not exhaustive source analysis.
The planning subagent has written the design in architecture.md. Check its current
milestone against the relevant source before substantial implementation:
proposed modules/files, responsibilities, public interfaces, state owners, data
flow, scheduling, shared-library use, uncertainties and an implementation order.
Choose boundaries around game concepts rather than assembly banks or routine lists.
Proceed with the selected milestone once the plan is actionable; no approval is required.
Record discoveries and proposed future-plan changes for the next planning subagent.

Maintain that plan as understanding changes. Keep modules cohesive, state ownership
explicit and interfaces small. Express algorithms with clear names, focused
functions and understandable control/data flow. Avoid one growing game.rs containing
simulation, assets, rendering, audio and UI. Splitting a tangled implementation
across files without improving ownership/coupling does not satisfy this requirement.
There are no arbitrary file-count, line-count or abstraction quotas.

After a substantial feature, briefly inspect the affected design against the plan;
fix concrete coupling, duplication or unclear algorithms before building on them.
This is part of implementation, not a new review phase or repeated whole-project
audit. Update the plan only when decisions change. Apply the naming/comment
requirements during implementation; do not schedule a separate milestone
readability review, review report or rewrite phase. At the final implementation milestone, make one
bounded maintainability pass, including unexplained semantic numeric literals,
scattered bare addresses/offsets and missing or stale explanations of non-obvious code;
do not keep redesigning a coherent working implementation.

## Implementation and behavior accounting

Use ledger.json to account for the selected milestone's archived regions and meaningful units as
implemented, merged, replaced or excluded, with a behavior explanation and native
equivalents. Add discovered units while preserving original IDs and ranges.
Implement every gameplay-relevant equivalent in this milestone; modern methods may merge routines
or remove original mechanisms. Judge obvious irrelevant junk/unreachable paths
from source without a formal reachability proof. Favor implementing borderline
or unclear behavior. Lack of observed execution alone does not justify exclusion.

Incomplete disassembly/classification is allowed. Assess unknown ranges and
semantic ambiguities by their consumers and port impact as implementation reaches
them. Do not complete the whole-ROM mapping task as a port prerequisite. Prefer
source reading, data inspection and existing documentation. Where a reasonable
source-informed implementation is possible, implement it, record the assumption,
basis and residual uncertainty, and continue. Distinguish an implementation
assumption from an established fact about the original. Do not demand proof of
every interpretation or runtime confirmation of every branch.

For uncertain data, keep original values ROM-backed when a supported consumer can
interpret them; unit-test loading/bounds where useful. Preserving code bytes is
not native behavior. Do not silently skip handlers, zero unknown fields or label
behavior irrelevant to finish accounting. Missing required implementations remain
incomplete; implemented best-effort behavior with documented assumptions does not
remain blocked merely because fidelity has not been independently confirmed.
Use prompts/reference-updates.md for supported source corrections; its map and
byte-identical rebuild requirements protect reference integrity only. Never publish
an assumption as confirmed classification. Frozen reference/ stays unchanged.

## Runtime library, assets and player interface

The default target is a faithful representation of the source game, with no extra
game features. Preserve its intended gameplay, progression and presentation within
the documented best-effort implementation scope. Unless the user specifically
instructs otherwise, do not add visible debug artifacts or developer conveniences:
level/stage selects, sound tests, debug overlays, diagnostic screens, cheats,
unlock menus or extra gameplay modes. Preserve such features when they belong to
the source game, with their original access conditions; do not expose hidden source
debug functions as new player-facing controls. Keep implementation diagnostics in
logs or headless development tools, outside the player interface. Required frontend
settings and asset replacement/extension support below remain supported; they do
not authorize extra in-game features or menus for selecting added content. This
presentation target is checked only within the bounded smoke-test scope below.

Faithfulness does not require reproducing hardware-induced lag, slowdown or
sprite flicker. The native port may run smoothly and render sprites without those
artifacts; this is not an extra game feature. Preserve intentional gameplay timing
and visual effects, such as deliberate slow motion or damage/invulnerability
flashing.

Use romatlas-port for common mechanisms and platform-specific effects, including
available audio/video helpers. Review their documented limits. Keep game rules,
sequence interpretation and unusual tricks in the game when appropriate. Simulate
at the console cadence independently of monitor refresh. Implement the intended
behavior from source using the available facilities; document approximations.

All original graphics, fonts, music, sound and gameplay data, including maps,
come from the matching user's ROM at startup through recipes. Do not embed
extracted originals in code/distributable files. Support validated file overrides
and added named slots: original ROM-backed levels and added file-backed levels
must work together without a fixed level count. Keep decoded originals and
regenerated assembly local. Never download commercial ROMs or commit ROMs/assets.
Developer-authored content may ship.

Implement needed shared-library changes directly in the toolkit recorded in
workspace.json. Read its AGENTS.md and project.md. Leave edits
uncommitted, preserve unrelated work and distinguish the initial state in
review/toolkit-before.json and review/toolkit-before/. Log each change's requirement,
files, reusable interface rationale, focused tests and limits in library-changes.md.
Two consumers are not required to justify a reusable utility. Keep uncertain
abstractions local. Complete needed library work during implementation rather than
merely filing an issue. Known library limitations do not authorize fidelity audits.

Supply persisted keyboard/gamepad mapping, controller selection, dead zones,
window/fullscreen/scaling and volume settings through a usable in-game menu.
Keep simulation independent of frontend devices so headless unit tests need neither a
window nor an audio device. Compile desktop code; do not launch it on this server.

## Checks and debugger boundary

Validate the native implementation with focused unit tests of its intended logic.
Reserve unit tests for complex or error-prone behavior that benefits from explicit
verification; skip trivial arithmetic, obvious getters/setters and tests that merely
restate the implementation.
Write a useful normal amount of tests for meaningful branches, bounds and system
interfaces; assertions should explain the intended behavior, not merely mirror
implementation details. Do not turn the source inventory into an exhaustive test
matrix. Use synthetic fixtures or local ROM data without distributing originals.

Before handing off, execute the applicable portion of the bounded [initial smoke test](../smoke-test.md).
It covers boot/title-screen accuracy, representative colors, menu interaction and
starting play, basic movement/actions/enemy or equivalent interactions, and the
first area's exit or normal death as appropriate. Fix exposed defects and recheck
affected steps; record route, build/ROM identity and results in checks/smoke-test.md.
Use device-free controls/captures and targeted original-ROM observations for these
checks. Record future smoke steps as deferred with milestone dependencies; never
call them passed. When the full route is implemented it is required. This porting
subagent does not execute QA; the coordinator invokes the debug subagent next.

Broader runtime fidelity validation is outside this porting subprocess. Do not expand the smoke
test into pixel-perfect/frame-by-frame proof, exhaustive scenarios, later areas,
audio equivalence or repeated passing routes for confidence. Once its defined
route passes, stop. A failed or unperformed required smoke test blocks completion;
report concrete tooling or implementation blockers rather than assuming it passed.

The debugger may also answer a specific unresolved source question that static
inspection cannot reasonably answer. Record the question, needed observation and
stopping condition; run a bounded experiment and return to implementation. Do not
repeat inconclusive runs without a revised hypothesis/method or reopen answered
questions without contradictory evidence. Document reasonable assumptions where
needed; assumptions do not excuse broken smoke-test steps. Full-game QA belongs
to the debug subprocess; device testing remains separately requested.

Run python3 run.py check for formatting, clippy, unit tests and desktop compilation.
After shared-library changes run the toolkit's required relevant regression checks,
including portability/dependency boundaries, and report actual results and limits. Those toolkit checks protect library/tool behavior; they do not
expand the native game's comparisons beyond the bounded smoke-test scope. Once a
relevant check passes, rerun it only after a pertinent change, failure or concrete
new defect. Do not add more validation simply because time or goal budget remains.

## Completion and stopping rule

Maintain README.md with running/controls, asset replacement, added levels and game
rule locations. Adapt BUILDING.md with build recipes, exact source/lockfile/path
requirements and actual build/unit-test/smoke-test results. Other-platform/device
execution remains untested when unavailable. The debug subprocess records fidelity results. Provide an
original-content-only extension manifest. Keep progress.md and differences.md
current, recording assumptions, library limits and known differences without a
checklist of fidelity experiments to perform before completion.

Finish this subprocess when the selected milestone is implemented, its ledger
entries and relevant uncertainties are assessed (including implemented assumptions),
required checks and applicable smoke steps pass, and progress.md records changed
systems, tests, deferred smoke steps and QA entry routes. Leave future ledger
entries pending. Mark the milestone implemented, awaiting QA; do not mark it QA
complete. Return to the coordinator rather than implementing another milestone.
On the final milestone, finish whole-game accounting and uncertainty assessment,
make the bounded maintainability pass and set ledger completion.status=complete.
Run python3 run.py audit for the final implementation only; after it passes, stop
this subprocess. This audit does not complete the coordinator goal: final QA must
still execute two complete runs through its whole-game test plan.

Leave shared-library edits for the separate reviewer with the actual diff,
requirements and check results. No check-in, publication or agent launch is
authorized. Release packaging and immutable dependency pinning are later work;
Cargo.lock does not freeze a local path dependency. If actual required code cannot
be implemented, preserve work and report the concrete blocker; do not mark absent
behavior implemented. Fidelity uncertainty alone does not make the port incomplete.

## Desktop and browser contract

Future ports must build for both desktop SDL2 and browser Wasm. Keep game rules,
ROM-backed startup extraction (`Port::load_assets`), decoded defaults, asset
validation and battery-save representation shared. Use the generic Game API;
frontends translate input, rendering, audio and persistence automatically. Do not
introduce browser/SDL branches into game rules. Keep dimensions, TickRate and settings defaults in shared config.rs when replacing
the scaffold; both entrypoints use that configuration automatically. Use MemoryAssets::collection for fetched or filesystem overrides;
load only game-owned manifests on desktop. Never embed original ROM data.

If the original supported battery saves, implement optional Game save hooks with
versioned persistent data, transactional validation and dirty/acknowledgement.
The desktop frontend loads/stores ROM-bound files; browser localStorage uses the
same envelope. No full runtime save states are required or supported. Explain
original saving behavior and save format in the port documentation. No-battery
originals keep the default hooks. Run check/audit including Wasm compilation;
the smoke test can use headless controls; no separate browser/device audit is required.
Document `python3 run.py web` in BUILDING.md. Existing ports are not migrated.

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
