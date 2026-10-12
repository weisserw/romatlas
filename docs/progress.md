# Milestone checklist

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
