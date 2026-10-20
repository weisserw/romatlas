# Genesis integration choices

Genesis joins the existing platforms using the approved GPLv3
[jgenesis v0.11.4 revision](https://github.com/jsgroth/jgenesis/tree/7b3cc06196f9cb9ac5cb067ffb3cdbc8c4efe282/backend/genesis-core).
The same revision supplies genesis-core, genesis-config and m68000-emu; its
existing Z80, PSG, YM2612, VDP, controller, mapper and serialization components
remain in the debugger. Upstream licenses and attribution remain in vendor/jgenesis.
No emulator implementation enters the MIT native runtime.

The assembler and main disassembler are external GNU binutils 2.42:
[GNU as M680x0 documentation](https://sourceware.org/binutils/docs/as/M68K_002dDependent.html),
[objdump](https://sourceware.org/binutils/docs/binutils/objdump.html), ld and objcopy.
GPLv3+ binutils can assemble 68000 code, link sections with separate CPU virtual
and physical load addresses, decode bounded raw images and emit headerless binary
cartridges. This avoids another handwritten instruction table. Setup accepts the
2.42 Debian/Ubuntu cross package or builds the SHA-256-pinned upstream 2.42 source.
Tools built in tools/m68k take precedence. GNU's exact syntax and version should
be retained when distributing reconstruction instructions.

[Musashi](https://github.com/kstenerud/Musashi) has an MIT C disassembler;
[Capstone](https://github.com/capstone-engine/capstone) has an M68K backend;
[r68k](https://github.com/marhel/r68k) exposes Rust disassembly.
They are viable alternatives, but sharing GNU's assembler/disassembler syntax
avoids an additional instruction table dependency and dialect conversion.
[vasm](https://sun.hasenbraten.de/vasm/release/vasm.pdf) was also located; its
source-available copyright terms were not selected as the open-source assembler.
This is an integration choice, not a comparative accuracy ranking. DeZ80 4.0.1
and WLA-DX 10.6, already pinned for SMS/GG, cover the Genesis Z80 sound CPU.

The adapter retains original-file SHA-256 and reconstructs big-endian,
word-swapped and SMD-interleaved files, including their copier prefix. Analysis
uses a canonical big-endian physical backing; encoding conversion is explicit,
reversible and separate from CPU alias translation. Header checksums are inspected
and preserved, never repaired. Runtime uses an exact cartridge constructor that
skips upstream image conversion and game-specific ROM rewrites.

Probes distinguish accepted 68000 instructions from prefetch reads, interrupt
entry, STOP and DMA waits. Prefetch backing/context are retained with the CPU.
Z80 fetches, actual CPU bus operations, interrupts, committed VDP writes, DMA
sources/targets and bank transitions enter bounded trace APIs. Peripheral timing
is upstream instruction-driven timing; events within an instruction do not claim
individually resolved hardware bus cycles. Broad hardware/game fidelity is not
established by synthetic integration tests.

Baseline: NTSC/PAL Genesis/Mega Drive cartridge hardware, two standard six-button
pads (three-button game software is compatible), linear ROM, SRAM and Sega SSF
banking. Header/serial mapper identification is an explicit hypothesis, not a
universal mapper ID. SVP, Sega CD, 32X, uncommon boards and exotic peripherals
are outside this baseline. EEPROM-backed serial I/O never becomes a physical
ROM claim. See genesis.md for APIs and genesis-native.md for native helper limits.
