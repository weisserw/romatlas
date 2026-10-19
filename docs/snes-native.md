# Independent SNES native-port helpers

`romatlas_port::snes::{ppu,audio}` are original MIT Rust implementations. They
have no console emulator dependency, graphics/audio device access or game rules.
They complement the existing tile, palette and Mode 7 character decoders. The
port still loads original bytes from its owner's ROM and decides layer setup,
OAM staging, scanline register changes, sequence interpretation and event timing.
No existing port workspace is rewritten automatically.

## Rendering

All VRAM addresses below are **byte** addresses with 64-KiB wrapping. Convert
word-addressed SNES register values before constructing configurations.

- `tilemap_address` implements 32/64-tile quadrants and coordinate wrapping.
- `tile_pixel` samples 2/4/8-bit planar VRAM, including paired plane layout.
- `Background::sample` adds scroll, decodes tilemap priority/palette/flips and
  samples 8/16-pixel tiles; zero remains transparent before palette selection.
  For Mode 0 supply a distinct `palette_base` of 0, 32, 64 or 96 per BG.
- `Object::from_oam` decodes low/high OAM, signed X, size, name table, priority,
  palette and flips. `object_size` covers all eight OBSEL size configurations.
  `Object::sample` implements ordinary non-interlaced tile/palette addressing.
  Pass OBSEL's size field and name-gap field, and its converted byte base.
- `window_mask` combines inclusive windows, inversions and OR/AND/XOR/XNOR;
  both disabled means no mask. Use independent main/sub screen enables.
- `object_scanline` resolves a full OBJ line in supplied OAM order, including
  palette-based color math eligibility. It accepts raw OBSEL and native CGRAM.
- `screen_pixel` selects highest rank, preserving earlier candidates on ties.
  Process OBJ in hardware OAM order (including any port-managed rotation) and
  resolve the first nontransparent OBJ before comparing its priority against BGs.
  A later OBJ's larger BG-comparison rank must not displace an earlier OBJ.
  `mode1_bg_rank` provides Mode 1 BG ranks; OBJ ranks are 2, 5, 8, 11.
- `Composition::compose` and `compose_scanline` apply native BGR555 add/subtract,
  halving, fixed/subscreen color, clipping, math exclusion and brightness.
  Supply `math:false` for OBJ palettes 0..3 and per-pixel color-window decisions.
  Set brightness to 15 for full brightness; its default is zero.

Example layer sampling (the port supplies configuration and palette):

```rust,ignore
if let Some((index, high)) = bg.sample(&vram, x, y)? {
    candidates.push(Pixel {
        color: cgram[index as usize],
        rank: mode1_bg_rank(0, high, bg3_high)?,
        layer: 0,
        math: true,
    });
}
let main = screen_pixel(candidates, cgram[0]);
let rgba = composition.compose(main, sub);
```

These are explicit composition primitives, not a complete PPU register machine.
Mode 7 transforms, direct color, mosaic, offset-per-tile, hires/interlace,
32-object/34-tile overflow and bus timing are not implemented here. Ports must
implement required behavior or record it as unresolved. Priority ranks for modes
other than Mode 1 and per-scanline/window configuration remain caller supplied.

## Audio

`BrrDecoder::decode` accepts exactly nine bytes and returns 16 PCM16 samples,
end and loop flags. Predictor history survives blocks and loop boundaries;
`reset` starts a new source. Rejected block lengths preserve decoder state.
`Sample::new` validates complete blocks, a terminal end block, no early ends and
an in-range loop block. There is no implicit read beyond the supplied ROM slice.

`Mixer` owns eight configurable `Voice`s. A port sets 14-bit pitch (4096 = native
rate), signed stereo volumes, raw ADSR1/ADSR2/GAIN values, optional noise/echo and
calls `key_on(sample)` or `key_off()`. `sample()` advances one 32-kHz stereo
frame. `render(frames, output)` appends normalized interleaved f32 at that same
rate; resample explicitly before sending to a frontend configured for another
rate. All playback state is cloneable for deterministic local replay.

```rust,ignore
let source = Sample::new(rom_brr_bytes, loop_block)?;
let mut mixer = Mixer::default();
let voice = &mut mixer.voices[0];
voice.pitch = 4096;
voice.volume = [100, 100];
voice.adsr1 = 0x8f;
voice.adsr2 = 0xe0;
voice.key_on(source);
let mut samples = Vec::new();
mixer.render(320, &mut samples); // 10 ms at 32 kHz
```

Standalone `Envelope` and `Noise` accept a shared down-counter; the mixer owns
that deterministic counter. `Echo::new` allocates 1..7680 stereo delay frames
(512 per ordinary EDL unit), eight signed FIR coefficients, signed feedback and
stereo volume. `write_enabled:false` preserves delay contents while reads/filter
history continue. No echo addresses refer to emulator ARAM.

This is a native synthesis helper, not full SPC700/S-DSP emulation. Linear pitch
interpolation replaces Gaussian interpolation; exact sample-pipeline saturation,
key-on latency, register read/write races, pitch modulation, ENDX/ENVX/OUTX and
echo ARAM overlap/bus behavior are not modeled. Envelopes/noise use native rate
counters, but cycle-exact interaction with SPC writes is outside this API. The
port supplies the game's driver or sequence interpreter and commands. Do not
claim hardware-equivalent audio or complete SPC behavior from these helpers.

Hardware reference: Martin Korth's original
[fullsnes specifications](https://problemkaputt.de/fullsnes.htm), particularly
PPU backgrounds/objects/windows/color math and APU DSP BRR/envelopes/echo.
Implementation is independent of the GPL emulator and does not copy its code.
Tests use original synthetic tiles, OAM and BRR vectors, without game assets.
