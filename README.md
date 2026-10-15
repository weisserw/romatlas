# ROMAtlas

Rust tools for ROM-free Game Boy maps. The project.md milestone scope is complete; documented hardware limits remain. This build supports ROM inspection, SM83 decoding, stock mgbdis map import/export, raw-data maps, source generation, and exact byte checks. It also supports bounded DMG ROM-only/MBC1/MBC3/MBC5 execution. A persistent JSON-RPC server supports agent sessions. Bounded CPU tracing and coverage are available through the server. The server also proposes code regions with bounded recursive discovery. The 89-case source-built default Mooneye gate and 55-case CGB selection pass. Full hardware compatibility is not claimed.

## Setup

On a new Ubuntu/Debian VM, run `bash scripts/setup.sh` as your normal user with sudo access (running directly as root is also supported). Then add `$HOME/.cargo/bin` to PATH. The script installs Rust 1.90.0, pinned mgbdis, RGBDS 0.9.4 and SDL2 development libraries for native ports. Only system package installation and RGBDS installation use sudo; Rust and local builds stay under the invoking user. Do not wrap the whole script in sudo when using an existing user Rust installation.

```sh
cargo build --locked
./target/debug/romatlas inspect game.gb
./target/debug/romatlas decode game.gb --offset 256 --count 16
./target/debug/romatlas map init game.gb --output game.sym
./target/debug/romatlas map validate game.sym --rom game.gb
./target/debug/romatlas generate game.gb --map game.sym --out build/game
./target/debug/romatlas verify game.gb --generated build/game
```

Use a new output folder for each generation. Maps contain labels and region directives, never ROM bytes. Initial maps mark every bank as raw data for lossless output; this does not claim that unexecuted bytes are data. Source and assets stay local and are not shareable map files.

Checks: `cargo fmt --check`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo test --locked`, `python3 scripts/test-portability.py`.

See [progress](docs/progress.md), [architecture and limits](docs/architecture.md), and [map syntax](docs/sym.md).

## Agent disassembly workspaces

See [INSTRUCTIONS.md](INSTRUCTIONS.md) for setup and Codex startup steps.

Prepare one separate local workspace per owned ROM, including agent instructions,
start/resume prompts, an identity record, and baseline/checkpoint commands:

```sh
cargo build --locked --release
python3 scripts/disassemble.py /path/to/game.gb --root /root/rom-work
# Multiple ROMs can be supplied to the same command.
cd /root/rom-work/game
python3 run.py inspect
python3 run.py baseline
```

Run your agent from the game workspace and give it `prompts/start.md`; use
`prompts/resume.md` for subsequent sessions. `AGENTS.md` and `workflow.md` guide
static discovery, bounded gameplay tracing, evidence-supported code/data mapping,
and verified assembly output. The tool prepares the workflow; it does not launch
an agent or automatically classify the ROM. No agent provider is required.

The initializer prints Codex goal-mode startup instructions for each game: run
`codex --enable goals -C /path/to/workspace`, then paste the printed `/goal ...`
objective inside Codex. It also prints a goal for subsequent disassembly passes
and the commands for inspecting, pausing and resuming an existing goal. Goal
support is required in your Codex version. Workspace results remain JSON lines
on stdout; human instructions go to stderr. Add `--json` to suppress instructions.

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
behavioral accounting. Completion uses a versioned `analysis/behavior.md` inventory,
resolved implementation/asset dependencies and reviewed unknown-region dispositions,
with no byte-classification percentage target. All ROMs, traces, internal state,
generated source and extracted
assets stay local; `.gitignore` ignores the entire workspace. Only exported stock
maps are intended for sharing. Test with `python3 scripts/test-disassemble.py`.

New workspaces default to the release binary and include immutable decode caching,
a compact investigation queue and reusable batch reporting. Existing workspaces can
be upgraded in place with backups while preserving ROMs, maps and historical evidence:
`python3 scripts/upgrade-disassembly.py /path/to/workspace --apply`.
See [efficient workspace operation](docs/workspace-efficiency.md) for usage,
resumption, model-trial accounting and rollback. Exercise these helpers with
`python3 scripts/test-disassembly-tools.py`.

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
pending source-behavior ledger and demonstration scaffold, and prints whole-game
start/resume goals and a separate library-review prompt. It does not port gameplay
or launch an agent. Game Boy is the initial scaffold platform. Incomplete
classification is accepted; inherited behavioral obligations remain explicit.
Commentary and port audits require behavioral accounting as well as source/region
records. Opaque preservation is separate from semantic review; any phase can
refine ROM understanding. See [the shared contract](docs/behavioral-accounting.md).

The new `romatlas-port` library provides ROM-backed assets, replacement/additional
slots, console timing, tile/palette rendering, input/settings and an optional SDL2
desktop shell. The port agent can evolve that library directly in ROMAtlas,
uncommitted, with a change log and an initial-file snapshot for separate review.
Normal tests and headless execution need no display or audio device. On a desktop,
`python3 run.py desktop` opens the game; F1 or the controller Guide button opens
settings. Never run that command on the headless server.

Original assets must load from the ROM at startup and stay out of distributions.
Code/data behavior accounting and useful passing tests establish the agent's
completion contract; automated audit cannot assess gameplay fidelity. Release
packaging and library publication/pinning are later work. See [porting guide](docs/porting.md)
and [agreed design](docs/port-workflow-design.md). Test with `python3 scripts/test-port.py`.

## Internal analysis store

Export `.sym` for stock-tool portability. Final analysis metadata can also be
preserved by the reviewed-result archive workflow below. Offsets and lengths
below use decimal physical ROM offsets.

```sh
./target/debug/romatlas store init game.gb --output analysis.json
./target/debug/romatlas store annotate game.gb --store analysis.json --offset 256 --length 3 --kind code --provenance manual --confidence 1
./target/debug/romatlas store get analysis.json --limit 100
./target/debug/romatlas store validate game.gb --store analysis.json
./target/debug/romatlas store export game.gb --store analysis.json --output game.sym
```

Use `--id N` on annotate to edit a region. Use `store remove analysis.json --id N` to remove it. Region kinds: code, data, text, padding, image, unknown. Provenance: manual, static, observed. User-supplied provenance is an assertion; it is not proof of execution. Export fills gaps with lossless raw data and creates stable labels for code starts. The store keeps those gaps unknown. Failed edits leave the file unchanged.

## Headless execution

```sh
./target/debug/romatlas run game.gb --operations 10000 --timeout-ms 1000
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

Archiving requires a fresh editorial audit and verified rebuild. It saves comments
and symbol edits separately from `.sym`; it excludes assembly, ROMs, extracted
assets, scratch evidence and the toolkit-issues report. Restoration checks the
original ROM and reconstructs the exact final source in `source/`, plus a
`reading-after.asm`, with pinned tools. Existing archives are replaced only after
successful reconstruction; failures preserve the previous archive. Restore output
must be new and outside the repository. See steps 6–7 for artifact selection and
options. Test with `python3 scripts/test-archive.py`.
