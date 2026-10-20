//! Original synthetic native-helper tests; no emulator code/assets.
use romatlas_port::nes::{self, apu::Apu, ppu::*};
#[test]
fn chr_attribute_scroll_and_sprite_priority() {
    let mut chr = vec![0; 8192];
    chr[0] = 0x80;
    chr[8] = 0x40;
    chr[16..24].fill(255);
    assert_eq!(&nes::tile(&chr[..16]).unwrap()[..3], &[1, 2, 0]);
    let mut nt = vec![0; 4096];
    nt[960] = 0b11_10_01_00;
    nt[1024] = 1;
    let mut bg = Background {
        chr: &chr,
        chr_banks: [0, 1, 2, 3, 4, 5, 6, 7],
        nametables: &nt,
        mirroring: Mirroring::FourScreen,
        pattern_table: 0,
        scroll: Scroll::default(),
    };
    assert_eq!(bg.pixel(0, 0).unwrap(), 1);
    bg.scroll.x = 256;
    assert_eq!(bg.pixel(0, 0).unwrap(), 1);
    let mut oam = [255; 256];
    for i in 0..9 {
        oam[i * 4..i * 4 + 4].copy_from_slice(&[9, 1, 0x20, i as u8 * 8]);
    }
    let line = sprites(&chr, [0, 1, 2, 3, 4, 5, 6, 7], &oam, 10, false, 0).unwrap();
    assert!(line.overflow);
    assert!(line.pixels[64].is_none());
    assert_eq!(
        compose(2, line.pixels[0], 0, 0x1e),
        Pixel {
            palette: 2,
            sprite_zero_hit: true
        }
    );
    assert_eq!(
        compose(2, line.pixels[0], 0, 0x18),
        Pixel {
            palette: 0,
            sprite_zero_hit: false
        }
    );
    assert!(!compose(2, line.pixels[0], 255, 0x1e).sprite_zero_hit);
    assert_eq!(nametable_index(0x2c00, Mirroring::Vertical), 1024);
    assert_eq!(nametable_index(0x2800, Mirroring::Horizontal), 1024);
    assert_eq!(palette_index(0x10), 0);
    assert_eq!(palette_index(0x1c), 12);
}
#[test]
fn tall_sprite_flips_and_chr_banks() {
    let mut chr = vec![0; 16384];
    chr[8192 + 16 + 7] = 1;
    let mut oam = [255; 256];
    oam[..4].copy_from_slice(&[0, 0, 0xc0, 12]);
    let line = sprites(&chr, [8, 1, 2, 3, 4, 5, 6, 7], &oam, 1, true, 0).unwrap();
    assert_eq!(line.pixels[12].unwrap().palette, 17);
    assert!(line.pixels[13].is_none());
}
#[test]
fn audio_registers_dmc_fetch_irq_and_replay() {
    let mut a = Apu::default();
    a.write(0x4015, 0x1f).unwrap();
    a.write(0x4000, 0xbf).unwrap();
    a.write(0x4001, 8).unwrap();
    a.write(0x4002, 100).unwrap();
    a.write(0x4003, 8).unwrap();
    a.write(0x4008, 0xff).unwrap();
    a.write(0x400a, 50).unwrap();
    a.write(0x400b, 8).unwrap();
    a.write(0x400c, 0x3f).unwrap();
    a.write(0x400e, 3).unwrap();
    a.write(0x400f, 8).unwrap();
    a.write(0x4010, 0x8f).unwrap();
    a.write(0x4012, 0).unwrap();
    a.write(0x4013, 0).unwrap();
    a.write(0x4015, 0x1f).unwrap();
    let mut fetched = Vec::new();
    for _ in 0..30000 {
        a.clock(&mut |address| {
            fetched.push(address);
            0b10101010
        });
    }
    assert_eq!(fetched, vec![0xc000]);
    assert!(a.irq());
    assert!(a.status() & 0x80 != 0);
    assert!(a.irq());
    a.write(0x4015, 0x0f).unwrap();
    assert!(!a.irq());
    let mut b = a.clone();
    let mut left = Vec::new();
    let mut right = Vec::new();
    a.render(4000, 48000, |_| 0, &mut left).unwrap();
    b.render(4000, 48000, |_| 0, &mut right).unwrap();
    assert_eq!(left, right);
    assert!(left.windows(2).any(|w| w[0] != w[1]));
    assert!(left.iter().all(|x| x.is_finite() && *x >= 0.0 && *x <= 1.0));
}
#[test]
fn dmc_wrap_loop_and_frame_counter_modes() {
    let mut a = Apu::default();
    a.write(0x4017, 0xc0).unwrap();
    a.write(0x4010, 0x4f).unwrap();
    a.write(0x4012, 0xff).unwrap();
    a.write(0x4013, 4).unwrap();
    a.write(0x4015, 16).unwrap();
    let mut reads = Vec::new();
    for _ in 0..40000 {
        a.clock(&mut |address| {
            reads.push(address);
            255
        });
    }
    assert_eq!(reads[0], 0xffc0);
    assert!(reads.contains(&0xffff) && reads.contains(&0x8000));
    assert!(reads.iter().filter(|a| **a == 0xffc0).count() > 1);
    assert!(!a.irq());
    a.write(0x4010, 0).unwrap();
    a.write(0x4017, 0).unwrap();
    for _ in 0..30000 {
        a.clock(&mut |_| 0);
    }
    assert!(a.status() & 64 != 0);
    assert!(!a.irq());
}
