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
