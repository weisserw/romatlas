# Independent Genesis native helpers

`romatlas_port::genesis` is original MIT code and imports no console emulator,
adapter, instruction table or GPL implementation. Native games own gameplay,
cartridge mapping, CPU-to-native logic, sound/music drivers and raster scheduling.
The hardware vocabulary follows Sega's
[Genesis Software Manual](https://nemesis.hacking-cult.org/MegaDrive/Documentation/GenesisSoftwareManual.pdf).
These helpers provide source-informed implementation building blocks, not a
hardware equivalence claim or native-versus-ROM validation gate.

`tile` decodes packed 4-bpp high-nibble-first 8x8 patterns. `color` implements
9-bit CRAM with normal/shadow/highlight levels. Settings supply directions,
A/B/C/X/Y/Z/Start/Mode and 320x224 defaults; TickRate provides NTSC and PAL.

`vdp::Vdp` owns 64 KiB VRAM, 64 CRAM words, 40 VSRAM words and 24 registers.
Its scanline renderer supports planes A/B, window, 32/64/128-cell plane sizes,
full/cell/line horizontal scroll, full/two-cell vertical scroll, palettes,
transparency, priority, backdrop/blanking, H32/H40, 224/240-line output,
interlace-mode-2 patterns, sprite links and column-major pattern selection,
horizontal/vertical flips, scanline sprite/pixel limits, collision/overflow and
shadow/highlight sprite operators. Render with the game-owned register/memory
state appropriate to each line; split lines externally for midline effects.

Control/data helpers support register/address commands, auto-increment, CRAM and
VSRAM masking. `dma` accepts a big-endian CPU-word callback for bus DMA and handles
VRAM copies; fill is triggered by the next data write after a fill command.
No CPU callback is used for VRAM copy/fill. Transfers are immediate at the caller's
scheduled time. FIFO contention, bus arbitration, exact fill/copy timing,
invalid-mode quirks, sprite cache timing and analog output remain limits.

`audio::Ym2612` accepts two address/data banks or direct register writes, and
provides six channels, four operators per channel, eight FM algorithms, feedback,
detune/multipliers, total levels, key-on/off, ADSR-like envelopes, sustain levels,
SSG-style envelope controls, LFO amplitude/phase modulation, channel-three special
frequencies, timers/status/reset, CSM-style key triggering, stereo masks and channel
six DAC replacement. `clock` advances timer clocks; `sample` advances synthesis at
the configured output rate. `Mixer` combines it with the existing independent
clocked Sega PSG, preserving fractional FM/PSG clock remainders and gains.

FM envelopes, detune, key scaling, LFO and SSG/CSM behavior use explicit digital
approximations. Exact Yamaha phase/envelope tables, frequency-write latches,
slot-level timing, busy behavior, DAC ladder distortion, analog filters and
subsample register changes are not claimed. Ports may replace individual music
voices or supply a more accurate independently licensed synthesizer if needed.
The debugger continues to use jgenesis hardware emulation, separately licensed.

Focused headless tests exercise plane/window/priority/shadow composition, linked
sprite ordering/flips/collision, bus DMA and VRAM-copy addressing, timer status,
DAC panning, all eight FM algorithms and clone replay. They do not assert broad
hardware fidelity or require display/audio devices.
