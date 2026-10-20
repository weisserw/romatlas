# Milestone checklist

T001 region explanations are now available through map.annotate: optional name, role, evidence and comment strings are stored in region_metadata, exposed by map.get and retained by map.save/load. Null removes a field; unchanged extents preserve imported directives and explanations. Structural edits discard old metadata; all edits stage and validate the full proposed map before committing. Names export as stock symbols; explanatory text remains internal. Metadata no longer requires a sym directive. Rust 1.90.0 fmt, locked workspace clippy/tests/build, synthetic session, API and stock portability pass, including byte-identical rebuilding with a named region and imported 1bpp metadata. A focused mapping unit test also passes for explanation-only metadata and named export. No commercial ROMs or extracted assets were used.

Generated disassembly workflow and start/resume prompts now require game familiarization using available online manuals, FAQs/walkthroughs and technical resources after game/version identification. New workspaces initialize analysis/gameplay.md for source URLs, version uncertainty, actionable controls/mechanics/progression/password routes and local confirmation status. Findings feed targeted runtime scenarios; external claims do not independently justify structural classification. Agents revisit research for specific obstacles and check documented routes before exhausting gameplay-dependent paths, preserving research across sessions and continuing local analysis if browsing/resources are unavailable. INSTRUCTIONS.md documents the behavior. Rust 1.90.0 fmt, locked workspace clippy/tests, synthetic workspace regression, stock portability, session tests and git diff --check pass. Regression checks cover generated prompts, gameplay record initialization, evidence distinction and exhaustion review. No live browsing/disassembly-agent behavior was tested; existing workspaces are not automatically updated.

Alleyway toolkit report fixes (RA-001–RA-003): cartridge loading no longer gates
execution on .gb/.gbc filenames. The synthetic RPC workflow now runs from .rom;
an adapter regression also covers extensionless/uppercase filenames, reset and
unchanged input bytes. trace.query returns next_offset, with cursor traversal,
final/empty/out-of-range pages and zero limits covered by the session regression.
Raw export gaps and Unknown regions split at pinned stock vector and DMG/CGB
header block starts, overriding interior default code/text blocks without changing
the classification store. Synthetic empty/mixed stores now verify generated
statement kinds and byte-identical rebuilds for both header variants. No commercial
ROMs or extracted assets were added.

Tested on Rust 1.90.0: locked build, workspace fmt/clippy/tests (55 adapter tests),
all nine standalone core tests, session, fidelity, stock portability, API,
disassembly-workspace and crate-boundary regressions pass. Whole-tree
`cargo fmt --all --check` retains the previously documented unrelated vendor
formatting failures; workspace `cargo fmt -- --check` passes. The modified
cartridge source is formatted. The rebuilt debug toolkit is available locally.


Disassembly workspace initialization now defaults to the caller's current directory,
creating a per-ROM subdirectory there. Explicit --root/--workspace options and the
repository-local destination guard remain supported. README and startup instructions
document the default. The synthetic workspace regression verifies initialization from
a separate current directory with spaces. Workspace regression, cargo fmt -- --check,
locked workspace clippy/tests and stock portability pass on Rust 1.90.0.
The broader cargo fmt --all -- --check reports existing vendor/mizu formatting issues;
no vendored Rust files were changed for this Python default adjustment.

## M1 — complete

- [x] Rust workspace, pinned toolchain, lockfile, basic CLI and tests
- [x] ROM parser, SHA-256 inspection, physical bank translation
- [x] Stock mgbdis `.sym` parsing/export and overlap diagnostics
- [x] Pinned mgbdis/RGBDS generation and exact verification
- [x] Clean recipient portability test for a four-bank synthetic ROM
- [x] Complete SM83 decoder; all opcode lengths/timing, CB operands/flags, invalid and truncated input tests
- [x] Independent hand-derived base operand/flag matrix for all 256 opcodes, signed/little-endian values and indirect/post-update fields
- [x] Internal store with ROM hash validation, provenance, transactional edits and overlap checks (Rust API)
- [x] Store CLI operations and Game Boy export bridge; annotated-code clean portability test
- [x] Platform-neutral JSON-RPC session, mock adapter, Game Boy emulator/map command routing
- [x] Persistent .sym import, aliases/arguments/labels retained through internal state save/load
- [x] Bounded PPU-cycle and real VBlank-frame stepping, LCD-off budget test
- [x] Minimal stock-output workaround for undefined RAM/I/O labels, tested with stock RGBDS -D reproduction
- [x] ROM content and directive-argument checks, label-boundary checks, and full generation rebuild gate; unsafe annotations fail with raw-data guidance

## M2 — complete

- [x] Mizu selection, pinned MIT fork and license/maintenance assessment
- [x] Headless DMG ROM-only/MBC1 stepping; RAM, bank aliases, reset, peek, snapshot replay tests
- [x] Pinned source-built MIT Mooneye runner and 83-case DMG/MBC1/MBC5 plus CGB boot-state regression gate
- [x] Fix LCD-start reads and mode-2-to-mode-0/mode-3 interrupt timing; add three regression gate cases
- [x] LCD-write boundary timing: separate read/write access locks and physical trace translation
- [x] SCX/HBlank phase timing and regression gate case
- [x] Sprite fetch timing and mid-fetch replay; all 87 broad DMG cases pass
- [x] Step/run/input/state/snapshot APIs and persistent JSON-RPC server
- [x] Cycle/frame RPC modes and deterministic frame replay
- [x] Typed execution/load/state/input/snapshot/inspection/translation/event boundary; generic wide mock and Game Boy implementation

## M3 — complete

- [x] Actual CPU opcode/operand/data bus tracing, effective ROM banks, controller writes, interrupt/successor events
- [x] Bounded event buffer, filters, coverage/successor aggregates and agent trace queries
- [x] OAM DMA byte-transfer and real PPU frame events with trace equivalence tests
- [x] CGB general/HBlank DMA byte events and snapshot replay; HBlank waits for active LCD
- [x] Effective physical RAM/VRAM/OAM/HRAM/I/O and RTC trace identities, echo aliases, raw physical inspection
- [x] Fixed ROM-window and CGB RAM/VRAM bank-selection events; blocked access evidence
- [x] Optional bounded JSONL file stream, per-record cap, filters, status/errors and execution-equivalence tests
- [x] Fetched-byte edge kinds, pre-operation conditional flags, HALT/STOP traversal barriers, CB and HALT-bug trace tests
- [x] Bounded platform-neutral recursive discovery, reference records, code candidates and ambiguity diagnostics
- [x] Observed successor queries and recursive traversal with reference provenance/hit counts
- [x] Bounded little-endian pointer-table hypotheses, explicit bank context and ambiguity/conflict reports
- [x] Explicit bank8/16-first/last little-endian pointer layouts, ninth-bit bank targets, invalid-bank diagnostics and width-aware RPC conflicts
- [x] Explicit CPU-window context propagates through recursive static paths, with physical barriers and deduplicated map candidates
- [x] Observed CPU-window roots and edges retain bank state before/after operations and merge safely into contextual discovery
- [x] Agent session/input/snapshot/manual-map round-trip integration test
- [x] Traced entrypoint discovery-to-map clean stock-tool round-trip integration test

## M4 — complete

- [x] DMG MBC5, 9-bit ROM selection, switchable bank zero, RAM/rumble-bit separation and snapshots
- [x] DMG MBC3 ROM/RAM, deterministic tick-driven RTC, rollover/latch/halt/resume and snapshot replay
- [x] CGB model selection, WRAM/VRAM banking, basic double-speed/GDMA/HBlank tests and boot-state probes
- [x] CGB 2050-M-cycle speed-switch CPU pause, DIV freeze, both directions and mid-pause snapshot replay
- [x] CGB speed-switch frozen VRAM/OAM access and rendering in all four starting PPU modes
- [x] Source-verified CGB hardware matrix: all 55 cases pass; CGB HALT wake timing fixed
- [x] Expanded CGB peripheral tests and two nontrivial homebrew exit workflows; native-color game breadth remains a documented limit
- [x] Pinned Galactic Armada and Unbricked source builds: actual gameplay input, snapshot replay, observed maps and clean stock recipient byte rebuilds
- [x] Shared external-tool deadlines, process-group cleanup, bounded captured output and atomic generation
- [x] Protocol call table covers all advertised methods; stale pending-tool notes removed
- [x] Performance/budget controls, API/default/error review, clean setup and final requirement audit

The original project.md scope is complete. See docs/completion-audit.md for requirement-by-requirement evidence and documented limits. The chronological notes below record earlier states and do not override the current checklist.

## Latest checks

CI investigation (2026-10-12): GitHub Actions run 38317348213 at e369fd3 failed in `Install pinned tools` for all three jobs (test, hardware, homebrew); subsequent checks were skipped. The supplied log identifies `error: rustup is not installed at '/root/.cargo'`: CI wrapped the entire setup in sudo while keeping the runner's Rust executable on PATH. CI now invokes setup as the runner user. Both setup scripts elevate only apt commands, plus RGBDS system installation in setup.sh; Rust, checkouts and local builds retain the invoking user's identity. setup.sh adds the user's Cargo bin directory before detecting rustup. README documents normal-user setup with sudo access and direct-root support.

After the fix, shell syntax, root setup, locked build, fmt, locked workspace clippy/tests, portability, hardware setup and both hardware gates (89 default, 55 CGB) pass. A local temporary command-stub harness ran both setup scripts under actual UID 0 and UID 65534: the unprivileged branch invokes sudo for exactly the five system installation commands, while rustup/cargo and local tool builds retain user identity and HOME; the root branch requires no sudo. Before the fix, the full local acceptance sweep also passed standalone mizu-core tests, release build, boundaries, fidelity, session, disassembly-workspace, API and both homebrew regressions. These are local checks; the corrected workflow has not yet run on GitHub. Generated files remain local.

Control-flow updates passed the full Rust unit suite, clippy, agent session, portable stock rebuild, crate boundary check, and both pinned homebrew regressions. HALT-bug relative targets use the actual fetch PC. Verification rejects output paths that alias the input ROM, including symlinks and hard links; unit tests and clippy pass. Hardware timing gaps and other unchecked requirements above remain open.

Generation now validates direct CLI maps against ROM content. It shares one deadline across stock emission and a pinned assembly/link/byte comparison, and publishes only byte-identical output. The synthetic fidelity regression covers all text bytes, both image bit depths and reversed palettes, invalid opcodes, malformed image/padding content, missing charmaps, lossy palettes, and a corrupt linker result. Unit tests, clippy, session, portability, fidelity and crate-boundary checks pass. CI runs the new fidelity script. External charmap resources remain unsupported and cause a clear generation failure.

Banked pointer layouts passed five pointer unit tests and the full Rust suite. The RPC regression proves pagination, full-width conflict checks, invalid layouts, bank-boundary rejection and unchanged map output. Clippy, session, stock portability, fidelity and crate-boundary checks pass. Static discovery still needs full CPU-window context; this change does not resolve that separate requirement.

Static discovery now supports conventional MBC3/MBC5 CPU windows. Unit tests verify fixed-bank targets, same switch-bank targets, unresolved upper-window targets from fixed code, and controller-write barriers. Full Rust tests, clippy, persistent session and clean stock portability pass. Full CPU-window context remains pending, including fixed/switch aliases and context propagation across paths.

Explicit single-instruction CPU-window decoding now supports MBC1 fixed-window aliases and MBC5 bank-zero upper-window aliases. The analysis.structure API checks source offset/window identity and retains asserted banks in references. Alias unit tests, clippy, session and portability checks pass. Recursive context propagation remains open; the full CPU-window checklist item stays unchecked.

Context-aware recursion passed alias/cross-window/physical-barrier tests and the full Rust suite. Session, stock portability, crate-boundary and both homebrew regressions pass. The shared canonical-address hook keeps traversal state separate from physical claims; a wide neutral mock tests distinct contexts, shared coverage, barriers and budgets. Observed-edge context merging remains open.

Observed context merging passed the full Rust suite, clippy, persistent RPC, stock portability, fidelity, dependency-boundary and both homebrew checks. The bank-write regression uses zero event-detail capacity and proves that a reached but unfetched target stays Static. Neutral wide-address tests verify filter-independent context aggregates, nontraversable-edge exclusion and clear behavior. Summary fields report bounded context root/edge counts. Hardware timing, performance controls, protocol audit and final requirement audit remain open.

The speed-switch CPU delay follows Pan Docs KEY1 timing. Synthetic tests verify 4100/8200 PPU ticks in the two directions, unchanged PC/instruction coverage during the wait, DIV freeze and a saved countdown replay. Full Rust tests, clippy, session and stock portability pass. Fork snapshot version 5 includes the new countdown. Special PPU locks/rendering during this pause remain open.

The rebuilt release binary also passed all 83 hardware-gate cases after the speed-switch change. This gate does not test the remaining special CGB speed-switch PPU behavior.

The live api.describe result lists 43 methods; each appears in the protocol call table. docs/requirements-review.md keeps the full brief in view and names the evidence needed for the final audit. Method-name coverage does not prove full protocol correctness or completion. Hardware, performance, fresh-setup, source-boundary and delivery audits remain open.

Local metadata reads now enforce byte caps before parsing: 1 MiB for .sym files and 64 MiB for internal stores. The reader also caps bytes read after opening, to handle file growth. Sparse-file unit tests prove early rejection. Full Rust tests and clippy pass; session tests prove rejected large imports/loads retain the current map, and stock portability passes. Remaining performance and API audits stay open.

PPU access diagnosis found the normal-line VRAM mode-2-to-3 boundary lock missing from both CPU reads/writes and physical trace translation. These now use one lock predicate. The full Rust suite, clippy, session, portability and all 83 release hardware-gate cases pass. The broad probe still has 81/87 passes and the same six PPU failures. An OAM-lock experiment passed LCD-start checks but regressed interrupt/OAM timing, so it was reverted; docs/hardware-tests.md records the exact failed rounds and the coupled timing issue. No PPU failure is marked complete.

The joint STAT/OAM timing fix passes the full Rust suite, clippy, session, stock portability and both homebrew regressions. The release gate passed all 83 prior cases, and the broad probe improved from 81/87 to 84/87. Three verified cases now join the required gate. The remaining failures are hblank_ly_scx_timing-GS, intr_2_mode0_timing_sprites and lcdon_write_timing-GS. The project remains incomplete.

The LCD-write boundary fix passes the full Rust suite, clippy, session, portability and both homebrew regressions. All 87 expanded hardware-gate cases pass. The broad DMG probe is 85/87, with SCX/HBlank and sprite timing still failing. Special CGB speed-switch PPU behavior and the final audits also remain open.

The one-dot HBlank phase fix passes the focused SCX test, both LCD-start tests and 11/12 PPU probe cases. The remaining focused failure is sprite timing. Rust, clippy, session, stock portability and both homebrew regressions pass. No new saved state is needed: the existing LCD pixel position records the pending boundary. Snapshot format stays at version 5.

The final release passed all 87 prior gate cases and 86/87 broad DMG cases. SCX/HBlank now joins the required gate. Only intr_2_mode0_timing_sprites remains in the known-failure list.

Sprite fetching now charges per-object and per-background-tile timing and saves pending fetch state (snapshot version 6). Its 105-layout source test and the synthetic mid-fetch replay pass. Full Rust tests, clippy, session, portability and both homebrew regressions pass. The release gate passed all 88 previous cases and the broad DMG probe passed all 87. The sprite case joins the required gate; the selected DMG known-failure list is empty. This proves those cases, not full hardware compatibility. CGB speed-switch PPU effects, broader peripheral checks, performance/protocol reviews and the final requirement audit remain open.

CGB speed switching now saves the starting PPU access state through the 2050-M-cycle pause. HBlank/VBlank pauses render black; OAM-scan pauses render background without objects; rendering-mode pauses keep both. Four synthetic pixel/lock tests check these states and resumed output. The standalone core tests now run in CI with their own pinned lockfile; local core build output is ignored. Workspace tests, clippy, core tests, session, portability, all 89 hardware-gate cases and both homebrew regressions pass. Snapshot format is now 7. Broader CGB peripheral/game checks and the full performance/protocol/completion audits remain open.

CPU internal cycles now advance the bus clock directly. They no longer record the synthetic ROM-zero reads caused by the old read-to-clock helper. A HALT regression checks fixed PC, elapsed time, empty data traces and snapshot replay. Full workspace tests, clippy, core tests, session, portability, all 89 hardware-gate cases and both homebrew regressions pass. Runtime trace correctness improved; broader CGB checks and the remaining audits stay open.

CGB background/object palette data now returns ff during rendering and rejects data writes while still advancing an enabled index. Wrap from 63 to 0 works; rejected writes preserve palette bytes. Palette data obeys frozen speed-switch access state. A core test checks both palettes and a CPU program checks stored values, index wrap and blocked/unmapped trace flags. Workspace/core tests, clippy, session, stock portability, all 89 hardware-gate cases and both homebrew regressions pass. Broader CGB games/peripherals and final audits remain open.

Paged API results now convert only selected records for coverage, edges, maps, references and unknown ranges. A 100000-record regression checks two conversions at offset 50000, none for empty/zero pages and none for an invalid limit. CLI tests, clippy, session and stock portability pass. This removes whole-result JSON copies; traversal and range sorting keep their existing bounds. The remaining performance and final audits stay open.

Session emulator.start now checks the loaded emulator image against the ROM opened for analysis. The core exposes its loaded cartridge hash so emulator loading also checks both file reads for a content change. Generation/verification reject a changed source file before invoking external tools. The existing reset guard retains the prior emulator on failure. Session regressions modify a non-header byte and prove rejected start/reset/generate/verify preserve state, map and snapshots and create no output. Workspace tests, clippy, session and stock portability pass. Final audits and broader CGB checks remain open.

The new --all-cgb runner selects all 53 acceptance sources that explicitly verify CGB/CGBABCDE, checks their ROM builds exist, and adds two CGB boot cases. It forces CGB execution and keeps DMG-specific cases out. The first probe found five mode-2 timing failures; removing the core's extra CGB HALT-wake cycle fixes all five. All 55 CGB cases and all 89 default gate cases pass on the release build. CI now runs both selections. Workspace tests, clippy, session, stock portability and both homebrew regressions pass. Native-color game/peripheral checks and final audits still remain.

The shared API now caps encoded JSON response payloads at 1 MiB. A counting writer checks escaped output without creating a whole serialized copy; oversized replies return a small error and leave the stream usable. The error notes that commands may already have completed. File-only map export (include_sym=false with path) returns ok/path/bytes while writing the full stock map. Tests cover escape expansion, command effects, oversized IDs, stream recovery, valid file-only export and rejected options. Full workspace tests, clippy, session and stock portability pass. Final audit work remains open.

Map/state output paths now reject aliases of the input ROM. The shared CLI guard compares resolved paths and Unix file identity. RPC map.save/map.export and CLI map init/store export use it before writing. Session regressions cover the source path, symlink and hard link, check clear errors and verify unchanged ROM bytes. CLI tests, clippy, session and stock portability pass. Remaining setup/API/final audits remain open.

A current acceptance sweep passed fmt, workspace clippy/tests, all nine standalone core tests, debug/release builds, boundary checks, session, stock portability, fidelity faults, 89 default hardware cases, 55 CGB cases and both licensed homebrew workflows. The architecture/limits guide was rewritten from current source and evidence to remove obsolete pending-feature and failure-count claims. Requirement review still keeps the final scope open; setup verification is underway.

The documented setup script also completed successfully on this Ubuntu VM: required packages, pinned Rust 1.90.0, pinned mgbdis/RGBDS and the locked build. This is an idempotent setup check in an existing environment, not proof of a clean-VM install. A clean environment and final API/source/artifact reviews still need evidence.

A clean Ubuntu noble minbase validation is running via scripts/test-fresh-setup.sh. It uses debootstrap/chroot, copies source without tools/build/target caches, proves empty Cargo/tools/target locations, and then runs setup plus the full acceptance command sequence. Base installation passed; the clean environment has installed Rust and is building RGBDS. Live exec session: 28198. Root path is recorded in build/fresh-setup-root.txt; logs are build/fresh-bootstrap.log and build/fresh-setup.log. Do not treat this pending run as a pass.

Added scripts/test-api.py and its CI call. It checks object-parameter enforcement for all 43 methods and 37 invalid type/range/edit/path cases, verifies nonempty errors and unchanged emulator/map/trace state, and checks no rejected stream/generation output exists. It passes on the host and the independently compiled clean Ubuntu debug binary. This is targeted parameter/error evidence, not the full completion audit. The original clean-setup session 28198 remains live at release compilation.

The original clean Ubuntu noble minbase run completed successfully. With no reused Cargo/tools/target cache it installed pinned tools, built source, passed fmt/clippy/workspace and core tests, boundary/session/stock portability/fidelity checks, rebuilt public suites and both games, and passed release gates (89 default, 55 CGB) plus both homebrew input/replay/exact-rebuild workflows. build/fresh-setup-result.json records success. This proves a clean root filesystem on the host kernel, not a separate VM; final source/API/delivery audit remains open.

Delivery/source inspection found no GB implementation imports or bank-width assumptions in shared core/analysis/agent/disasm/store source. Byte-budget constants were distinguished from hardware widths. Source-tree artifacts and notices were inspected; generated Python bytecode was removed/ignored. Current README/homebrew counts replace old failure claims. Fresh-run copying now excludes Python caches, clears stale success markers and includes test-api.py. Boundary/API checks and shell syntax pass. Full requirement/API closure is still pending.

Added positive default/result checks to the API test: read/decode defaults, empty pages/unknown span, capabilities/storage sizes, step/run defaults, cycle progress and exact snapshot replay. They pass along with all 37 rejection cases. Dispatch/adapter parameter checks were compared to the protocol; obsolete MBC5/static and RTC snapshot-version wording was fixed. Requirement-by-requirement conclusion remains open.

Final review: docs/completion-audit.md maps every product behavior, architecture/mapping/API contract, quality/delivery requirement and M1–M4 exit condition to inspected source and acceptance evidence. Latest fmt/clippy/workspace tests, API, boundaries, session, stock portability and fidelity checks pass. Clean setup/core/public hardware/homebrew evidence also passed without reused build/tool caches. Remaining hardware breadth and native-color game paths are documented limits under the original incremental-support scope, not missing required methods or milestone exits.

Added scripts/disassemble.py for separate per-game agent workspaces, including local ROM copies/identity records, start/resume prompts, structural classification instructions, and a workspace runner for inspection, persistent sessions, verified raw-byte baselines and named assembly checkpoints. It supports multiple local ROMs, rejects existing workspaces and repository-local destinations, and does not launch an agent or automatically classify a ROM. Checkpoints retain the exact exported map and record ROM/map hashes only after byte-identical verification. The synthetic scripts/test-disassemble.py regression passed for two game workspaces, paths containing spaces, invalid/duplicate inputs, preserved source bytes, changed-ROM rejection, and a real code annotation exported/generated/rebuilt from a separate working directory. Baseline output is explicitly not classification completion. fmt, workspace clippy, locked Rust tests, stock portability and session tests pass. The new regression is included in CI; no commercial ROMs or extracted assets were added.

Workspace initialization now prints per-game Codex startup commands and explicit first-pass/resume /goal objectives on stderr, while preserving JSON-line results on stdout. --json suppresses the instructions. Local Codex 0.162.1 help and installed goal-command strings establish --enable goals, -C and /goal objective/status/pause/resume syntax; no live authenticated Codex session was launched. Regression assertions cover instructions for both games, shell-quoted workspace paths, first/resume objectives, raw-baseline scope and JSON-only output. The workspace regression, fmt, locked workspace clippy/tests and stock portability pass.

New disassembly workspaces now include analysis/toolkit-issues.md. Generated agent instructions, workflow, start/resume prompts and printed goals require recording confirmed/suspected bugs and substantial evidence-backed missing features as encountered, preserving findings across passes, and summarizing/linking the report at completion or blocked handoff (explicitly reporting no findings when empty). Entries include affected tools, evidence/reproduction, impact, workarounds and suggested improvements; toolkit fixes remain outside the disassembly task. INSTRUCTIONS.md documents the report and new-workspace scope. scripts/test-disassemble.py passes, including synthetic baseline/code checkpoint verification. A separate synthetic workspace smoke check confirms report initialization, prompt/goal references and preservation when initialization rejects an existing workspace. git diff --check passes. No live disassembly-agent behavior was tested.

Revised generated start/resume prompts and printed goals to pursue whole-ROM completion rather than one verified checkpoint. Structural and semantic investigation now proceed together; required annotations record role and evidence. Completion requires every physical byte classified, annotations, valid maps and byte-identical rebuilding. The new persistent analysis/investigation.md ledger requires per-range method applicability, experiments, hypotheses, blockers and reopening on new evidence. Incomplete termination requires exhaustion of concrete investigation paths, not a total budget, stagnant pass count or unsupported unknowability claim; it never authorizes marking the whole-ROM goal complete. INSTRUCTIONS.md and project.md align with the contract. Rust 1.90.0 fmt (the CI cargo fmt --check command), locked workspace clippy/tests/build, synthetic workspace regression, stock portability and session tests pass. An initial cargo fmt --all --check also checked excluded vendor sources and reported existing formatting differences; vendor files were unchanged. Regression checks cover both generated prompts, both printed goals, ledger initialization and completion/exhaustion requirements. These are prompt-contract checks, not evidence that a live agent will fully classify a commercial ROM or always terminate. Existing workspaces are not automatically updated; no commercial ROMs or extracted assets were added.

Added scripts/commentary.py as phase 2 for existing game workspaces. Preparation
validates identity, internal state and current stock map, generates/verifies a
fresh local baseline, and creates a separate editable source tree, frozen reading
copy, after copy, prompt, printed goal and full-ROM review scaffold. Phase-1 files
are preserved. Static semantic interpretation, conventional proportionate
comments, descriptive symbols and thoroughly explained data are the default;
investigated ambiguity can satisfy editorial review without declaring phase-1
classification complete. verify/read/audit commands check source fidelity,
refresh the reading copy and validate inventory/status/editorial notes. The audit
cannot assess semantic correctness or comment quality; those remain agent review
obligations. Stock .sym alone does not preserve the final commentary.

The synthetic scripts/test-commentary.py regression passes preparation from an
existing workspace with spaces, unchanged phase-1 files, duplicate-preparation
rejection, reviewed unknown gaps, pending/tampered inventory rejection, faithful
comment edits, byte-changing edit rejection, changed-ROM rejection and cyclic/
escaping assembly include rejection. It is included in CI. Rust 1.90.0 fmt,
locked workspace clippy/tests, existing workspace regression and stock portability
pass. Session Rust/API code and dependencies were not changed. Local Alleyway
preparation also passed fresh byte-identical rebuilding and created 161 scaffold
entries covering all 32768 bytes, including its preserved 31-byte unknown gap;
before/after reading copies initially match. No commentary agent was launched,
and no commercial ROMs, assembly or extracted assets were added to the repository.

Added scripts/archive.py and documented archive/restore commands in INSTRUCTIONS.md,
README.md and docs/archives.md. Reviewed results default to
atlas/<platform>/<game>/<full-ROM-SHA256>/; the archive contains final stock maps,
classification/review metadata, audit records and final agent documentation.
Versioned editorial.json stores comments, symbols, constants/formatting and
references into a regenerated baseline. It preserves exact commentary separately
from .sym, without storing assembly or extracted assets. Scratch evidence and
analysis/toolkit-issues.md are excluded; historical documents remain verbatim with
an explicit omitted-evidence note. Extra final Markdown can be selected explicitly;
unknown documents are reported. Archive replacements are staged and validated,
with the previous result retained on failure. Restore publishes only after ROM,
archive, baseline and final-file hashes and a byte-identical rebuild pass.

The synthetic scripts/test-archive.py regression passes exact source/reading-copy
recovery, comments/constants/includes, reviewed unknowns, evidence/toolkit exclusions,
extra/scratch-document selection, unfinished-review rejection, stale-file removal,
binary-asset rejection, failed-replacement preservation, wrong-ROM and tampered
archive rejection and existing-output protection. It is included in CI.
Rust 1.90.0 cargo fmt --check, locked workspace clippy/tests/build, stock portability,
session, workspace and commentary regressions pass. Rust/session code and
dependencies were unchanged.

Archived the completed local Alleyway result under atlas/gb/alleyway/
ed8070e011713527bdc03e2b9cec9f9c4a7e3aaa00e57e8786a186b265da1bb2.
A separate restore command recovered every authoritative final source/asset and
the consolidated reading copy exactly and rebuilt all 32768 original bytes.
Editorial review records 161 regions and 1077 units; phase-one classification
remains incomplete with the 31-byte unknown interval at $0426–$0444. No commercial
ROM, generated assembly, extracted game asset or scratch trace was added to the
repository. Archive metadata does not establish semantic correctness, hardware
compatibility or completion of the remaining phase-one classification.

Added the confirmed native-port workflow in scripts/port.py and its generated
templates. Preparation accepts a reviewed Game Boy archive plus its matching local
ROM, restores and byte-verifies exact commented reference source, and publishes a
new workspace outside the repository. It carries incomplete classification into
pending gameplay-behavior accounting, prints whole-game start/resume goals and a
separate reviewer prompt, and does not launch agents or implement game behavior.
The workspace preserves an initial toolkit file/hash/HEAD/status snapshot so
port-driven library edits can be reviewed separately from pre-existing work.
Agents may implement needed shared changes directly here, uncommitted, with a
requirements/rationale/validation log. Accounting audits reject missing original
units, altered source identities, pending dispositions and missing equivalents;
they do not assess semantic correctness or test quality.

The new romatlas-port library supplies identity-checked bounded ROM access,
named asset collections with replacements/additional slots and explicit ordering,
game-owned decoded-ROM recipe hooks, rational console timing, sample budgets,
Game Boy tile/palette and CPU tile-map/sprite composition, logical input edges and
focus rearming, and persisted player settings/menu logic. The optional SDL2 shell
adds window/fullscreen/aspect/integer scaling, keyboard/controller binding,
controller/dead-zone selection, volume and focus-loss pause. Identical controller
models receive separate GUID/ordinal choices; ordinals follow enumeration, not
serial identity. Desktop initialization is separate from scripted headless
execution. SDL2 development libraries are installed by setup; compatible Rust
dependencies are locked. Music drivers, complete PPU/APU behavior, unusual game
tricks, other console modules, immutable release pinning and packaging are not
claimed by this starting library. No original game assets are embedded.

Rust 1.90.0 setup/build, fmt, locked workspace clippy/tests, desktop-feature
clippy and all 14 display/device-independent port-library tests pass. Tests cover
fifteen ROM-backed levels plus replacement/new sixteenth slot, modern-schema
decoded defaults, traversal/symlink/duplicate/order rejection, sparse oversized
ROM rejection, exact timing/sample budgets, tile/palette composition, input
rearming, persisted settings/rebinding/reset, default SDL names and duplicate
controller selectors. scripts/test-port.py passes synthetic archive preparation
with spaces, reference fidelity, unknown regions, prompts/review snapshots,
offline lockfile generation, actual headless execution and desktop compilation,
asset overrides, accounting acceptance/rejection, frozen-reference/ROM guards,
wrong/tampered inputs, JSON-only mode and no-display desktop rejection. Existing
session, disassembly, commentary, archive, stock portability and dependency
boundary regressions pass. CI and clean-setup command lists include the new tests.
The revised setup passed here; the clean-root setup was not rerun for this change.

Native Alleyway port work adds `romatlas-port::gb::sound::Sound`, a device-free
register-driven synthesis component: pulse duty, wave RAM/sample buffer, noise
LFSR, DAC/trigger state, length/envelope clocks, master volume, routing and
deterministic sample budgets. The20Hz high-pass filter is approximate; pulse1
hardware sweep and register-write edge quirks remain pending. This is selected
native-port audio, not full APU emulation. Native/desktop-feature port-library
tests, desktop-feature clippy, boundaries and stock portability pass. Two new
sound tests cover routing/DAC/length and sample partition invariance. Synthetic
port-workspace regression passes; no device/desktop process was launched.

Follow-up sound work adds pulse1 hardware sweep shadow/timer, initial and second
overflow checks, negate-clear disabling and length-enable/trigger extra-clock
behavior. Two new regressions pass alongside existing desktop-feature sound and
runtime tests (18 total); desktop-feature clippy and boundaries pass. Portability
and synthetic port-workspace regressions pass again. Other model-specific
register quirks and analogue filter accuracy are not claimed.

Native-port graphics now retain indexed backgrounds separately from RGBA and
provide background_tile/background_pixel plus priority-aware sprite_tile.
Priority compares nonzero source index even when palette colors coincide.
Frame::clear resets that mask; generic tile drawing is unchanged. New regression
and existing desktop-feature library tests pass (19 total), as do desktop clippy,
dependency boundaries, portability and game checks. Synthetic port-workspace
regression passes. This is BG priority, not a claim of full PPU/OAM behavior.

Native ports now have gb::objects::draw for40-record DMG8x8 OAM composition.
Per-line selection counts the first10 Y-matching records even with hidden X;
opaque drawing prefers lower X then earlier OAM, and BG priority is checked only
after selecting the winning object pixel. Supports flip/transparency/palettes.
Two new tests cover overlap/masking and offscreen selection. Desktop-feature
library tests21, package fmt/clippy, boundaries, stock portability and synthetic
port-workspace regression pass, as do Alleyway game checks/scripted headless
execution. No PPU timing/device claim. Unrelated vendor formatting was restored;
only port-library source and this progress document remain modified.

README.md, INSTRUCTIONS.md, AGENTS.md, project.md and docs/porting.md describe the
new phase and keep native-port accounting distinct from whole-ROM structural
classification. No window, audio device, physical controller or live port/review
agent was launched. These checks prove the initial workflow/library and generated
build contract, not a completed real-game port, desktop runtime behavior or other
OS compatibility. No commercial ROM, regenerated assembly or extracted game asset
was added to the repository; all synthetic generated material stayed local.

Future native port preparation now renders BUILDING.md from a shared template,
substituting the game name/executable and toolkit path, and links it from the
generated README. Baseline recipes cover private macOS/Linux/Windows builds,
SDL linking (including Homebrew LIBRARY_PATH), source transfer without target/
products, path-dependency relocation, device-free builds and later packaging.
Start/resume goals and workspace workflow require the port agent to adapt it
for final dependencies/flags and record actual validation without inheriting
another game's platform success. Existing workspaces are not overwritten.

Validation: scripts/test-port.py passes, including generated-document/name/path
checks, README link and start/resume/workflow requirements in its synthetic
archive-to-workspace regression. Stock portability and dependency boundaries
pass; romatlas-port package formatting, desktop-feature clippy and all 21 unit/
integration tests pass under Rust 1.90.0 with locked dependencies. No dependency
or runtime changes, GUI launch, commercial-asset distribution, check-in or
publication. Platform recipes remain documentation rather than new Mac/Windows
execution evidence.


2026-10-16: Added explicit debugger state mutation and addressed workspace reports
T001–T007. emulator.registers.set validates complete CPU register patches before
applying; emulator.memory.write performs bounded unticked mapped bus writes,
including I/O/controller side effects. Neither emits execution evidence. ROM
backing storage remains immutable. T001 exposes the core RGB8 framebuffer with
ROM/frame/cycle/operation identity. T002 independently pages contextual and ordinary
discovery references/diagnostics. T003 preserves preflight deadline errors without
claiming missing installation. T004 constructs imports privately and validates once;
ordered overlap enumeration preserves every conflicting pair, including nested
ranges and separate address contexts. An 8192-directive synthetic import measured
486.8 ms in this debug build; this is not a commercial-ROM performance claim.

T005 tracks literal BC/DE/HL values along bounded straight-line contextual paths;
known non-controller destinations preserve fallthrough, while unsupported effects
forget facts and uncertain/controller writes retain conservative stops. Analysis
facts stay separate from physical regions and recorded-edge identity. This is
literal tracking, not general interval or interprocedural analysis. T006 adds
explicit call profiles qualified by ROM identity, exact source/inline bytes, CPU
and effective banks, and caller evidence covering all arrivals. Profiles suppress
presumed returns or resume after a proved inline extent; they do not infer selector
domains, automatically classify tables, or imply execution. T007 adds local version/
ROM/model-qualified snapshot export/import, preserving core machine state and
adapter counters, with rejected incompatible/corrupt imports retaining state.
Named snapshots and trace history retain their existing lifecycle semantics.

Validation: Rust 1.90.0 cargo fmt --check, locked workspace all-target Clippy with
warnings denied, and locked workspace tests pass. The synthetic session regression
covers edits, rejected edits, execution of edited RAM, unchanged time/trace during
edits, framebuffer dimensions, independent reference/diagnostic pagination, profile
application/rejection, persistent snapshot recovery after reset, restored RAM, and
rejected ROM/model/version/corrupt snapshot imports. Store tests compare ordered
conflicts to exhaustive pairs; discovery tests cover SRAM/controller destinations,
inline return suppression and explicit continuation. The expired-preflight test
checks the deadline diagnostic. Stock-tool portability (deterministic source and
exact four-bank rebuild) and dependency-boundary regressions pass. No desktop
window, commercial ROM, extracted asset, or game workspace modification was used.


2026-10-16: Restored only release-build defaults from the reset workflow changes,
and aligned the remaining toolkit entry points. setup.sh, README/AGENTS/starting/
porting instructions and CI build with cargo build --locked --release. Disassembly
creation, archive restoration, native-port preparation and CLI integration scripts
select target/release/romatlas by default; explicit --binary overrides remain.
Generated disassembly instructions identify the release build and the need to
restart servers after replacement. Existing workspace metadata/runtime processes
are not migrated or restarted. No cache/queue/upgrade or completion-contract
changes were restored as part of this update.

Validation: locked release build, setup shell syntax, cargo fmt --check and git
diff --check pass. Release-backed disassembly, commentary, archive, port, session,
API, stock portability and fidelity regressions pass. The workspace setup test
also asserts that the default recorded binary is the release executable. Port
regressions compile/test locked headless and desktop paths without a window.
No commercial ROM, extracted asset or generated game file was added.

2026-10-16: Replaced per-script printed goal commands with written coordinator
prompts. New disassembly workspaces contain goal.md plus exactly three workflow
prompts under prompts/: disassembly.txt, commentary.txt and redirect.txt.
Disassembly now stops incomplete immediately when obvious substantial startup,
basic-input or supplied-lead gains end; difficult remaining investigations no
longer require exhaustive method/blocker reviews. Whole-ROM classification and
byte-fidelity requirements for a complete result remain unchanged. Commentary
accepts incomplete source, uses (?), (??), (???) uncertainty markers and hands
unresolved investigations to redirect. Redirect specifies one static-first lead
search with optional bounded debugger state construction, persistent lead IDs,
evidence/recipes/outcomes and no repeated leads without additional information.
The coordinator preserves prior commentary across changed-map rounds, resumes
recorded phases and stops incomplete when no new actionable leads remain.
Native-port preparation writes goal.md with the existing whole-game objective
and resume contract. Existing workspaces are not automatically migrated.

Validation: locked release build, cargo fmt --check, locked workspace all-target
Clippy with warnings denied, locked workspace tests, desktop-feature port Clippy
and all 21 port tests, synthetic disassembly/commentary/native-port regressions and stock-tool
portability pass. Prompt-contract regressions check sequencing, lead history,
debugger APIs, incomplete stopping and uncertainty markers. No live analysis agent,
commercial ROM, generated game asset or desktop device was used. These checks do
not prove an agent's semantic quality or easy-gain stopping judgment.

2026-10-16: Reviewed workflow leftovers after the three-pass change. Removed
new-port generation of the duplicate prompts/resume.md; goal.md now holds the
resume instructions once instead of repeating the entire objective. Removed
commentary's redundant generated instructions.txt, inlined the disassembly
prompt read, renamed the Codex-specific output helper and corrected stale active
docs that still described printed goals. Commentary's prompt-location helper
now chooses the existing fallback for older workspaces. Existing game workspaces,
archive evidence and historical progress entries remain untouched. No whole
tracked source file was obsolete solely because of this workflow change.

Validation: release build, fmt, locked workspace all-target Clippy with warnings
denied, locked workspace tests, synthetic disassembly/commentary/native-port
regressions (including generated headless/desktop builds), Python compilation and stock portability pass. Regressions assert that redundant
resume/instruction files are no longer generated and resume preservation remains
in goal.md. No runtime/session implementation or dependencies changed.

2026-10-16: The disassembly prompt now explicitly recommends emulator.memory.write
and emulator.registers.set for concrete lead verification when direct state changes
can immediately expose relevant code, pointers or tables. It requires snapshots,
exact patch/bank/budget records, separates artificial observations from natural
reachability and applies the existing easy-gain stop rule to speculative edits.
Validation: scripts/test-disassemble.py passes, including generated-prompt checks
and synthetic baseline/code checkpoint rebuilding. git diff --check passes. This
is a prompt-only change; no runtime behavior or existing workspace was changed.

2026-10-16: Coordinator and disassembly prompts now require an explicit user-facing
ROM classification percentage after every disassembly pass, before commentary,
and again in the final handoff. The report includes classified/total/unknown byte
counts and is also saved in progress.md. Its denominator is the full physical ROM;
unknown regions/gaps and raw fallback are excluded from classified bytes. Execution
and editorial coverage stay separate, and rounding cannot establish completion.
Validation: scripts/test-disassemble.py passes generated-prompt assertions and
synthetic baseline/code checkpoint verification. git diff --check passes. Existing
workspaces are not automatically updated; no runtime implementation changed.

2026-10-17: Broadened disassembly from basic startup inputs to debugger-driven
normal gameplay scenarios. Prompts now require progressing through ordinary game
states while they yield substantial evidence or usable understanding, without a
preexisting lead. Bounded input sequences and reproducible routes/gains remain
required; obvious productive alternatives are considered before stopping, while
repetitive play and difficult progression without substantial prospects are
abandoned. Coordinator, generated workflow and active user docs agree.
Validation: scripts/test-disassemble.py passes generated-prompt assertions and
synthetic baseline/code checkpoint verification; git diff --check passes. No live
gameplay agent or commercial ROM was used. Existing workspaces are not migrated.

2026-10-17: Commentary prompts now explicitly require revisiting earlier (?),
(??), (???) sections and uncertainty entries affected by new disassembly/evidence.
The review includes callers, callees, tables and shared variables outside new
classification ranges; previous reviewed status grants no exemption. Explanations,
names and markers change only with supporting evidence, unresolved uncertainty
remains explicit, and progress/inventory/uncertainty records must stay consistent.
Validation: synthetic disassembly and commentary regressions pass, including both
generated prompt contracts, source fidelity and review audits. git diff --check
passes. No live agent behavior or existing workspace migration was tested.

2026-10-17: Commentary now explicitly inspects uncommented sections and generic
labels every pass, including previously reviewed ranges without uncertainty markers.
Unclear sections need useful nearby notes on observable behavior, remaining unknowns
and references/questions for redirect, plus range/source-qualified review records;
a bare marker or silence cannot stand for investigation. Genuinely unreviewed work
stays pending and must be listed at handoff rather than marked reviewed to satisfy
the audit. Redirect also inspects uncommented/generic/unreviewed gaps directly.
Validation: synthetic disassembly and commentary regressions pass prompt contracts,
baseline/code checkpoint rebuilds, source fidelity and inventory audits. git diff
--check passes. No semantic-comment quality enforcement is claimed: those judgments
remain agent obligations. Existing workspace prompts are not automatically updated.

2026-10-17: Added Android build/integration guidance to the generated native-port
BUILDING.md and linked it from generated README and active porting documentation.
It covers SDK/NDK setup, Rust ARM64 cross-target checking, SDL2 Activity/native
library packaging, ROM selection through app-private storage, lifecycle/input
adaptation and debug APK build/install commands after integration. The current
scaffold still supplies no Android wrapper; the guide explicitly separates required
implementation from available commands and records Android as unvalidated.
Official SDL2 and Android documentation support the platform guidance. Existing
workspaces are not rewritten.

Validation: locked release build, cargo fmt --check, desktop-feature port Clippy
with warnings denied and all 21 port tests pass under Rust 1.90.0. Synthetic
scripts/test-port.py, stock portability, dependency boundaries and git diff --check
pass. No Android SDK/toolchain was installed, APK built, device/GUI launched or
commercial ROM/assets used. Changes are documentation and generated README text;
runtime code and dependencies are unchanged.

2026-10-17: Intermediate commentary now has incremental research scope: new/changed
material and affected dependencies, preserved unaffected review credit, coherent
provisional names and ordinary tentative prose. It can hand useful findings to
redirect with pending review, stopping at addressed scope or diminishing returns.
Prompts require clear, light functional prose, replacement of redundant comments and compact
routine/structure records rather than exhaustive branch inventories. Changed-map
carry-forward preserves historical source and invalidates affected records only;
this remains an agent-directed operation, not automatic semantic migration.
Verification is requested for coherent batches/handoffs and the reading copy at
handoff; the unchanged strict audit remains for final editorial cleanup/archive.
Whole-ROM evidence-backed classification and archive acceptance are not weakened.

Coordinator prompts now require sequential separate phase contexts, phase-specific
instructions and compact durable handoffs, with an explicit separate-session fallback
when worker launching is unavailable. Preparation still writes instructions rather
than launching agents. Active docs and generated workflow agree. Existing game workspaces, including Kirby,
were not modified or migrated; no ROM or extracted asset was added.

Validation on Rust 1.90.0: locked release build, cargo fmt --check, locked workspace
all-target Clippy with warnings denied, locked workspace tests, synthetic disassembly
and commentary regressions, archive regression, stock-tool portability, Python
compilation and git diff --check pass. The commentary regression demonstrates a
verified/readable partial handoff while final audit still rejects pending review;
full synthetic editorial acceptance and altered-scaffold/fidelity rejection remain
covered. No session/runtime/dependency code changed. Checks validate instructions
and acceptance behavior, not live agent semantic quality, context isolation or speed.

2026-10-17: Refined commentary's editorial standard to clear, light prose: short
functional comments with useful detail, avoiding compressed address-heavy shorthand
and mechanical opcode narration. Final acceptance addresses pending review and known
editorial issues using existing valid review credit; it no longer prescribes a
whole-source reread or cosmetic rewrite. Reviewed material is revisited for changed
evidence/dependencies or concrete inconsistencies. Strict inventory, editorial
notes and byte-identical archive acceptance remain unchanged. Removed the reasoning
setting comparison guidance; that evaluation is outside this workflow.
Validation: synthetic commentary and disassembly regressions and git diff --check
pass. Changes are prompts/docs and matching regression assertions; no runtime,
existing game workspace or asset changes, and no live-agent quality claim.

2026-10-17: Exhausted easy-gain cycles now automatically schedule one eligible
bounded investigation pass before final termination, including resumes of older
stops without a recorded investigation. Coordinator, workflow, disassembly and
redirect instructions allow groundwork and small supported findings without a
premeasured substantial byte gain. Each pass selects at most three concrete
hypotheses, one focused static investigation and at most three runtime tests per
hypothesis (one million operations/30 seconds each). Durable scope/budgets/outcomes,
interruption resumption and materially changed retry requirements prevent cycling.
Artificial-state evidence remains distinct from natural reachability; classification,
validation, fidelity and final editorial requirements remain unchanged.

Updated the active Castlevania workspace goal/workflow/disassembly/redirect prompts
and appended a migration note, preserving historical handoffs and all ROM/map/state/
source artifacts. Its next resume targets one further level/substage descriptor,
with a known audio command/stream as independent fallback. No game investigation,
new classification, execution or verification was performed by this migration.

Validation: synthetic scripts/test-disassemble.py passes generation, new prompt
contracts, baseline and code checkpoint verification. scripts/test-portability.py
passes deterministic stock generation and exact four-bank rebuilding. Python
compilation and git diff --check pass. Prompt/docs/test-only change; no Rust runtime
or dependency changes, commercial ROM/assets added, or live analysis agent launched.

2026-10-17: Superseded the one-off bounded investigation policy above. Every
redirect now investigates promising unresolved questions thoroughly but reasonably,
including groundwork, consumer/writer tracing and debugger experiments. No fixed
hypothesis count, experiment quota or separate disassembly investigation mode
remains. Evidence, useful learning and credible next steps govern effort; repetitive
results, accumulating unsupported assumptions and slim prospects end an avenue.
Per-call emulator execution limits still apply. Further rounds accept concrete
useful ready leads without requiring immediate substantial byte gains. Retry history,
interruption records, structural evidence, map fidelity and final acceptance remain
required. Older easy-gain stops resume redirect under this policy.

Updated active Castlevania goal/workflow/disassembly/redirect instructions and added
a superseding migration note. Historical evidence, map, state, source and verification
artifacts remain unchanged. No live research or new classification was performed.
Validation: synthetic disassembly regression passes prompt contracts, workspace guards,
baseline and code checkpoint verification; stock portability passes deterministic
source and exact four-bank rebuilding. Python compilation and git diff --check pass.
These checks validate generated instructions, not live-agent research judgment.

2026-10-17: Fixed Castlevania toolkit-issues.md T001 (reported in the request as
T0001): held joypad buttons were explicitly skipped by the core snapshot serializer.
The bitmask now saves/restores alongside row selection and interrupt-edge history,
so named and exported snapshots replace live input consistently. Core snapshot
format advances from 7 to 8; old files are rejected without changing live state
because their missing input cannot be recovered. Recreate snapshots after restarting
the server with the rebuilt release binary. The outer JSON envelope is unchanged.

The synthetic session regression reproduced the original Right-held named-load
failure before the fix. It now checks all eight buttons, both rows, named/file
restoration over different live buttons and row selection, held-input operation
replay, released-button restoration, and state/input preservation after old-version
and truncated-core imports fail. Rust 1.90.0 locked release build, workspace fmt,
all-target workspace Clippy with warnings denied, workspace tests, nine standalone
core tests, session and deterministic byte-identical stock portability pass.
Joypad source formatting and git diff --check pass. A separate vendor lib.rs format
check still reports existing unrelated formatting; the only change there is the
version constant. Standalone core tests retain an existing raw_screen_buffer dead
code warning. No commercial ROM/assets or game-workspace artifacts were changed.

2026-10-17: Restored readable assembly as an explicit agent-workflow deliverable
for downstream porting, separate from evidence-backed structural classification.
Commentary requires explanations of meaningful routines/control flow/data structures,
consistent routine/branch/RAM/field/constant symbols and useful mechanical descriptions
where game meaning remains unresolved. Intermediate passes also advance an older
readability backlog; final acceptance addresses concrete defects in prior reviewed
material and records an explicit source-quality assessment with examples. Inventory
audit and byte fidelity alone do not establish readability. Updated project scope,
generated coordinator/disassembly/commentary/redirect and port prompts, plus the
paused Castlevania workspace instructions and migration record. No game source,
ROM, map, review status or classification changed and no paused run was resumed.

Validation: synthetic commentary and disassembly regressions and stock portability
pass, including deterministic source and exact four-bank rebuild; git diff --check
passes. Prompt assertions cover readable listing, backlog and separate reporting.
No Rust/runtime changes; these checks validate instructions and fidelity, not live
agent prose quality or semantic completeness.

2026-10-17: Superseded commentary's small-backlog-slice stopping policy. Every
invocation must now complete readable explanations and naming across all currently
classified source, including older backlog and concrete defects in prior reviews.
Sound prior work is reusable; unresolved meanings receive mechanical descriptions
and explicit uncertainty. Useful findings, small repairs and research diminishing
returns do not end the source-editing task. Context/time interruption records exact
remaining ranges and resumes commentary. Coordinator handoff explicitly rejects
remaining classified-source readability gaps before redirect. Updated generated
phase instructions/workflow, coordinator and project contract and active Castlevania
instruction files only; no game source, map, classification or review-status edits,
and no running worker was launched or interrupted. Already-running contexts must
reread instructions to acquire the changed contract.

Validation: synthetic commentary/disassembly regressions pass, including positive
completion/resumption wording and rejection of the former early-stop instruction;
stock portability passes deterministic generation and exact four-bank rebuilding.
git diff --check passes. No Rust/runtime changes. Prompt tests cannot guarantee
live-agent judgment or establish semantic quality of a game's assembly.

2026-10-17: Redirect now investigates connected dispatch/handler/state/table/asset
behavior as a coherent batch while concrete evidence and useful prospects continue.
An initial range boundary or first ready lead does not justify handoff by itself.
Ordinary bank-aware static reasoning can support code proposals without executing
every branch; reached targets/pointers remain insufficient evidence by themselves.
Handoffs explain deferred obvious continuations; interrupted productive redirect
work resumes in that phase. Disassembly validates supported connected batches,
reuses valid experiments and records missing prerequisites rather than treating
lead ranges as byte ceilings. No byte quota, exhaustive recursive closure or
weakened classification/fidelity gate. Coordinator and active Castlevania instruction
files align; redirect also respects the current-source commentary handoff gate.
No game source, maps, reviews, classification or runtime evidence was changed.

Validation: synthetic disassembly and commentary regressions pass, including new
connected-batch/static-evidence/deferral/resumption prompt assertions. Stock
portability passes deterministic source and exact four-bank rebuilding. Python
compilation and git diff --check pass. Prompt-only changes cannot establish live
agent investigation quality or speed; no paused run was resumed.

2026-10-18: Reviewed local Castlevania round019 redirect's 495-byte handoff. It
closed the selected type6/8 graphs but deferred other accepted actor machines on
the basis that they were independent investigations. Redirect now treats connected
graph closure as a checkpoint and continues promising actionable independent
avenues in the same phase, accumulating proposals before a full workflow cycle.
Independence/selected-scope boundaries alone no longer justify deferral; require a
missing prerequisite, negative evidence or diminished prospect. Context rollover
resumes redirect rather than forcing classification/commentary. No byte quota or
exhaustive actor/unknown sweep; existing evidence and fidelity standards remain.
Coordinator/generated workflow and active Castlevania instructions updated. No
source/map/review/runtime artifacts changed, no live worker launched or interrupted.

Validation: synthetic disassembly/commentary regressions and stock portability
pass, including deterministic exact four-bank rebuilding. Added prompt assertions
for independent-avenue continuation and coordinator completion. Python compilation
and git diff --check pass. Instruction checks do not prove live-agent performance.

2026-10-18: Added a final-investigation phase for stalled near-complete ROMs.
Generated coordinators schedule it after exhausted redirect when the exact unknown
complement is positive, at most 5% of physical ROM and at most 16 KiB; users can
invoke its prompt manually at other sizes. It audits every remaining range's
disposition, structural evidence independent of live callers, selector producers
and targeted artificial-state experiments. It preserves evidence standards, routes
proposals through disassembly/commentary/redirect, resumes interruptions and prevents
automatic repetition without new evidence or materially improved methods. Preparation
writes prompts only; it launches no background agent.

Migrated the existing Castlevania coordinator/workflow and added the prompt, with
a progress migration note identifying round022 as eligible. No game investigation,
ROM/map/state/source/review/verification change was performed. Historical handoffs
remain intact. Synthetic disassembly and commentary regressions, stock portability,
Python compilation and git diff --check pass. Tests cover prompt generation,
threshold/resume instructions and existing byte-identical checkpoint behavior;
they do not establish live-agent research quality or promise full classification.
Additional validation on Rust 1.90.0: locked release build, cargo fmt --check,
locked all-target workspace Clippy with warnings denied and locked workspace tests
pass. No runtime/session implementation or dependency changes.

2026-10-18: Clarified structural classification versus reachability across the
project contract, generated workspace instructions/workflow/coordinator and ordinary
disassembly, redirect and final-investigation prompts. Independent static evidence
may establish roles, exact bounds and bank context from the first pass without a
live caller, observed fetch, naturally produced selector or gameplay route. Record
reachability uncertainty separately; lack of a caller does not establish dead or
unused status. Caller/selector tracing remains useful evidence, and sparse selected
entries cannot establish a full table extent without independent support. Artificial
execution, pattern resemblance and successful decoding alone remain insufficient.
This is an explicit application of the existing static-evidence policy, not a new
relaxed final-phase acceptance rule. Existing game workspaces were not migrated.

Validation: synthetic disassembly and commentary regressions, stock portability,
Python compilation, git diff --check, locked release build, fmt and locked all-target
workspace Clippy with warnings denied pass. Prompt assertions cover ordinary and
final phases, separate reachability claims and independent table-extent evidence.
No runtime/dependency or game-artifact changes; prompt tests do not prove live-agent
classification quality.
Locked workspace tests also pass under Rust 1.90.0.

2026-10-18: Final investigation now iterates automatically from prior final and
redirect handoffs and residual ledgers after accepted batches. A ready redirect
lead is not required to plan concrete alternative methods against remaining
barriers. Useful intermediate constraints can continue the phase without an empty
disassembly/commentary cycle. Each retry records its changed evidence/method and
discriminating outcome; terminal exhaustion stops unchanged restarts. No minimum
byte gain or fixed pass quota; full classification/editorial/fidelity gates remain.
Unique attempt directories preserve handoffs and evidence, including legacy records
before replacing the top-level handoff with a latest-attempt index.

Updated generated instructions, README and prompt regressions, plus the existing
Castlevania coordinator/workflow/final prompt and redirect guidance with a migration
note. No investigation launched or ROM/map/state/source/review/verification artifacts
changed. Synthetic disassembly/commentary regressions, stock portability, Python
compilation, diff checks, release build, fmt and locked all-target workspace Clippy
with warnings denied pass. Tests establish instruction generation and existing
fidelity behavior, not live-agent loop termination or semantic research quality.
Locked workspace tests also pass on Rust 1.90.0. No runtime/dependency changes.

2026-10-18: Archived the local Castlevania Adventure workspace under
atlas/gb/castlevania-adventure/edb101e924f22149bdcbcfe6603801fdb4ec0139a40493d700fa0205f6dab30c/.
Corrected the workspace and original local ROM filename to castlevania-adventure,
updating 95 existing UTF-8 files with internal spelling/path references. ROM and
assembly bytes remain unchanged; historical checkpoint hashes remain historical.
Reused round025 readability review and inspected source contracts for final
editorial acceptance; filled seven empty Code-unit inventory lists from existing
reviewed summaries/source. Final strict audit passes 2807 regions/3007 units.
Classification remains incomplete: 64955/65536 classified, 581 Unknown in 85 ranges.
Final method-exhaustion documents and residual ledger retained; scratch excluded.
Archive validation passes store/map checks, exact-source restoration and byte-identical
65536-byte rebuilding using the configured release binary. No toolkit code or
session changes; no commercial ROM, generated assembly or extracted assets added
to the archive. Archive remains uncommitted.

2026-10-18: Added an isolated SNES feasibility evaluation under
experiments/snes-spike with a separate Cargo.lock and reproducible pinned setup.
Production crates, root lockfile/dependencies, current GB prompts and game
workspaces remain unchanged. Agreed SNES scope and the review gate are linked
from project.md/architecture.md; docs/snes-evaluation.md records candidates,
licenses, measured results, prototype limits and remaining implementation.
Leading technical candidate is GPLv3 jgenesis, pending user license/packaging
review and broader testing. Super Sabicom MIT compile smoke passes but documented
standard-PPU gaps remain; RustySNES has source-level review only and an upstream
open provenance question. No production core choice/adoption is claimed.

jgenesis master b4f943b fails Cargo 1.90 manifest parsing; pinned v0.11.4 7b3cc06
passes its upstream locked snes-core check. Local evaluation hooks expose immutable
memory, actual main/SPC fetches/accesses and fixed initial RAM. An ExecutionAdapter
prototype runs original synthetic LoROM/HiROM fixtures, standard IPL upload,
serial B input, SRAM and nonzero DSP audio. Snapshot replay matches core state,
held input, PCM, frame hashes and bounded raw traces; capture does not alter state
or PCM. Foreign/truncated restores preserve live state, cold reset is deterministic
and preserves SRAM, and bounded main-CPU instruction stepping ends after exactly
ten fetched instructions. Each three-frame capture has 2397 stereo frames/4794
nonzero channel samples in valid 48-kHz PCM16 WAVs. Trace caps/drops are explicit.
No GUI/audio device or commercial ROM/assets was used.

832 sampled MIT SingleStepTests cases (512 65816, 320 SPC700) pass registers,
final memory and cycle counts. This does not compare bus order/signals or establish
a full hardware gate. Asar 1.91 6a6dbda builds; upstream opcode expected streams
match 601/519 bytes. Two small reconstruction probes regenerate deterministic
source and exact 64-KiB images from a ROM-free experimental mapping and local ROM,
preserving width/alias/branch/SPC storage-context cases and invalid checksums.
Unknown-M immediate remains raw locally; independent fixed-width decoding proceeds.
Python decoder/emitter is a limited feasibility subset, not completed SNES
classification, editorial output or production Rust disassembly support.

Read-only prompt/script audit in docs/snes-prompt-audit.md identifies platform
selection needed for .sym, archive/porter gates, RGBDS and source inventories.
Proposed SNES-only guidance accepts supported static context and confines ambiguity
to affected instructions; it adds no runtime-evidence requirement to Game Boy.
No active prompt was migrated. Production trace/session conversion, per-SPC stepping,
broader hardware/games, full decoder/maps/backend and native runtime remain pending.
Representative owned SNES images were requested but have not been supplied.

Validation on Rust 1.90.0: reproducible experiment setup, locked release build,
fmt check, own-target Clippy with warnings denied, Python compilation and complete
synthetic/CPU/reconstruction driver pass. Root locked release build, workspace fmt,
all-target Clippy with warnings denied, workspace tests, dependency boundaries,
stock GB deterministic byte-identical portability and git diff --check pass.
All emulator/tool sources, local patches, licensed CPU datasets/notices and generated
ROMs/source/WAVs/reports remain ignored/local. Original harness/scripts are MIT;
its locally linked jgenesis executable is subject to GPLv3, not MIT-only.

2026-10-18: Following user approval of GPL adoption, integrated the pinned
jgenesis v0.11.4 core into production as romatlas-snes. LICENSES.md documents
GPL-3.0-only combined CLI/SNES licensing and preserved independent MIT runtime
licensing. The vendored core contains source/notices only; Asar remains a pinned
external build. docs/snes.md describes the measured baseline and API limits.

Production supports baseline NTSC LoROM/HiROM, optional copier headers, SRAM,
two ordinary controllers, deterministic reset/snapshots, bounded main/SPC stepping,
actual fetch/access traces, safe backing-memory inspection, RGB frame exports and
48-kHz stereo WAV capture. Main/SPC clocks and artificial patches/provenance remain
explicit. Unsupported cartridge features and unsafe I/O inspection are rejected.
Both CPU decoders cover all opcode tables; M/X ambiguity is confined to affected
instructions. ROM-free JSON maps retain alternate CPU views and upload provenance;
static discovery and observed execution remain distinct. Asar generation/verification
rebuilds the entire original file byte-identically, including copier headers.

SNES preparation, commentary, reviewed source archives and native-port preparation
now select platform-specific maps, assembler and instructions. Existing GB prompt
constants are byte-identical to their previous versions; GB evidence requirements
are unchanged. Native runtime adds planar tiles, BGR555 palettes, tilemap/Mode7
helpers, twelve-button defaults and NTSC cadence without an emulator dependency.
Preparation is not a completed native game, and raw-data reconstruction does not
satisfy classification or editorial completion contracts.

Validation on Rust 1.90.0 passes: locked release build, fmt check, workspace tests
and all-target Clippy with warnings denied; desktop-feature runtime Clippy/tests;
session, portability, dependency-boundary, disassembly, commentary, archive and
native-port regression scripts. Production test-snes.py passes synthetic LoROM/HiROM
execution, serial input on both ports, IPL upload, SRAM, audio, deterministic replay,
rejected transactional edits/restores, bounded traces/context, exact reconstruction,
copier headers and the preparation/commentary/archive/native scaffold workflow.
All 256 main opcodes across four M/X combinations plus 256 SPC opcodes rebuild
exactly (1280 views). The production CPU harness passes 832 pinned sampled
SingleStepTests cases for registers, memory and cycle counts, not bus-order signals.
No representative owned SNES games, broad SNES hardware suite, GUI or audio device
were tested. Rare hardware, enhancement chips and PAL remain outside this baseline.
No commercial ROMs or extracted assets were downloaded or added; generated binary
fixtures and evaluation datasets remain local and ignored. Changes are uncommitted.

2026-10-18: Removed the retired experiments directory. Moved the original SNES
assembly fixture to scripts/fixtures/snes/fixture.asm and the pinned MIT CPU-vector
downloader to scripts/fetch-snes-cpu-tests.py; production tests no longer depend on
the spike. Updated historical evaluation reproduction instructions to production
commands. Production SNES workflow regression, pinned CPU-vector tests, Game Boy
portability, Python compilation and git diff --check pass. No Rust code changed.

2026-10-18: Fixed TI-001 from the local Super Off Road toolkit issue report.
SNES analysis.discover and analysis.discover_context now independently page
candidates, references and diagnostics using offset/reference_offset/diagnostic_offset
and limit (default 100, max 1000). Totals and next offsets describe the full bounded
search; paging preserves traversal budgets, observed merging and evidence semantics.
Queries recompute discovery, so callers must retain the same inputs/map/traces.
Documented the response contract and added synthetic RPC regressions for independent
list reconstruction, zero/oversized offsets, invalid paging inputs and a 100-root,
4000-instruction graph. No commercial ROM/assets or live game session changes.

Validation on Rust 1.90.0: locked release build, fmt check, workspace all-target
Clippy with warnings denied, locked workspace tests, pinned SNES CPU vectors,
dependency boundaries, Game Boy session regression and deterministic byte-identical
four-bank stock portability pass. Python compilation and git diff --check pass.
Production test-snes.py also passes LoROM/HiROM RPC paging, snapshots, audio, SRAM,
exact rebuilding and the preparation/commentary/archive/native-port workflow.

2026-10-18: Audited repeated local goal-agent terminal stops against primary
investigation artifacts. Prompts allowed static inference but under-specified how
to combine independent constraints, limit counterexample scope and correct prior
rejections on unchanged evidence. Whole-gap/whole-generator failures became broad
vetoes; coordinator preservation checks did not examine rejection sufficiency.
docs/agent-inference-audit.md distinguishes these observed decisions from a claim
that prompt wording is their sole cause.

Generated workspaces now include a shared evidence-policy.md referenced by the
coordinator and phases. It gives a positive structural inference process, separates
role/extent from historical generator recovery and reachability, localizes uncertain
subclaims and permits justified correction of prior reasoning without new runtime
data. Terminal handoffs require a compact rejection review inspected against primary
artifacts by the coordinator. Sound reviews are reusable and genuine exhaustion
still stops incomplete; classification, editorial and fidelity gates remain intact.
Both Game Boy and SNES use the same policy without extra Game Boy context demands.

Migrated only the existing Super Off Road instruction files, retaining originals
and a hash manifest locally. All135 protected ROM/map/state/commentary/investigation
files matched before/after migration. No classification, source text, trace, input,
snapshot, goal status or historical handoff changed; no phase/worker was launched.
The next coordinator/worker must reread updated instructions. No commercial content
or game artifacts were added to this repository.

Validation on Rust1.90.0: locked release build, fmt, locked all-target workspace
Clippy with warnings denied and locked workspace tests pass. Synthetic disassembly
and commentary regressions, production SNES workflow, deterministic byte-identical
four-bank stock portability, Python compilation and git diff --check pass. Added
checks cover policy delivery, references and preserved positive/negative guidance
in both platform profiles. These tests do not establish live-agent inference quality
or guarantee that remaining game bytes can be classified; subsequent run behavior
must supply that evidence. No Rust/session/dependency implementation changed.

2026-10-18: Retired the separate final-investigation phase and its5%/16KiB
scheduling threshold. Generated workspaces now expose only disassembly, commentary
and redirect. Redirect incorporates exact residual ledgers, whole-table/container
and consumer contracts, selector reasoning, targeted bounded experiments and scoped
rejection review at every classification percentage. Useful intermediate evidence
or justified next methods continue redirect without an empty classification cycle;
supported combined proposals pass through disassembly and full current-source
commentary. There is no fixed pass quota. Genuine reviewed exhaustion still stops
incomplete; final editorial acceptance and exact verification remain completion gates.

Updated preparation results, both platform profiles, coordinator/workflow/phase
instructions, project contract, README, inference audit and regression assertions.
Historical SNES audit references are explicitly marked as historical. Legacy
final-investigation attempts remain immutable evidence; unfinished research resumes
as redirect. Migrated the existing Super Off Road instructions after checking the
prior migration hashes, retaining all original instructions and the retired prompt
locally. All135 protected ROM/map/state/commentary/investigation files still match;
no classification, source, runtime, goal-status or historical research changes.
No agent/worker was launched. Other existing workspaces are not silently rewritten.

Validation on Rust1.90.0 passes locked release build, fmt, locked all-target workspace
Clippy with warnings denied and locked workspace tests; synthetic disassembly and
commentary regressions, production SNES workflow, deterministic byte-identical
four-bank stock portability, Python compilation and git diff --check pass. Tests
check the three returned/generated prompts, absence of retired scheduling, continued
research and legacy resumption guidance in both profiles, alongside preserved
classification/readability/fidelity requirements. They establish instruction delivery
and tooling behavior, not the number or quality of future live-agent inferences.
No Rust/session/dependency implementation changed; no commercial content added.

2026-10-19: Native-port preparation now seeds uncertainties.json from every reviewed
Unknown interval, preserving source IDs/ranges and allowing additional semantic
ambiguities from classified material. Port/start/resume/reviewer prompts assess
consumer context, behavioral impact, evidence, useful tests and limits as implementation
proceeds. Full structural classification is not a prerequisite for starting or
finishing a port; supported ROM-backed data preservation and explained no-impact
exclusions are allowed. Preserving instruction bytes is not native behavior, and
unresolved gameplay-affecting behavior still prevents completion. New-workspace
audits reject pending/dropped seed entries, bad ledger links and preserved behavior;
they validate accounting, not interpretation truth. Older untracked workspaces
retain their prior audit contract and are not rewritten automatically.

Added prompts/reference-updates.md and local reference-init/reference-verify commands.
Working source/state/cumulative corrections/evidence live under ignored research/.
Verification snapshots the candidate, validates/exports its map, independently
generates/rebuilds structural source and rebuilds edited commentary byte-identically.
Successful named immutable revisions include fresh reading copies and verification;
an atomic pointer adopts the supplement while frozen archive/source and original
ledger identities remain unchanged. Failed checks retain the old revision and
publish no candidate; adopted-artifact hash changes are rejected. Updates remain
local, with an explicit evidence-linked upstream handoff rather than automatic
overwriting of an archive or active analysis workspace. Rebuilding does not prove
semantic correctness or map/comment consistency. No commercial workspace was edited
and no game-port agent was launched.

Validation on Rust 1.90.0: locked release build, workspace fmt, locked all-target
workspace Clippy with warnings denied and locked workspace tests pass, alongside
romatlas-port desktop-feature Clippy/tests without initializing a window/device.
test-port.py passes incomplete-archive preparation, seeded pending gaps, data versus
behavior disposition checks, removed-entry rejection, named reference corrections,
evidence retention, parent revisions, failed source/map rollback/retry, tamper
rejection and unchanged frozen reference; locked headless/desktop scaffold checks
remain passing. Production test-snes.py also passes SNES reference correction with
explicit platform selection for input/game.rom, JSON mapping and exact edited-source
rebuild, plus the existing RPC/replay/reconstruction/archive/port workflow. Stock
four-bank deterministic byte-identical portability, Python compilation and
git diff --check pass. These synthetic checks do not establish a real game's
interpretation or port correctness. No Rust/session/dependency implementation changed.

2026-10-19: Archive creation accepts incomplete commentary reviews while still
requiring ROM identity, map validation, exact source reconstruction and a fresh
byte-identical verify. The archive manifest records editorial completion, reviewed
counts and concise reasons. Native-port preparation now accepts incomplete editorial
review and classification while requiring a valid full-ROM region partition,
in-range unit records, verified archive artifacts, matching ROM identity and a fresh
byte-identical source restoration. Pending source entries remain in the generated
port ledger and uncertainty seed. `python3 scripts/test-port.py` and
`python3 scripts/test-snes.py` pass, including preparation from synthetic
incomplete-review GB and SNES archives. The previously blocked Super Off Road archive
prepares successfully to `/root/rom-ports/super-off-road/` with byte-identical
reference restoration. Its archive remains unchanged and no ROM or extracted assets
were added to the repository.

2026-10-19: Added `MultiInputState` with four independent held/pressed/released
action sets, an opt-in `Game::tick_players` method that preserves legacy
`Game::tick`, and `InputMapper::sample_players`. SDL desktop input keeps the
keyboard and selected controller in player zero and routes up to three other
controllers to their own slots; latches and edge history remain per-player.
ROMAtlas port tests (11 unit, 11 runtime), desktop-feature Clippy, the native
Super Off Road `python3 run.py check`, and `scripts/test-port.py` pass. No
desktop device was initialized. Game-specific multiplayer behavior is not
implemented by this toolkit change.

2026-10-19: Added `headless_players`, which accepts scripted four-slot input
and advances `Game::tick_players` without initializing frontend devices. The
legacy `headless` function now adapts its input into slot zero and uses the same
path. A runtime test verifies secondary-slot delivery. `cargo test --locked -p
romatlas-port` (11 unit, 12 runtime), desktop-feature Clippy,
`scripts/test-port.py`, and `scripts/test-boundaries.py` pass.

2026-10-19: Implemented toolkit additions requested by the Super Off Road porting
interview. SNES `emulator.run_fast` advances frame/cycle units with lightweight
core progress counters, suspended debug probes and opt-in framebuffer/audio
capture. Timeout/tick-budget stops return partial progress and remaining counts;
existing snapshot export/import provides durable resumable checkpoints. Ordinary
stepping still supports traced instruction evidence and now avoids unnecessary
full CPU-state collection for progress checks. Fast gaps split successor chains,
produce no coverage, clear stale frame capture and preserve artificial SRAM-change
provenance. No game workspace or original ROM/assets were changed.

Independent MIT `romatlas-port::snes::ppu` adds wrapped planar BG/tilemap sampling,
large-tile flips, OAM decoding/sampling and ordered OBJ scanlines, windows, Mode 1
ranks, main/subscreen selection, five-bit color math and scanline composition.
Independent native audio adds BRR predictors, validated looping sources, pitched
voices, ADSR/GAIN, noise, stereo mixing and private-buffer FIR echo. Ports supply
layer/driver/game timing decisions. docs/snes-native.md documents APIs and limits:
these are not a full PPU/SPC700/S-DSP implementation; linear audio interpolation,
non-cycle-exact DSP behavior and unsupported rendering modes remain explicit.
No GPL implementation was copied into the native runtime.

Synthetic fast/traced regression compares final CPU state, WRAM, framebuffer and
WAV bytes; checks tick-budget checkpoint export/import/resume, timeout progress,
zero/invalid requests, cycle advancement, no fast-run evidence and resumed tracing.
Eight native helper tests cover planar/map/OBJ ordering, windows/math, BRR predictor
vectors, envelopes/noise, source termination/looping and deterministic echo/mix replay.
A local synthetic 120-frame sample measured ordinary untraced stepping at 6.458 s
and fast at 4.960 s during concurrent builds. A separate fast 1200-frame run
completed in 22.449 s; a 30 s segment under load returned 903/1200 frames with 297
remaining. These are environment-dependent synthetic observations, not commercial
game timings, hardware fidelity claims or a promised performance multiplier.

Validation on Rust 1.90.0 passes: locked release build, workspace fmt, all-target
workspace Clippy with warnings denied and locked workspace tests; native-runtime
desktop-feature Clippy/tests without devices; test-port.py, test-session.py,
test-boundaries.py, deterministic stock Game Boy portability, pinned licensed
homebrew gameplay/replay/rebuild regressions and Python compilation. Full
production test-snes.py passes both LoROM/HiROM fast replay, durable checkpoint
resume, artificial SRAM/reset provenance and existing reconstruction/workspace
flows. test-snes-cpu.sh passes the 832 pinned sampled CPU vectors. All generated
fixtures, snapshots, audio, framebuffers and game assets remain local/ignored.
Changes are uncommitted; pre-existing multiplayer and emulator changes remain.

2026-10-19, Super Off Road Plan 02: added opt-in native
`Mixer::with_buffered_noise()` to preserve a caller's initial held noise output
and decrement-before-generation phase. The default constructor is unchanged.
The original MIT test compares 4000 frames against the standalone Noise helper,
including four rates, mute/silent intervals and clone replay. No adapter/core
or dependency changed. Rust 1.90.0 locked release build, workspace fmt/Clippy/tests,
desktop-feature native-runtime Clippy/tests, test-port.py, stock portability and
boundary checks pass. No devices were initialized. Edits remain uncommitted.


2026-10-19, Super Off Road Plan 03: independent MIT optional
ppu::object_scanline_limited now selects 32 objects and fetches 34 tile columns,
with reverse fetch priority, transparent fetch consumption, X=-256 budget behavior
and per-line overflow summaries. Original unlimited API is preserved. New original
synthetic tests/snes_obj_limits.rs covers four boundary/ordering cases; no GPL
console implementation or game assets were read/copied into this implementation.
An original synthetic 35-sliver ROM compares exactly for unflipped partial
overflow; flipped partial overflow disagrees by 128 pixels (helper left column,
local adapter right column). This specific hardware/adapter question remains
unresolved; no hardware equivalence claim follows from synthetic expectations.
Interlace and timed/frame-latched STAT77 flags remain outside the API.

Rust 1.90.0 locked release build, workspace fmt/Clippy/tests, romatlas-port desktop
Clippy/tests, test-port.py, test-portability.py, test-boundaries.py, production
SNES RPC regression, pinned CPU vectors, licensed homebrew replay/rebuild and
89-case hardware regression pass. No device or desktop graphics initialized.
Existing audio.rs, tests/snes.rs and documentation edits are preserved; all edits
remain uncommitted. See docs/snes-native.md and the game library-changes.md.

2026-10-19 native SNES brightness precision: new independent MIT
color_with_brightness retains five-bit components until rounded output conversion;
color remains unchanged at full brightness. Composition now uses it after color
math, replacing RGBA8 truncation. A native game highscore fade exposed the gap:
54 natural-source images including fade/scroll/closure now match, plus90 paired
window states. New snes_brightness test covers partial levels, endpoints/clamp and
forced blank. Locked release, workspace fmt/Clippy/tests and desktop-feature
Clippy/tests pass; additional relevant scripts are running and will be recorded
in the game handoff. Prior audio/OBJ-limit edits are preserved and uncommitted.
Brightness follow-up validation complete: native-port integration, portability,
dependency boundaries, production SNES, SNES CPU, licensed homebrew and hardware
scripts all pass in addition to release/workspace/desktop-feature checks. Logs
and commands are retained in the game workspace checks/plan-03-toolkit-brightness.
No desktop graphics run; prior audio/OBJ changes remain preserved/uncommitted.

2026-10-20: Revised the complete generated native-port instruction set for
maintainable best-effort implementation, applying the same contract to Game Boy
and SNES. Preparation now includes architecture.md: a bounded representative
source survey and actionable module/state/interface/algorithm plan before substantial
coding, maintained when decisions change. Feature work includes brief affected-design
checks and one bounded final maintainability pass, without arbitrary file/line
quotas, cosmetic splitting or repeated architecture audits.

Goal/resume text, generated AGENTS/workflow/README/status documents, start/reviewer/
reference-correction/build templates and active port design/project documentation
now agree that runtime fidelity validation is outside the implementation goal.
No native-versus-ROM frame/audio/state/scenario comparisons, exhaustive replay
matrices or device/playthrough gates. Debugger access answers a named unresolved
source question with a declared bound/stopping condition; inconclusive experiments
do not justify endless retries. Focused native unit tests plus required format,
static and build checks establish implementation checks. Finish whole-game
implementation/accounting and handoff, run the administrative ledger/build audit
and stop once it passes. Existing passing checks are repeated only for relevant
changes, failures or concrete defects, not confidence or remaining goal budget.

Uncertainty accounting distinguishes missing required code from an implemented,
reasonable source-informed assumption. The latter can be assessed as resolved
for port accounting while explicitly retaining its basis and unverified source/
fidelity limits; no new schema or runtime proof gate was introduced. Assumptions
cannot be published as confirmed source classification. Original mapping evidence,
classification/editorial and exact-reference-rebuild contracts remain intact and
are explicitly scoped separately. Optional reference corrections protect source
integrity; they are not a native fidelity-validation phase. Existing game workspaces
and prior migration plans were not rewritten. Pre-existing runtime-library work
was preserved; no native gameplay or console implementation changed in this task.

Validation passes: locked release build, workspace fmt, Python compilation,
test-port.py (including generated architecture/policy delivery and administrative
acceptance of documented implemented assumptions), test-snes.py (the same policy
in generated SNES ports plus existing workflow regressions), stock deterministic
Game Boy portability, dependency boundaries and git diff --check. The port
regression runs generated locked headless unit tests, Clippy and desktop builds
without devices. These checks establish instruction delivery/tooling behavior,
not that every future model will follow the stopping or maintainability rules.
Changes remain uncommitted.

2026-10-20: Added one sentence to the shared native-port unit-test guidance reserving
tests for complex/error-prone behavior and excluding trivial arithmetic, obvious
getters/setters and implementation restatements. Documentation-only change;
`git diff --check` passes. No tests run.

2026-10-20: Added the NTSC NES platform alongside Game Boy and SNES. Selected
jgenesis v0.11.4 at the existing pinned fork revision after source review of
jgenesis, TetaNES and pinky; docs/nes-evaluation.md explains the integration choice
and distinguishes source findings from measured compatibility. No non-Rust core
port was needed. Preserved upstream GPL notices and added ca65/ld65 V2.19 setup
pinned to 555282497c3ecf8b313d87d5973093af19c35bd5. Shared/native crates remain
independent of console adapters/cores; the native runtime retains MIT licensing.

romatlas-nes adds cartridge inspection, iNES/NES 2.0 section parsing, automatic
.nes/explicit platform selection, persistent CommandAdapter RPC, bounded
instruction/cycle/frame advancement, two controllers, CPU state/edits, backing
RAM edits, immutable CPU/PPU inspection, framebuffer/WAV output, reset and
identity/checksummed transactional same-build snapshots. Actual opcode/bus/backing
hooks distinguish physical PRG/CHR/RAM from CPU aliases and unmapped accesses;
interrupt dummy fetches do not become executed instructions. Trace filters,
streaming, aggregates, observed edges, effective PRG-slot contexts, static
hypotheses, barriers and transactional maps use the existing workflow contracts.
Baseline execution accepts mappers 0/1/2/3/4/7 with explicit unsupported-feature
errors. Other mappers, nonzero submappers, trainer execution, PAL and rare
peripherals are outside this baseline. Snapshot files retain a local raw image
copy and must remain local.

The complete 256-encoding NMOS decoder preserves lengths, operands and addressing
widths. ROM-free JSON maps carry physical roles separately from CPU/bank views.
ca65/ld65 reconstruct whole images, including headers/trainers/CHR/trailing data;
unofficial aliases and wrapping branches retain exact bytes with decoded mnemonic
comments. Empty/raw maps remain unknown and do not satisfy classification. New
NES disassembly/commentary/archive/port profiles preserve structural versus
reachability, whole-ROM classification, full current-source readability and
editorial gates; existing workspaces are not rewritten.

Independent MIT romatlas-port::nes helpers implement planar CHR, banked background
sampling, nametable mirroring/attributes/scroll, 8x8/8x16 sprites/flips/selection,
clipping/priority/zero-hit composition, grayscale/emphasis, and NTSC pulse/triangle/
noise/DMC primitives with envelopes, sweeps, counters, frame IRQs, sample callbacks
and nonlinear mixing. Ports own game rules, mapper/raster scheduling and drivers.
Analog/composite fidelity, expansion audio, hardware sprite-overflow bugs and
cycle-exact DMA interactions are documented limits, not completed hardware gates.

Validation on Rust 1.90.0 passes: locked release build; workspace fmt, all-target
Clippy with warnings denied and locked workspace tests; desktop-feature native
Clippy/tests; NES synthetic CLI/RPC/replay/capture/trace/map tests, all 256 opcode
rebuilds, 1 MiB reconstruction, trainer/trailer retention, executed MMC1/MMC3/UxROM
bank writes, debugger edits and new-workspace/commentary/archive/native-port flows.
Three NES structural tests and four independent native-helper tests pass. Existing
session, stock four-bank portability, disassembly, commentary, archive, native-port,
production SNES, pinned SNES CPU-vector, licensed homebrew, dependency-boundary
and 89-case Game Boy hardware regressions pass. Python compilation and
git diff --check pass. Parallel scaffold builds pressed local disk capacity;
cleared only generated assembler-library/CLI-debug build artifacts, and all
checks completed successfully. No display/audio device was initialized. All
synthetic images, captures, snapshots and generated workspaces remain local;
no commercial ROM or extracted game assets were downloaded or committed.
These results do not establish broad NES hardware or commercial-game fidelity.
Changes remain uncommitted.

2026-10-20: Added the SMS platform alongside Game Boy, NES and SNES. The debugger
uses the existing pinned GPLv3 jgenesis revision (smsgg-core, smsgg-config, z80-emu,
PSG and YM2413), with actual fetch/memory/I/O/VDP-port/interrupt probes, immutable
backing inspection, detached I/O peeks, register/RAM/VRAM/CRAM/I/O edits, physical
CPU aliases, contextual coverage/edges, bounded stepping and deterministic guarded
snapshots. Instruction-driven peripheral timing remains upstream behavior; events
share instruction-start cycles. Baseline is NTSC SMS2, standard pads and Sega/
Codemasters boards; rare peripherals/boards, BIOS boot, GG/SG-1000 and broad hardware
compatibility are not claimed. Runtime currently requires power-of-two ROM payloads.

ROM inspection preserves complete-file SHA-256 identity, optional copier prefix,
TMR SEGA metadata/checksum and explicit heuristic mapper-identification basis.
ROM-free SMS JSON maps separate physical roles from CPU execution views; static
analysis retains unresolved windows, data barriers and uncertainty. Explicit
little-endian pointers remain hypotheses. DeZ80 4.0.1 (MIT, pinned source) supplies
Z80 decoding instead of another instruction table. Pinned WLA-DX 10.6 reconstructs
complete images with six CPU slots and fixed physical banks; ignored prefixes,
aliases and assembler-boundary instructions retain exact encodings with decoded
comments. Raw reconstruction does not establish classification completion.

New SMS agent, commentary, archive, reference-correction and native-port profiles
retain whole-ROM classification, structural versus reachability, full current-source
readability and editorial gates. No existing workspace is rewritten. Native
scaffolds use SMS buttons, dimensions/cadence and four-plane tile loading.
Independent MIT native helpers provide Mode 4 tilemap/scroll/priority/sprite/zoom/
mask/palette/line-counter effects, clocked Sega PSG and approximate nine-voice OPLL
FM/custom/preset/rhythm synthesis with audio gating. Only MIT emu2413 instrument
preset data is reused, with its notice; no GPL emulator implementation enters the
native library. FM key scaling/slot timing/exact rhythm wiring, legacy VDP modes,
SMS1 masking and analog/cycle-exact output remain explicit limits.

Validation recorded so far: Rust 1.90.0 locked release build, workspace tests and
all-target Clippy with warnings denied; native desktop-feature Clippy/tests without
devices; SMS synthetic debugger/replay/capture/context/banking, all 1,792 opcode-family
samples and byte-identical reconstruction, prefix/Codemasters/1 MiB images, 300
annotations, generated-workspace/commentary/archive/restore/native-scaffold flows.
Game Boy session, stock deterministic four-bank portability, disassembly, commentary,
archive, licensed homebrew and 89-case hardware checks pass. Final follow-up results are recorded below.
No commercial ROM or extracted game assets were downloaded or committed; generated
fixtures, captures, snapshots and workspaces remain local. Changes are uncommitted.

Final SMS follow-up: the latest locked release build and SMS release-mode
Clippy/unit tests pass. The synthetic integration gate passes actual Sega SRAM
bank switching and backing values, known-bank SRAM pointer resolution, Codemasters
writes, disabled-cartridge CPU identity, VDP physical writes, immutable I/O peeks,
VRAM/I/O debugger edits and mapped RAM translation. The complete new SMS workflow
still reconstructs byte-identically. An evidence-linked SMS JSON reference
correction independently rebuilds structural and edited source while preserving
the frozen baseline. New SMS build instructions use game.sms and local ignores
include SMS images. Final workspace formatting, Python compilation, dependency
boundaries and git diff checks pass. Existing native-port preparation/accounting/
headless/desktop, NES and production SNES integrations pass, as do native-runtime
desktop Clippy/tests. Pinned SMS setup builds wla-z80/wlalink successfully.
The pinned SNES CPU-vector gate also passes. No GUI/audio device was initialized.
All requested toolkit changes and validation are complete within the documented
baseline limits; no commercial-game or full-hardware fidelity claim is made.

2026-10-20: Added Game Gear alongside SMS, Game Boy, NES and SNES through the
shared Sega adapter. `.gg`/`--platform gg` selects the existing pinned GPLv3
jgenesis GameGear hardware, 160x144 LCD, Start polling, stereo PSG and 64-byte
CRAM without the SMS FM unit. Separate map/snapshot format identities preserve
platform validation. Existing register/shadow-state edits, immutable mapped and
physical inspection, I/O/backing edits, actual fetch/bus/interrupt/VDP traces,
contextual coverage/edges, bounded operation/cycle/frame stepping, reset, guarded
snapshots and image/audio capture are available. Both committed CRAM bytes are
traced on an odd-address write; even-address writes remain latch-only.

Inspection retains whole-file identity, optional copier prefix, TMR SEGA metadata,
layout and explicit mapper-identification basis. Shared effective-bank analysis,
references, pointer hypotheses, data barriers and transactional annotation/import/
export validation apply to romatlas-gg-map JSON. The existing pinned DeZ80 decoder
and external WLA-DX assembler/linker reconstruct complete images, including exact
fallback encodings and unchanged headers/checksums. Tool selection also reviewed
crater's primary documentation; see docs/gg-evaluation.md. No new instruction
table, assembler dependency or GPL implementation in the native library was added.

New Game Gear workspace, classification, commentary, archive, reference-correction
and native-port profiles retain structural versus reachability, whole-ROM
classification, full current-source readability and editorial requirements.
Header-identified GG images using .sms also select the GG workspace profile;
explicit platform selection is honored. Source generation preserves annotation
text mentioning SMS filenames. Existing game workspaces are not rewritten.
Independent MIT romatlas-port::gg supplies four-plane tiles, Mode 4 effects and
sprite selection before LCD cropping, 12-bit palette conversion (including the
full-line gg::vdp::rgba API), 224-line viewport adjustment, stereo PSG routing,
fractional sample clocks, buttons, dimensions and cadence. Ports own game rules,
raster scheduling and music drivers. Serial/EXT link peripherals, uncommon boards,
BIOS boot, exact analog/subscanline native output and broad hardware/game fidelity
remain documented limits; runtime execution still requires power-of-two payloads.

Validation passes on Rust 1.90.0: locked release build; workspace format,
all-target Clippy with warnings denied and locked release workspace tests;
final native desktop-feature Clippy/tests without devices; Game Gear synthetic
CLI/RPC/replay/input/CRAM/trace/context/banking tests, all 1,792 shared Z80
encoding samples and exact reconstruction, prefix/Codemasters/1 MiB images,
headerless explicit selection, cross-platform map rejection, annotation text
preservation, generated workspace/commentary/archive/restore/reference-correction/
native-scaffold flows. Native helper tests cover LCD palette/cropping, full-line
palette validation, stereo routing and deterministic replay. Final SMS and NES
integrations and production SNES integration pass. Existing session, stock
four-bank portability, dependency boundaries, disassembly/commentary/archive,
native-port accounting/build, pinned licensed homebrew, 89-case Game Boy hardware
and pinned sampled SNES CPU-vector gates pass. Python compilation and git diff
checks pass. Two initial concurrent SMS/NES scaffold builds exceeded their
five-minute harness timeout; serial reruns with a shared local Cargo cache pass.
No display/audio device was initialized. All fixtures, snapshots, captures and
generated workspaces remain local; no commercial ROM or extracted game assets
were downloaded or committed. Changes remain uncommitted.

2026-10-20: Added Genesis/Mega Drive alongside SMS, Game Gear, Game Boy, NES and
SNES. The debugger uses the existing approved GPLv3 jgenesis revision, adding its
genesis-core, genesis-config and m68000-emu sources with upstream notices intact.
Accepted 68000 instruction hooks distinguish interrupt/STOP/DMA waits from code;
prefetch backing/context survives mapper changes and snapshots. Actual main/Z80
bus operations, committed VDP writes, DMA and interrupts feed bounded traces,
streaming, coverage, contextual roots and observed edges. Z80 bridge reads and
ignored ROM writes do not invent CPU aliases or immutable backing writes.

The adapter supplies bounded operation/master-cycle/frame stepping for either CPU,
two standard six-button pads, full main/Z80 register state and edits, backing RAM/
VRAM/CRAM/VSRAM/cartridge RAM edits, immutable mapped/physical inspection, detached
I/O peeks, reset, guarded snapshots, framebuffer and WAV capture. Snapshot replay
preserves partially rendered frames and interlace history rather than relying on
upstream's omitted framebuffer. The exact cartridge constructor avoids automatic
format conversion and game-specific ROM rewrites. Baseline is region-selected NTSC/PAL cartridge
hardware, linear ROM/SRAM and Sega SSF banking; recognized unsupported enhancements
and 32X are rejected for execution. Exotic peripherals, SVP, Sega CD/32X, uncommon
boards and full hardware/game compatibility remain outside the baseline.

Inspection retains complete original-file SHA-256, headers/titles/serial/regions,
reset vectors, ROM/SRAM layout, checksum and explicit mapper/save-hardware
identification basis. Canonical physical backing is the unchanged copier prefix
followed by normalized big-endian payload; original-file positions/encoding remain
separate and reversible. Big-endian, word-swapped and SMD images reconstruct exactly.
ROM-free Genesis JSON mappings separate physical roles from main/Z80 execution
views, effective SSF 512 KiB banks, SRAM overlays/byte lanes and Z80 32 KiB main-bus
windows. Unknown/EEPROM overlays remain local and unresolved. Annotation validation
is transactional; references, discovery and explicit big-endian absolute/signed
relative or Z80 little-endian pointers remain hypotheses, never execution coverage
or automatic classification.

Selected external GNU binutils 2.42 for complete 68000 instruction decoding and
as/ld/objcopy reconstruction after evaluating available open-source alternatives.
Setup pins upstream release/SHA-256 or accepts the tested 2.42 cross package.
DeZ80/WLA-DX retain their existing pins for independently rebuilt Z80 source units.
CPU virtual addresses and physical load placement are separate; file encoding is
restored after linking without header/checksum repair. GNU bare-hex indexed
operands are made explicit; ignored extension bits/canonicalization and relative
transfers retain exact bytes with decoded comments where needed. A/F-line exception
opwords remain 68000 traps, not later-family coprocessor instructions. Generation
shares one tool deadline and publishes only an exact rebuild; verification rebuilds
objects. Tool evaluation and limits are in docs/genesis-evaluation.md and genesis.md.

New Genesis agent/workspace/classification/commentary/archive/reference-correction/
native-port profiles preserve structural versus reachability, whole-ROM
classification, full current-source readability and editorial gates. Reading copies
include separate Z80 source units. Existing game workspaces are not rewritten.
Archives reconstruct local original bytes from an independently supplied ROM;
preparation initializes pending behavior accounting and a native scaffold, not a
ported game. Generated defaults use Genesis buttons, 320x224 and NTSC cadence.

Independent MIT romatlas-port::genesis helpers implement packed tiles, 9-bit CRAM,
planes A/B/window, scrolling, palettes/priority/transparency/backdrop, H32/H40,
interlace patterns, linked/column-major/flipped sprites, line limits/collision,
shadow/highlight and explicit VDP port/DMA helpers. Original six-channel four-operator
YM2612-oriented synthesis supplies eight algorithms, feedback, envelopes, LFO,
special frequencies, timers/status, SSG/CSM-style controls, stereo and DAC, combined
with the independent clocked PSG and fractional sample clocks. Source scheduling
and drivers remain game-owned. Exact bus/FIFO/raster quirks, Yamaha tables/slot
phase, analog filters/ladder and hardware fidelity remain documented limits; no GPL
emulator implementation was copied into the native library.

Validation passes on Rust 1.90.0: locked release build; workspace format, release
all-target Clippy with warnings denied and locked release workspace tests; native
runtime desktop-feature Clippy/tests without devices; focused Genesis structural
and native-helper tests. Synthetic Genesis integration passes dual-CPU execution,
input, deterministic snapshot/trace/frame replay, audio, backing edits, immutable
I/O, zero-detail contextual traces, bank writes, pointers, rejected annotation/
snapshot edits, stale-object rebuilding, main instruction families and Z80 source
reconstruction, prefix/swapped/SMD/5 MiB SSF layouts, generated workspace/commentary,
archive restoration, evidence-linked reference corrections and locked headless/
desktop scaffold checks. These are toolkit behavior checks, not broad game fidelity.

Existing Game Boy session, deterministic stock four-bank portability, disassembly,
commentary, archive, native-port accounting/build, SMS and GG shared 1,792-encoding
regressions, NES, production SNES, pinned sampled SNES CPU vectors, licensed
homebrew, 89-case hardware and dependency-boundary checks pass. Python compilation
and git diff checks pass. Only generated debug build artifacts were cleared to
retain disk space; all generated fixtures, source trees, captures, snapshots and
native workspaces remain local. No commercial ROM or extracted game assets were
downloaded or committed, and no display/audio device was initialized. Edits remain
uncommitted. Detailed platform/helper limits are in docs/genesis.md and
docs/genesis-native.md.

Genesis debugger state also includes immutable decoded VDP register groups and
PSG tone/attenuation/noise/LFSR values, plus detached YM2612 status and explicit
last-write shadow metadata for its write-only registers. The shadow remains in
snapshots; it is not presented as hardware readback.
Final Genesis follow-up: the complete synthetic gate also passes PAL region
selection/cadence with a Mega Drive header, while retaining jgenesis's documented
Genesis-brand header precedence. Latest locked release build, all-target workspace
Clippy, workspace tests, focused adapter tests, native desktop-feature Clippy/tests,
Genesis workflow/reference/native-scaffold regression and repeated SMS/GG gates
pass. Final format, Python compilation, boundary and diff checks pass. Native
scaffolds use the shared 64-index tile interface; no devices were initialized.

2026-10-20: Expanded Game Boy Color support on the existing MIT Mizu/gb platform;
no emulator migration, new map format or assembler dependency. Added immutable
64-byte CGB BG/OBJ palette backing spaces to execution capabilities/inspection;
CPU port locks and index side effects remain separate. Cartridge title bytes are
lossy display metadata rather than a UTF-8 execution gate. LCD snapshots now retain
both completed/partial buffers and their selection/position (core snapshot version
9, under the existing same-build snapshot contract). The session regression checks
version-8 rejection and transactional recovery; public RPC signatures are unchanged.

Independent MIT romatlas-port::gb::cgb adds palette RAM/index behavior and a banked
native-color BG/window/object compositor: signed/unsigned tile addressing, palette/
bank/flip attributes, scroll wrap, explicit window-line state, 8x8/8x16 objects,
ten-object selection including hidden entries, OAM/coordinate priority and CGB
BG/OBJ masking. Raster scheduling, window triggering and gameplay remain port-owned.
TickRate::CGB preserves the normal LCD cadence; existing DMG graphics, sound and
all port helper signatures/default behavior remain unchanged. Shared APU synthesis
uses normal-speed oscillator ticks; revision-specific CGB wave/bus/analog quirks
and FIFO/dot-exact native rendering are documented limits, not completed claims.

New mapping/native workspaces link docs/gbc.md while retaining platform gb,
stock .sym/RGBDS, archive identity and every classification/reachability/current-
source readability/editorial gate. Existing workspaces are not rewritten.
The new test-gbc.py is in CI and builds only an authored synthetic cartridge.
It passes CGB-only automatic selection/rejected DMG override, CGB-enhanced explicit
DMG/reset behavior and cross-model snapshot rejection; bank-2 execution coverage/
edges, MBC5 switching, banked VRAM/WRAM, actual DMA, speed switching, rendered color,
immutable palette inspection and exact immediate/mid-frame snapshot replay.
CGB-enhanced and CGB-only images rebuild byte-identically with both toolkit and
independent stock-recipient .sym/RGBDS paths including code banks and RAM labels.
CGB mapping/commentary/archive/native preparation and locked scaffold checks pass.

Validation passes on Rust 1.90.0: locked release build, workspace format/all-target
release Clippy with warnings denied and locked release workspace tests; focused
CGB helper tests, native desktop-feature Clippy/tests without devices, standalone
Mizu tests (nine pass), session, deterministic four-bank stock portability, fidelity,
API, disassembly, commentary, archive, native-port accounting/headless/desktop,
dependency boundaries, setup-homebrew-tests.sh and both licensed homebrew input/
replay/exact-rebuild regressions. Hardware gates pass 89/89 default and 55/55 selected
CGB cases. Final Python compilation, core/workspace formatting and git diff checks
pass. These measurements do not establish broad commercial-game or full hardware
compatibility. Generated synthetic images, snapshots, captures, source and native
workspaces remain local; no commercial ROMs or extracted assets were downloaded
or committed, and no display/audio device was initialized. Changes are uncommitted.

2026-10-20: Added TurboGrafx-16 / PC Engine HuCard support alongside all existing
platforms. The debugger uses an isolated GPLv3 jgenesis v0.13.0 commit
94bc43c8a375fc83be121658a0c520e6fd0999fb in vendor/jgenesis-pce. Existing
instrumented platform cores remain unchanged. The Rust alternative evaluated
currently documents missing PSG audio, so no emulator replacement was proposed.
Pinned ca65/ld65 supplies assembly; bounded HuC6280 decoding reuses the selected
core's mnemonic table after evaluating the existing Go huc6280disasm and WLA-DX.
See docs/pce-evaluation.md for selection and attribution.

The adapter provides actual accepted-fetch/bus/interrupt/backing/DMA observations,
MPR-aware coverage/contexts/edges and bounded JSONL streaming, operation/master-
cycle/frame stepping, register/MPR/backing edits, immutable mapped/physical peeks,
regional pad input, reset, guarded snapshots, framebuffer and stereo WAV capture.
Instruction accounting excludes interrupt entry and incremental block-transfer
continuation. Original-file identity includes the optional copier prefix; mirrored
runtime expansion stays separate. HuCards have no universal cartridge header or
mapper field: inspection explicitly reports size/CRC-based identification basis.
384/512 KiB chip aliases and incomplete-chip mirroring are explicit; SRAM, I/O,
CD/unmapped windows never become immutable ROM claims. Mapper identification is
performed before runtime expansion. CPU aliases and effective bus pages remain
separate from original physical file-bank identity.

ROM-free romatlas-pce-map JSON separates physical annotations and CPU/MPR execution
views; transactional validation/import/export, references, data barriers, bounded
static/observed discovery and explicit little-endian/bank-field pointer hypotheses
are available. Unknown context remains local; targets do not prove code/execution.
ca65 generation retains exact encodings, original prefix and all unknown bytes;
verification rebuilds stale objects. Raw fidelity does not complete classification.
New workspace/commentary/archive/reference-correction/native preparation profiles
preserve structural versus reachability, whole-ROM classification, full current-
source readability and editorial gates, without rewriting existing workspaces.

Independent MIT romatlas-port::pce implements planar background/sprite tiles,
GRB333 palettes, VCE palette ports/greyscale/dot clocks/262-263 line cadence,
VDC tilemaps/scroll/priority/flips/size/CG plane masks/collision/line limits,
raster IRQ/status, explicit VRAM and SATB DMA, and clocked six-channel wavetable/
DDA/noise/LFO/stereo PSG with fractional sample clocks. Defaults are I/II/Run/
Select/directions and 256x240. Ports own raster/music scheduling and gameplay;
no GPL implementation enters the native runtime. CD, SuperGrafx, esoteric
peripherals and uncommon boards are outside scope; exact bus/raster/analog quirks
and broad hardware/game fidelity remain documented limits.

Initial validation passes: Rust 1.90 locked release builds, release all-target
workspace Clippy with warnings denied, locked release workspace tests; four focused
native helper tests and four structural/context/identity tests; synthetic PC Engine
inspect/RPC/immutable-peek/replay/trace/backing-edit/mapping checks, all 256 opcode
encodings reconstructed exactly, special chip layouts/prefixes and independent
workspace/commentary/archive/restore/native headless/desktop scaffold flows.
Existing session, stock portability, disassembly, commentary, archive, native-port
and Genesis regressions pass. Follow-up results are recorded below. All fixtures,
captures, snapshots and generated workspaces remain local; no commercial ROM or
extracted asset was downloaded or committed, and no GUI/audio device was started.

Final PC Engine follow-up: locked release build, all-target workspace release
Clippy with warnings denied, workspace release tests and native desktop-feature
Clippy/tests pass. Five adapter tests and five independent native-helper tests
pass, including invalid SF2 selectors remaining unresolved, chip backing markers,
CG plane masks, partial wide-sprite line selection and VDC register-byte/read-latch
behavior. The expanded synthetic gate passes TurboGrafx/PCE regional input/reset/
cross-region snapshot rejection, real PSG/WAV output, pad polling, immutable peeks,
exact replay, block-transfer fetch accounting, bounded trace streaming, MPR
translation, explicit pointer layouts/conflicts and transactional edits. Known
SRAM/SF2 board behavior is tested using CRC-forged authored synthetic fixtures,
not commercial ROMs. All 256 opcode encodings and special chip/prefix layouts
rebuild byte-identically, including direct stock ca65/ld65 recipient reconstruction.
Agent/commentary/archive/restore/native preparation and evidence-linked reference
corrections pass while preserving the frozen reference.

Existing Game Boy session, stock portability, disassembly, commentary, archive,
native-port accounting/headless/desktop, Genesis, SMS, Game Gear, NES, production
SNES, pinned SNES CPU vectors, CGB, fidelity/API, both pinned licensed homebrew
regressions and the 89/89 Game Boy hardware gate pass. Final Python compilation,
workspace/vendor formatting, dependency boundaries and git diff checks pass.
No GUI/audio device was initialized. Generated images, source, captures, snapshots
and native workspaces remain local; no commercial ROMs or extracted assets were
downloaded or committed. Changes remain uncommitted. Detailed hardware/native
fidelity limits are documented in docs/pce.md and docs/pce-native.md.

2026-10-20: Revised disassembly agent instructions to prioritize code-versus-data
classification. Inspected repeated patterns, uniform runs and other clear non-code
regions may be unlabeled generic data, with physical bounds and a short static
rationale, without proving a consumer, schema or game meaning. Such bytes count
as classified; unknown semantics alone must not block completion. Commentary
accepts a short uncertainty note for generic data. Material code/data ambiguity
still remains unknown; lack of execution and raw rebuilding alone are insufficient.
Updated repository instructions, project contract, generated workflow/goal/phase
prompts and evidence policy consistently. Existing game workspaces were not rewritten.
Validation: scripts/test-disassemble.py, scripts/test-commentary.py and
scripts/test-portability.py pass using the existing release binary and local
synthetic ROMs; stock four-bank reconstruction remains deterministic and exact.
No Rust or session implementation changed.

2026-10-20: Revised gameplay exploration instructions in repository guidance,
project contract and generated disassembly/redirect/goal/workflow prompts, and
applied the same policy to the existing /root/rygar instruction files. Basic
gameplay now means deliberate broad progression through later areas, distinct
mechanics, bosses, transitions and ending screens, using proactive documented RAM
shortcuts when manual progression is slow or difficult. A progression checklist
tracks actual visits and concrete obstacles; arbitrary RAM brute force and exhaustive
collision/state permutations are excluded. Actual fetches after edits establish
execution in an artificial scenario, without claiming natural reachability.
Validation: Python compilation, disassembly workspace integration, commentary
regression, deterministic stock four-bank portability and git diff checks pass.
No runtime, maps, ROM bytes, assembly or archived Rygar records were changed.

2026-10-20: Clarified generic-data fallback ordering throughout repository
instructions, generated workflow/phase/evidence prompts and active /root/rygar
instructions. Agents must first inspect available references/consumers, nearby
code/labels and obvious formats, use supported specific roles and meaningful names
when justified, and briefly record unresolved questions before falling back. Prior
sound inspection may be reused; exhaustive semantic research remains unnecessary
and unknown meaning alone still does not block classification. Validation passes:
Python compilation, disassembly workspace integration, commentary regression,
stock deterministic four-bank portability and git diff checks. Only instructions
and this progress record changed; no ROM, mapping or assembly edits were made.
