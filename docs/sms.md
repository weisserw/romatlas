# Master System adapter

Install with `bash scripts/setup.sh`, or `bash scripts/setup-sms.sh` for just the
pinned WLA-DX 10.6 tools, then `cargo build --locked --release` with Rust 1.90.0.

```sh
target/release/romatlas inspect /path/to/owned.sms
target/release/romatlas map init /path/to/owned.sms --output /tmp/owned.json
target/release/romatlas serve --rom /path/to/owned.sms
target/release/romatlas generate /path/to/owned.sms --map /tmp/owned.json --out /tmp/sms-source
target/release/romatlas verify /path/to/owned.sms --generated /tmp/sms-source
python3 scripts/disassemble.py /path/to/owned.sms --workspace /tmp/sms-workspace --json
```

`.sms` selects SMS automatically; `--platform sms` selects it for other extensions.
Success/error JSON and timeout contracts match the other platforms. `decode` accepts
`--cpu z80`, physical `--offset`, `--cpu-address` and `--count`; persistent decoding
also accepts bank context. Default debugger hardware is NTSC SMS2 (roughly 59.92 Hz),
256x192 active display with 224/240 extended heights available to software. Standard
controller names are up/down/left/right/button1/button2, aliases 1/2, and console
pause. `emulator.input` takes port 1 or 2. Input is sampled at frame boundaries by
the upstream core. Pause drives NMI; it is separate from software reset.

## Inspection and addresses

Inspection reports SHA-256 of the complete input, image/data offsets and sizes,
16 KiB bank count, optional 512-byte copier prefix, TMR SEGA header location,
checksum/validation when the declared extent is available, product/version,
region and declared size. Headerless SMS images are accepted. Raw ROM sizes use
8 KiB units up to 4 MiB; debugger execution currently requires a power-of-two
payload. Whole-image rebuilding also handles non-power-of-two payloads.

SMS headers do not identify mappers. Sega is a default hypothesis; Codemasters
uses the pinned core's word-checksum heuristic. Inspection labels this basis
explicitly. Uncommon boards cannot reliably be identified from a generic SMS
header and are outside the baseline. A heuristic is not proof of board identity.

Physical `rom` addresses are complete-file offsets, including the copier prefix.
CPU aliases live in `cpu`, port accesses in `io`; `ram` (8 KiB), `cart_ram` (32 KiB),
`vram` (16 KiB) and `cram` (32 bytes) are separate backing spaces. CPU $C000/$E000
mirror the same RAM. Sega fixes $0000-$03FF to ROM bank zero; the remaining three
16 KiB windows are banked, and SRAM may replace $8000-$BFFF. Codemasters can bank
the first KiB too and overlay RAM at $A000-$BFFF. Disabled cartridge reads retain
CPU identity, not a fabricated physical ROM binding.

## Persistent debugger and analysis

`serve` uses the shared CommandAdapter transport. `api.describe` lists methods:
ROM inspection/reads, start/reset, capabilities/state, bounded operation/cycle/frame
stepping, two controllers, register/RAM edits, mapped/backing inspection, snapshot
save/load/export/import, framebuffer PPM, 48 kHz stereo WAV, bounded trace filters,
streaming, summaries, coverage, contextual roots and observed edges. Architectural
state exposes main/shadow register pairs, IX/IY/SP/PC/I/R/IFF/IM and HALT plus video
position and decoded VDP register fields. Register and RAM edits mark subsequent evidence artificial.

Operation stepping advances one core instruction, interrupt entry or HALT wait.
Cycle stepping advances to an instruction boundary and may overshoot the requested
T-cycle count. Budgets/timeouts are checked at boundaries; repeated prefixes are
limited to 64 bytes. Read/write/I/O events share the starting cycle of the core
instruction. The first actual opcode fetch is recorded once; interrupt entry and
HALT waits do not become executed instructions. Frame and bank-change events are
recorded separately. Side-effect-free CPU and backing inspection never executes
I/O reads. I/O peeks use a detached core clone; debugger writes also support
VRAM, CRAM and explicit I/O, marking the resulting state artificial. Snapshots are transactional, identity/version/checksum guarded, same-build
local artifacts. They must not be distributed with a game mapping.

Analysis includes decode, references, discover/discover_context, pointers, unknown
ranges, contexts, coverage and edges. Supply `context.rom_banks` as three effective
physical 16 KiB indices, not mapper register values, and `ram_enabled` and
`cartridge_enabled` when known. For example:

```json
{"cpu_address":16384,"context":{"rom_banks":[0,3,2],"ram_enabled":false,"cartridge_enabled":true}}
```

Known SRAM banks resolve to cartridge-RAM backing; unknown windows remain unresolved.
Static entry contracts can establish context.
Discovery respects data barriers and combines compatible observed paths when
`use_observed=true`; false is static-only. Calls and possible mapper/memory-control
writes stop static propagation. Reached targets and pointer values are hypotheses,
not executed coverage or proof of code. `analysis.pointers` accepts explicit bounded
16-bit little-endian tables, with the supplied bank context. RAM execution does
not identify immutable upload origins or lifetimes automatically.

## Mapping and reconstruction

The portable format is `romatlas-sms-map` JSON, version 1, with image SHA-256/size,
regions and next_id. Regions carry physical offset/length, kind, provenance,
confidence, evidence, optional label/comment and code execution views:

```json
{"id":0,"offset":0,"length":3,"kind":"Code","provenance":"Manual",
 "label":"Reset","comment":"Inspect the reset jump target.",
 "views":[{"cpu":"main","cpu_address":0,"context":{}}]}
```

Use `map.annotate`/`map.remove` over RPC for edits; map.get/validate/save/load and
import/export are transactional in memory. CLI init/validate/normalize and store
init/import/get/validate/export use the same identity-checked schema. Overlaps,
invalid labels, conflicting physical CPU views, truncated instructions and unsupported
uploaded views are rejected. Split existing regions first. Maps contain no ROM
bytes; unknown bytes remain unknown even though raw rebuilding preserves them.

Generated local source consists of `game.asm`, `linkfile`, `mapping.json`, diagnostics
and a local `image.sms` input. WLA memory slots retain CPU origins while fixed 8 KiB
physical banks (and a separate copier-prefix bank) preserve the exact file order. Canonical encodings become mnemonics;
ignored prefixes, unsupported aliases and wrapping branches use exact `.db` with
decoded comments. Raw regions use `.incbin ... SKIP ... READ ...`; no automatic
checksum repair or header synthesis is allowed. Fresh assembly/linking is always
compared with the entire original. Failed generation publishes no final directory;
verification rebuilds objects and rejects source changes that alter any byte.

New SMS disassembly/commentary/archive/native-port profiles preserve whole-ROM
classification, static structure versus reachability, full current-source readability
and editorial gates. Existing workspaces are not rewritten. Archives retain source
recipes and commentary corrections without game bytes; restore and port preparation
require the independently supplied identical ROM. Native scaffolds use SMS dimensions,
buttons, cadence and 4bpp helpers. Preparing a scaffold does not implement a game.

Run `python3 scripts/test-sms.py` for synthetic integration checks, plus the workspace,
port desktop/headless, session, portability and other-platform regression checks.
No full hardware or commercial-game compatibility claim is supported by these tests.
