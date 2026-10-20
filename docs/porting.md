# Native port workflow

The port phase starts from a reviewed archive and a matching local ROM, creating
a separate development workspace. It writes goal.md to direct implementation of the
whole game as maintainable best-effort native Rust. It does not launch an agent or translate the game
automatically. Game Boy, SMS, Game Gear, NES and SNES archives select independent platform scaffolds.
The shared library keeps platform helpers separate without imposing a universal
hardware model. SNES helpers and current limits are documented in [snes.md](snes.md).

## Prepare and start

Run `bash scripts/setup.sh` on Ubuntu/Debian and `cargo build --locked --release` in ROMAtlas.
Setup installs SDL2 development libraries as well as the existing pinned tools.

```sh
python3 scripts/port.py prepare /path/to/archive --rom /path/to/owned.gb --out /root/rom-ports/game
codex -C /root/rom-ports/game
```

Ask the agent to read the generated `goal.md`; it includes the whole-game
objective and resume instructions. No goal command is printed. `--json` suppresses human instructions; preparation results are JSON
on stdout and instructions are on stderr. Use `--binary PATH` to select a toolkit
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
| `input/game.rom` | Local matching ROM; never distribute it. |
| `reference/archive/` | Frozen archive metadata and final documentation. |
| `reference/source/`, `reference/reading.asm` | Frozen restored, verified source and reading copy. |
| `port/` | Editable Cargo project with locked dependencies. |
| `ledger.json` | Original source regions/units and their native behavior equivalents. |
| `uncertainties.json` | Seeded Unknown intervals plus discovered semantic gaps, consumers, port impact, evidence and validation limits. |
| `research/working/` | Optional editable disassembly state, source, correction log and local evidence. |
| `research/revisions/`, `research/current.json` | Verified immutable local corrections and the adopted revision pointer. |
| `goal.md` | Overall native-port objective and resume instructions. |
| `architecture.md` | Source-informed module/state/algorithm plan, maintained during implementation. |
| `prompts/start.md`, `reference-updates.md`, `review.md` | Port contract, reference correction procedure and separate reviewer instructions. |
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

Before substantial coding, read representative startup, gameplay, rendering and
sound source paths and write a short actionable `architecture.md`: modules/files,
responsibilities, state owners, interfaces, algorithms, scheduling, shared platform
helpers and implementation order. This survey is bounded; exhaustive source
understanding and user approval are not prerequisites. Organize code around game
concepts. Keep algorithms readable and modules cohesive; avoid both a growing
monolithic game file and cosmetic splitting of tightly coupled code. No arbitrary
file/line quotas apply. Inspect affected boundaries after substantial features,
fix concrete design problems and update the plan when decisions change. Finish
with one bounded maintainability pass, not repeated redesign/audit cycles.

The native goal implements the whole game on a source-informed best-effort basis.
Focused unit tests check intended logic; formatting, static checks and compilation
complete the required implementation checks. **Runtime fidelity validation is
outside this porting goal.** Do not compare native frames, animation, screenshots,
audio, state trajectories or gameplay scenarios against emulator/ROM execution.
Do not create exhaustive scenario matrices, confidence-building replay loops,
playthrough requirements or device tests as completion criteria. A later fidelity
or device audit requires an explicit user request.

Debugger use is for a specific unresolved source-understanding question that
static inspection cannot reasonably answer. Record the needed observation and
stopping condition, perform a bounded experiment and return to implementation.
Stop at that bound even if unanswered; do not repeat inconclusive runs without
new evidence.
If inconclusive, choose and document a reasonable source-informed implementation
assumption where possible. Do not reopen the question without new contradictory
evidence. Lack of similarity evidence or an untested screen is not such a question.
Once whole-game implementation/accounting and a coherent architecture are ready,
required checks pass and the handoff is written, run the administrative audit and
stop when it passes. Assume the implemented/unit-tested behavior works for this
handoff without claiming independent fidelity. More time/budget is not a reason
to expand acceptance criteria or repeat passing checks.

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
and keep the prior selection. Subsequent workspace commands check adopted artifact
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
recipes and game rules are code; extracted originals remain in memory/local
caches and out of distributions. Developer-created assets may ship.

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
aspect-preserving presentation, and optional integer scaling. Fullscreen uses the
desktop's display mode; configured window dimensions are restored on leaving it.
Internal game resolution is separate from window size.

F1 or an available controller Guide button opens/closes settings. Escape closes
the menu or cancels rebinding. Arrows/D-pad select options; Enter/A accept; left/right
adjust. Backspace/X clears an action's bindings; accepting that action captures an
additional key/button/axis binding. Game actions can have multiple alternatives.
F1 and Guide are shell controls. The Guide button may be intercepted by the OS;
keyboard F1 remains available. The menu includes controller selection, dead zones,
window size/fullscreen/scaling, volume, focus-loss pause and reset-to-defaults.

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
Focused unit tests cover intended game logic and CPU-side utilities. The optional
headless command remains available for later requested work; executing it is not
a required validation phase.

For each original region and unit, set ledger disposition to `implemented`,
`merged`, `replaced` or `excluded`, give a summary and list equivalents (except
exclusions). Preserve the original identity/source/ranges. Add discovered units
with unique IDs and valid physical ROM ranges. All gameplay-affecting source
behavior must have an equivalent; modern control flow/framework features may
replace entire original routines without a line-to-line translation.

The agent judges obvious irrelevant/unreachable code from the commented source
without formal proof; favor implementing ambiguous cases. Complete classification
is not a gate, and runtime fidelity comparisons are outside the goal. A coherent
architecture/plan, focused passing unit tests, documented assumptions/differences,
finished accounting, editor instructions and a handoff are required.
Assess the uncertainty ledger too; new workspaces cannot finish with pending gaps,
removed original uncertainty entries or preserved unknown behavior. Documented
implemented assumptions can be assessed without fidelity proof. Set ledger
completion status/notes and run `audit`. Audit validates inventory,
dispositions and builds; it cannot assess semantic correctness or test quality.
Unassessed gameplay/audiovisual fidelity and device behavior are recorded without
blocking completion or creating another investigation phase. After a successful
administrative audit, stop. Rerun checks only after relevant edits, failures or a
concrete new defect.

The agent may edit the shared library directly in ROMAtlas, uncommitted, logging
the actual changes and validation and preserving pre-existing work. It runs the
toolkit's relevant checks and updates docs/progress.md with tested facts. Use
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
