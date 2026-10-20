# TurboGrafx-16 / PC Engine HuCard toolkit

Select `.pce`/`.tg16` automatically, or use `--platform pce`. CD-ROM, SuperGrafx,
multitaps/light guns and uncommon boards are outside the baseline. Debugging uses
an isolated GPLv3 jgenesis v0.13.0 fork; existing platform cores are unchanged.
Read [evaluation](pce-evaluation.md) and [native helpers](pce-native.md).

```sh
bash scripts/setup-pce.sh
cargo build --locked --release
./target/release/romatlas inspect /path/to/owned.pce
./target/release/romatlas map init /path/to/owned.pce --output /tmp/owned.json
./target/release/romatlas serve --rom /path/to/owned.pce --store /tmp/owned.json
./target/release/romatlas generate /path/to/owned.pce --map /tmp/owned.json --out /tmp/owned-source
./target/release/romatlas verify /path/to/owned.pce --generated /tmp/owned-source
python3 scripts/test-pce.py
```

Inspection preserves SHA-256 of the entire original file, payload size, 8 KiB
physical banks, bank-zero reset vector and optional 512-byte copier prefix.
HuCards have no universal cartridge header, title or mapper ID: `cartridge_header`
is null and mapper identification includes its basis. Known upstream CRC32 values
identify SRAM/SF2 boards; size otherwise suggests linear or unknown. Inspection
never repairs content or claims heuristic identification is authoritative.
Byte-order is ordinary HuCard bytes; automatic bit-reversal guessing is not done.
Unknown large boards may still rebuild but are rejected for execution.

Physical `rom` offsets include the copier prefix; CPU aliases exclude it.
The 16-bit HuC6280 CPU uses eight 8 KiB MPR windows into a 21-bit bus. `context.mpr`
contains eight effective bus-page values or nulls; they are not immutable file-bank
indices. Reset guarantees MPR7=0; debugger initial values of other registers/RAM
are deterministic zeros rather than random hardware power-on claims. Context also
has optional `sf2_bank` for the cartridge 512 KiB selector. 384/512 KiB chip layouts
have explicit mirrored aliases; incomplete chips mirror recursively. Mapper
identification precedes runtime expansion so known CRCs remain identifiable.
Physical ROM identity is never the expanded runtime buffer.

Bus $000000-$0FFFFF is HuCard mapping, $1F0000-$1F7FFF mirrors 8 KiB RAM, and
$1FE000-$1FFFFF is I/O. CD/unmapped pages do not resolve as ROM. Known Populous
SRAM pages remain SRAM. Zero page/stack are CPU $2000/$2100, through MPR1.
Static translation requires the relevant MPR; unknowns remain local to the
instruction/view. TAM invalidates its selected MPRs; calls do not prove the returned
bank state. Discovery can merge compatible observed context/edges, with data
barriers and unresolved-window diagnostics. Reached targets are not execution
coverage. RAM fetches never prove immutable ROM upload origins or lifetimes.

`romatlas-pce-map` version 1 is ROM-free JSON with image identity, `next_id` and
physical `regions`: id/offset/length/kind/provenance/confidence/label/comment/evidence/
views. Code views contain `cpu:"main"`, `cpu_address` and `context`. Only code has
execution views. Each instruction byte must map contiguously to its declared
physical byte. Uploaded views are rejected. Regions cannot overlap; split existing
regions first. Rejected annotation/import edits leave the active map unchanged.
Use map.import/export or map.load/save; no stock Game Boy .sym syntax applies.

Example code region, under a supported MPR entry contract:

```json
{"id":0,"offset":0,"length":2,"kind":"Code","provenance":"Manual",
 "label":"Entry","evidence":"supported entry contract",
 "views":[{"cpu":"main","cpu_address":57344,
 "context":{"mpr":[null,null,null,null,null,null,null,0]}}]}
```

JSON-RPC uses the shared stdio transport and method names in docs/protocol.md.
`api.describe` enumerates supported methods. `emulator.start`, state/reset/input,
registers.set, memory.write, bounded step/run (operation/cycle/frame), snapshots,
framebuffer/PPM and audio/WAV are available. Operation counts accepted instructions,
finishing an incremental block transfer; interrupt handling and transfer continuation
are not fabricated instruction fetches. Cycle stepping counts master clocks and
may overshoot to a core advancement boundary. Both timeout and advancement budgets
apply. Pad port 1 accepts `i`, `ii`, `run`, `select`, `up`, `down`, `left`, `right`.
The default uses TurboGrafx-16 region input-identification bits; emulator.start
accepts region:"pce" or region:"tg16". Reset preserves region; cross-region
snapshot restores are rejected.

State includes main CPU registers/flags, MPR context and VDC/VCE/PSG/timer/IRQ
register information. Debug register/backing edits are marked artificial and break
observed edge chains. `emulator.memory` supports CPU, bus and detached I/O reads;
immutable `emulator.physical_memory` supports ROM/RAM/SRAM/VRAM/CRAM/SATB backing.
VRAM/CRAM/SATB addresses are byte offsets, words little-endian. CPU/bus writes invoke
hardware behavior; backing edits are direct and validated before modification.
ROM writes do not change immutable bytes. Detached I/O inspection operates on a
clone and does not acknowledge interrupts, advance latches or time in live state.

Snapshots guard ROM identity/version/core checksum, exact serialization consumption,
clock and storage configuration before replacing live state. They preserve CPU,
MPR/mapper, timer, audio phases, pending DMA and partial VDC frame storage. Snapshot
replay rewinds cycle values; trace file sequence stays monotonic. Snapshots are
same-build local artifacts, not a stable interchange format or a security boundary.

Trace configure/query/summary/clear, bounded JSONL streaming, coverage/context roots
and edges use the shared analysis engine. Actual fetches, bus reads/writes,
interrupt entry, bank changes, committed VRAM/CRAM writes, DMA and frames are exposed.
Fetch context keeps the exact CPU alias; ordinary bus probes retain physical bus
address and possible aliases separately. VDC internal events retain backing identity rather than a
fabricated CPU alias. Zero detail capacity retains coverage/context/edge summaries.
Filters do not turn next-PC hypotheses into execution coverage. CPU bus timestamps
use master clocks; peripheral bursts may share the enclosing synchronization time.

analysis.decode/references/discover/discover_context/pointers are bounded and do
not change maps. CLI physical decoding uses a canonical $E000 window for display;
that default is not proof of a runtime MPR. RPC supplies explicit CPU/MPR context.
Pointers use `width:2,layout:"le16"`, or explicit `width:3,layout:"bank8_first"` /
`"bank8_last"` for immutable payload 8 KiB bank fields. Conflicting/out-of-range banks
remain unresolved. Indexed/indirect references remain hypotheses without an effective
address. Every pointer target retains `code_proven:false`.

The pinned ca65 HuC6280 assembler/ld65 linker reconstruct all original file bytes.
Generated `game.asm` uses `.setcpu "huc6280"`, `.segment "IMAGE"`, `.org` CPU views,
size overrides, and local `.incbin "image.pce",offset,length` for data/unknowns.
`layout.cfg` fixes physical output placement. No automatic header/checksum repair.
Undefined/assembler-boundary instructions retain exact bytes with decoded comments.
Generation assembles and compares before publishing; failures leave no final output.
Generation/verification share one bounded deadline; verification rebuilds game.o.
Raw reconstruction alone never establishes classification or editorial completion.

New disassemble/commentary/archive/reference-correction/native-port workspaces
select this profile without rewriting existing workspaces. All structural-versus-
reachability, whole-ROM classification, full current-source readability and
editorial gates remain. Archives contain metadata/source recipes, not game bytes;
restoration needs an independently supplied identical ROM. Native preparation
restores the exact reference and pending behavior accounting; it is not a ported game.

The integration tests use only authored synthetic cartridges. Full hardware or
commercial-game compatibility is not claimed; upstream peripheral behavior and
known limits remain. No copyrighted game is needed for validation.
