use romatlas_port::genesis::{Brightness, audio::Ym2612, color, vdp::Vdp};
fn word(v: &mut Vdp, a: usize, w: u16) {
    v.vram[a..a + 2].copy_from_slice(&w.to_be_bytes());
}
fn command(v: &mut Vdp, address: u16, code: u8) {
    v.control((address & 0x3fff) | (u16::from(code & 3) << 14));
    v.control((address >> 14) | (u16::from(code & 0x3c) << 2));
}
#[test]
fn raster_planes_window_priority_and_shadow() {
    let mut v = Vdp::default();
    v.registers[1] = 64;
    v.registers[2] = 0x30;
    v.registers[4] = 7;
    v.registers[3] = 0x2c;
    v.registers[5] = 0x40;
    v.vram[32..64].fill(0x11);
    v.vram[64..96].fill(0x22);
    v.cram[1] = 0xe;
    v.cram[2] = 0xe0;
    word(&mut v, 0xc000, 1);
    word(&mut v, 0xe000, 0x8002);
    let line = v.scanline(0).unwrap();
    assert_eq!(line.rgba[0], color(0xe0, Brightness::Normal));
    word(&mut v, 0xc000, 0x8001);
    assert_eq!(
        v.scanline(0).unwrap().rgba[0],
        color(0xe, Brightness::Normal)
    );
    v.registers[17] = 1;
    word(&mut v, 0xb000, 0x8002);
    assert_eq!(
        v.scanline(0).unwrap().rgba[0],
        color(0xe0, Brightness::Normal)
    );
    v.registers[12] = 8;
    word(&mut v, 0xb000, 1);
    word(&mut v, 0xe000, 0);
    assert_eq!(
        v.scanline(0).unwrap().rgba[0],
        color(0xe, Brightness::Shadow)
    );
}
#[test]
fn linked_sprites_column_major_collision_and_flip() {
    let mut v = Vdp::default();
    v.registers[1] = 64;
    v.registers[5] = 0x40;
    v.vram[32..64].fill(0x11);
    v.vram[64..96].fill(0x22);
    v.cram[1] = 0xe;
    v.cram[2] = 0xe0;
    word(&mut v, 0x8000, 128);
    word(&mut v, 0x8002, 0x0401);
    word(&mut v, 0x8004, 1);
    word(&mut v, 0x8006, 128);
    word(&mut v, 0x8008, 128);
    word(&mut v, 0x800a, 0);
    word(&mut v, 0x800c, 2);
    word(&mut v, 0x800e, 128);
    let line = v.scanline(0).unwrap();
    assert!(line.sprite_collision);
    assert_eq!(line.rgba[0], color(0xe, Brightness::Normal));
    assert_eq!(line.rgba[8], color(0xe0, Brightness::Normal));
    word(&mut v, 0x8004, 0x0801);
    assert_eq!(
        v.scanline(0).unwrap().rgba[0],
        color(0xe0, Brightness::Normal)
    );
}
#[test]
fn dma_copy_bus_words_and_port_increment() {
    let mut v = Vdp::default();
    v.registers[1] = 16;
    v.registers[15] = 2;
    v.registers[19] = 3;
    command(&mut v, 0x100, 33);
    let mut addresses = Vec::new();
    v.dma(|a| {
        addresses.push(a);
        0x1000 + a as u16
    });
    assert_eq!(addresses, vec![0, 2, 4]);
    assert_eq!(&v.vram[0x100..0x106], &[0x10, 0, 0x10, 2, 0x10, 4]);
    assert_eq!(&v.registers[19..21], &[0, 0]);
    v.registers[19] = 3;
    v.registers[21] = 0;
    v.registers[22] = 1;
    v.registers[23] = 0xc0;
    v.registers[15] = 1;
    command(&mut v, 0x200, 33);
    v.dma(|_| panic!("VRAM copy must not read CPU bus"));
    assert_eq!(&v.vram[0x200..0x203], &[0x10, 0, 0x10]);
}
#[test]
fn fm_algorithms_timer_dac_stereo_and_replay() {
    let mut fm = Ym2612::new(7_670_453, 48000).unwrap();
    fm.write(0, 0x24, 255);
    fm.write(0, 0x25, 3);
    fm.write(0, 0x27, 5);
    fm.clock(72);
    assert_eq!(fm.read_status(), 1);
    fm.write(0, 0x27, 16);
    assert_eq!(fm.read_status(), 0);
    fm.write(0, 0x2b, 128);
    fm.write(0, 0x2a, 255);
    fm.write(1, 0xb6, 128);
    let s = fm.sample();
    assert!(s[0] > 0.0 && s[1] == 0.0);
    fm.write(0, 0x2b, 0);
    for slot in [0, 4, 8, 12] {
        fm.write(0, 0x30 + slot, 1);
        fm.write(0, 0x50 + slot, 31);
        fm.write(0, 0x80 + slot, 15);
    }
    fm.write(0, 0xa0, 0xa0);
    fm.write(0, 0xa4, 0x22);
    fm.write(0, 0xb4, 0xc0);
    fm.write(0, 0x28, 0xf0);
    let mut hashes = Vec::new();
    for algorithm in 0..8 {
        fm.write(0, 0xb0, algorithm);
        let mut cloned = fm.clone();
        let samples = (0..200)
            .map(|_| {
                fm.clock(160);
                fm.sample()
            })
            .collect::<Vec<_>>();
        assert_eq!(
            samples,
            (0..200)
                .map(|_| {
                    cloned.clock(160);
                    cloned.sample()
                })
                .collect::<Vec<_>>()
        );
        assert!(samples.iter().any(|s| s[0] != 0.0));
        assert!(samples.iter().all(|s| s[0] == s[1]));
        hashes.push(samples.iter().map(|s| s[0].to_bits()).collect::<Vec<_>>());
    }
    assert!(hashes.windows(2).all(|p| p[0] != p[1]));
}
