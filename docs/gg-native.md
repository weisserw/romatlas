# Game Gear native helpers

`romatlas-port::gg` is independent MIT code. It reuses the native SMS four-plane
tile decoder and Mode 4 renderer, without importing emulator code. `scanline`
composes the full 256-pixel line before taking LCD columns 48..208 and lines
24..168 (40..184 in 224-line mode). This preserves full-line sprite selection/collision/overflow behavior.
Scroll, tile flips, background priority, tall/zoomed sprites, masking, backdrop,
blanking and line-counter helpers are inherited through `gg::vdp`.
`color(u16)` converts little-endian 12-bit CRAM words (RGB nibbles);
`gg::vdp::rgba` colors a full Mode 4 line with 64-byte CRAM; the top-level
`scanline` colors and crops it immediately, allowing port-owned
scanline palette and register effects. Ports own raster/interrupt scheduling.

`gg::audio::Mixer` supplies clocked three-tone/noise PSG synthesis and independent
left/right channel gates from I/O $06 (high/low nibbles). It defaults to all
channels on both sides, 3,579,545 Hz input and 48 kHz samples, with deterministic
fractional clock accumulation and clone replay. `Psg` exposes the existing latch/
data protocol, attenuation, noise and channel outputs. There is no Game Gear FM
unit. Ports own music drivers and register-write timing.

WIDTH/HEIGHT are 160×144; TickRate::GG_NTSC supplies cadence. default_settings
uses a 480×432 window and directions/button1/button2/start bindings.
Helpers are not a complete VDP/PSG emulator: subscanline fetch timing, exact analog
filtering, chip quirks and LCD persistence are outside their contract. Link
peripherals and uncommon mappers are deferred. No GUI or audio device is needed
for helper tests or scaffold checks. Assets load from the user's ROM at startup
with the existing replacement/additional-slot APIs.
