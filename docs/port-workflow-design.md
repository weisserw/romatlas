# Port workflow: confirmed design

This document consolidates the confirmed port-workflow interview. It specifies a new phase
beyond the original mapping scope in project.md. It does not establish that any
port workflow or runtime library has been implemented or tested.

## Purpose and initial scope

Prepare a separate per-game workspace from an archived game and its matching,
locally supplied ROM. Write a goal.md coordinator prompt directing an agent to
port the entire game into readable Rust. Do not launch the agent automatically.
The result is a native reimplementation of game behavior, not execution of the
original instructions inside an emulator.

Begin with an educated starting library and workflow. The user will run the
workflow manually and evaluate it; completing a pilot game is not a prerequisite
for delivering the workflow. Rust is the initial supported language. Python
bindings and additional language targets are outside the initial scope.

Target Windows, Linux and macOS desktops. Verify what the available headless
Linux environment permits and explicitly report other platforms and desktop
runtime behavior as untested. Release packaging and library publication are
separate later work, not prerequisites for a completed development port.

## Preparation and workspace

Initialization accepts an archive, a matching local ROM and a new output
directory. Validate identity and restore the archive's commented source using
the existing restoration contract. Preserve the archive and create generated
material outside the ROMAtlas repository. Reject an existing destination.

Create a Rust port scaffold, commented-source reference, source-behavior ledger,
library-change log, development instructions, a coordinator prompt and a separate
library-review prompt. Keep ROMs, regenerated source and decoded assets local.
Never download or commit commercial ROMs or extracted game assets.

Archives with incomplete structural classification are eligible. Carry relevant
unknowns into the port ledger and investigate gaps that could affect gameplay.
An archived editorial review is a useful starting point, not proof of semantic
correctness or complete gameplay understanding.

## Completion and fidelity

The agent pursues the whole game, not merely a scaffold, one level or a runnable
demo. Account for all archived source routines and meaningful data structures
with dispositions such as implemented, merged, replaced by modern facilities,
or excluded with an explanation. Entries identify port equivalents and the
behavior they preserve; no instruction-by-instruction correspondence is needed.

Every source behavior affecting gameplay must have an equivalent. Modern
techniques may combine routines or make original mechanisms unnecessary.
The agent judges relevance from the commented source. Obviously irrelevant
junk, unreachable elements and non-gameplay paths may be excluded without a
formal reachability proof. Favor implementing borderline or unclear behavior.
Unvisited execution alone does not explain an exclusion.

Completion requires no unexplained ledger entries, implemented gameplay
equivalents, a useful normal amount of passing tests, and a documented handoff.
Document known behavioral differences. Exact frame compatibility is not a
requirement and cannot be promised for all original hardware tricks. Missing
desktop runtime validation on this server is reported and does not itself block
completion. The ledger cannot mechanically establish semantic correctness.

If blocked, preserve progress and identify concrete unresolved behaviors,
attempts and what would unblock the work. Do not mark an incomplete port complete
or stop at an intermediate milestone while necessary work remains achievable.

## Shared library and development loop

Use one Rust library with common utilities and explicit platform-specific
modules. Implement an initial Game Boy set. Future modules could include SNES
Mode 7 rendering; do not implement other consoles merely to demonstrate the
structure. Preserve existing platform-neutral dependency boundaries.

Initial facilities:

- ROM identity checks and bounded byte access.
- Named asset collections, ROM-backed loaders, file overrides and extension
  manifests.
- Fixed simulation timing, logical input state and headless execution helpers.
- Game Boy tile/palette decoding and basic tile/sprite composition.
- An audio interface separating sound generation from playback.
- Shared player settings, input mapping and reusable settings-menu behavior.

The library supplies composable effects rather than a complete second console
emulator. Ports may update tile storage, palettes and timed effects through
explicit APIs, or use modern rendering directly when clearer. Add register-like
interfaces only when they meaningfully simplify a particular effect. Unusual
hardware behavior and game rules may remain hand-coded in the port.

Game-specific decompression, music-command interpretation and sound-driver
behavior initially belong to the port. ROM audio is not necessarily stored as
ready-to-play sound files. There is no universal audio-extraction requirement.

The port agent may implement needed shared-library changes directly in the
ROMAtlas directory, leaving them uncommitted. Record the starting repository
state, distinguish pre-existing changes, and log each port-driven change with
its requirement, interface rationale and validation. Preserve unrelated work.

Reusable mechanisms belong in the library; game rules belong in the port. Each
shared feature needs a useful game-independent contract, but does not need two
existing consumers. Keep uncertain abstractions local until they become clear.
An issue log records unresolved findings; it does not replace implementing
necessary library work during port development.

The user launches a separate review agent using the supplied prompt. Review the
actual diff, requirements and test evidence before check-in. The port agent does
not check in its own shared-library changes. Review findings may require fixes.

During development use a local Cargo path dependency on the library. Record
library provenance and edits; Cargo.lock does not freeze local source files.
After user review and check-in, a later distribution step can pin the required
Git commit. Do not require publishing library packages now or claim that an
uncommitted local dependency is an immutable release reference.

## ROM-backed assets and extension

At startup require the user's matching ROM, validate its identity and obtain
original visual/audio assets and gameplay data, including maps, through loader
recipes. Distributable code must not embed extracted originals as constants or
files. Original decoded assets may exist in memory or local caches, which remain
outside distributions. Developer-created content may be distributed.

Expose named collections with stable logical slots, ROM-backed defaults,
validated file replacements and additional entries. A manifest identifies
entries, ordering and supported formats. Replacement assets obey the port's
supported schemas; arbitrary new formats or dimensions are not promised.

Extensions must be practical: a developer can retain fifteen ROM-backed levels
and append level sixteen as a file. The playable sequence comes from the
collection rather than a fixed original level count. Other collections define
appropriate extension rules. Broader modding systems and graphical editors are
outside the initial scope.

Include concise instructions for replacing assets, adding a level, finding game
rules and running the port, plus an example extension manifest containing no
original assets.

## Desktop backend and customization

Use SDL2 through Rust bindings as the initial desktop backend for keyboard and
controller input, audio playback and 2D presentation. Keep simulation and core
utilities independent of desktop initialization. Verify compatibility with
Rust 1.90.0 during implementation and pin compatible dependencies with lockfiles.

Framework comparison informed this choice: Macroquad has a small API but its
current input documentation still describes gamepads as forthcoming; ggez offers
controller and display facilities; SDL2 directly exposes the required device,
audio and window operations. SDL2 brings native build/linking and later packaging
work. This is a documented-fit decision, not completed cross-platform testing.

Implement per-port persisted settings for keyboard/gamepad bindings, controller
selection, stick dead zones, window size, fullscreen/windowed mode,
aspect-preserving scaling and audio volume. Map physical inputs to named game
actions. Keep original internal rendering resolution distinct from window size.

Provide an in-game settings menu with interactive rebinding, cancellation,
reset-to-defaults and sensible controller-disconnection handling. SDL2 provides
the device/window primitives; the shared library supplies settings, persistence
and reusable menu behavior.

Pause on focus loss by default, clear held inputs on focus changes, and limit
catch-up after stalls. Allow explicit game-specific overrides. Advance simulation
at the original platform's intended cadence independently of monitor refresh;
display vsync paces presentation. Support finer scheduling for console effects
where a port needs it.

## Headless testing and validation

The same game logic supports desktop play and scripted headless execution.
Allow inspection of game state, fixed-step advancement and deterministic seeds
where relevant. Headless execution must not initialize windows or audio devices.

Write normal useful tests that run on this server, including asset loading,
overrides/extensions, simulation, timing, settings, bindings and CPU-side image
composition where applicable. Do not write tests requiring unavailable display,
GPU or controller environments. Do not spend effort on virtual displays or
offscreen graphics infrastructure merely to test this workflow. Report desktop
rendering, playback and physical-device behavior as untested when unavailable.

Use synthetic or appropriately licensed fixtures. Follow repository requirements
for Rust 1.90.0, Cargo.lock, fmt, clippy, tests, portability and other relevant
workflow/session/boundary regressions. Update docs/progress.md with tested facts
as implementation lands. Workflow tests establish preparation and prompt
contracts, not that an autonomous agent has completed an arbitrary real game.

## References for the framework and dependency decisions

- [Cargo dependency specification](https://doc.rust-lang.org/cargo/reference/specifying-dependencies.html)
- [Macroquad input](https://docs.rs/macroquad/latest/macroquad/input/)
- [ggez gamepad interface](https://docs.rs/ggez/latest/ggez/input/gamepad/)
- [Rust SDL2 modules](https://docs.rs/sdl2/latest/sdl2/)
- [Rust SDL2 build and linking options](https://github.com/rust-sdl2/rust-sdl2)

The user confirmed shared understanding and authorized implementation. See
docs/porting.md for the implementation contract and docs/progress.md for tested
facts; this design document alone does not establish implementation or validation.
