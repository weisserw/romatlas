# Independent PC Engine native runtime helpers

`romatlas_port::pce` is original MIT code, independent of the GPL debugger and
without a console-core dependency. Ports own source-informed game rules, memory,
mapper use, raster/audio scheduling and music drivers. Default screen is 256x240,
with I/II/Run/Select/directions and `TickRate::PCE_NTSC` (master clock divided by
1365*262 clocks). VCE selects 263 lines where required; runtime display dimensions
and tick rate are configurable for source timing.

* `tile`: 32-byte four-plane 8x8 backgrounds; `sprite_tile`: 128-byte four-plane
  16x16 sprite cells. `color`: linear GRB333 and luma greyscale.
* `Vce`: 512 nine-bit colors, palette address/read/write auto-increment, greyscale,
  three dot-clock divisors, 262/263-line timing and indexed RGBA conversion.
* `Vdc`: 64 KiB word VRAM, 64-entry SATB, register port/address increments and read
  latch; background BAT sizes/wrapping, scroll, palettes/transparency; 16/32-wide,
  two-plane CG masks, 16/32/64-high sprites with flips/pattern alignment, priority, sprite-zero
  collision and line overflow; raster compare, IRQ/status acknowledgement,
  VRAM source/destination increment/decrement DMA and VBlank SATB DMA/repeat.
  `scanline(y,width)` is called between raster register/palette updates. DMA
  methods complete explicitly at a game-selected scheduling boundary.
* `Psg`: six 32-sample five-bit wave channels, frequency/register drivers, direct
  D/A samples, noise on channels 4/5, channel-1-to-0 LFO and stereo logarithmic
  attenuation. `Mixer` advances PSG clocks using fractional output-sample clocks
  and supports deterministic cloning/replay. CPU speed switches do not alter PSG
  input clock. Ports can write DDA at their source sample/driver cadence.

These helpers implement native rendering/audio mechanisms, not a CPU emulator.
Exact VDC VRAM arbitration/bus waits, sub-dot fetch latching, undocumented register/open-bus effects, exact PSG revision
quirks/analog filters and waveform integration are not hardware fidelity claims.
Native helpers use straightforward scanline rendering and sample synthesis;
ports explain source-specific approximations. Full hardware/game compatibility
and CD/SuperGrafx peripherals are outside this baseline.

Hardware behavior references include [Charles MacDonald’s measured hardware notes](https://github.com/asterick/TurboSharp/blob/master/Text/pcetech.txt), the
[VDC reference](https://datacrystal.tcrf.net/wiki/VDC_Reference_Manual),
[MagicEngine PSG programming documentation](https://www.magicengine.com/mkit/doc_hard_psg.html)
and the [HuC6280 PSG reference](https://github.com/drhelius/Geargrafx/blob/main/skills/geargrafx-debugging/references/huc6280_psg.md).
No GPL emulator implementation was copied into these helpers.

Run `cargo test --locked --release -p romatlas-port --test pce`, the desktop-feature
Clippy/tests (without initializing devices), `python3 scripts/test-port.py` and
`python3 scripts/test-pce.py` after relevant changes.
