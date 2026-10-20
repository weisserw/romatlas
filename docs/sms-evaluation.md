# Sega Master System integration choices

SMS is added alongside Game Boy, NES and SNES. Baseline execution is NTSC SMS2,
standard two-player pads, Sega banking/SRAM and Codemasters banking/RAM. No Game
Gear, SG-1000, BIOS boot, light gun, four-player adapter or uncommon mapper claim.

The debugger reuses [jgenesis v0.11.4](https://github.com/jsgroth/jgenesis/tree/7b3cc06196f9cb9ac5cb067ffb3cdbc8c4efe282/backend/smsgg-core),
the existing approved GPLv3 fork. Its Rust SMS/GG core has instruction-driven Z80,
VDP, SN76489 and optional YM2413, headless sinks and bincode snapshots. Added hooks
observe actual memory/I/O calls and first fetches after interrupt/HALT handling;
backing identity is derived from mapper state, never matching byte values. The
upstream core advances peripherals after an instruction, so bus events share the
instruction's starting cycle. This is not a cycle-exact debugger. Prefix chains
are bounded before execution. Core-only sources and upstream license stay vendored.

[WLA-DX](https://github.com/vhelin/wla-dx/tree/89a90a56be5c2b8cf19a9afa3e1b32384ddb1a97)
10.6, revision 89a90a56be5c2b8cf19a9afa3e1b32384ddb1a97, supplies external
`wla-z80` and `wlalink`. Its physical ROM-bank layout and CPU slots allow
exact image reconstruction while retaining each code view's CPU address. No SMS
header/checksum generation directive is used; original headers remain unchanged.

[DeZ80](https://github.com/rzumer/dez80/tree/c6a82972c4e42c0affec4ecaeb5b707b75983219)
4.0.1, revision c6a82972c4e42c0affec4ecaeb5b707b75983219, is a separate MIT Rust
Z80 disassembler library with base/CB/ED/DD/FD/indexed-CB decoding, ignored-prefix
retention, operands and exact byte recovery. Selected rather than writing another
opcode table. ROMAtlas handles SMS contexts, successors, references and WLA syntax;
DeZ80 does not infer SMS banks or establish structural classification. The vendored
source retains its MIT license; development benchmark dependencies are omitted.
Standalone z80dismblr and smart-disassembler tools were located, but a separate
process/runtime and their linear-image assumptions offer no demonstrated advantage
for bounded typed decoding in this toolkit. No accuracy ranking is claimed.

Native runtime helpers remain MIT, independent of the GPL core and the decoder.
Only the instrument preset data is taken from MIT emu2413; its notice is included.
Native FM synthesis is an approximate original implementation, not copied jgenesis
or a hardware-equivalence promise. See [sms-native.md](sms-native.md).
