use romatlas_core::{Provenance, RegionKind};
use romatlas_genesis::{
    Rom,
    decode::{self, Context, Cpu},
    mapping::{Entry, Mapping, View, resolve_cpu},
};
fn image() -> Vec<u8> {
    let mut b = vec![0; 32768];
    b[..8].copy_from_slice(&[0, 255, 255, 0, 0, 0, 2, 0]);
    b[0x100..0x110].copy_from_slice(b"SEGA GENESIS    ");
    b[512..518].copy_from_slice(&[0x70, 1, 0x52, 0x80, 0x60, 0xfc]);
    b
}
#[test]
fn layouts_keep_file_identity_and_canonical_backing() {
    let b = image();
    let raw = Rom::from_bytes(b.clone()).unwrap();
    let swapped: Vec<_> = b.chunks_exact(2).flat_map(|p| [p[1], p[0]]).collect();
    let other = Rom::from_bytes(swapped.clone()).unwrap();
    assert_eq!(other.info.layout, "word_swapped");
    assert_eq!(other.bytes, raw.bytes);
    assert_eq!(other.original(), swapped);
    assert_ne!(raw.info.sha256, other.info.sha256);
    let mut smd = vec![0; 512];
    smd[8..11].copy_from_slice(&[0xaa, 0xbb, 6]);
    for block in b.chunks_exact(16384) {
        smd.extend(block.iter().skip(1).step_by(2));
        smd.extend(block.iter().step_by(2));
    }
    let smd = Rom::from_bytes(smd).unwrap();
    assert_eq!(smd.info.layout, "smd_interleaved");
    assert_eq!(&smd.bytes[512..], raw.bytes);
    assert_eq!(smd.file_offset(512), Some(8960));
}
#[test]
fn contexts_and_transactional_regions() {
    let mut b = image();
    b[0x100..0x110].copy_from_slice(b"SEGA SSF        ");
    b.resize(0x500000, 0);
    let rom = Rom::from_bytes(b).unwrap();
    assert!(resolve_cpu(&rom, Cpu::Main, 0x80000, Context::default()).is_none());
    let mut ctx = Context::default();
    ctx.rom_banks[1] = Some(8);
    assert_eq!(
        resolve_cpu(&rom, Cpu::Main, 0x80123, ctx).unwrap().offset,
        0x400123
    );
    ctx.z80_bank = Some(16);
    assert_eq!(
        resolve_cpu(&rom, Cpu::Z80, 0x8123, ctx).unwrap().offset,
        0x400123
    );
    assert_eq!(
        resolve_cpu(&rom, Cpu::Main, 0xe12345, ctx).unwrap().offset,
        0x2345
    );
    let rom = Rom::from_bytes(image()).unwrap();
    let mut map = Mapping::new(&rom);
    let entry = Entry {
        id: 0,
        offset: 512,
        length: 6,
        kind: RegionKind::Code,
        provenance: Provenance::Manual,
        confidence: 1.0,
        label: Some("Reset".into()),
        comment: "Original synthetic loop".into(),
        evidence: "Test fixture".into(),
        views: vec![View {
            cpu: Cpu::Main,
            cpu_address: 512,
            context: Context::default(),
            uploaded: false,
        }],
    };
    map.annotate(&rom, entry.clone()).unwrap();
    let before = serde_json::to_value(&map).unwrap();
    let mut overlap = entry.clone();
    overlap.offset += 2;
    assert!(map.annotate(&rom, overlap).is_err());
    assert_eq!(before, serde_json::to_value(&map).unwrap());
    let mut partial = entry;
    partial.id = 1;
    partial.length = 5;
    assert!(map.annotate(&rom, partial).is_err());
    assert_eq!(map.unknown(), vec![(0, 512), (518, 32768 - 518)]);
}
#[test]
fn decoder_control_flow_and_truncation() {
    let i = decode::decode(Cpu::Main, &[0x60, 0xfe], 512, Context::default()).unwrap();
    assert_eq!(i.successors, vec![512]);
    assert_eq!(i.length, 2);
    assert!(i.terminal);
    let i = decode::decode(
        Cpu::Main,
        &[0x4e, 0xb9, 0, 0x12, 0x34, 0x56],
        0x1000,
        Context::default(),
    )
    .unwrap();
    assert!(i.successors.contains(&0x123456));
    assert_eq!(i.length, 6);
    assert!(decode::decode(Cpu::Main, &[0x4e, 0xb9, 0], 0, Context::default()).is_err());
    assert!(decode::decode(Cpu::Main, &[0x4e, 0x71], 1, Context::default()).is_err());
    let pc_relative =
        decode::decode(Cpu::Main, &[0x41, 0xfa, 0x00, 0x04], 0, Context::default()).unwrap();
    assert!(
        pc_relative
            .references
            .iter()
            .any(|r| r["cpu_address"] == 6 && r["indirect"] == false && r["indexed"] == false)
    );
    for word in [0xa000u16, 0xf000u16] {
        let i = decode::decode(Cpu::Main, &word.to_be_bytes(), 0, Context::default()).unwrap();
        assert!(i.terminal && i.successors.is_empty());
        assert_eq!(i.length, 2);
    }
    let z = decode::decode(
        Cpu::Z80,
        &[0xdd, 0xcb, 0xfe, 0x46],
        0x8000,
        Context::default(),
    )
    .unwrap();
    assert_eq!(z.length, 4);
}
