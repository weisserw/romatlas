# ROMAtlas

This project contains tools, libraries and prompts for custom AI agents which can decompile and recompile games from second-generation cartridge-based consoles. The main implementation language
is Rust, with some scripts written in Python. The overall goal is to enable users to go from a ROM file to a finished port in modern Rust code.

The current consoles supported are:

* Game Boy
* NES
* SNES
* Game Gear
* Master System
* Genesis
* TurboGrafx-16

The current build targets for Rust ports are:

* Linux
* Mac
* Windows
* Web Assembly

The project consists of three general parts:

* Customized emulators for each console which are designed to be driven directly by an agent via a persistent JSON-RPC server. Agents can run games, send button inputs, view the screen and read and write internal RAM / CPU states, etc.
* A runtime library which implements utility functions useful for porting games from this era (e.g. implementing timing by waiting for VSync), as well as functions specific to each platform (e.g. Mode 7 graphics on SNES)
* A set of scripts which create working directories pre-populated with prompts that direct agents to disassemble or port a game in goal mode, using tactics that tend to produce good results out of the box.

## Why?

The once-niche practice of resurrecting games via decompilation and recompilation (decomp/recomp) has suddenly exploded in popularity due to the advent of AI-assisted coding and debugging. While using an AI agent to recomp a specific game is generally straightforward if one understands the underlying
technical details, the next generation of these efforts will likely involve automated custom agent processes like the ones in this repository, that combine pre-made prompts with specialized tools built to speed up the agent's work. This will, in time, enable us to recomp many more games with minimal effort.

This project has a particular focus: game consoles which were originally programmed in assembly language, rather than a higher-level language like C. These platforms pose an interesting challenge for both decompilation and recompilation. For the former, even though disassembly is a simpler process than decompilation, it requires that we locate the areas of the ROM cartridge that consist of actual assembly code as opposed to data. Since game developers in the 80s and early 90s did not have standardized file formats and layouts, and were desperate to save space in expensive ROM chips, they often
packed code and data together in ways which are difficult to reverse engineer.

Once successfully disassembled, an assembly-based game can be easily modified by an agent in place ("rom hacking"), or can be read and understood to find, e.g., cheats, easter eggs, or glitches for speed running. But recompiling such a game as a straight assembly language translation will always have the disadvantage that assembly is not a very good platform in terms of maintainability or flexibility. Fixing bugs and adding enhancements and new features will be difficult as long as the game remains in its original form.

That's why this project takes an unusual step: once a game is disasssembled, we recompile it not to an assembly language equivalent, but to a modern, high-level language (Rust). While this will inevitably lose some fidelity compared to the original, it means that these games which would otherwise remain ossified forever can now be given new life in surprising and interesting ways, because their code is now truly platform-independant, easy to reason about and easy to extend and modify.

At least, that's the theory. This project is still in heavy development and should be regarded partially as an experiment. That said, we've already gotten quite a few ports working as part of our testing, and more will continue to be added. Hopefully, in time, the feasibility of porting early Nintendo, Sega and similar platform's games to Rust will be proven out.

## Instructions for Humans

In general, I recommend interacting with this software via an AI agent rather than running commands manually. An agent will be able to understand all the commands and diagnose / fix any problems
instantly.

As of this writing the models / effort that seem to work best are the default "workhouse" settings from each vendor. Bleeding edge models tend to use too many tokens, and
lower tier / last generation models will produce poor output and end up wasting more time and tokens than they save.

### Decompiling / Disassembling a ROM

Before a ROM can be recompiled, it needs to be disassembled. For this proccess, ROMAtlas runs three subagents in sequence:

* A disassemble agent plays the game in the debugger and tries to determine based on accessed regions of memory which parts of the ROM file are code and which are data. It writes these mappings
to a file that a disassembler can use to produce an uncommented source listing for the game.
* A commenter agent reads the disassembled source and intuits what each function, variable and jump label is supposed to do, using the debugger as needed for clarification. It then writes source
comments to a file format that lets an external tool produce a fully commented source listing from the file and the original ROM.
* A redirect agent reads the commented source and looks for clues that could lead to the discovery of further structure within the ROM file. It writes the list of leads to a file so that
the disassembler can use them on its next run.

This process continues until the goal reaches a "BLOCKED" state (no more leads) or the ROM is 100% disassembled and commented.

To start the process, build the Rust code (see "Setup" below), then run this command:

`python3 scripts/disassembly.py <rom>`

Or ask an agent to do it for you. This creates a working directory under the root (the default root is the current directory) with the rom file and a set of prompts / scaffolding in it. Open a
new instance of your agent in that directory and type:

`/goal Read goal.md`

The rest of the process should proceed automatically.

### Archiving

Once a game has been successfully disassembled (or has reached a "blocked" state where only a small unknown portion remains), you should archive it using `scripts/archive.py` (this script is better invoked via an AI agent, as it depends on the outputs of the disassembly process which may not be 100% reliable since AI agents can make mistakes; an agent running the script can fix any of these problems as it goes).

Archiving a game copies its disassembly artifacts into the `atlas/` directory which is published in this repo. It does not copy the original ROM file or the full game source code (found in `commentary/reading-after.asm`). Users of this repo must supply their own ROM files to reconstruct the original source, which can be done via `scripts/archive.py restore`.

### Porting

The `scripts/port.py` script initializes a workspace for porting an archived game. By default it creates a directory named `rom-ports/<romfile>`. Inside this directory, run an agent and type `/goal Read goal.md` to begin the porting process to Rust.

### Debugging

The porting goal does minimal checking of its own work; it is concerned mainly with reproducing the original game logic. Once a port is complete, it needs to be tested and compared with the original game to find defects. To do this, run `/goal debug/goal.md` in the port directory. The porting agent is instructed to include headless / scripted interaction in the port's feature set so that the agent can play the game and compare the result to the emulated copy using the toolkit's built-in emulator.

Debugging works by first generating a test plan unique to the game, then running through the entire test plan twice and fixing all issues found. Additional debugging cycles may be neccessary for large and complex games.

### Running a Port in The Browser

Run:

`python3 run.py web`

To run a local web server on port 8000 which hosts the WebAsssembly build of the port. You must press the "Start" button on the web site to initialize sound and input before you can begin playing.

### Caveats / Issues

A persistent issue with goal-based workflows in today's AI agents is their tendency to "go down rabbit holes" or run forever in loops if they can't solve a problem. The prompts in this repository have been engineered to attempt to avoid this behavior as much as possible, but they also do not run under any "budget" by default. Users should periodically check on the processes or instruct their agents to operate under a time or token budget if neccessary.

Some 8- and 16-bit era games were simple and some were not. Because testing on large games is time consuming and expensive, this project is likely to have trouble with the most complex games for the time being, but work is ongoing.

There are many features which could be added to ports out of the box, as the current implementation is quite basic and only provides customizable display
and input controls (with basic gamepad support). An advantage of the architecture of this project is that ports share a runtime library which can be used to add QoL features to all future ports built by this system, not just to individual games.

All the rest of the information below is automatically generated and meant for AI to consume.

## Setup

On a new Ubuntu/Debian VM, run `bash scripts/setup.sh` as your normal user with sudo access (running directly as root is also supported). Then add `$HOME/.cargo/bin` to PATH. The script installs Rust 1.90.0, pinned mgbdis, RGBDS 0.9.4, SDL2 development libraries, the Wasm target and pinned wasm-bindgen tooling for ports. Only system package installation and RGBDS installation use sudo; Rust and local builds stay under the invoking user. Do not wrap the whole script in sudo when using an existing user Rust installation.

```sh
cargo build --locked --release
./target/release/romatlas inspect game.gb
./target/release/romatlas decode game.gb --offset 256 --count 16
./target/release/romatlas map init game.gb --output game.sym
./target/release/romatlas map validate game.sym --rom game.gb
./target/release/romatlas generate game.gb --map game.sym --out build/game
./target/release/romatlas verify game.gb --generated build/game
```

Use a new output folder for each generation. Maps contain labels and region directives, never ROM bytes. Initial maps mark every bank as raw data for lossless output; this does not claim that unexecuted bytes are data. Source and assets stay local and are not shareable map files.

Checks: `cargo fmt --check`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo test --locked`, `python3 scripts/test-portability.py`.

See [progress](docs/progress.md), [architecture and limits](docs/architecture.md), and [map syntax](docs/sym.md).

## Agent disassembly workspaces

See [INSTRUCTIONS.md](INSTRUCTIONS.md) for setup and startup steps.

Prepare one separate local workspace per owned ROM, including agent instructions,
start/resume prompts, an identity record, and baseline/checkpoint commands:

```sh
python3 scripts/disassemble.py /path/to/game.gb --root /root/rom-work
# Multiple ROMs can be supplied to the same command.
cd /root/rom-work/game
python3 run.py inspect
python3 run.py baseline
```

Run your agent from the game workspace and ask it to read `goal.md`. The generated
`prompts/disassembly.txt`, `commentary.txt` and `redirect.txt` define the three phases.
Disassembly drives normal gameplay scenarios while they yield substantial gains,
alongside static discovery and lead verification, then stops incomplete when no
obvious substantial gain remains. Commentary incrementally interprets changed material and affected dependencies,
preserving valid prior review credit and using readable provisional names. It hands
off useful findings without waiting for full-ROM editorial review; redirect continues
structural investigation and develops new leads. Separate sequential phase workers
exchange compact durable handoffs.
Structural classification and reachability are separate claims throughout every
pass. Independent static evidence can establish roles and exact boundaries without
a live caller or gameplay route; natural use remains explicitly unresolved. Caller
tracing is useful evidence, rather than a universal classification prerequisite.

Investigation belongs to redirect at every classification percentage, with no
separate final-investigation phase or percentage threshold. It examines remaining
structures, selector producers and targeted experiments while supported findings,
useful constraints or justified next methods continue. Intermediate progress without
a classification proposal continues redirect directly; ready batches pass through
disassembly and full current-source commentary. There is no fixed pass quota.
Attempt history prevents unchanged retries, and rejection review precedes a genuine
incomplete stop. Older investigation records remain evidence; unfinished research
resumes as redirect. Preparation writes instructions without launching an agent.

Full editorial cleanup/audit is reserved for final acceptance and archiving. The coordinator repeats only with new actionable information,
otherwise reports incomplete. Whole-ROM completion still requires every byte to be
classified with evidence and a verified rebuild. No agent provider is required.

Scripts write prompts. JSON results remain on
stdout; prompt locations go to stderr (`--json` suppresses those locations).
Existing workspaces are not automatically migrated.

Generated `evidence-policy.md` explains how to combine structural evidence, localize
failed hypotheses and correct rejection reasoning on unchanged ROM evidence. Before
a terminal stop the worker provides a rejection review and the coordinator checks
its decisive artifacts. This preserves genuine incomplete stops and all classification
and rebuild requirements; see [the inference audit](docs/agent-inference-audit.md).

The default parent is the current directory; `--workspace /exact/path` selects a workspace
for a single ROM. Existing workspaces and paths inside this repository are
rejected. Use `--binary /path/to/romatlas` to select a different built binary.
The toolkit and pinned tools must stay installed at their configured locations.
Batch initialization stops on the first error; earlier successful workspaces
remain available. Resolve the error and rerun only the remaining ROMs.

The agent uses `python3 run.py serve` for persistent JSON-RPC. After saving
`analysis/state.json` and exporting `maps/current.sym`, run
`python3 run.py checkpoint pass-001` to save the exact map used, generate assembly,
and record byte-identical verification. Choose a new checkpoint name each time.
Failed checkpoints may retain their map or generated source for inspection; use
a new name for the next attempt. Baseline is raw-byte output and does not complete
classification. All ROMs, traces, internal state, generated source and extracted
assets stay local; `.gitignore` ignores the entire workspace. Only exported stock
maps are intended for sharing. Test with `python3 scripts/test-disassemble.py`.

## Native port workspaces

Prepare a separate Rust game-port workspace from a reviewed archive and your
matching local ROM:

```sh
python3 scripts/port.py prepare /path/to/archive --rom /path/to/game.gb --out /root/rom-ports/game
cd /root/rom-ports/game
python3 run.py headless --frames 120
python3 run.py check
```

Preparation restores and byte-verifies the commented reference source, creates a
pending source-behavior ledger and demonstration scaffold, and writes `goal.md`
with a separate library-review prompt. It does not port gameplay
or launch an agent. Game Boy/Color, SMS, Game Gear, NES, SNES, Genesis and PC Engine archives have platform-selected scaffolds. Incomplete
classification is accepted; relevant unknown behavior must be investigated.
New workspaces seed `uncertainties.json` from unknown ranges and carry consumer,
port-impact and validation assessments. The agent can correct a local reference
with `reference-init` and `reference-verify --revision NAME`; verified revisions
supplement the frozen archive without rewriting it. Original mapping completeness
and native-port behavior completeness remain separate requirements.

The new `romatlas-port` library provides ROM-backed assets, replacement/additional
slots, console timing, tile/palette rendering, input/settings and an optional SDL2
desktop shell. The port agent can evolve that library directly in ROMAtlas,
uncommitted, with a change log and an initial-file snapshot for separate review.
Normal tests and headless execution need no display or audio device. On a desktop,
`python3 run.py desktop` opens the game; F1 or the controller Guide button opens
settings. Never run that command on the headless server.

New workspaces also use the same Game API with a Canvas/Web Audio frontend:
`python3 run.py web --port 8000` builds and serves the bundle on `0.0.0.0` by default; use `--host ADDRESS` to change the bind address.
The server supplies the configured ROM and optional `--assets` overrides for
identity-checked startup extraction; original assets are never bundled.
Browser controls/settings persist locally. Original battery saves use optional
shared game-owned hooks, with browser localStorage or desktop files; save states
are not supported. New-workspace audits lint/build Wasm alongside desktop.
Existing workspaces are not migrated. See the [browser and save guide](docs/porting.md#browser-ports-and-battery-saves)
for setup, storage, limits and the synthetic Chromium regression.

Original assets must load from the ROM at startup and stay out of distributions.
The agent first writes a source-informed architecture plan, then implements the
whole game as maintainable best-effort Rust. Focused unit tests and required
format/static/build checks establish implementation checks. Runtime fidelity
validation is outside the porting goal; debugger use answers bounded source
questions only. Document implemented assumptions, finish behavior accounting and
the handoff, run the administrative audit and stop. Fidelity auditing requires a
separate request; automated audit cannot assess gameplay fidelity. Release
packaging and library publication/pinning are later work. See [porting guide](docs/porting.md)
and [agreed design](docs/port-workflow-design.md). Test with `python3 scripts/test-port.py`.

## Internal analysis store

Export `.sym` for stock-tool portability. Final analysis metadata can also be
preserved by the reviewed-result archive workflow below. Offsets and lengths
below use decimal physical ROM offsets.

```sh
./target/release/romatlas store init game.gb --output analysis.json
./target/release/romatlas store annotate game.gb --store analysis.json --offset 256 --length 3 --kind code --provenance manual --confidence 1
./target/release/romatlas store get analysis.json --limit 100
./target/release/romatlas store validate game.gb --store analysis.json
./target/release/romatlas store export game.gb --store analysis.json --output game.sym
```

Use `--id N` on annotate to edit a region. Use `store remove analysis.json --id N` to remove it. Region kinds: code, data, text, padding, image, unknown. Provenance: manual, static, observed. User-supplied provenance is an assertion; it is not proof of execution. Export fills gaps with lossless raw data and creates stable labels for code starts. The store keeps those gaps unknown. Failed edits leave the file unchanged.

## Headless execution

```sh
./target/release/romatlas run game.gb --operations 10000 --timeout-ms 1000
```

Returns registers, operation/cycle counts, and effective ROM banks as JSON. Limits: one million CPU operations and 30 seconds per call. The Rust adapter also exposes input, reset, snapshots, and mapped memory peeks; a persistent JSON-RPC server exposes these calls. CGB execution selects its model from the cartridge flag; emulator.start can override it. Basic banking, speed and DMA/replay tests pass; wider timing tests remain. The adapter does not read or write battery save files.

Run `romatlas serve --rom game.gb` for persistent control. See [protocol.md](docs/protocol.md) for calls and limits. Run `python3 scripts/test-session.py` to test replay, map edits, generation, and stock-tool portability.

Use `store import game.gb --map community.sym --output analysis.json` to keep community map annotations in the internal store. The server uses map.import. See docs/sym.md for the stock RAM/I/O label workaround.

MBC3 RTC uses emulated time from an initial zero clock. Snapshots preserve clock and latch state; host time does not affect replay.

State includes model, WRAM/VRAM banks and speed. The CGB gate adds two public boot-state cases; see docs/hardware-tests.md for limits.

Generation and verification use a shared 120-second external-tool deadline. Set --timeout-ms (1..300000) to change it. Generation validates the map and publishes source only after a byte-identical rebuild. If it rejects an annotation, use raw data for that region and retry.

Licensed gameplay and clean recipient rebuild checks are in [docs/homebrew-tests.md](docs/homebrew-tests.md). They build Galactic Armada and Unbricked from pinned MIT/zlib source; no commercial ROM is required.

To check setup without reusing installed tools or build caches on a root-capable Ubuntu host, install debootstrap and run `sudo bash scripts/test-fresh-setup.sh`. The script creates an Ubuntu noble minbase under ignored build/, copies only source, runs setup and the acceptance checks, and unmounts its /dev and /proc on exit. Logs and the success-only result JSON stay in build/. It needs several GiB of free disk and network access. This checks a clean root filesystem on the host kernel, not a separate VM.

The final requirement review is in [docs/completion-audit.md](docs/completion-audit.md). It records verified scope, milestone exits and hardware limits.

## Archive and restore reviewed commentary

After the two-phase workflow in [INSTRUCTIONS.md](INSTRUCTIONS.md), retain final
maps, commentary metadata, reviews and documents in `atlas/<platform>/<game>/<sha256>/`:

```sh
python3 scripts/archive.py archive /root/rom-work/game --name game
python3 scripts/archive.py restore atlas/gb/game/FULL_ROM_SHA256 --rom /path/to/original.gb --out /tmp/game-restored
```

Archiving supports incomplete worktrees and records whether editorial review is
complete. It still requires a valid map and a verified, byte-identical source
rebuild. It saves comments and symbol edits separately from `.sym`; it excludes
assembly, ROMs, extracted assets, scratch evidence and the toolkit-issues report. Restoration checks the
original ROM and reconstructs the exact final source in `source/`, plus a
`reading-after.asm`, with pinned tools. Existing archives are replaced only after
successful reconstruction; failures preserve the previous archive. Restore output
must be new and outside the repository. See steps 6–7 for artifact selection and
options. Test with `python3 scripts/test-archive.py`.

## SNES

Baseline headless SNES support uses pinned jgenesis and Asar, alongside the existing
Game Boy workflow. See [SNES commands, maps, runtime and limits](docs/snes.md),
[the prompt audit](docs/snes-prompt-audit.md) and [licensing](LICENSES.md).
Use locally owned `.sfc`/`.smc` images; no commercial ROMs or extracted assets are
bundled. Native ports use the independent MIT runtime, with SNES asset helpers.

NES support: see [usage and supported limits](docs/nes.md),
[core selection](docs/nes-evaluation.md), and [native graphics/audio helpers](docs/nes-native.md).
Use `.nes` or `--platform nes`, JSON maps and pinned ca65/ld65; Game Boy and SNES
commands and formats remain platform-specific.

## Sega Master System

SMS joins Game Boy, NES and SNES. Use `.sms` or `--platform sms`, JSON maps,
`serve` debugging and WLA-DX exact reconstruction. Install with
`bash scripts/setup-sms.sh`; see [SMS usage and limits](docs/sms.md),
[tool selection](docs/sms-evaluation.md) and [native helpers](docs/sms-native.md).
Run `python3 scripts/test-sms.py` for original synthetic regressions.

## Sega Game Gear

Use `.gg` or `--platform gg` for jgenesis debugging, bank-aware JSON maps and
byte-identical WLA-DX reconstruction through the shared Sega Z80 adapter. Native
helpers include Mode 4 LCD graphics, 12-bit colors and stereo PSG. See
[usage and limits](docs/gg.md), [tool selection](docs/gg-evaluation.md) and
[native helpers](docs/gg-native.md). Run `python3 scripts/test-gg.py`.

## Sega Genesis / Mega Drive

Genesis support includes the pinned jgenesis 68000/Z80 debugger, Genesis JSON
maps and contextual analysis, GNU binutils + DeZ80/WLA source reconstruction,
agent/commentary/archive/reference/native-port workflows and independent MIT
VDP/YM2612/PSG helpers. `.md`/`.gen`/`.smd`/`.bin` or `--platform genesis` selects it.
See [Genesis usage](docs/genesis.md), [tool evaluation](docs/genesis-evaluation.md)
and [native helpers](docs/genesis-native.md). Run `python3 scripts/test-genesis.py`.
NTSC/PAL cartridge baseline excludes exotic peripherals, SVP, Sega CD/32X and uncommon
boards. Tests use original synthetic cartridges; broad hardware fidelity is not claimed.

Game Boy Color shares the existing `gb` platform, stock .sym/RGBDS and Mizu core. See [docs/gbc.md](docs/gbc.md) for hardware models, immutable palette inspection, native color helpers and validation.

PC Engine / TurboGrafx-16 HuCards: use `.pce`/`.tg16` or `--platform pce`.
See [toolkit/API](docs/pce.md), [core/tool evaluation](docs/pce-evaluation.md) and
[independent native VDC/VCE/PSG helpers](docs/pce-native.md). CD and SuperGrafx are outside scope.
