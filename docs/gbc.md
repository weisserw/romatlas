# Game Boy Color

Game Boy Color is a hardware model of the existing `gb` platform. It keeps the
Mizu Rust core, SM83 decoder, stock mgbdis `.sym` mappings and RGBDS assembler.
No separate platform/map format or emulator dependency is needed. Existing DMG
commands, mappings, archives and port helpers retain their interfaces and behavior.

## Execution and inspection

`.gbc`, `.gb` and workspace `.rom` images use header byte $143 for automatic
model selection: bit 7 selects CGB. `emulator.start` accepts `model: "auto"`,
`"cgb"` or `"dmg"`; CGB-only images (bits 7/6 set) reject DMG execution without
replacing a running emulator. Reset preserves the chosen hardware model. Header
text is decoded lossily for display; non-UTF8 title bytes do not block execution
or change the original image/hash.

Supported common controllers remain ROM-only, MBC1, MBC3 and MBC5. Execution
includes CGB banked VRAM/WRAM, color BG/window/object rendering, palettes, OPRI,
KEY1 speed switching, general/HBlank DMA and selected peripheral behavior.
Uncommon controllers, link peripherals, early CGB revisions and broad commercial
game compatibility are outside the measured baseline. MBC3 RTC advances from
emulated time only; double speed does not double the RTC, LCD or APU clock.

Query `emulator.capabilities` for storage sizes. Physical `vram` is bank 0 followed
by bank 1 (16 KiB); `wram` has eight 4 KiB banks (32 KiB). CPU aliases and access
locks remain separate. `emulator.memory` reads the mapped bus; blocked reads do
not establish physical backing evidence.

Two additive immutable backing spaces expose all palette bytes, including during
mode 3, without changing the palette index, cycles, snapshots or trace evidence:

```json
{"method":"emulator.physical_memory","params":{"space":"cgb-bg-palette","offset":0,"length":64}}
{"method":"emulator.physical_memory","params":{"space":"cgb-obj-palette","offset":0,"length":64}}
```

Each contains eight four-color palettes, little-endian RGB555, with original bit
15 retained. They are unavailable on DMG hardware (capability size zero). CPU
palette ports retain their locking and auto-increment behavior. These spaces are
inspection results; palette reads are not ROM classification evidence.

Snapshots now preserve both LCD frame buffers, the selected buffer and the partial
render position, so restore immediately returns the saved image and partial-frame
replay remains deterministic. The internal Mizu snapshot version is 9. As before,
external snapshot files are same-build local artifacts, not a portable persistence
format; older core versions are explicitly rejected. Public snapshot RPCs retain
their existing signatures. No proprietary boot ROM is included.

## Disassembly, rebuilding and workflows

CGB uses the same complete SM83 instruction decoder and physical 16 KiB ROM banks.
Speed switching introduces no new instruction encoding. Stock `.sym` directives
and RGBDS preserve CGB header bytes, palette tables and attributes exactly. Use
`map.import` for `.sym`, `map.load` for internal JSON, and existing annotate/export/
generate/verify commands. Read [sym.md](sym.md) for RAM labels and stock-only EQU
workarounds. Never run header-repair tools during exact verification.

New GB mapping and native-port workspaces point here; existing workspaces are not
rewritten. Archives retain platform `gb` even for CGB-only cartridges. Generation
and byte-identical verification establish reconstruction, not classification of
unknown bytes. Whole-ROM classification, reachability, current-source readability
and editorial requirements remain unchanged.

## Native port helpers

`romatlas_port::gb::cgb` adds independent MIT code alongside the existing `gb`
helpers. Existing `gb::objects::draw`, `gb::sound::Sound`, palette conversion and
shared graphics APIs keep their signatures and DMG semantics. CGB native helpers
have no emulator dependency or devices. `TickRate::CGB` equals `TickRate::DMG`:
4,194,304 / 70,224 frames per second, regardless of CPU double speed.

- `Palettes` owns 64 raw bytes and an indexed data port, including blocked-write
  increment/wrap behavior. `color` converts RGB555 with the existing `cgb_color`.
- `Registers` supplies LCDC, scrolling, window coordinates and OPRI selection.
- `Video` borrows two physical VRAM banks, up to forty raw OAM entries and separate
  BG/OBJ palettes. `render` draws a static-register 160x144 frame.
- `render_scanline` allows game-owned raster updates and takes an explicit internal
  window line (`None` disables the window). It returns whether the window rendered;
  the port owns WY triggering and increments the counter only on rendered lines.

Rendering supports unsigned/signed tiles, bank and palette attributes, both flips,
scroll wrap, BG/window map selection, 8x8/8x16 objects, ten-object selection including
X-hidden entries, OAM or coordinate priority, transparency and CGB BG/OBJ priority.
Object priority resolves before BG masking; LCDC bit 0 controls master priority
rather than disabling CGB background rendering. Source indices remain available
through `Frame::background_index`.

The common `gb::sound::Sound` synthesis remains usable at the normal 4,194,304 Hz
APU clock; do not feed double-speed CPU ticks directly into it. It models selected
pulse/wave/noise/sweep/envelope/stereo effects, not every CGB APU revision or wave
RAM bus quirk. Ports own audio drivers, source scheduling, DMA-equivalent state
updates and gameplay. Native rendering is line-based, not FIFO/dot-exact; LCD
response/color correction, mid-line window glitches and bus contention are outside
these helpers. Native colors use RGB555 expansion; the emulator framebuffer uses
its existing display color transform. No native-versus-ROM fidelity is asserted.

Hardware rules follow [Pan Docs tile maps](https://gbdev.io/pandocs/Tile_Maps.html),
[OAM](https://gbdev.io/pandocs/OAM.html) and [LCDC](https://gbdev.io/pandocs/LCDC.html).

## Checks

Run `python3 scripts/test-gbc.py` after CGB changes. It builds only an authored
synthetic cartridge and checks runtime banked color rendering, DMA/speed switching,
immutable palette inspection, mid-frame snapshot replay, CGB-enhanced/CGB-only
stock reconstruction with banked code/RAM labels, and archive-to-native preparation.
Run the existing session, portability, fidelity, native-port, licensed homebrew and
hardware gates as well. The 89-case default and separate 55-case CGB Mooneye
selections remain limited measurements; they do not prove full compatibility.
