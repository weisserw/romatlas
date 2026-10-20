use romatlas_port::pce::*;
#[test]
fn planes_palette_and_cadence() {
    let mut bytes = [0; 32];
    bytes[0] = 128;
    bytes[1] = 64;
    bytes[16] = 32;
    bytes[17] = 16;
    assert_eq!(&tile(&bytes).unwrap()[..4], &[1, 2, 4, 8]);
    assert_eq!(color(0x1ff, false), [255, 255, 255, 255]);
    assert_eq!(color(7, false), [0, 0, 255, 255]);
    let gray = color(7, true);
    assert_eq!(gray[0], gray[1]);
    assert_eq!(gray[1], gray[2]);
    let mut v = Vce::default();
    v.write(2, 255);
    v.write(3, 1);
    v.write(4, 42);
    v.write(5, 1);
    assert_eq!(v.address, 0);
    assert_eq!(v.palette[511], 298);
    let fast = v.tick_rate();
    v.write(0, 6);
    assert_eq!(v.dot_divider(), 2);
    assert!(v.tick_rate().denominator > fast.denominator);
    let s = default_settings();
    for key in ["i", "ii", "run", "select"] {
        assert!(s.bindings.contains_key(key));
    }
}
#[test]
fn background_sprites_raster_and_dma() {
    let mut v = Vdc::default();
    v.registers[5] = 0xc7;
    v.registers[6] = 64;
    // All BG cells use tile 1, one colored pixel each row.
    v.vram[..1024].fill(1);
    for y in 0..8 {
        v.vram[16 + y] = 128;
    }
    // Sprite 0 and sprite 1 at 0,0; pattern 32 starts past the BG table.
    v.satb[..8].copy_from_slice(&[64, 32, 64, 128, 64, 32, 64, 128]);
    v.vram[32 * 64] = 0x8000;
    let line = v.scanline(0, 256).unwrap();
    assert!(line.sprite_collision && line.raster_irq);
    assert_eq!(line.indices[0], 257);
    assert!(v.irq_pending());
    v.satb[3] = 0;
    let line = v.scanline(0, 256).unwrap();
    assert_eq!(line.indices[0], 1);
    v.registers[7] = 1;
    let line = v.scanline(0, 256).unwrap();
    assert_eq!(line.indices[0], 257);
    for n in 0..17 {
        v.satb[n * 4] = 64;
        v.satb[n * 4 + 1] = 32;
    }
    assert!(v.scanline(0, 256).unwrap().sprite_overflow);
    v.vram[5000] = 0x1234;
    v.registers[16] = 5000;
    v.registers[17] = 5100;
    v.registers[18] = 0;
    v.vram_dma();
    assert_eq!(v.vram[5100], 0x1234);
    assert_ne!(v.status & 16, 0);
    v.registers[19] = 5000;
    v.registers[15] |= 16;
    v.vblank();
    assert_eq!(v.satb[0], 0x1234);
    assert_ne!(v.status & 8, 0);
}
#[test]
fn wave_dda_noise_lfo_and_replay() {
    let mut p = Psg::default();
    p.write(1, 0xff);
    p.write(0, 0);
    p.write(5, 0xf0);
    p.write(4, 0xdf);
    p.write(6, 31);
    let out = p.output();
    assert!(out[0] > 0.0 && out[0] > out[1]);
    p.write(4, 0);
    for n in 0..32 {
        p.write(6, n);
    }
    assert_eq!(p.channels[0].wave, std::array::from_fn(|i| i as u8));
    p.write(2, 4);
    p.write(4, 0x9f);
    p.clock(16);
    assert_ne!(p.output(), out);
    p.write(0, 4);
    p.write(5, 0xff);
    p.write(4, 0x9f);
    p.write(7, 0x9f);
    p.clock(128);
    p.write(0, 1);
    for n in 0..32 {
        p.write(6, n);
    }
    p.write(8, 2);
    p.write(9, 2);
    let mut mixer = Mixer::new(3_579_545, 48000).unwrap();
    mixer.psg = p;
    let mut copy = mixer.clone();
    let a: Vec<_> = (0..256).map(|_| mixer.sample()).collect();
    let b: Vec<_> = (0..256).map(|_| copy.sample()).collect();
    assert_eq!(a, b);
    assert!(a.iter().any(|s| s[0] != 0.0));
}
#[test]
fn reduced_plane_modes_and_wide_sprite_partial_selection() {
    let mut v = Vdc::default();
    v.registers[5] = 128;
    v.vram[0] = 100;
    v.vram[1600] = 128;
    v.vram[1608] = 128;
    assert_eq!(v.scanline(0, 1).unwrap().indices, [5]);
    v.registers[9] = 3;
    assert_eq!(v.scanline(0, 1).unwrap().indices, [1]);
    v.registers[9] = 131;
    assert_eq!(v.scanline(0, 1).unwrap().indices, [4]);
    v.registers[5] = 64;
    v.registers[9] = 8;
    for n in 0..16 {
        v.satb[n * 4] = 64;
        v.satb[n * 4 + 1] = 32;
        v.satb[n * 4 + 2] = 200;
        v.satb[n * 4 + 3] = 128;
    }
    v.vram[100 * 64] = 0x8000;
    v.vram[100 * 64 + 32] = 0x8000;
    assert_eq!(v.scanline(0, 1).unwrap().indices, [257]);
    v.satb[2] |= 1;
    assert_eq!(v.scanline(0, 1).unwrap().indices, [260]);
    // Fifteen narrow cells leave one slot for the left half of the next wide sprite.
    for n in 0..15 {
        v.satb[n * 4 + 1] = 0;
    }
    v.satb[15 * 4 + 1] = 32;
    v.satb[15 * 4 + 3] = 128 | 256;
    v.satb[15 * 4 + 2] = 200;
    let line = v.scanline(0, 32).unwrap();
    assert!(line.sprite_overflow);
    assert_eq!(line.indices[0], 257);
    assert_eq!(line.indices[16], 0);
}

#[test]
fn register_byte_writes_and_vram_latches() {
    let mut v = Vdc::default();
    v.write(0, 5);
    v.write(2, 128);
    assert_eq!(v.registers[5], 128);
    v.write(0, 7);
    v.write(2, 42);
    v.write(3, 1);
    assert_eq!(v.registers[7], 298);
    v.write(0, 5);
    v.write(3, 8);
    assert_eq!(v.registers[5], 0x880);
    v.write(0, 0);
    v.write(2, 4);
    v.write(3, 0);
    v.write(0, 2);
    v.write(2, 0x34);
    v.write(3, 0x12);
    assert_eq!(v.vram[4], 0x1234);
    assert_eq!(v.registers[0], 36);
    v.write(0, 1);
    v.write(2, 4);
    v.write(3, 0);
    assert_eq!(v.read(2), 0x34);
    assert_eq!(v.read(3), 0x12);
    assert_eq!(v.registers[1], 36);
}
