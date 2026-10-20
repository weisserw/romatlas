# NES native-port helpers

`romatlas-port::nes` is independent MIT Rust, with no emulator or adapter
imports. It supplies mechanisms to source-informed native game implementations;
it does not execute a ROM or implement game rules. Keep ROM bytes/assets local.

`tile` decodes 16-byte NES CHR tiles (planes separated by eight bytes).
`ppu::Background` samples nametables and attribute quadrants with horizontal,
vertical, single-screen or four-screen mirroring, caller-selected 1 KiB CHR pages,
pattern-table selection and scroll spanning four nametables. Ports can supply
scanline/pixel-specific configurations for split screens, mapper IRQ scrolling,
CHR animation/bank effects and palette changes. `sprites` selects eight in-range
sprites per line, supports 8x8/8x16 geometry, pattern-table selection, flips and
OAM priority. `compose` applies PPUMASK enables and left-edge clipping, background
priority and sprite-zero-hit conditions. Palette aliasing, grayscale and RGB
emphasis helpers operate on a caller-supplied 64-color palette.

This is sampled composition, not a PPU bus/timing emulator. The caller owns raster
effect scheduling, scroll-register semantics, mapper IRQs, frame-latched flags and
game-specific sprite flicker. Overflow reports more than eight in-range sprites;
the diagonal hardware overflow bug is not reproduced. Emphasis uses a documented
RGB attenuation approximation, not NTSC composite voltage decoding. No fixed
palette or game assets are copied from an emulator.

`apu::Apu` provides CPU-clocked NTSC register-driven pulse 1/2, triangle, noise
and DMC. It includes envelopes, length/linear counters, duty sequencing, pulse
sweep and channel-one negate difference, noise LFSR modes, DMC address wrap/loop/
IRQ and held DAC level, frame sequencer modes/IRQ, and nonlinear pulse/TND mixing.
`write` accepts APU hardware register addresses. `clock` advances one CPU clock,
fetching DMC data through a caller callback and returning the fetched address.
The port owns bank translation, driver sequencing, scheduling and any CPU/DMA
stall accounting. `status` clears the frame IRQ; `irq` inspects without clearing.
`levels` exposes five DAC levels. `render` box-averages clocks into interleaved
stereo f32 output at a requested sample rate, preserving fractional timing and
clone replay.

The helper excludes expansion-chip audio, PAL, analog filtering/composite effects,
cycle-exact DMC DMA/controller conflicts and the CPU bus. Audio is unipolar before
caller filtering/DC removal. It is a native sound primitive, not a validated
hardware APU replacement. Original synthetic tests exercise register scheduling,
DMC fetch/wrap/loop/IRQ, frame IRQ modes, deterministic audio replay, planar layout,
scroll/attributes, CHR paging, tall sprite flips, clipping/priority and zero hit.
Hardware/real-game fidelity remains unverified.

Hardware descriptions: [NESdev PPU rendering](https://www.nesdev.org/wiki/PPU_rendering),
[APU registers](https://www.nesdev.org/wiki/APU_registers),
[pulse sweep](https://www.nesdev.org/wiki/APU_Sweep),
[nonlinear mixer](https://www.nesdev.org/wiki/APU_Mixer).
No GPL emulator implementation was copied into these helpers.
