use romatlas_port::snes::{audio::*, ppu::*};

#[test]
fn quadrant_maps_flipped_large_tiles_and_vram_wrap() {
    assert_eq!(tilemap_address(0xf800, true, true, 32, 32), 0x1000);
    assert_eq!(tilemap_address(0, true, true, 63, 63), 8190);
    assert_eq!(tilemap_address(0, false, true, 32, 32), 2048);
    let mut vram = [0; 65536];
    // A 16x16 tile flipped both ways chooses subtile 17, last row/column.
    vram[0..2].copy_from_slice(&0xc000u16.to_le_bytes());
    vram[0x2000 + 17 * 32 + 14] = 1;
    let bg = Background {
        map_base: 0,
        tile_base: 0x2000,
        wide: false,
        tall: false,
        large_tiles: true,
        bpp: 4,
        palette_base: 0,
        scroll_x: 0,
        scroll_y: 0,
    };
    assert_eq!(bg.sample(&vram, 0, 0).unwrap(), Some((1, false)));
    assert_eq!(bg.sample(&vram, 1, 0).unwrap(), None);
    vram[0xffff] = 0x80;
    assert_eq!(tile_pixel(&vram, 0xffff, 0, 2, 0, 0).unwrap(), 1);
    assert!(tile_pixel(&vram, 0, 0, 3, 0, 0).is_err());
}
#[test]
fn oam_high_bits_name_select_and_transparency() {
    let mut oam = [0; 544];
    oam[0] = 255;
    oam[2] = 15;
    oam[3] = 0xff;
    oam[512] = 3;
    let obj = Object::from_oam(&oam, 0).unwrap();
    assert_eq!(
        (obj.x, obj.palette, obj.priority, obj.large),
        (-1, 7, 3, true)
    );
    assert_eq!(object_size(6, true), (32, 64));
    assert!(Object::from_oam(&oam, 128).is_err());
    let mut vram = [0; 65536];
    let obj = Object {
        x: 0,
        y: 255,
        tile: 15,
        name_table: true,
        palette: 1,
        priority: 2,
        flip_x: false,
        flip_y: false,
        large: true,
    };
    // Crossing the tile row wraps column 15 to column 0, retains row.
    vram[8192 + 2] = 0x80;
    assert_eq!(obj.sample(&vram, 0, 0, 0, 8, 0).unwrap(), Some(145));
    assert_eq!(obj.sample(&vram, 0, 0, 0, 9, 0).unwrap(), None);
}
#[test]
fn windows_priority_and_native_color_math() {
    let a = Window {
        left: 10,
        right: 20,
        enabled: true,
        invert: false,
    };
    let b = Window {
        left: 15,
        right: 25,
        ..a
    };
    assert!(window_mask(10, a, b, WindowLogic::Or));
    assert!(!window_mask(15, a, b, WindowLogic::Xor));
    assert!(window_mask(15, a, b, WindowLogic::And));
    assert!(!window_mask(
        15,
        Window::default(),
        Window::default(),
        WindowLogic::Xnor
    ));
    assert_eq!(color_math(31, 31, false, true), 31);
    assert_eq!(color_math(1, 31, true, true), 0);
    let p = Pixel {
        color: 31,
        rank: 5,
        layer: 4,
        math: false,
    };
    let q = Pixel { color: 992, ..p };
    assert_eq!(screen_pixel([p, q], 0).color, 31);
    assert!(mode1_bg_rank(2, true, true).unwrap() > 11);
    let c = Composition {
        math_layers: 63,
        brightness: 15,
        fixed_color: 31,
        ..Default::default()
    };
    assert_eq!(c.compose(p, q), [255, 0, 0, 255]); // OBJ math exemption
    let p = Pixel {
        math: true,
        color: 0,
        layer: 0,
        ..p
    };
    assert_eq!(c.compose(p, q), [255, 0, 0, 255]);
    let sub = screen_pixel([], 0);
    let c = Composition {
        use_subscreen: true,
        half: true,
        ..c
    };
    assert_eq!(c.compose(p, sub), [255, 0, 0, 255]); // fixed fallback is not halved
    let line = compose_scanline(&[p; 256], &[sub; 256], |_| c);
    assert_eq!(line[255], [255, 0, 0, 255]);
}
#[test]
fn brr_vectors_predictors_and_transactional_rejection() {
    let mut decoder = BrrDecoder::default();
    let b = decoder
        .decode(&[0x10, 0x17, 0x8f, 0, 0, 0, 0, 0, 0])
        .unwrap();
    assert_eq!(&b.samples[..4], &[2, 14, -16, -2]);
    let mut seeded = BrrDecoder::default();
    seeded.decode(&[0xc0, 0, 0, 0, 0, 0, 0, 0, 0x11]).unwrap();
    let mut baseline = seeded.clone();
    assert!(seeded.decode(&[0; 8]).is_err());
    let block = [0x04, 0, 0, 0, 0, 0, 0, 0, 0];
    let b = seeded.decode(&block).unwrap();
    assert_eq!(b, baseline.decode(&block).unwrap());
    assert_eq!(b.samples[0], 3840); // filter 1: 2048 * 15/16, doubled
    let b = decoder.decode(&[0xf3, 0x78, 0, 0, 0, 0, 0, 0, 0]).unwrap();
    assert_eq!(&b.samples[..2], &[0, -4096]);
    assert!(b.end && b.looping);
    assert!(Sample::new(&[0; 9], 0).is_err());
}
#[test]
fn voice_pitch_loop_release_mixing_echo_and_replay() {
    let source = Sample::new(&[0x13, 0x77, 0x77, 0x77, 0x77, 0x77, 0x77, 0x77, 0x77], 0).unwrap();
    let mut mixer = Mixer::default();
    let v = &mut mixer.voices[0];
    v.pitch = 4096;
    v.volume = [127, -128];
    v.gain = 127;
    v.key_on(source);
    let sample = mixer.sample();
    assert!(sample[0] > 0 && sample[1] < 0);
    for _ in 0..40 {
        mixer.sample();
    }
    assert!(mixer.voices[0].active());
    let mut echo = Echo::new(1).unwrap();
    echo.fir[0] = 64;
    echo.volume = [127; 2];
    mixer.echo = Some(echo);
    mixer.voices[0].echo = true;
    let mut replay = mixer.clone();
    let mut a = Vec::new();
    let mut b = Vec::new();
    mixer.render(500, &mut a);
    replay.render(500, &mut b);
    assert_eq!(a, b);
    assert_eq!(a.len(), 1000);
    mixer.voices[0].key_off();
    for _ in 0..256 {
        mixer.sample();
    }
    assert_eq!(mixer.voices[0].envelope.level, 0);
    assert!(Echo::new(0).is_err());
    mixer.mute = true;
    assert_eq!(mixer.sample(), [0; 2]);
}
#[test]
fn envelope_attack_gain_release_and_noise_replay() {
    let mut e = Envelope::default();
    e.key_on();
    assert_eq!(e.tick(0, 0x8f, 0, 0), 1024);
    assert_eq!(e.tick(0, 0x8f, 0, 0), 2047);
    assert_eq!(e.stage, EnvelopeStage::Decay);
    e.key_off();
    assert_eq!(e.tick(0, 0, 0, 0), 2039);
    e.key_on();
    assert_eq!(e.tick(0, 0, 0, 100), 1600);
    let mut a = Noise::default();
    let mut b = a.clone();
    for clock in (0..100).rev() {
        assert_eq!(a.tick(clock, 31), b.tick(clock, 31));
    }
}

#[test]
fn oam_order_precedes_object_bg_rank() {
    let mut vram = [0; 65536];
    vram[0] = 0x80;
    let mut cgram = [0; 256];
    cgram[129] = 31;
    cgram[193] = 992;
    let first = Object {
        x: 0,
        y: 0,
        tile: 0,
        name_table: false,
        palette: 0,
        priority: 0,
        flip_x: false,
        flip_y: false,
        large: false,
    };
    let second = Object {
        palette: 4,
        priority: 3,
        ..first
    };
    // The first object wins even though the later one would rank above more BGs.
    let line = object_scanline(&[first, second], &vram, &cgram, 4, 0, [2, 5, 8, 11]).unwrap();
    assert_eq!(line[0].unwrap().color, 31);
    assert!(!line[0].unwrap().math);
    assert!(line[1].is_none());
    let line = object_scanline(&[second, first], &vram, &cgram, 0, 0, [2, 5, 8, 11]).unwrap();
    assert_eq!(line[0].unwrap().color, 992);
    assert!(line[0].unwrap().math);
}

#[test]
fn nonlooping_source_stops_and_zero_pitch_holds() {
    let source = Sample::new(&[0x11, 0x77, 0x77, 0x77, 0x77, 0x77, 0x77, 0x77, 0x77], 0).unwrap();
    let mut mixer = Mixer::default();
    let v = &mut mixer.voices[0];
    v.pitch = 0;
    v.volume = [127; 2];
    v.gain = 127;
    v.key_on(source);
    let held = mixer.sample();
    for _ in 0..64 {
        assert_eq!(mixer.sample(), held);
    }
    mixer.voices[0].pitch = 4096;
    for _ in 0..16 {
        mixer.sample();
    }
    assert!(!mixer.voices[0].active());
    assert_eq!(mixer.sample(), [0; 2]);
}
