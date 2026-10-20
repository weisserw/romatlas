use romatlas_core::{
    Provenance, RegionKind,
    execution::{ExecutionAdapter, StepRequest, StepUnit},
};
use romatlas_pce::{
    Rom,
    decode::{self, Context, Cpu},
    mapping::{Entry, Mapping, View, resolve},
};
fn rom(size: usize, prefix: bool) -> Rom {
    let mut data = vec![0xea; size];
    data[0x1ffe..0x2000].copy_from_slice(&[0, 0xe0]);
    if prefix {
        data.splice(0..0, [0x17; 512]);
    }
    Rom::from_bytes(data).unwrap()
}
#[test]
fn huc6280_extensions_and_context_invalidation() {
    for (data, length) in [
        (&[0x73, 1, 2, 3, 4, 5, 6][..], 7),
        (&[0x83, 1, 2][..], 3),
        (&[0x93, 1, 2, 3][..], 4),
        (&[0x0f, 1, 127][..], 3),
        (&[0x53, 1][..], 2),
    ] {
        let mut ctx = Context::default();
        ctx.mpr[0] = Some(0);
        let i = decode::decode(Cpu::Main, data, 0xe000, ctx).unwrap();
        assert_eq!(i.length, length);
        if data[0] == 0x53 {
            assert_eq!(i.after.mpr[0], None);
        }
        assert!(decode::decode(Cpu::Main, &data[..length - 1], 0xe000, ctx).is_err());
    }
    let bitbranch =
        decode::decode(Cpu::Main, &[0x0f, 0x24, 0xfc], 0xe000, Context::default()).unwrap();
    assert!(bitbranch.successors.contains(&0xdfff));
    assert_eq!(bitbranch.references[0]["cpu_address"], 0x2024);
    let indirect = decode::decode(Cpu::Main, &[0x6c, 0, 0x40], 0xe000, Context::default()).unwrap();
    assert!(indirect.successors.is_empty());
    assert_eq!(indirect.references[0]["indirect"], true);
}
#[test]
fn chip_layouts_aliases_and_unmapped_pages() {
    for (size, cases) in [
        (
            393216,
            vec![(0x40000, 0), (0x80000, 0x40000), (0xa0000, 0x40000)],
        ),
        (
            524288,
            vec![(0x40000, 0x40000), (0x80000, 0x40000), (0xc0000, 0x40000)],
        ),
        (786432, vec![(0xc0000, 0x80000)]),
    ] {
        let r = rom(size, true);
        for (bus, offset) in cases {
            assert_eq!(r.bus_binding(bus, None).unwrap().offset, offset + 512);
        }
        let mut ctx = Context::default();
        ctx.mpr[7] = Some(0);
        assert_eq!(resolve(&r, 0xe000, ctx).unwrap().offset, 512);
        assert_eq!(resolve(&r, 0xc000, ctx), None);
        ctx.mpr[6] = Some(0xf8);
        assert_eq!(resolve(&r, 0xc000, ctx).unwrap().space, "ram");
        ctx.mpr[6] = Some(0x80);
        assert_eq!(resolve(&r, 0xc000, ctx), None);
    }
}
#[test]
fn bank_context_validation_and_transactional_annotations() {
    let r = rom(16384, false);
    let mut m = Mapping::new(&r);
    let mut ctx = Context::default();
    ctx.mpr[7] = Some(0);
    let region = Entry {
        id: 0,
        offset: 0,
        length: 1,
        kind: RegionKind::Code,
        provenance: Provenance::Manual,
        confidence: 1.0,
        label: Some("Entry".into()),
        comment: String::new(),
        evidence: "test".into(),
        views: vec![View {
            cpu: Cpu::Main,
            cpu_address: 0xe000,
            context: ctx,
            uploaded: false,
        }],
    };
    m.annotate(&r, region.clone()).unwrap();
    let saved = serde_json::to_value(&m).unwrap();
    assert!(m.annotate(&r, region.clone()).is_err());
    assert_eq!(serde_json::to_value(&m).unwrap(), saved);
    let mut bad = region;
    bad.id = 1;
    bad.views[0].context.mpr[7] = Some(1);
    assert!(m.annotate(&r, bad).is_err());
    assert_eq!(serde_json::to_value(&m).unwrap(), saved);
    let mut wrong = r.clone();
    wrong.bytes[10] = 1;
    let wrong = Rom::from_bytes(wrong.bytes).unwrap();
    assert!(m.validate(&wrong).is_err());
}
#[test]
fn live_translation_matches_original_chip_backing_and_deterministic_reset() {
    use romatlas_pce::{emulator::Emulator, image::address};
    for size in [8192, 393216, 524288, 786432] {
        let mut r = rom(size, true);
        for bank in 1..size / 8192 {
            r.bytes[512 + bank * 8192] = bank as u8;
        }
        let r = Rom::from_bytes(r.bytes).unwrap();
        let mut e = Emulator::from_rom(r.clone()).unwrap();
        for page in [0, 1, 32, 48, 64, 96, 127, 248, 255] {
            e.set_registers(Cpu::Main, &serde_json::json!({"mpr2":page}))
                .unwrap();
            let a = e.translate(&address("cpu", 0x4000)).unwrap();
            assert_eq!(a, r.bus_binding(page * 8192, None));
            if let Some(a) = a.filter(|a| a.space == "rom") {
                assert_eq!(
                    e.inspect(&address("cpu", 0x4000), 1).unwrap(),
                    [r.bytes[a.offset as usize]]
                );
            }
        }
        e.reset().unwrap();
        let first = serde_json::to_value(e.state()).unwrap();
        e.execute(StepRequest {
            unit: StepUnit::Operation,
            count: 1,
            max_operations: 10,
            timeout_ms: 1000,
        })
        .unwrap();
        e.reset().unwrap();
        assert_eq!(serde_json::to_value(e.state()).unwrap(), first);
    }
}

#[test]
fn invalid_sf2_window_context_never_wraps_to_rom() {
    let mut r = rom(2560 * 1024, false);
    r.info.mapper = "sf2".into();
    for bank in [None, Some(0), Some(5), Some(255)] {
        assert!(r.bus_binding(0x80000, bank).is_none());
    }
    for bank in 1..=4 {
        assert_eq!(
            r.bus_binding(0x80000, Some(bank)).unwrap().offset,
            u64::from(bank) * 0x80000
        );
    }
}
