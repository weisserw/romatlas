# NES adapter

NES runs alongside Game Boy and SNES. `.nes` selects it automatically;
`--platform nes` selects it for other filenames. Install pinned ca65/ld65 with
`bash scripts/setup-nes.sh` (also called by `scripts/setup.sh`). Build with Rust
1.90.0 using `cargo build --locked --release`.

```sh
target/release/romatlas inspect owned.nes
target/release/romatlas map init owned.nes --output /tmp/owned.json
target/release/romatlas map validate /tmp/owned.json --rom owned.nes
target/release/romatlas generate owned.nes --map /tmp/owned.json --out /tmp/source
target/release/romatlas verify owned.nes --generated /tmp/source
target/release/romatlas serve --rom owned.nes
```

Generation and verification use the existing timeout budget and staging contract.
Failed generation publishes no final directory; retry the same new path. Verify
reassembles game.o before linking and comparing the whole file. Headers, optional
trainers, PRG, CHR and trailing bytes are retained. Empty maps rebuild exactly but
leave every byte unknown. This does not satisfy structural or editorial completion.

## Debugger

The selected [jgenesis core](nes-evaluation.md) runs NTSC with ordinary two-port
controllers and mappers 0 (NROM), 1 (MMC1), 2 (UxROM), 3 (CNROM), 4 (MMC3) and 7
(AxROM). Nonzero submappers, trainer execution and other boards fail explicitly.
Runtime storage sizes must be powers of two. Reconstruction is independent of
these execution restrictions. NES 2.0 section sizes are parsed, including exponent
encoding; unsupported console types fail explicitly. PAL is outside the baseline.

Use the same JSON-RPC transport as docs/protocol.md. `api.describe` enumerates
supported methods. `rom.open/info/read/regions`, `emulator.start/capabilities/state`,
`emulator.step/run/reset/input`, memory, snapshots, frame/audio capture, trace,
analysis and map/disassembly calls are provided by the NES CommandAdapter.
`emulator.step` uses unit `operation`, `cycle` or `frame`, count, max_operations
(core CPU ticks, at most 100000000) and timeout_ms (1..300000). Operations finish
at an instruction boundary; cycle stepping may stop mid-instruction. A KIL stops
operation stepping with an explicit error; frame/cycle stepping remains bounded.
State exposes main CPU registers, frozen/mid-instruction flags, CPU cycles,
frames and effective PRG slot context. Input buttons are a/b/start/select/up/down/
left/right, with port 1 or 2. Reset preserves battery RAM and clears output sinks.
RTC/host-time behavior is absent.

`emulator.memory` accepts CPU mapped space or PPU mapped space plus address and
length (at most 4096 in RPC). CPU I/O peeks fail explicitly; they never read/clear
hardware registers. `emulator.physical_memory` requires backing space: rom (whole
file), ram, prg_ram, chr_ram, vram, palette, oam, prg_rom or chr_rom. Physical file
ROM offsets include the header/trainer. Mapped CPU RAM aliases collapse onto the
2 KiB backing. PPU reads in traces have origin ppu; unresolved PPU mappings remain
PPU addresses. Writes to mapper registers stay CPU addresses, never immutable ROM.

CPU registers can be edited at instruction boundaries with
`emulator.registers.set` and `{registers:{pc,a,x,y,p,stack}}`. `emulator.memory.write`
currently edits backing internal RAM (space ram or cpu, range 0..2048). Debug
edits mark subsequent state/evidence artificial and split successor chains.
These APIs do not inject natural execution coverage.

Trace configuration/filtering, bounded detail buffers, JSONL streaming, summary,
coverage, contextual roots and observed edges use the shared implementation.
Zero detail capacity retains bounded aggregates. A next PC is not coverage.
DMA requests and actual DMA bus reads have separate provenance; the baseline
reports OAM/DMC requests, not a cycle-exact inventory of all internal DMA signals.
BankSwitch reports effective PRG-slot changes; CHR-only mapper changes remain
visible as CPU writes and subsequent physical PPU reads.

Snapshots save/load by name in memory, or export/import through files. Use a path
for durable snapshots because serialized resampler state can exceed the transport's
1 MiB inline response limit. These are same-build snapshots, not a stable external
emulator format. Core state includes the local raw image copy; keep snapshot
files local alongside other generated artifacts. Invalid identity/checksum/trailing bytes are rejected before state
adoption. Captures clear on restore; trace sequence/evidence stays session-local.
Framebuffer is 256x240 RGB8 or local PPM; audio exports 48 kHz stereo PCM16 WAV,
with bounded buffers and dropped-sample reporting. No window/device is initialized.

## Mapping, decoding and analysis

The ROM-free format is `romatlas-nes-map`, version 1, with image_sha256,
image_size, next_id and regions. A code annotation example:

```json
{"id":0,"offset":16,"length":3,"kind":"Code","provenance":"Static",
 "label":"Reset","evidence":"Document the actual structural evidence here",
 "views":[{"cpu":"main","cpu_address":32768,
            "context":{"prg_banks":[0,1,2,3]}}]}
```

Call map.annotate with `{region:...}`, then map.validate and map.export/save.
Existing regions must be split before adding overlapping smaller regions. Rejected
edits preserve state. IDs, labels, comments, evidence and explicit execution views
survive save/load. Maps contain no ROM byte payloads. Uploaded execution views are
currently rejected; RAM execution does not prove immutable upload origins.

prg_banks holds effective physical 8 KiB PRG banks for the four CPU windows
$8000/$A000/$C000/$E000. NROM/CNROM can resolve static aliases without explicit
banks; other boards require context. Context must resolve every instruction and
operand byte to the specified physical file region. The decoder covers all 256
NMOS 6502 encodings with bounded operands, control successors and references.
`decode --offset N --count N --cpu-address N` is physical decoding; without an
explicit CPU address only unambiguous static mappings are accepted. JSON-RPC
analysis.decode/references additionally accepts context. Indirect/indexed operands
are explicitly marked and are not proven final effective targets.

analysis.discover/discover_context produces bounded hypotheses through code/unknown
regions, preserving data barriers and unresolved bank diagnostics. Effective observed
contexts and successor edges can supplement static paths; use_observed=false
selects static-only traversal. Mapper-affecting stores/calls conservatively stop
static bank propagation. analysis.pointers accepts 16-bit little-endian tables,
explicit stride and slot context; results remain hypotheses and do not change maps.
Unvisited bytes remain unknown. All original structural/reachability, whole-ROM
classification, readable current-source and editorial gates continue to apply.

Generated `game.asm` uses ca65, `layout.cfg` fixes physical output order, and local
`image.nes` supplies undecoded data ranges. Official instructions use mnemonic
assembly with explicit address widths. Unofficial aliases and wrapping branches
use byte directives with decoded mnemonic comments. An exported map plus an
independently supplied identical ROM and pinned tools reproduces the source.

New disassembly/commentary/archive/port workspaces select the NES JSON/ca65 profile;
existing Game Boy and SNES workspaces are not rewritten. Native helper APIs and
limits are in [nes-native.md](nes-native.md). Run `python3 scripts/test-nes.py`.
Synthetic tests do not establish full hardware/game compatibility.
