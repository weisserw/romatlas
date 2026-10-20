# Game Gear tool selection

The existing pinned [jgenesis smsgg-core](https://github.com/jsgroth/jgenesis/tree/7b3cc06196f9cb9ac5cb067ffb3cdbc8c4efe282/backend/smsgg-core)
is the debugger as requested. Source inspection confirms GameGear hardware,
12-bit CRAM, LCD cropping, Start polling and stereo PSG routing. The GPL core
and its probes remain isolated from the independent MIT native port library.
The underlying serial/EXT implementation is incomplete; no link support claim follows.

[WLA-DX 10.6](https://github.com/vhelin/wla-dx/tree/89a90a56be5c2b8cf19a9afa3e1b32384ddb1a97)
already supplies pinned wla-z80/wlalink. Its documentation covers SMS/GG tags,
and ordinary Z80 code plus fixed physical banks can reconstruct raw GG images
without applying automatic header/checksum directives. The existing Sega backend
therefore fits GG without another assembler dependency.

[DeZ80 4.0.1](https://github.com/rzumer/dez80/tree/c6a82972c4e42c0affec4ecaeb5b707b75983219)
is the existing MIT Rust decoder for the same Z80 instruction set. ROMAtlas owns
bank contexts, structural annotations and WLA syntax, rather than implementing
another instruction table. The full shared encoding regression is run under GG.

Also located [crater](https://github.com/earwig/crater), an MIT C Game Gear
assembler/disassembler. Its README describes assembly to .gg and reverse
conversion, while explicitly noting that disassembly cannot distinguish code
from data. It adds no demonstrated advantage over the already integrated bounded
DeZ80 API and tested WLA reconstruction pipeline. This is a fit assessment from
primary documentation, not a comparative accuracy or performance benchmark.
