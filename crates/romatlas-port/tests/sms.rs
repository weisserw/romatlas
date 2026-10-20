use romatlas_port::sms::{
    self,
    audio::{Fm, Mixer, Patch, Psg},
    vdp::{self, Config},
};
#[test]
fn mode4_priority_scroll_and_sprite_limits() {
    let mut v = vec![0; 16384];
    v[..32].fill(255);
    // Make background tile zero high-priority palette one; all other map entries tile 1 (transparent).
    for i in (0x3800..0x3f00).step_by(2) {
        v[i] = 1;
    }
    v[0x3800] = 0;
    v[0x3801] = 0x18;
    v[0x3f00] = 255;
    v[0x3f01] = 0xd0;
    v[0x3f80] = 0;
    v[0x3f81] = 0;
    let mut c = Config::default();
    let line = vdp::scanline(&v, 0, c).unwrap();
    assert_eq!(line.indices[0], 31);
    assert!(!line.collision);
    c.scroll_x = 8;
    assert_eq!(vdp::background(&v, 8, 0, c).unwrap().index, 31);
    c.top_scroll_lock = true;
    assert_eq!(vdp::background(&v, 0, 0, c).unwrap().index, 31);
    for n in 0..9 {
        v[0x3f00 + n] = 255;
        v[0x3f80 + 2 * n] = 0;
        v[0x3f81 + 2 * n] = 0;
    }
    v[0x3f09] = 0xd0;
    let line = vdp::scanline(&v, 0, c).unwrap();
    assert!(line.collision && line.overflow);
    assert_eq!(line.selected.len(), 8);
    assert_eq!(sms::tile(&v[..32]).unwrap(), [15; 64]);
}
#[test]
fn psg_latch_noise_and_replay() {
    let mut p = Psg::default();
    p.write(0x85);
    p.write(0x12);
    p.write(0x90);
    assert_eq!(p.tone[0], 0x125);
    p.write(0xe4);
    p.write(0xf0);
    let mut other = p.clone();
    let a: Vec<_> = (0..2000)
        .map(|_| {
            p.clock(81);
            p.sample()
        })
        .collect();
    let b: Vec<_> = (0..2000)
        .map(|_| {
            other.clock(81);
            other.sample()
        })
        .collect();
    assert_eq!(a, b);
    assert!(a.iter().any(|v| *v != a[0]));
}
#[test]
fn fm_keys_custom_patch_and_mixer_gating() {
    let mut patches = [Patch::default(); 16];
    patches[1] = Patch([1, 1, 0, 0, 0xf0, 0xf0, 0x05, 0x05]);
    let mut fm = Fm::new(patches);
    fm.write(0x30, 0x10);
    fm.write(0x10, 0xff);
    fm.write(0x20, 0x1b);
    let samples: Vec<_> = (0..4000)
        .map(|_| fm.sample(48000, 3_579_545.0).unwrap())
        .collect();
    assert!(samples.iter().all(|v| v.is_finite()));
    assert!(samples.iter().any(|v| v.abs() > 0.01));
    fm.write(0x20, 0x0b);
    for _ in 0..40000 {
        fm.sample(48000, 3_579_545.0).unwrap();
    }
    assert!(fm.sample(48000, 3_579_545.0).unwrap().abs() < 0.001);
    let mut mix = Mixer::new(patches);
    mix.control = 2;
    assert_eq!(mix.sample(48000).unwrap(), 0.0);
    assert!(mix.sample(0).is_err());
}
