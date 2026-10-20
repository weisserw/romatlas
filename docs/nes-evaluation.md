# NES core selection

2026-10-20. Target: NTSC standard controllers, baseline 2A03/2C02 hardware,
common cartridge boards; rare peripherals, expansion audio, PAL and uncommon
mappers may follow. No non-Rust core port is needed.

| Candidate | Source findings | Integration decision |
| --- | --- | --- |
| [jgenesis](https://github.com/jsgroth/jgenesis/tree/7b3cc06196f9cb9ac5cb067ffb3cdbc8c4efe282/backend/nes-core) | Rust cycle-driven CPU, PPU/APU, OAM/DMC DMA, common and additional mappers, bincode state support, separate headless sinks. GPLv3. CPU registers and instruction completion already exposed. Mapper implementation exposes immutable baseline CPU reads; backing getters allow exact physical identity hooks. | **Selected.** Shares the existing pinned SNES fork, common sinks, serialization and CPU-cycle architecture. Small local hooks can observe actual accesses without replacing hardware. This is the strongest integration fit for this repository, not a measured accuracy ranking. |
| [TetaNES](https://github.com/lukexor/tetanes/tree/a0a6b17f8ba9c5ee451453fb2409753fd06e5a31/tetanes-core) | Separate Rust core, MIT/Apache-2.0, Rust 1.88 declared. Instruction/frame stepping, pure CPU peeks, serialized states and debugger access hooks. Source exposes configurable startup RAM and debugger/code-map support. | Credible alternative with richer existing debugger machinery. Adopting it would add a separate state/bus/mapper integration and dependency family rather than reuse the current fork. Source reviewed; no equivalent compatibility or performance gate was run. |
| [pinky](https://github.com/koute/pinky) | Rust NES implementation with standalone library and frontend components. | Secondary option; no demonstrated repository integration advantage over the two leading candidates. Not built/tested. |

The pinned jgenesis revision is v0.11.4, the revision already used for SNES;
it is not claimed to be the newest version. Modern upstream still advertises
NES support and common mappers. Newer fixes may merit later backports. Neither
upstream feature lists nor synthetic tests establish universal compatibility.

## Adaptation

Vendor only NES core, 6502 CPU, NES config and its YM-OPLL build dependency
beside the existing core-only workspace. Preserve upstream GPL notices and
revision attribution. The linked CLI is already GPLv3; the independent native
runtime retains MIT licensing and has no console-core dependency. The user
explicitly accepted GPL for this extension.

Instrumentation records actual opcode fetches (excluding interrupt dummy fetch),
reads/writes, PPU reads, DMA requests, interrupt entry and effective PRG mapping
changes. Physical binding comes from actual cartridge backing getters, never
matching byte values. Baseline immutable CPU inspection calls only immutable
mapper read methods; PPU inspection uses a detached clone so mapper latches and
hardware registers remain unchanged. RAM is initialized deterministically.
Adapter-owned snapshots carry ROM identity, version and payload checksum, and
restore a decoded candidate transactionally before adopting it.

[ca65/ld65](https://cc65.github.io/doc/ca65.html) V2.19, revision
555282497c3ecf8b313d87d5973093af19c35bd5, are external build tools. Their
6502 operand width overrides and physical output segments support exact rebuilding
while preserving absolute-vs-zero-page encodings. The Rust decoder covers all
256 NMOS encodings. Ambiguous unofficial opcode aliases and wrapping branches
remain exact byte directives with decoded mnemonic comments. This preserves
encoding identity without guessing which spelling an assembler will select.

The measured integration gate uses original synthetic images only. See
[nes.md](nes.md) and the dated progress entry for tested facts and remaining limits.
