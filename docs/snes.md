# Baseline SNES support

ROMAtlas supports a headless NTSC SNES adapter alongside Game Boy. It uses the
pinned GPLv3 jgenesis v0.11.4 core fork and external Asar 1.91. The combined CLI
is GPLv3; shared MIT grants and the independent native-port runtime remain intact.
See [licensing](../LICENSES.md), [evaluation](snes-evaluation.md) and
[prompt audit](snes-prompt-audit.md).

## Supported image and tool workflow

Baseline LoROM/HiROM, 32 KiB–4 MiB in complete 32 KiB blocks, optional 512-byte
copier header, NTSC Japan/USA, ROM/ROM+RAM/ROM+RAM+battery, standard controllers,
65816, SPC700 and DSP. Other mapping modes, enhancement cartridges, PAL and
unsupported headers fail explicitly before execution. The core's mapper selection
must agree with the validated image layout. No coprocessor firmware is loaded.

```sh
bash scripts/setup.sh
cargo build --locked --release
./target/release/romatlas inspect /path/to/owned.sfc
./target/release/romatlas map init /path/to/owned.sfc --output /tmp/game-map.json
./target/release/romatlas map validate /tmp/game-map.json --rom /path/to/owned.sfc
./target/release/romatlas generate /path/to/owned.sfc --map /tmp/game-map.json --out /tmp/game-source
./target/release/romatlas verify /path/to/owned.sfc --generated /tmp/game-source
./target/release/romatlas serve --rom /path/to/owned.sfc
```

`.sfc`/`.smc` selects SNES automatically. Use `--platform snes` for other filenames
or to start a server without an image. An existing server keeps its selected
platform; open another platform in a separate server. Game Boy's default commands
and stock `.sym` contract remain unchanged. A SNES `.sym` from an assembler is
symbol interchange, not a portable structural map.

Success and errors use the existing JSON/exit-status contract. Generation publishes
a new directory only after a byte-identical rebuild, and failed generation leaves
no final directory. Verification always assembles fresh into a private directory.
Asar uses `--fix-checksum=off`; invalid original checksums and copier headers survive.
Physical `norom` layout disables Asar's bank-crossing check so file blocks/copier
headers can span file boundaries; the Rust decoder validates CPU view boundaries.
Timeouts share the bounded external process budget (default 120000 ms, max 300000).
First mismatch errors identify the physical file offset and differing values.

## Address and decoding rules

`rom` offsets identify bytes of the supplied **whole file**, including a copier
header. CPU cartridge translation adds that header offset; it does not execute
header bytes. The original file SHA-256 and size identify mappings/snapshots.
LoROM/HiROM mirrors resolve to original backing bytes, including non-power-of-two
chip layouts. Preserve the encoded CPU alias when generating an operand: equal
physical targets do not imply equal encodings.

Spaces: `cpu` (24-bit 65816 mapped bus), `spc_cpu` (16-bit mapped audio bus),
`rom`, `wram` (128 KiB), `sram` (header-defined), `aram` (64 KiB), `ipl` (64 bytes),
`vram` (64 KiB), `cgram` (512 bytes), `oam` (544 bytes). Physical storage peeks
are immutable. IPL backing is readable even when unmapped. I/O/unmapped peeks
return unsupported rather than invoking hardware reads or inventing storage.

Both CPUs have complete opcode decoding/formatting. 65816 accumulator immediates
need E/M context; index immediates need E/X. Fixed-length instructions need neither.
Static contracts and sound flag reasoning may establish context; runtime evidence
is optional. REP/SEP, CLC/SEC/XCE and restored status are tracked conservatively.
DB/direct-page uncertainty affects references independently of instruction length.
Indirect/indexed references remain unresolved unless the required context exists.
A pointer or reached target proves neither code nor natural reachability.

```sh
./target/release/romatlas decode /path/to/owned.sfc --offset 0 --count 16 --e true
./target/release/romatlas decode /path/to/owned.sfc --offset 4096 --cpu-address 32768 --m false --x true
./target/release/romatlas decode /path/to/owned.sfc --offset 8192 --cpu spc700 --cpu-address 512
```

Only supply contexts established for that view. A decode diagnostic stops that
linear request at the affected boundary; independent requests/ranges can continue.
Instruction operand fetches that cross the CPU address-space/bank boundary need
an explicit mapped execution view and currently produce a diagnostic in the
physical decoder. Relative branch wrapping is supported; SPC bit branches which
Asar cannot faithfully encode use raw bytes with the decoded instruction comment.

## JSON-RPC session

Use `api.describe` to discover the current platform/methods. The shared transport
has 1 MiB request/response limits; use bounded pages and file-only snapshot export.
The examples below are method/params; wrap each in JSON-RPC 2.0 with an id.

| Method | Parameters / behavior |
|---|---|
| rom.open | `path`; resets image, map, emulator and named snapshots transactionally |
| rom.info / rom.read | metadata; read `offset`, `length` (max 4096) |
| rom.regions / analysis.unknown | exact unknown complement, including explicitly Unknown regions; paged |
| emulator.start | create a deterministic NTSC machine with zero WRAM, no host-time or disk battery persistence |
| emulator.state / emulator.capabilities | both CPUs, emulation clocks and supported spaces/units |
| emulator.step | `cpu: "main"` or `"spc700"`, `unit: "operation"`, `"cycle"`, `"frame"`, `count`, `max_operations`, `timeout_ms` |
| emulator.run | same; `operations` aliases count |
| emulator.input | complete pressed `buttons` array for selected `port` (1 default, or 2); empty releases it; the other controller is preserved |
| emulator.reset | deterministic cold reset, preserving SRAM; clears held input and named snapshots |
| emulator.memory | mapped `space` (default cpu), `address`, `length` (max 4096) |
| emulator.physical_memory | backing `space` (default rom), `address`, `length`; no CPU bus reads |
| emulator.registers.set | `cpu`, partial `registers` object; requires that CPU at an instruction boundary |
| emulator.memory.write | `space`, `address`, `bytes`; WRAM/ARAM/SRAM only, max 4096, validated before mutation |
| emulator.snapshot.save / load | session-local `name` (max 64); held input and artificial-state status restore |
| emulator.snapshot.export | new `path` recommended, otherwise snapshot JSON returned |
| emulator.snapshot.import | `path` (max 32 MiB) or `snapshot`; validates format/version/hash/checksum/cartridge before replacing live state |
| emulator.framebuffer | optional new `path` writes PPM RGB; otherwise width/height/RGB pixels and bounded frame hashes |
| emulator.audio.clear / export | clear the capture; export to new `path` as 48 kHz stereo PCM16 WAV; reports buffer/drop counts |
| trace.configure | `enabled`, `capacity` (max 100000), optional `kinds`, `space`, `[start,end]` range |
| trace.summary / query / clear | aggregates/drops; query `offset`, `limit` (max 1000); clear also splits prior successor chains |
| trace.stream.start / stop | new `path`, bounded `max_bytes`; JSONL independent of detail capacity |
| analysis.coverage / edges / contexts | paged actual opcode locations, observed successor counts and processor contexts |
| analysis.decode / references | `offset`, `count` (max 4096), `cpu`, `cpu_address`, `context`; references retain unresolved bank/index/pointer diagnostics |
| analysis.discover / discover_context | bounded `entrypoints`, `max_instructions`; `use_observed` defaults true, false is static-only |
| analysis.pointers | `offset`, `count` (max 4096), `width` 2 or 3, optional stride; width 2 needs an effective CPU `bank` for resolution |
| map.get / annotate / remove / validate | paged regions; annotate a `region` object; remove `id`; rejected edits preserve live map |
| map.save / export / load / import | SNES JSON via `path`; imported/exported labels, comments, contexts and evidence survive; Game Boy `.sym` is not accepted |
| disasm.generate / verify | `out` or `generated`, optional timeout_ms; uses the loaded map/image and validates on-disk identity |

Paged analysis/map results use `items`, `total`, `next_offset`. Trace queries use
`events`. Event-kind filters use the shared serialized names, e.g. `Instruction`,
`Read`, `Write`, `Dma`, `Interrupt`, `Frame`, `ControlFlow`.

Operations count **newly fetched and completed instructions** on the selected
CPU. If already mid-instruction, finish that partial instruction first. The entire
machine continues; this does not freeze the other CPU/peripherals. Cycles count
master clocks and may overshoot to the next core tick boundary. Execution is capped
at 100 million core ticks and 300000 ms per call; budget/timeout errors leave the
advanced machine inspectable. Stopped/waiting CPUs cannot defeat those bounds.

Memory/register edits are artificial experiments. Trace contexts and snapshot
state retain that distinction; they do not establish natural gameplay reachability.
Reset clears volatile experiment provenance, but retains it when artificial
SRAM contents survive. emulator.start creates a fresh machine without that retained
state. Main CPU patches accept pc/a/x/y/p/e/db/direct_page/stack; SPC accepts pc/a/x/y/p/stack.
Emulation/index flags enforce register width/stack invariants. ROM and I/O patches
are rejected; live edits never change the supplied image. Unknown buttons/registers
and rejected ranges leave state unchanged.

### Trace interpretation

Records come from actual opcode/bus calls, not guessed next PCs or inspection.
Main bus CPU/DMA origin is explicit. SPC700 bus accesses, IPL versus ARAM, DSP
sample/directory/echo reads and echo writes are distinguished. Main timestamps
identify the core master-tick start; SPC/DSP records additionally carry their
SPC cycle clock. Do not interpret grouping within one tick as finer hardware
subcycle timing. Derived fetch/instruction records can share one observation id.

Observed edges join actual successive opcode fetches on the same CPU; main
interrupt handling splits chains. A next PC by itself never enters coverage.
Context roots/edges survive zero detail capacity. Static discovery retains
competing compatible processor views and respects physical non-code barriers.
Calls with unknown contracts leave return-state context unresolved; supply supported
entry/callee context where it matters instead of demanding gameplay traces.

ARAM/WRAM observations identify execution storage, which may be overwritten.
They do not automatically prove which immutable ROM bytes supplied that code.
Use upload traces and confirmed mapping views to record that separate relationship
and lifetime. Automatic RAM-to-ROM upload attribution is not implemented. Uploaded
static discovery requires an explicit confirmed source `length` and stays within it.

Trace buffers, aggregate counts, stream bytes and observation fanout are bounded.
Inspect drops/truncation. Snapshots rewind emulated clocks but not host stream
sequence numbers. Audio capture retains at most ten seconds; clear/export between
bounded segments and inspect reported dropped frames. Frame hash history retains
600 frames. No display, sound device or host wall-clock RTC is used.

## Portable SNES mapping v1

`map init` creates `format: "romatlas-snes-map"`, version 1, image SHA-256/size,
and an empty `regions` list. Unknown bytes stay unknown. The map contains metadata,
not ROM bytes, decoded asset payloads or raw trace values. Symbols are globally
unique ASCII identifiers. Comments/evidence describe behavior and structure, not
extracted original strings/assets.

Example `map.annotate` params for a confirmed region (use actual offsets/context):

```json
{"region":{"id":0,"offset":0,"length":14,"kind":"Code","provenance":"Static","confidence":0.9,"label":"reset_console","comment":"Enter native mode and establish the stack.","evidence":"Supported reset routine contract and control flow.","views":[{"cpu":"main","cpu_address":32768,"context":{"e":true,"m":true,"x":true}}]}}
```

An id of 0 creates a region; a nonzero id edits that region. The exported next_id
counter prevents removed IDs being reused. Split existing physical
regions before inserting smaller ones. Regions must be nonempty, in range and
nonoverlapping; alternate CPU interpretations belong in a region's `views` (max 16).
Code needs a valid explicit view; each view must decode to the exact region end.
Labels/comments/evidence/contexts survive export/import and deterministic generation.
`provenance` is Observed/Static/Manual; confidence defaults to 1 and must be finite
in [0,1]. Validation establishes consistency/fidelity, not the truth of a structural
or semantic claim. Agent evidence/readability review remains required.

SPC700 source stored in ROM uses `cpu: "spc700"`, its ARAM `cpu_address`, and
`uploaded: true`. A main-CPU WRAM upload similarly uses a WRAM CPU execution address.
Keep ROM storage offset/length separate from execution location and describe upload
and overwrite evidence/lifetime. These are confirmed metadata claims, not an
inference made merely from matching bytes. Static uploaded discovery entries use
`{"offset":...,"length":...,"view":{...}}`.

The primary view generates Asar source with explicit widths and CPU bases. Raw
fallback is lossless and diagnosed, never classification proof. Generated source
and local `image.sfc` are private outputs; export the mapping for another owner to
regenerate independently. Editorial source changes can introduce branch/RAM/field
symbols and includes, followed by exact verification. Asar symbol interchange can
be produced separately with `--symbols=wla --symbols-path=/tmp/game.sym`; it omits
the structural/context information required by the portable JSON mapping.

## Agent workspaces and native ports

```sh
python3 scripts/disassemble.py /path/to/owned.sfc --workspace /tmp/game-analysis
python3 scripts/commentary.py prepare /tmp/game-analysis
python3 scripts/archive.py archive /tmp/game-analysis --output /tmp/game-archive --name game-name
python3 scripts/archive.py restore /tmp/game-archive --rom /path/to/owned.sfc --out /tmp/game-restored
python3 scripts/port.py prepare /tmp/game-archive --rom /path/to/owned.sfc --out /tmp/game-port
```

These commands do not launch agents. Use generated prompts and checkpoint runner;
do not prepare commentary/archive before the corresponding mapping/review work.
New SNES profiles select JSON mappings and Asar while retaining the shared
structural/reachability, complete-current-source readability, redirect continuation
and final audit contracts. Existing Game Boy profile text and workspaces are
preserved. An archive can honestly retain unknown regions after editorial review;
it does not thereby establish whole-ROM classification completion.

The native-port scaffold loads assets from the supplied ROM, uses independent
SNES planar tile/palette/Mode 7 character/tilemap helpers, twelve-button default
bindings and nominal NTSC cadence.
It is an arbitrary demonstration, not a ported game. Existing settings, asset
replacements/additions, headless tests and SDL desktop audio interfaces apply.
SNES-specific composition, DSP behavior and additional helpers grow from the first
port's demonstrated needs; no emulator is linked into generated ports.

## Measured tests and remaining limits

Run `python3 scripts/test-snes.py`, `bash scripts/test-snes-cpu.sh`, locked workspace
fmt/Clippy/tests, dependency boundaries and Game Boy portability/session/workflow
regressions. The SNES test uses original synthetic images and requires pinned Asar.
The optional CPU-vector script downloads only pinned MIT JSON vectors/notices.

Tests cover LoROM/HiROM boot, controller input, SPC upload, nonzero audio,
immutable storage peeks, snapshot/input/audio replay, SRAM reset, bounded execution,
trace contexts at zero detail capacity, maps, deterministic exact reconstruction,
commentary, metadata-only archive restoration and native-port scaffold validation.
All 256 65816 opcodes under four M/X combinations and all 256 SPC700 opcodes rebuild
with nonzero operands. 832 sampled external CPU vectors check registers/final
memory/cycle counts; bus-order/signals and a full hardware suite are not established.

Representative locally owned games have not been supplied. No broad commercial-game,
all graphics mode, peripheral or cycle-exact hardware compatibility claim is made.
The SNES baseline is incremental, like the Game Boy adapter. Live debugger playback,
unsafe I/O inspection, automatic upload attribution, external coprocessors and rare
hardware remain outside this initial implementation. Do not launch desktop graphics
on the headless server; desktop-feature tests compile code without opening devices.
