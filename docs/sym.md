# mgbdis map compatibility

Tested revision: `95d951a9881a0d7ff1a42a94d1be175da92b1777`.

Stock `Symbols.load_sym_file` reads two whitespace-separated fields per line: hexadecimal `bank:address` and a label or directive. Blank lines and lines starting with `;` are comments. Inline comments are not supported. Stock looks for the map next to the ROM, with the same stem and `.sym` extension.

```text
00:0100 entry_00_0100
00:0100 .code:4
01:4000 .data:100:2
01:4100 .text:10
01:4200 .padding:20
02:4000 .image:100:w128,pe4
```

Supported directives: `.code`, `.data` (alias `.byt`), `.text` (alias `.asc`), `.padding`, `.image`. Lengths use hex. A third field after `:` supplies directive arguments: data bytes per line, text charmap selection, or image options. Image widths use decimal. ROMAtlas preserves supported arguments and checks their syntax. With a ROM, validation also checks instruction boundaries, padding bytes, tile lengths, and label positions. Labels inside instructions, padding, or image blocks need a region split or raw data. Generation rebuilds with pinned RGBDS and compares every byte before it publishes the source folder. A failed build or mismatch rejects the map and leaves no output folder; use raw data for the affected region and retry. External charmaps need a separate file and are not yet wired into the CLI.

Bank 0 ROM addresses are 0000–3fff; other ROM banks use 4000–7fff. RAM/I/O labels can use addresses through ffff. Stock treats labels outside the switchable ROM window as bank 0. ROMAtlas rejects duplicate label names, duplicate effective label addresses, overlapping regions, unknown directives, zero lengths, and regions crossing a bank. Local labels with a leading dot and no colon are labels, not directives.

Export sorts entries by bank, address, and annotation. It preserves supported annotations, not comment formatting or line order. Unknown regions stay internal; there is no `.unknown` directive. No hash header is required. Maps do not contain bytes or extracted assets. Mapping metadata can still raise legal questions; export does not give legal clearance.

## Stock RAM/I/O label limit

The pinned generator uses non-ROM labels in operands but emits no EQU definitions for them. An imported label such as `00:c000 wBuffer` can therefore cause an undefined-symbol link error. ROMAtlas leaves the stock emitter intact and adds a separate metadata-only constants include. This does not change any ROM byte.

A recipient can use only the exported `.sym`, their ROM, and stock tools. Supply undefined non-ROM labels with RGBDS -D (do not redefine symbols already in hardware.inc):

```sh
python3 tools/mgbdis/mgbdis.py game.gb --output-dir source
cd source
# Build image assets first if the map includes .image blocks.
make gfx/Tiles.1bpp
rgbasm -D 'wBuffer=$c000' -o game.o game.asm
rgblink -o rebuilt.gb game.o
```

The same definitions come from the bank:address label entries at addresses 8000–ffff. No internal analysis state is needed. Skip rgbfix to preserve headers/checksums exactly. The session integration test proves this path for an imported map with aliases, text, a 1bpp image, and a RAM label. Custom charmap resources and local RAM label scopes need separate support; generation returns a clear error where it cannot preserve them.

## Byte fidelity checks

`python3 scripts/test-fidelity.py` checks all 256 byte values in text, 1bpp/2bpp images, reversed palettes, padding and code. It also checks rejected invalid content, missing charmaps, lossy palettes, and an injected linker mismatch. Generation shares one deadline across emission, assembly, linking, and byte comparison. It removes its check object and rebuilt ROM before it publishes the source folder.
