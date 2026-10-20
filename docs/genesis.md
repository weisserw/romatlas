# Sega Genesis / Mega Drive

Build with Rust 1.90.0 and Cargo.lock:

```sh
bash scripts/setup.sh
cargo build --locked --release
./target/release/romatlas inspect /path/to/owned.md
./target/release/romatlas map init /path/to/owned.md --output /tmp/game.json
./target/release/romatlas generate /path/to/owned.md --map /tmp/game.json --out /tmp/game-source
./target/release/romatlas verify /path/to/owned.md --generated /tmp/game-source
./target/release/romatlas serve --rom /path/to/owned.md
python3 scripts/test-genesis.py
```

`.md`, `.gen`, `.smd` and `.bin` select Genesis; `--platform genesis` selects it
for another extension. Existing platform selectors are unchanged. Success/errors
use the existing JSON protocol. Decoder CPU names are `main`/`m68k` and `z80`.
Inspection reports original-file SHA-256, sizes/layout, header system/titles/serial,
regions, vectors, ROM extent, SRAM description, checksum and mapper hypothesis.
Headers are not proof of hardware identity; linear versus SSF is based on system
and recognized serial conventions, not a universal mapper identifier.

## Address and mapping contract

Genesis uses `romatlas-genesis-map` version 1 JSON, with the existing transactional
region schema: offset/length, kind, provenance/confidence, label/comment/evidence
and code execution views. Maps contain metadata, never ROM bytes. Identity is
SHA-256 of the complete original file, including its prefix/encoding. Overlaps,
invalid contexts, partial instructions, duplicate labels, wrong identities and
foreign-platform maps are rejected. Empty/raw reconstruction leaves bytes unknown.

**Physical `rom` offsets are canonical backing offsets:** optional original copier
prefix, followed by normalized big-endian ROM payload. Big-endian files therefore
retain their file offsets. Word-swapped and interleaved files have a different
original-file encoding; `Rom::file_offset(payload_offset)` translates a canonical
payload byte to that original position. `Rom::original()` retains the untouched
file. CPU aliases and effective banks always address canonical backing. Never
interpret an original interleaved file position as a CPU-visible byte.

Example code annotation (after inspecting its role and boundaries):

```json
{"id":0,"offset":512,"length":2,"kind":"Code","provenance":"Manual",
 "label":"Reset","comment":"Explain the routine's behavior here.",
 "evidence":"Record the inspected source constraints.",
 "views":[{"cpu":"main","cpu_address":512,"context":{}}]}
```

Main CPU addresses are 24-bit and word-aligned for instructions. Work RAM mirrors
across $E00000–$FFFFFF. SSF maps eight 512 KiB windows using
`context.rom_banks` effective physical indices. SRAM uses `ram_enabled` and header
byte lanes. Unknown SRAM overlays or bank windows remain unresolved. Sound CPU
`z80_cpu` addresses are 16-bit: $0000–$3FFF aliases 8 KiB `zram`; $8000–$FFFF
selects a main-bus 32 KiB window through `context.z80_bank`. ROM views of sound
code require that explicit context. RAM execution does not prove upload origins
or lifetimes; uploaded-source views remain rejected rather than invented.

## Debugger and analysis

The persistent CommandAdapter exposes the same session methods as other adapters:
rom.open/info/read/regions, emulator.start/capabilities/state/reset/input,
emulator.step/run, emulator.registers.set, emulator.memory.write,
emulator.memory/physical_memory/translate, snapshots, framebuffer and WAV capture;
trace.configure/query/summary/clear/stream.start/stop; analysis.coverage/contexts/
edges/decode/discover/discover_context/references/pointers/unknown;
map.get/annotate/remove/validate/save/load/import/export and disasm.generate/verify.
Use api.describe for the complete method list and docs/protocol.md for envelopes,
trace filters, paging and streaming. Map import/export is Genesis JSON, not .sym.

`emulator.step` accepts `cpu:"main"` or `cpu:"z80"`, operation/cycle/frame,
count, max_operations and timeout_ms. Cycle units are **master clocks**, not 68000
or Z80 clocks. Operations count accepted instructions of the selected CPU, with
bounded core ticks; other CPUs and devices continue. Reset/waits/interrupt entry
are not instruction coverage. Frame/cycle advancement can overshoot by a core tick.
Input accepts up/down/left/right/a/b/c/x/y/z/start/mode for port 1 or 2.

State exposes 68000 data/address registers, PC, USP/SSP/SR and next prefetch;
Z80 architectural/shadow registers, PC/HALT, banks, scanline and master clocks.
Register/RAM/I/O edits are artificial and transactional at validation; evidence
retains artificial provenance. Use `emulator.memory` for mapped reads and
`emulator.physical_memory` for immutable backing (`rom`, `ram`, `zram`, `vram`,
`cram`, `vsram`, `cart_ram`). `io` uses a detached core copy for side-effect-free
register inspection. Blocked/unmapped accesses have CPU aliases, never invented
physical ROM identities. Snapshot files include local core state and remain local.
Snapshots require matching image/configuration and guarded serialized state.

Discovery and references are bounded hypotheses, respect data barriers and keep
unresolved indirect/window diagnostics. Compatible observed edges may supplement
static contexts; a reached successor is not execution coverage. Main pointers
use width 4 big-endian bus addresses or width 2 signed offsets with explicit
`base`; sound pointers use `cpu:"z80"`, width 2 little-endian plus z80_bank.
Pointer interpretations never prove code or change maps. Unvisited stays unknown.

## Source, agents, archives and ports

GNU 68000 sources use game.asm and layout.ld with distinct CPU VMAs and physical
load addresses. Separate Z80 regions use z80-ID.asm/link and the pinned WLA tools;
parent sections embed those rebuilt slices. GNU's `|` comments and WLA's `;`
comments are retained. Noncanonical encodings/relative control transfers can use
exact byte directives with decoded comments. Data/unknown intervals use local
incbin slices. encoding.json restores the original file layout after linking;
all headers/prefix/trailers are preserved. Both generation and verification rebuild
objects and compare the complete original file. External calls share the bounded
timeout; failed generation leaves no final output. Tools remain external.

`disassemble.py`, commentary, archives, reference corrections and port.py prepare
select the Genesis profile only for newly created workspaces. They preserve
structural versus reachability, whole-ROM classification, complete current-source
readability and editorial gates. Raw byte-identical restoration does not satisfy
those gates. Archives reconstruct local bytes from an independently supplied ROM;
no original image or extracted assets are distributed. Native preparation creates
a pending scaffold, not implemented gameplay. Read generated architecture/workflow
instructions and docs/porting.md; native fidelity auditing is a separate request.

NTSC defaults are 320x224 and master-clock/(3420*262) cadence. Native games can
select H32 256px, PAL, 240-line and interlaced modes explicitly. Native helpers are
independent MIT code. See genesis-native.md. No commercial ROM/assets are bundled,
no GUI/audio device is needed for tests, and no complete hardware/game compatibility
claim is made. SVP, Sega CD/32X, uncommon mappers and exotic peripherals are deferred.

Z80 state's `registers` array order is AF, BC, DE, HL, IX, IY, SP, PC, alternate
AF/BC/DE/HL, I, R, IFF mask and interrupt mode. Main arrays are D0–D7 and A0–A6;
USP, SSP and SR are separate fields. Native `tile` returns 64 row-major indices,
matching the shared framebuffer tile interface.

`emulator.state.video.registers` exposes the VDP's decoded register groups.
`audio.psg` reports tone periods, attenuation, noise control and LFSR;
`audio.ym2612` reports status plus 512 last software register writes (bank 0 then
bank 1). The latter is debugger shadow metadata for write-only registers, not
hardware readback. These inspections do not tick devices or mutate live status.

Debugger timing follows jgenesis region detection: Americas/Japan select NTSC and
Europe selects PAL. State reports timing_mode and nominal_fps. Native scaffold
defaults remain NTSC; a PAL port can select TickRate::GENESIS_PAL explicitly.
Region detection retains upstream header/serial conventions: a `SEGA GENESIS`
system name overrides an inconsistent Europe marker as Americas. PAL images
normally identify as Mega Drive. Inspect timing_mode when checking unusual headers.
