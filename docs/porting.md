# Native port workflow

The port phase starts from a reviewed archive and a matching local ROM, creating
a separate development workspace. It writes goal.md to direct implementation of the
whole game as maintainable best-effort native Rust. It does not launch an agent or translate the game
automatically. Game Boy/Color, SMS, Game Gear, NES, SNES, Genesis and PC Engine
archives select independent platform scaffolds.
The shared library keeps platform helpers separate without imposing a universal
hardware model. SNES helpers and current limits are documented in [snes.md](snes.md).

## Prepare and start

Run `bash scripts/setup.sh` on Ubuntu/Debian and `cargo build --locked --release` in ROMAtlas.
Setup installs SDL2 development libraries as well as the existing pinned tools.

```sh
python3 scripts/port.py prepare /path/to/archive --rom /path/to/owned.gb --out /root/rom-ports/game
codex -C /root/rom-ports/game
```

Ask the agent to read the generated `goal.md`. It coordinates a fixed pipeline
of sequential subagents:

1. Skeleton (`prompts/skeleton.md`): ROM loading, settings, desktop/browser
   integration, actor/state/level/audio/asset identifiers and semantic headless
   debug/checkpoint groundwork. No gameplay, title or game-menu implementation.
2. Behavior planning (`prompts/plan.md`): inventory all source behavior with
   references in behaviors.md and refine the source-informed architecture.
3. Behavior contract (`prompts/contract.md`): coordinate a separate worker per
   behavior area. Independently trace source, cross-call side effects and relevant
   CPU/flag/branch details, then write executable unit tests with adjacent source
   evidence. Create logical source files/interfaces; pending behavior may fail
   assertions but tests must compile and be discovered.
4. Game implementation (`prompts/start.md`): implement the whole game in those
   files against source and contracts until all tests and implementation checks/
   accounting pass. Every system supports the semantic debug API.
5. QA (`debug/goal.md`): run only a short smoke test and the initial exact-frame
   comparisons, using bounded representative checks and validated setup. Repair
   failed/affected checks in place; no whole-game QA passes follow.

Each stage hands off before the next starts. Contract workers run sequentially;
only one agent edits the workspace at a time. Stage/worker status and resumption
live in progress.md; design/state ownership live in architecture.md; behavior/
source/trace/test/implementation links live in behaviors.md. Newly discovered
scope updates affected artifacts within the active stage, without a milestone
loop. Preparation launches no agents and does not migrate existing workspaces.
Standalone QA remains available by invoking **Read debug/goal.md.**
No goal command is printed. `--json` suppresses human instructions; preparation
results are JSON on stdout and instructions are on stderr. Use `--binary PATH` to select a toolkit
binary. Existing destinations and repository-local workspaces are rejected.
Preparation requires an existing archive; it accepts incomplete editorial review
and phase-1 classification, without altering the archive. It validates archive hashes, restores
exact commented source and verifies a byte-identical rebuild before publishing
the workspace. Failed validation leaves no final workspace.

The toolkit and its source must remain at the configured path. Scaffold
dependencies are locked offline during preparation, using packages already
resolved by toolkit setup/build. If preparation cannot find a dependency, build
the toolkit before retrying. Rust 1.90.0 is pinned in the generated project.

## Workspace contents

| Path | Purpose |
| --- | --- |
| `input/game.rom` | Matching ROM. |
| `reference/archive/` | Frozen archive metadata and final documentation. |
| `reference/source/`, `reference/reading.asm` | Frozen restored, verified source and reading copy. |
| `port/` | Editable Cargo project with locked dependencies. |
| `ledger.json` | Original source regions/units and their native behavior equivalents. |
| `uncertainties.json` | Seeded Unknown intervals plus discovered semantic gaps, consumers, port impact, evidence and validation limits. |
| `research/working/` | Optional editable disassembly state, source, correction log and local evidence. |
| `research/revisions/`, `research/current.json` | Verified immutable local corrections and the adopted revision pointer. |
| `goal.md` | Overall native-port objective and resume instructions. |
| `debug/goal.md`, `DEBUGGING.md` | Short pipeline QA; standalone full-game QA and API guide. |
| `debug/acceptance.json`, `plan.json`, `qa.json` | QA acceptance contract, editable test plan and results. |
| `architecture.md`, `behaviors.md` | Source-informed design/identifiers and complete behavior-to-trace/test/implementation inventory. |
| `debug/api.md`, `debug/scenarios/` | Game-owned semantic debug schema and reusable scenarios, created by the pipeline. |
| `prompts/` | Stage prompts, shared contract/coding standards, optional reference corrections and separate library review. |
| `review/toolkit-before/`, `toolkit-before.json` | Initial toolkit files, hashes, Git HEAD and pre-existing status. |
| `library-changes.md` | Shared changes, requirements, rationale, validation and unresolved findings. |
| `progress.md`, `differences.md`, `README.md` | Progress/blockers, behavioral differences and editor instructions. |
| `checks/` | Local build/test logs and accounting audit results. |
| `examples/asset-extension.json` | Illustrative replacement-plus-new-slot manifest, without original assets. |

The scaffold uses an arbitrary first-ROM tile solely to demonstrate loading. It
implements no game rules and defaults to silent audio. A successfully built or
headlessly run scaffold is not a completed port. Its sample manifest does not
implement a real game's level loader. The port agent must replace both as needed.

## Architecture and implementation boundary

The skeleton surveys source enough to establish named identifiers, native state
and cohesive module boundaries. Behavior planning identifies all areas, modes,
systems, assets, transitions and special concerns with source references. Contract
workers reread the original independently and trace branches/call effects in
detail, writing unit tests and adjacent evidence in the source tree before game
implementation. Planning is a navigation aid, not an oracle. Source gaps and
reasonable assumptions must remain explicit.

The coding standards in `prompts/coding-standards.md` are interpolated verbatim
into the behavior-contract and game-implementation prompts. They apply to tests,
fixtures and debug adapters as well as game rules: use meaningful identifiers,
domain types, named constants and ROM definitions instead of bare semantic
numbers; comments explain rules, invariants, ordering and source quirks without
invented intent. Obvious literals and opaque table contents retain their stated
exceptions. Tests must have independent source-derived expectations and cover
each area's meaningful branches, boundaries and interactions, rather than mirror
the implementation. They must be registered for native cargo test; browser-only
module gating must not hide them.

The implementation stage completes the whole game, ROM-backed loading, asset
extensions, settings, original saves and shared desktop/browser logic. Keep
modules cohesive, ownership explicit and algorithms readable. Inspect affected
boundaries after substantial changes, then make one bounded maintainability pass
before handoff. No arbitrary module/file-size quotas or recurring redesign rounds
apply. Required infrastructure checks pass after skeleton; contract-stage tests
compile/run but can fail on pending behavior; all tests and implementation checks
must pass before QA. There is no separate implementation gameplay smoke route.

The player-facing target is the source game. Add no level selects, sound tests,
overlays, cheats or developer menus unless requested; retain source features with
their original access conditions. Required frontend settings and asset extensions
remain supported. Hardware-induced lag, slowdown and flicker need not be reproduced;
preserve intentional timing and effects such as slow motion or damage flashing.

The skeleton connects a versioned semantic observation callback to the existing
headless debug transport. Implementation exposes mode/progression/actor/timer/
audio state and relevant bounded events as needed, documenting units and retention
in debug/api.md. Inspection and rendering remain observational. Checkpoints replay
deterministic inputs/tick/audio calls, rather than serialize arbitrary objects.
Scripts validate semantic starting preconditions and transport output hashes;
code/asset changes require rebuilding checkpoints from retained setup recipes.
This debug facility is separate from player battery saves/save states.

Pipeline QA consists only of the short initial smoke/comparison suite. Check startup
and a representative interaction, plus selected exact-frame comparisons against the
original for rendering, scrolling and animation, and representative audio checks.
Document deterministic setup, input/frame alignment, expectations and reviewed
captures. Reuse valid equivalent initial evidence; rerun failed/affected checks
after repairs. Record one bounded suite execution for the QA record audit. This
establishes only the tested scope, not whole-game fidelity coverage.

Earlier debugger use answers a specific unresolved source question with a bounded
experiment and stopping condition. Document source-informed assumptions instead
of endlessly seeking confidence. Missing implementations block handoff; residual
uncertainty is recorded honestly. Whole-game accounting, passing contracts/checks,
documentation and the implementation audit precede QA. Overall completion also
requires the short smoke/comparison suite, resolved blockers and the QA audit. Device/GUI
compatibility remains untested where unavailable; never launch SDL on the server.

## Unfinished disassembly and discoveries during porting

Preparation accepts archives with incomplete editorial review or structural
classification. Unknown bytes remain present in the exact rebuilt source. They
may be unused capacity, uncertain data fields or missing logic; their size alone
does not establish porting risk. Classified code/comments can also have unresolved
meaning. Begin useful implementation without first completing the mapping goal.

Every archived Unknown interval seeds a pending entry in `uncertainties.json`.
Preserve its identity/range and add semantic ambiguities discovered in other
regions. As consumers become relevant, record context, impact, interpretations,
evidence, behavior-ledger links, useful tests and validation limits. Prioritize
control flow, pointers, bank fields, bounds and rare state transitions. Continue
independent port work while a bounded investigation is pending.

The statuses are `pending`, `resolved`, `preserved` and `excluded`; impacts are
`unknown`, `behavior`, `data` and `none`. Preserved is allowed only for data with a
supported ROM-backed consumer; copying instruction bytes does not implement native
logic. Excluded needs an explained no-impact assessment. Resolved records an implemented behavior equivalent based on source reasoning or
an explicit reasonable implementation assumption. Record its basis, alternatives
and residual source/fidelity uncertainty; resolved does not mean original-game
semantics were proven. The audit requires assessed
records and valid ledger links but cannot judge their truth. Missing required implementations prevent completion. Implemented assumptions and
unverified fidelity do not; assess them honestly rather than repeatedly seeking
confirmation. Preserving bytes cannot substitute for missing code. Original
mapping completion retains its stricter every-byte classification requirement.

Use `prompts/reference-updates.md` when a supported source correction helps
implementation. This optional procedure is not native fidelity validation;
implementation assumptions need not become source revisions:


```sh
python3 run.py reference-init
# Investigate, edit research/working/source and update its state.json through map APIs.
# Record cumulative corrections/evidence in research/working/changes.json and evidence/.
python3 run.py reference-verify --revision findings-001
```

The initializer creates a working copy once and refuses replacement. The verifier
snapshots it, exports/validates the map from its state, independently generates and
verifies structural assembly, and verifies the edited source against the owned ROM.
It retains evidence, a fresh reading copy and verification in an immutable revision,
then selects it through `research/current.json`. Failed checks publish no revision
and keep the prior selection. Subsequent source-accounting audit/reference commands check adopted artifact
hashes as well as frozen baseline hashes. Neither exact rebuilding nor adoption
proves the interpretation or map/comment consistency; the agent reviews those and
updates focused unit tests of affected native logic.

Corrections use `version: 1`, the workspace `rom_sha256` and a `changes` array.
Each record has `id`, `kind` (`commentary` or `classification`), physical `offset`
and `length`, `before`, `after`, `port_impact` and nonempty `evidence` strings. Keep
supporting files under the working `evidence/` directory. Verify coherent batches;
do not rebuild after every prose edit. Original archive/reference and ledger IDs
remain unchanged; add discovered native-ledger units and link adopted corrections
in uncertainty/behavior records. The active interpretation is the baseline plus
its explicit adopted corrections. Resumes inspect both before trusting old prose.

Keep research source/assets local. Upstream correction is an evidence-linked
handoff of the revision identity, change log, portable map and verification, not an
automatic overwrite of the original archive or concurrent disassembly workspace.
Existing port workspaces are not rewritten automatically. Older workspaces without
uncertainty tracking retain their prior audit contract; adopting new prompts alone
does not retroactively create the new ledger or frozen metadata.

## Runtime library

`crates/romatlas-port` has no emulator-adapter dependency. Its core works without
SDL2 initialization. `desktop` is an optional Cargo feature. Existing shared
analysis/session crates retain their platform boundaries.

- `assets::Rom` validates SHA-256 and provides immutable bounded physical reads.
  ROMs are capped at 64 MiB. Individual override assets are capped at 16 MiB,
  collections at 64 MiB and JSON manifests/settings at 1 MiB.
- `assets::Collection::load` loads named contiguous ROM ranges and applies files.
  `Collection::with_overrides` accepts game-owned decoded defaults, allowing
  custom decompression/noncontiguous recipes to produce editor-friendly formats
  at startup. `Origin::DerivedFromRom` identifies these recipes. Validation is
  game-owned and can inspect the asset origin when schemas differ.
- `timing::Clock` uses rational rates, with `TickRate::DMG` equal to
  4194304/70224 Hz. Catch-up is bounded and dropped backlog is reported. Reset
  discards pause debt. Host monitor refresh never defines game speed.
- `graphics` provides RGBA buffers, clipped/flipped tiles, transparent indices and
  tile maps. `gb` provides 2bpp decoding and DMG/CGB palette conversion. It does
  not emulate VRAM access locks, scanline timing or complete PPU priority rules.
- `audio` supplies exact per-tick sample budgets and a basic pulse oscillator.
  `Game::audio` produces stereo PCM; the desktop shell handles playback. This is
  not an accurate APU or universal sound-driver interpreter. Game-specific
  commands, compression and hardware effects require port implementation.
- `input` maps physical keys/buttons/axes to named game actions, tracks edges and
  suppresses held controls across focus/menu/controller transitions until neutral.
- `settings` supplies bounded validated persistence and device-independent menu
  logic. The optional `desktop` module presents that menu using an original tiny
  font, with no ROM-derived font dependency.

The `Game` trait separates `tick`, `render` and `audio`. `headless` advances the
same game logic with scripted inputs and returns an in-memory frame. Games own
their state and may expose richer test inspection. Test audio generation directly
without a playback device. No general console emulator or universal sound/asset
decoder is part of this library.

Ports can keep game rules, unusual effects and uncertain abstractions locally.
An agent may add shared helpers when it can describe a reusable contract, including
explicit platform-specific contracts. Two implemented consumers are not required.
Shared utilities should not encode one game's rules under generic names.

## Assets and added content

Every original visual/audio asset and gameplay data structure must be obtained
from the user's ROM at startup. A port must not embed extracted tiles, maps,
strings, sound data or other original assets into source constants. Extraction
recipes and game rules are code; extracted originals are loaded into memory/local
caches.

Use one manifest per collection:

```json
{
  "version": 1,
  "entries": [
    {"id": "level-01", "file": "levels/replacement-01.json"},
    {"id": "level-16", "file": "levels/new-16.json"}
  ],
  "order": []
}
```

Existing IDs replace defaults in place; new IDs append in manifest order. An
empty `order` retains every original slot then appends additions. An explicit
`order` must name every resulting asset exactly once. With fifteen default levels,
this example keeps original levels 2–15, replaces level 1 and appends level 16.
The game must iterate the resulting collection instead of hard-coding fifteen.

File paths must stay within the override directory, including after symlink
resolution. Duplicate IDs, invalid formats, missing files and inconsistent order
are errors. The game documents and validates the supported schema. For compressed
original levels, decode into a modern schema at startup before applying files
with `with_overrides`; editors need not author the original compressed format.
No graphical editor or arbitrary-format modding system is supplied.

## Player settings and desktop backend

Preparation also creates BUILDING.md from scripts/port-template/BUILDING.md,
substituting the port's name/executable and initial toolkit path. The generated
README links to this guide. It covers private macOS/Linux/Windows source builds,
SDL prerequisites/linking, source transfers excluding target/, path relocation,
headless builds and later packaging. Its Android section covers toolchain setup,
a Rust cross-target check, required SDL2/Activity/native-library integration, ROM
selection, mobile lifecycle/input, and debug APK build/install commands after
integration. Android app support is not generated or validated by preparation.
The coordinator prompt requires the port agent
to adapt it to final dependencies/flags and record actual build/unit-test results.
The baseline recipes are not platform-test evidence and do not authorize
distribution. Existing source ports can adopt the guide without regeneration;
preserve their source and edited library when adding later build automation.

SDL2 provides window operations, keyboard/controller input, 2D presentation and
audio queues. The shell uses desktop fullscreen, resizable windows, nearest-neighbor
aspect-preserving presentation with free scaling and no integer scaling setting. Fullscreen uses the
desktop's display mode; configured window dimensions are restored on leaving it.
Internal game resolution is separate from window size.

F6 or an available controller Guide button opens/closes settings. Escape closes
the menu or cancels rebinding. Arrows/D-pad select options; Enter/A accept; left/right
adjust. Backspace/X clears an action's bindings; accepting that action captures an
additional key/button/axis binding. Game actions can have multiple alternatives.
F1–F6 and Guide are shell controls. The Guide button may be intercepted by the OS;
keyboard F6 remains available. The menu includes four independent player assignments
(Automatic, None or a device), dead zones, height/fullscreen, volume, focus-loss
pause and reset-to-defaults. Width follows the game display aspect. Raw SDL
joysticks are selectable too: buttons use buttonN, axes use axisN, first-stick
axes retain leftx/lefty aliases and hats map to D-pad directions. Capture reads
all connected devices, including unassigned devices, and waits for held controls
to return to neutral.

Settings are per-port, under the user configuration directory (`APPDATA` on
Windows, `~/Library/Application Support` on macOS, and `XDG_CONFIG_HOME` or
`~/.config` on Linux). `--settings FILE` selects a different path. The default
Game Boy actions are arrows, Z/X, Return/Backspace, with standard controller
buttons and left-stick alternatives. SNES defaults add X/Y/L/R actions, use
Z/X/A/S/Q/W keyboard bindings and ordinary SNES controller layout; its scaffold
uses 256x224 and nominal NTSC cadence. Ports may supply different default actions.
Controller selection uses an SDL GUID plus an ordinal among identical models,
allowing selection of two controllers sharing a GUID. Ordinals follow connection
enumeration and may change when devices reconnect; they are not serial identities.

Focus loss pauses by default and held controls clear/rearm across transitions.
The simulation catch-up limit is eight ticks in the initial shell. Game-specific
policies can evolve through library changes. Missing audio devices are reported;
the shell can continue without playback. Physical-device/desktop behavior is not
established by compiling this backend.

## Check, finish and review

From the generated workspace:

```sh
python3 run.py instructions
python3 run.py check
python3 run.py audit
```

`check` runs formatting, clippy with the desktop feature, headless tests and a
desktop build, all with the generated lockfile where applicable. Output is in
`checks/latest.log`, with structured results in `checks/latest.json`. A failed
check does not leave a success result. `desktop` requires a graphical session and
is an explicit user command; it is rejected on Linux without DISPLAY/WAYLAND_DISPLAY.
Do not create tests requiring displays/GPUs/controllers unavailable on this server.
Build/check, instructions, headless, desktop and web commands validate the workspace
and original ROM identity without requiring frozen reference or research files.
Only audit and reference-management commands validate those development artifacts;
they are not runtime assets. Missing checks/ output directories are recreated.
Source-backed unit contracts cover game rules, meaningful branches and system
interfaces. The optional headless command alone does not establish QA completion;
the final stage executes reusable scenarios through the device-free debug API.

For each original region and unit, set ledger disposition to `implemented`,
`merged`, `replaced` or `excluded`, give a summary and list equivalents (except
exclusions). Preserve the original identity/source/ranges. Add discovered units
with unique IDs and valid physical ROM ranges. All gameplay-affecting source
behavior must have an equivalent; modern control flow/framework features may
replace entire original routines without a line-to-line translation.

The agent judges obvious irrelevant paths from source, favoring implementation
of ambiguous cases. Complete original-ROM classification is not a porting gate.
A coherent architecture/inventory, passing source-backed contracts, complete
behavior implementations, documentation and a handoff are required. Assess the
uncertainty ledger too; preserve original entries, assess pending gaps and never
preserve unknown code in place of implementation. Documented implemented
assumptions may be assessed while residual source/fidelity uncertainty stays
explicit. Set ledger completion status/notes and run `audit`. It validates
inventory, dispositions and builds, not semantic correctness or test quality.
After this audit, the implementation subagent hands off to final short smoke/comparison QA;
overall completion requires its successful record audit too. Rerun build/static
checks after pertinent changes/failures, rather than merely for confidence.

The agent may edit the shared library directly in ROMAtlas, uncommitted, logging
the actual changes and validation and preserving pre-existing work. It runs the
toolkit's relevant checks and reports actual results and limitations. Use
`prompts/review.md` to launch a separate reviewer manually. The initial snapshot
allows comparison against the starting files, including pre-existing dirty files.
The user decides check-in. A path dependency is editable; Cargo.lock does not
freeze it. Immutable Git pinning and release packaging occur in a later step.

## Framework decision references

SDL2 was chosen for direct controller/audio/window primitives and a small custom
shell around headlessly testable code. Macroquad's input documentation currently
describes gamepads as forthcoming; ggez provides another viable combination.
SDL2 adds native linking/packaging work. No choice establishes desktop compatibility
without testing it. Dependencies are pinned and checked with Rust 1.90.0 here.

- [Rust SDL2](https://docs.rs/sdl2/latest/sdl2/)
- [Rust SDL2 build/link options](https://github.com/rust-sdl2/rust-sdl2)
- [Macroquad input](https://docs.rs/macroquad/latest/macroquad/input/)
- [ggez gamepads](https://docs.rs/ggez/latest/ggez/input/gamepad/)
- [Cargo dependencies](https://doc.rust-lang.org/cargo/reference/specifying-dependencies.html)

SMS archives use JSON maps, WLA-DX source recipes and a 256x192 / SMS_NTSC
native scaffold with SMS button names and four-plane tile decoding. Reference
corrections follow the existing exact-rebuild workflow. See [sms.md](sms.md) and
[sms-native.md](sms-native.md) for address rules, helper APIs and precision limits.

Game Gear archives use romatlas-gg-map JSON and the shared Sega WLA-DX recipes.
Native scaffolds select 160x144 / GG_NTSC, button1/button2/start and four-plane
tiles. Runtime helpers add 12-bit CRAM, the Mode 4 LCD viewport and stereo PSG.
Reference corrections preserve exact reconstruction and the frozen reference.
See [gg.md](gg.md) and [gg-native.md](gg-native.md).

Game Boy Color uses the existing GB archive/platform and adds `gb::cgb` banked BG/window/object rendering and palette RAM helpers without changing DMG APIs. See [gbc.md](gbc.md) for contracts, scheduling and documented limits.

## Browser ports and battery saves

New workspaces include a `cdylib` browser entry point beside the desktop executable.
Game rules implement the same `Game` trait for SDL2 and Canvas/Web Audio; browser
builds do not enable SDL2. `Port::load_assets` performs shared ROM-backed startup
extraction using `MemoryAssets`; `Port::load` loads desktop override files then
calls that same constructor. Game-owned decompression and schema validation stay
in this shared path. `MemoryAssets::collection` and filesystem collections share
replacement, addition, ordering, ID, size and schema validation. The server exposes
one top-level JSON manifest per collection and only its referenced override files.

Run toolkit setup (`bash scripts/setup.sh`, Rust 1.90.0), then in a newly prepared
port workspace:

```sh
python3 run.py web-build
python3 run.py web --port 8000
python3 run.py web --assets /path/to/replacements --port 8000
```

`web-build` writes bindings, Wasm and the shared shell into local `web-dist/`.
`web` binds to `0.0.0.0:8000` by default; use `--host 127.0.0.1` for loopback
or `--host ADDRESS` to select another interface. Port 0 requests a free port.
The printed URL uses `127.0.0.1` for the wildcard bind; other machines can use
the server machine’s network address.
The server reads the configured local ROM, without copying it into the bundle,
and serves it to the browser for SHA-256 validation and in-memory extraction.
The bundle alone requires equivalent `/config.json`, `/rom` and override routes;
it is not a self-contained game distribution. No existing workspaces are migrated.

Press Start to enable audio. The shell uses Canvas RGBA frames, stereo Web Audio,
keyboard and Gamepad APIs for four independent player slots.
Settings provide rebinding, controller assignment, volume/dead zone, height and
fullscreen. Width follows config::DISPLAY_ASPECT, defaulting to 4:3 in new ports.
There is no integer scaling setting; ports can opt into a different display
aspect in config.rs without changing other games. Keyboard bindings use the shared SDL-style names;
the shell translates browser keys and controller buttons/axes, including devices
without the browser's standard mapping. First-stick axes retain the default
movement bindings; extra controls are exposed as `buttonN` and `axisN` for
rebinding. Raw layouts vary, so verify bindings in Settings. Binding capture reads
all connected controllers independently of player assignment and waits for
initially held controls to be released. Browser controller identities are separate from SDL GUIDs. Settings and assignments live
in localStorage under the ROM hash. This storage is also scoped to the browser's
origin: changing server port/browser/profile gives a separate storage location.
Keep the same origin to retain saves; clearing site storage removes them.

The browser page centers the game display and explains the initial Start click.
Pause and Settings pause gameplay; Start resumes it. Reset reloads the page and
returns to the initial Start prompt, preserving settings and original battery
saves. Focus loss and visibility changes clear held inputs and queued audio and
reset timing without pausing gameplay. Device changes also reset input/timing.
Browsers may throttle background tabs. Missed wall time is not replayed. Catch-up
is bounded to eight ticks. The desktop-browser baseline is current Chromium and
Firefox with single-threaded Wasm; Chromium is exercised automatically. Firefox
support is currently an implementation target, not a measured compatibility claim.
Touch/mobile, WASI, deployment and offline/PWA support are outside this change.

Battery persistence is optional and game-owned. For originals that saved progress,
implement `Game::save_version`, `load_save`, `save_data`, `save_dirty` and
`save_acknowledged`. Save only originally persistent data, never full live state.
A port may serialize a clean persistent representation; it need not reproduce the
original cartridge's byte layout. Validate an entire loaded payload before mutation.
Increment the format version for incompatible changes; automatic migration is not provided.
The default hooks declare no battery support, not an implemented saving feature.
Future port agents must implement original in-game saves when present.

The shared `BatterySave` envelope binds payload/version to the ROM hash and caps
payloads at 1 MiB. Mark dirty when original in-game saving changes persistent data;
keep it dirty until acknowledged. Frontends write promptly after advancement and
at pause/exit boundaries. Browser writes use localStorage (versioned ROM key);
SDL uses an atomic JSON file named `battery-<ROM hash>.json` beside settings.
Restoration happens before the first gameplay tick. Failed writes do not acknowledge
pending data; browser errors remain visible, desktop errors reach the CLI. Invalid
or incompatible browser saves stop startup rather than overwriting existing data.
This battery envelope contains no full save state and has no implicit emulator-SRAM
dependencies; player snapshots use the separate format described below.

New-workspace check/audit includes a locked release Wasm build alongside existing
desktop checks. Toolkit browser regression setup is separate:

```sh
bash scripts/setup-web-tests.sh
tools/web-tests/bin/python scripts/test-port-web.py
```

The test runs an authored synthetic ROM and native fixture through the actual
localhost server, bindings and Chromium. It checks rendering, input, audio startup,
overrides, settings, battery reload and storage failures. No display/audio device,
commercial ROM or native-versus-ROM fidelity comparison is needed. Setup pins
wasm-bindgen 0.2.129 (library/CLI match) and Playwright 1.56.0. Python handles build
and serving; no npm or JavaScript bundler is required.

## Final and standalone QA

[Native debug control and workflow](port-debugging.md) provides the headless API
and QA stage. New port goals invoke debug/goal.md after whole-game implementation.
No begin-cycle command or intermediate QA cycle is part of the fixed pipeline.
New pipeline workspaces require only the short smoke test and initial exact-frame
comparisons, recorded as one bounded suite. Separately installed standalone QA
retains its two-pass whole-game contract. Existing workspaces are not migrated.
Resource interruptions leave an incomplete resumable handoff. After completion run:

```sh
python3 scripts/port_debug.py audit WORKSPACE
```

Standalone invocation is **Read debug/goal.md.** For older workspaces without the
QA files, use `python3 scripts/port_debug.py install WORKSPACE`. Existing
instructions/records are preserved and not silently migrated. The toolkit retains
legacy cycle commands for existing workspaces. Preparation/installation never
launches agents. The transport exposes --debug-stdio with game-owned semantic
inspection supplied by the port; see debug/api.md in completed pipeline workspaces.

## Optional modernization

New preparations install `modernize/goal.md` and empty candidate/change/status
records. This is an optional separately invoked workflow, not a sixth required port stage.
Installation, port completion and reading about the workflow do not launch a pilot.
Existing workspaces can install it without starting QA or changing game code:

```sh
python3 scripts/port_modernize.py install /path/to/workspace
python3 scripts/port_modernize.py check /path/to/workspace
```

`check` verifies native source and Cargo.lock are available for baseline capture.
Full-game QA and two complete passes are not prerequisites. Available QA/smoke
records are hashed without treating them as completed acceptance. The command does
not execute QA, change records or certify gameplay truth. A later explicit
modernization request reviews available evidence and build/source/ROM/asset
provenance and runs applicable implementation checks, then captures an exclusive
baseline hash record with
`python3 scripts/port_modernize.py baseline /path/to/workspace`. This command never
launches an agent or edits gameplay and will not overwrite an existing baseline.
Keep a separate reproducible local faithful source/build snapshot; hashes alone
are not a backup. Candidates/change records remain empty until explicitly invoked.

Each modernization iteration presents a menu of up to 8 modernization targets and up to 2
refactoring targets, grounded in code inspection. Seek the full menu without
padding: first survey the major game subsystems in actual source, trace promising
alternatives and record inspected files, findings and concrete rejection/blocker
reasons in candidates.json. A few ready proposals or preliminary probes elsewhere
do not justify stopping. A shorter final menu requires an evidence-backed
explanation for each category; an unfinished investigation produces a provisional
list and next probes, not a request for selection. Each entry rates implementation
difficulty and codebase impact with reasons; entries rank by impact descending,
then difficulty ascending, with separate ranks for both dimensions. The user accepts
all or selects target IDs before implementation begins. Later iterations require
a fresh menu and selection.
Modernization targets are bounded conversions from hardware-shaped machinery to
idiomatic native game state, rules or actions. Helper extraction and deduplication
may be offered as refactoring targets, but alone are not modernization.
Refactoring improves the structure and maintainability of the existing design
while preserving its game model and observable behavior. Investigate duplicated
rules, mixed responsibilities, unclear interfaces and tangled control flow; each
candidate must explain the concrete maintenance problem, before/after structure
and benefit. Both categories preserve gameplay, and supporting refactoring within
a modernization target must not be counted twice in the menu.
Modernization candidates explain the before/after game model, removed
machinery and encodings retained at asset/runtime boundaries; selection prioritizes
substantive design improvement rather than existing test coverage or easy equivalence.
The agent identifies observable rules and affected integration tests, fills coverage
gaps, implements the selected targets and compares
fresh results against both the original and faithful baseline. Changes to opaque
hardware plumbing must retain relevant timing, ordering, layering, arithmetic and
intentional effects. Preserve semantic debug interfaces where practical. Use targeted
regressions, broadening based on dependency impact rather than automatically repeating
two complete QA passes per edit. Keep original-fidelity expectations; feature changes
and deliberately divergent behavior require separate authorization/contracts.
Retain or revert each selected target with evidence, update records and stop after
the selected scope. There is no mandated whole-game rewrite
or claim of modernizing every subsystem. Preparation does not start a pilot.

### Opt-in snapshot primitives

The `romatlas-port/snapshots` feature enables serialization of private browser
clock/input and NES APU state. `Runtime::driver_snapshot_json` captures driver
state; `restore_driver_snapshot_json` checks ROM/sample-rate/clock compatibility
before replacing it. These methods do not capture the game or its immutable
resources: an opting-in port supplies a versioned game serializer and restores
both pieces atomically. Float round trips remain exact with this feature.
New templates enable snapshots and expose Save State / Restore State in web and
SDL desktop frontends. Implement Game::snapshot_json, atomic
Game::restore_snapshot_json and Game::reset for the game, including all mutable
game/audio state; the scaffold's empty snapshot describes only its stateless demo.
The shared Runtime::restore_state_json validates driver state before invoking the
game restore. Unsupported ports retain working battery saves and report state
support as unavailable. Existing workspaces are not migrated automatically.

Both frontends use F1 resume, F2 pause, F3 reset, F4 save state, F5 restore state,
F6 settings. Desktop starts playing immediately. Restore pauses until F1; errors
retain the current game. Desktop state.json lives beside settings.json, separate
from battery files, with a 4 MiB limit, checksum, executable/ROM/override identity
and sample-clock validation. Sample phase resumes without replay; wall-clock and
held-input boundaries reset on restore. Web snapshots live in browser storage and
include game state and the driver, with Wasm/ROM/assets/audio identity and checksum.
Desktop and browser state formats are intentionally frontend-specific. Original
battery saves keep their existing format and behavior.

## Faster native development checks

Use `python3 run.py check-fast` for unoptimized incremental shared-library lint
and unit tests during edits. Add `--test-filter NAME` to execute matching tests,
or `--frontend desktop` / `--frontend web` to build only that frontend. Commands
run sequentially; avoid concurrent Cargo jobs sharing the same target directory.
Development records live in `checks/fast.*`; full optimized acceptance remains
`python3 run.py check` with its separate `checks/latest.*` records. The initial
development build is a separate cache warm-up. Existing workspaces using the
shared run.py wrapper gain the command; preparation installs explicit development
profile settings in new workspaces without migrating unrelated existing ports.
