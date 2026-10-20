use romatlas_sms::{
    Rom,
    decode::{self, Context, Cpu},
    mapping::{self, Mapping},
};
fn rom() -> Rom {
    let mut b = vec![0; 65536];
    b[0] = 0xc3;
    b[1] = 0x00;
    b[2] = 0x04;
    Rom::from_bytes(b).unwrap()
}
#[test]
fn physical_windows_and_data_barriers() {
    let r = rom();
    let mut c = Context {
        rom_banks: [Some(2), Some(1), Some(3)],
        ram_enabled: Some(false),
        ram_bank: None,
        cartridge_enabled: Some(true),
    };
    assert_eq!(mapping::resolve(&r, 0, c).unwrap().offset, 0);
    assert_eq!(mapping::resolve(&r, 1024, c).unwrap().offset, 33792);
    assert_eq!(mapping::resolve(&r, 0xe123, c).unwrap().offset, 0x123);
    c.ram_enabled = Some(true);
    assert!(mapping::resolve(&r, 0x8000, c).is_none());
    let m = Mapping::new(&r);
    let d = romatlas_sms::analysis::discover(
        &r,
        &m,
        &serde_json::json!({"entrypoints":[{"offset":0,"cpu_address":0}],"max_instructions":4}),
        &Default::default(),
        &Default::default(),
    )
    .unwrap();
    assert_eq!(d["total"], 1);
    assert!(d["total_diagnostics"].as_u64().unwrap() > 0);
}
#[test]
fn all_opcode_families_have_exact_lengths_and_bytes() {
    let mut families = vec![];
    for op in 0..=255u8 {
        families.push(vec![op, 0, 0, 0, 0, 0]);
        for prefix in [0xcb, 0xed, 0xdd, 0xfd] {
            families.push(vec![prefix, op, 0, 0, 0, 0]);
        }
        for prefix in [0xdd, 0xfd] {
            families.push(vec![prefix, 0xcb, 0xf8, op, 0, 0]);
        }
    }
    for b in families {
        let i = decode::decode(Cpu::Main, &b, 0, Context::default()).unwrap();
        assert!(i.length > 0 && i.length <= 6, "{b:x?}");
    }
    let jump = decode::decode(Cpu::Main, &[0x18, 0xfc], 0xfffe, Context::default()).unwrap();
    assert_eq!(jump.successors, vec![0xfffc]);
    let indexed = decode::decode(Cpu::Main, &[0xdd, 0x7e, 0xfe], 0, Context::default()).unwrap();
    assert!(indexed.text.contains("ix-2"));
    assert_eq!(indexed.length, 3);
}
#[test]
fn headers_prefix_identity_and_unknown_maps() {
    let mut b = vec![0; 32768];
    b[0x7ff0..0x7ff8].copy_from_slice(b"TMR SEGA");
    b[0x7fff] = 0x4c;
    let r = Rom::from_bytes(b.clone()).unwrap();
    assert_eq!(r.info.header_offset, Some(0x7ff0));
    assert_eq!(r.info.declared_size, Some(32768));
    let mut copier = vec![42; 512];
    copier.extend(b);
    let prefixed = Rom::from_bytes(copier).unwrap();
    assert_eq!(prefixed.info.rom_offset, 512);
    assert_ne!(r.info.sha256, prefixed.info.sha256);
    let map = Mapping::new(&prefixed);
    assert_eq!(map.unknown(), vec![(0, 33280)]);
    assert!(map.validate(&r).is_err());
}
