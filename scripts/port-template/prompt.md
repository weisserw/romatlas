# Implement the entire game as a maintainable best-effort native port

Read workflow.md, uncertainties.json and
prompts/reference-updates.md. Use reference/reading.asm and relevant archive
documents while surveying and implementing the source; reading the entire source
before starting is not required.
On resume read architecture.md, progress.md, differences.md, library-changes.md,
ledger.json and existing code. Inspect research/current.json and its adopted
reading/source supplement when present. Preserve work; do not reinitialize.
The scaffold is an arbitrary ROM-backed tile demonstration. Replace it with the
whole game; a scaffold, first level or playable demo is not completion.

## Source-informed architecture before substantial implementation

Read representative source paths for startup, gameplay, rendering and sound.
Identify major systems, state ownership, update order and dependencies on original
data or hardware. This is a bounded initial survey, not exhaustive source analysis.
Write a short high-level plan in architecture.md before substantial implementation:
proposed modules/files, responsibilities, public interfaces, state owners, data
flow, scheduling, shared-library use, uncertainties and an implementation order.
Choose boundaries around game concepts rather than assembly banks or routine lists.
Proceed with implementation once the plan is actionable; no approval is required.

Maintain that plan as understanding changes. Keep modules cohesive, state ownership
explicit and interfaces small. Express algorithms with clear names, focused
functions and understandable control/data flow. Avoid one growing game.rs containing
simulation, assets, rendering, audio and UI. Splitting a tangled implementation
across files without improving ownership/coupling does not satisfy this requirement.
There are no arbitrary file-count, line-count or abstraction quotas.

Give meaningful values descriptive names throughout the Rust implementation.
Prefer enums with descriptive variants for game modes, actor states, commands and
other finite semantic choices: write `state == ActorState::Falling` rather than
`state == 3`. When numeric storage or ROM compatibility makes integers appropriate,
use scoped named constants for state IDs, flags/masks, sentinels, table fields,
timing values and gameplay parameters. Name values by their role, not their number
(avoid names like STATE_3 or VALUE_16); include units where useful. Keep raw numeric
encodings at ROM/serialization boundaries, preserving exact values and handling
unknown values explicitly. Do not invent meanings for unresolved IDs: use honest
provisional names and nearby explanations of what is known or assumed. Ordinary
obvious literals such as zero initialization need no artificial constants, and
opaque ROM data tables need not be renamed element by element.

Include useful comments in the Rust code as you implement it. Add module-level
documentation for major systems and doc comments for important types/interfaces
where their purpose or contract needs explanation. Near non-obvious logic, explain
why it works: source-game behavior and quirks, algorithm invariants, update/timing
order, numeric formats and constants, and intentional differences or implementation
assumptions. Cite relevant source labels or routines when they help a maintainer
trace the reasoning; distinguish supported facts from assumptions. Keep these
explanations close to the code rather than only in architecture.md or the ledger.
Use concise natural language, avoid comments that merely restate Rust statements,
and update comments when the implementation changes. No comment-count quota applies.

After a substantial feature, briefly inspect the affected design against the plan;
fix concrete coupling, duplication or unclear algorithms before building on them.
This is part of implementation, not a new review phase or repeated whole-project
audit. Update the plan only when decisions change. At completion, make one bounded
maintainability pass, including unexplained semantic numeric literals and missing
or stale explanations of non-obvious code;
do not keep redesigning a coherent working implementation.

## Implementation and behavior accounting

Use ledger.json to account for every archived region and meaningful unit as
implemented, merged, replaced or excluded, with a behavior explanation and native
equivalents. Add discovered units while preserving original IDs and ranges.
Implement every gameplay-relevant equivalent; modern methods may merge routines
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
workspace.json. Read its AGENTS.md, project.md and docs/progress.md. Leave edits
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

Before declaring completion, execute the bounded [initial smoke test](../smoke-test.md).
It covers boot/title-screen accuracy, representative colors, menu interaction and
starting play, basic movement/actions/enemy or equivalent interactions, and the
first area's exit or normal death as appropriate. Fix exposed defects and recheck
affected steps; record route, build/ROM identity and results in checks/smoke-test.md.
Use device-free controls/captures and targeted original-ROM observations for these
checks. This does not invoke the separate debug/goal.md two-pass QA workflow.

Broader runtime fidelity validation is outside this goal. Do not expand the smoke
test into pixel-perfect/frame-by-frame proof, exhaustive scenarios, later areas,
audio equivalence or repeated passing routes for confidence. Once its defined
route passes, stop. A failed or unperformed required smoke test blocks completion;
report concrete tooling or implementation blockers rather than assuming it passed.

The debugger may also answer a specific unresolved source question that static
inspection cannot reasonably answer. Record the question, needed observation and
stopping condition; run a bounded experiment and return to implementation. Do not
repeat inconclusive runs without a revised hypothesis/method or reopen answered
questions without contradictory evidence. Document reasonable assumptions where
needed; assumptions do not excuse broken smoke-test steps. Full-game QA and device
testing require a separate user request.

Run python3 run.py check for formatting, clippy, unit tests and desktop compilation.
After shared-library changes run the toolkit's required relevant regression checks,
including portability/dependency boundaries, and update its docs/progress.md with
tested facts. Those toolkit checks protect library/tool behavior; they do not
expand the native game's comparisons beyond the bounded smoke-test scope. Once a
relevant check passes, rerun it only after a pertinent change, failure or concrete
new defect. Do not add more validation simply because time or goal budget remains.

## Completion and stopping rule

Maintain README.md with running/controls, asset replacement, added levels and game
rule locations. Adapt BUILDING.md with build recipes, exact source/lockfile/path
requirements and actual build/unit-test/smoke-test results. Other-platform/device
execution and fidelity beyond the smoke-test route remain unassessed unless requested. Provide an
original-content-only extension manifest. Keep progress.md and differences.md
current, recording assumptions, library limits and known differences without a
checklist of fidelity experiments to perform before completion.

Finish when the whole game's best-effort implementations and behavior accounting
are complete, uncertainty entries are assessed (including implemented assumptions),
the architecture is coherent and documented, the smoke test and required checks pass and the handoff
is written. Set ledger completion.status=complete with notes describing that scope;
run python3 run.py audit once to check accounting/builds. It is an administrative
check, not a fidelity audit. Fix actual failures; after it passes, stop. Assume
the implemented behavior outside the smoke-test route works for this handoff; do not claim
independent gameplay or audiovisual equivalence. Remaining unverified similarity
must not trigger another investigation, replay, audit or polishing cycle.

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
