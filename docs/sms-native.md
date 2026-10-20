# SMS native port helpers

`romatlas-port::sms` is MIT and independent of console cores and disassembly.
Ports own game rules, mapper/raster scheduling, music drivers and asset recipes.
Assets load from the user's ROM at startup through the existing replacement/extra
slot APIs. Do not copy GPL jgenesis implementations into this library.

`tile` decodes 32-byte interleaved four-plane patterns. `color` converts SMS 2-bit
RGB channels. `vdp::Config::registers` interprets Mode 4 registers; `background`
samples tilemaps with flips, palette selection, priority, horizontal/vertical scroll,
top-row horizontal lock and right-column vertical lock. `scanline` composes ordered
sprites, 8x8/8x16, zoom, left shift, eight-per-line selection, collision/overflow,
background priority, left masking, blanking and backdrop. 192/224/240 display heights
and wrapped VRAM are explicit. `rgba` converts immediately with caller-selected
CRAM, allowing palette/raster effects at a scanline boundary. `LineCounter` supplies
a deterministic reload/underflow primitive for port-owned interrupt scheduling.
These helpers target SMS2 Mode 4: TMS9918 legacy modes, SMS1 address-mask quirks,
subscanline fetch timing and hardware analog/composite output are not implemented.

`audio::Psg` implements the SN76489 latch/data protocol, three 10-bit tone periods,
four 2 dB attenuation registers, periodic/white noise, 16-bit Sega LFSR taps 0/3,
fixed or tone-2 noise clocks and the divide-by-16 input clock. `Mixer` bridges the
fractional input clock to samples and handles port $F2 PSG/FM gating. These are
native music primitives; ports supply the original scheduling and driver behavior.

`audio::Fm` provides nine approximate two-operator voices, register/custom/preset
patches, multiplier/pitch/block control, key transitions, attack/decay/sustain/release,
feedback, waveform selection, tremolo/vibrato and rhythm patch/noise synthesis.
`Fm::default`/`Mixer::default` include OPLL instrument data. The preset bytes alone
are adapted from [MIT emu2413](https://github.com/digital-sound-antiques/emu2413/tree/a1d5120903b75f93d3eb7690e34a29c0b4add903),
Copyright Mitsutaka Okazaki; see `src/sms/LICENSE-emu2413.txt`. No emulator algorithm
was copied. The original renderer is a musical approximation: logarithmic ROMs,
key scaling, cycle/slot envelopes and exact independent rhythm-operator wiring
are not reproduced. Analog output, chip revision quirks and bit-exact audio are
not claimed. The debugger uses jgenesis's separate hardware implementation.

Defaults supply 256x192 dimensions, SMS_NTSC cadence, optional SMS_PAL cadence,
button1/button2/directions/pause bindings and a three-times native window. PAL
cadence in the native helper does not enable a PAL debugger profile. No display
or audio device is initialized by these APIs or their tests.
