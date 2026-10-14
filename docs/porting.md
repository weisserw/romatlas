# Native port workflow

The port phase starts from a reviewed archive and a matching local ROM, creating
a separate development workspace. It prints an agent goal to implement the whole
game in readable native Rust. It does not execute the goal or translate the game
automatically. The initial scaffold supports Game Boy archives. The shared
library can add other explicit console modules later without imposing a universal
hardware model.

## Prepare and start

Run `bash scripts/setup.sh` on Ubuntu/Debian and `cargo build --locked` in ROMAtlas.
Setup installs SDL2 development libraries as well as the existing pinned tools.

```sh
python3 scripts/port.py prepare /path/to/archive --rom /path/to/owned.gb --out /root/rom-ports/game
codex --enable goals -C /root/rom-ports/game
```

Paste the printed first `/goal ...` into Codex. Later sessions use the printed
resume goal. `--json` suppresses human instructions; preparation results are JSON
on stdout and instructions are on stderr. Use `--binary PATH` to select a toolkit
binary. Existing destinations and repository-local workspaces are rejected.
Preparation requires an existing archive; it does not alter the archive or require
phase-one classification to be complete. It validates archive hashes, restores
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
| `prompts/start.md`, `resume.md`, `review.md` | Whole-game objectives and separate reviewer instructions. |
| `review/toolkit-before/`, `toolkit-before.json` | Initial toolkit files, hashes, Git HEAD and pre-existing status. |
| `library-changes.md` | Shared changes, requirements, rationale, validation and unresolved findings. |
| `progress.md`, `differences.md`, `README.md` | Progress/blockers, behavioral differences and editor instructions. |
| `checks/` | Local build/test logs and accounting audit results. |
| `examples/asset-extension.json` | Illustrative replacement-plus-new-slot manifest, without original assets. |

The scaffold uses an arbitrary first-ROM tile solely to demonstrate loading. It
implements no game rules and defaults to silent audio. A successfully built or
headlessly run scaffold is not a completed port. Its sample manifest does not
implement a real game's level loader. The port agent must replace both as needed.

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
buttons and left-stick alternatives. Ports may supply different default actions.
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
python3 run.py headless --frames 120
python3 run.py check
python3 run.py audit
```

`check` runs formatting, clippy with the desktop feature, headless tests and a
desktop build, all with the generated lockfile where applicable. Output is in
`checks/latest.log`, with structured results in `checks/latest.json`. A failed
check does not leave a success result. `desktop` requires a graphical session and
is an explicit user command; it is rejected on Linux without DISPLAY/WAYLAND_DISPLAY.
Do not create tests requiring displays/GPUs/controllers unavailable on this server.
Normal useful tests cover game behavior and CPU-side utilities instead.

For each original region and unit, set ledger disposition to `implemented`,
`merged`, `replaced` or `excluded`, give a summary and list equivalents (except
exclusions). Preserve the original identity/source/ranges. Add discovered units
with unique IDs and valid physical ROM ranges. All gameplay-affecting source
behavior must have an equivalent; modern control flow/framework features may
replace entire original routines without a line-to-line translation.

The agent judges obvious irrelevant/unreachable code from the commented source
without formal proof; favor implementing ambiguous cases. Complete classification
and exhaustive frame matching are not gates. Useful passing tests, documented
differences, finished accounting, editor instructions and a handoff are required.
Set ledger completion status/notes and run `audit`. Audit validates inventory,
dispositions and builds; it cannot assess semantic correctness or test quality.
Missing desktop runtime validation is recorded but does not alone block completion.

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
