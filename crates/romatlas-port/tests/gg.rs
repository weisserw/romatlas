use romatlas_port::gg::{self, vdp};
#[test]
fn lcd_crop_palette_and_scroll() {
    let mut vram = vec![0; 16384];
    // Solid tile 1, palette index 1, selected everywhere in the name table.
    for row in 0..8 {
        vram[32 + row * 4] = 255;
    }
    let mut regs = [0; 11];
    regs[1] = 0x40;
    regs[2] = 0x0e;
    regs[5] = 0x7e;
    let c = vdp::Config::registers(regs);
    for at in (0x3800..0x3f00).step_by(2) {
        vram[at] = 1;
    }
    vram[0x3f00] = 0xd0;
    let mut cram = [0; 64];
    cram[2] = 0x2f;
    cram[3] = 0x0a;
    let raw = vdp::scanline(&vram, 24, c).unwrap();
    assert_eq!(vdp::rgba(&raw, &cram).unwrap().len(), 256);
    assert!(vdp::rgba(&raw, &cram[..32]).is_err());
    let line = gg::scanline(&vram, &cram, c, 0).unwrap();
    assert_eq!(line, vec![[255, 34, 170, 255]; 160]);
    assert!(gg::scanline(&vram, &cram, c, 144).is_err());
    assert!(gg::scanline(&vram, &cram[..32], c, 0).is_err());
    // Change only tile at the first LCD coordinate (48,24); scroll shifts its boundary.
    vram[0x3800 + (3 * 32 + 6) * 2] = 0;
    assert_eq!(gg::scanline(&vram, &cram, c, 0).unwrap()[0], [0, 0, 0, 255]);
}
#[test]
fn stereo_routing_and_replay() {
    let mut m = gg::audio::Mixer::default();
    for value in [0x85, 0x10, 0x90] {
        m.write(value);
    }
    m.write_stereo(0x10);
    let mut replay = m.clone();
    let first: Vec<_> = (0..2048).map(|_| m.sample()).collect();
    assert!(first.iter().any(|v| v[0] != 0.0));
    assert!(first.iter().all(|v| v[1] == 0.0));
    assert_eq!(
        first,
        (0..2048).map(|_| replay.sample()).collect::<Vec<_>>()
    );
    m.write_stereo(1);
    assert_eq!(m.sample()[0], 0.0);
    m.write_stereo(0);
    assert_eq!(m.sample(), [0.0, 0.0]);
}
