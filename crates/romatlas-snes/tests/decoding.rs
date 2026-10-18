use romatlas_core::{Provenance, RegionKind};
use romatlas_disasm::DisassemblyBackend;
use romatlas_snes::{
    Rom,
    decode::{self, Context, Cpu},
    disassembly::Backend,
    mapping::{Entry, Mapping, View},
};
fn rom() -> Vec<u8> {
    let mut b = vec![0; 65536];
    b[0x7fc0..0x7fd5].copy_from_slice(b"ROMATLAS SYNTHETIC   ");
    b[0x7fd5] = 0x20;
    b[0x7fd7] = 6;
    b[0x7fd9] = 1;
    b[0x7ffc..0x7ffe].copy_from_slice(&0x8000u16.to_le_bytes());
    b
}
#[test]
fn only_relevant_context_controls_lengths() {
    assert!(decode::decode(Cpu::Main, &[0xa9, 0, 0], 0x8000, Context::default()).is_err());
    assert_eq!(
        decode::decode(Cpu::Main, &[0xea], 0x8000, Context::default())
            .unwrap()
            .length,
        1
    );
    assert_eq!(
        decode::decode(
            Cpu::Main,
            &[0xa9, 0, 0],
            0x8000,
            Context {
                m: Some(false),
                ..Default::default()
            }
        )
        .unwrap()
        .length,
        3
    );
    assert_eq!(
        decode::decode(
            Cpu::Main,
            &[0xa2, 0, 0],
            0x8000,
            Context {
                x: Some(true),
                ..Default::default()
            }
        )
        .unwrap()
        .length,
        2
    );
    let clc = decode::decode(
        Cpu::Main,
        &[0x18],
        0x8000,
        Context {
            e: Some(true),
            m: Some(true),
            x: Some(true),
            ..Default::default()
        },
    )
    .unwrap();
    let xce = decode::decode(Cpu::Main, &[0xfb], 0x8001, clc.after).unwrap();
    assert_eq!(xce.after.e, Some(false));
    let rep = decode::decode(Cpu::Main, &[0xc2, 0x30], 0x8002, xce.after).unwrap();
    assert_eq!(
        decode::decode(Cpu::Main, &[0xa9, 0, 0], 0x8004, rep.after)
            .unwrap()
            .length,
        3
    );
}
#[test]
fn every_opcode_and_width_combination_rebuilds() {
    let mut bytes = rom();
    let mut entries = Vec::new();
    let mut id = 1;
    for m in [false, true] {
        for x in [false, true] {
            for opcode in 0..=255u8 {
                let offset = 0x1000 + (id - 1) * 8;
                let data = [opcode, 0x12, 0x34, 0x56];
                bytes[offset..offset + 4].copy_from_slice(&data);
                let context = Context {
                    e: Some(false),
                    m: Some(m),
                    x: Some(x),
                    ..Default::default()
                };
                let pc = 0x8000 + offset as u32;
                let instruction = decode::decode(Cpu::Main, &data, pc, context).unwrap();
                entries.push(Entry {
                    id: id as u64,
                    offset,
                    length: instruction.length,
                    kind: RegionKind::Code,
                    provenance: Provenance::Static,
                    confidence: 1.0,
                    label: None,
                    comment: String::new(),
                    evidence: "synthetic opcode contract".into(),
                    views: vec![View {
                        cpu: Cpu::Main,
                        cpu_address: pc,
                        context,
                        uploaded: false,
                    }],
                });
                id += 1;
            }
        }
    }
    for opcode in 0..=255u8 {
        let offset = 0x1000 + (id - 1) * 8;
        let data = [opcode, 0x12, 0x34];
        bytes[offset..offset + 3].copy_from_slice(&data);
        let pc = 0x200 + u32::from(opcode) * 8;
        let context = Context::default();
        let instruction = decode::decode(Cpu::Spc700, &data, pc, context).unwrap();
        entries.push(Entry {
            id: id as u64,
            offset,
            length: instruction.length,
            kind: RegionKind::Code,
            provenance: Provenance::Static,
            confidence: 1.0,
            label: None,
            comment: String::new(),
            evidence: "synthetic SPC opcode contract".into(),
            views: vec![View {
                cpu: Cpu::Spc700,
                cpu_address: pc,
                context,
                uploaded: true,
            }],
        });
        id += 1;
    }
    let image = Rom::from_bytes(bytes.clone()).unwrap();
    let mut map = Mapping::new(&image);
    map.regions = entries;
    map.next_id = map.regions.iter().map(|r| r.id).max().unwrap() + 1;
    map.validate(&image).unwrap();
    let work = std::env::temp_dir().join(format!("romatlas-snes-opcodes-{}", std::process::id()));
    std::fs::create_dir(&work).unwrap();
    let path = work.join("original.sfc");
    std::fs::write(&path, bytes).unwrap();
    let (source, _) = romatlas_snes::disassembly::render(
        &image,
        &map,
        &romatlas_disasm::Budget::new(30000).unwrap(),
    )
    .unwrap();
    std::fs::write(work.join("game.asm"), source).unwrap();
    std::fs::write(work.join("image.sfc"), &image.bytes).unwrap();
    let result = Backend.generate(&path, &map, &work.join("generated"), 30000);
    if let Err(e) = result {
        panic!(
            "opcode reconstruction failed; artifacts {}: {e:#}",
            work.display()
        );
    }
    Backend
        .verify(&path, &work.join("generated"), 30000)
        .unwrap();
    std::fs::remove_dir_all(work).unwrap();
}

#[test]
fn aliases_copier_header_wrapping_and_alternate_views() {
    let mut payload = rom();
    payload[0x1000..0x1004].copy_from_slice(&[0xaf, 0x54, 0, 0xc0]);
    payload[0x1010..0x1014].copy_from_slice(&[0xaf, 0x54, 0, 0x40]);
    payload[0x2000..0x2003].copy_from_slice(&[0x03, 0x12, 0x20]);
    payload[0x3000..0x3003].copy_from_slice(&[0xa9, 0x12, 0xea]);
    payload[0x7ffe..0x8000].copy_from_slice(&[0x80, 0]);
    let mut bytes = vec![0x42; 512];
    bytes.extend(payload);
    let image = Rom::from_bytes(bytes.clone()).unwrap();
    let mut map = Mapping::new(&image);
    let cases = [
        (0x1000, 4, 0x9000, Cpu::Main, Context::default(), false),
        (0x1010, 4, 0x9010, Cpu::Main, Context::default(), false),
        (0x2000, 3, 0xfff0, Cpu::Spc700, Context::default(), true),
        (
            0x3000,
            3,
            0xb000,
            Cpu::Main,
            Context {
                m: Some(false),
                ..Default::default()
            },
            false,
        ),
        (0x7ffe, 2, 0xfffe, Cpu::Main, Context::default(), false),
    ];
    for (index, (offset, length, cpu_address, cpu, context, uploaded)) in
        cases.into_iter().enumerate()
    {
        map.regions.push(Entry {
            id: index as u64 + 1,
            offset: offset + 512,
            length,
            kind: RegionKind::Code,
            provenance: Provenance::Static,
            confidence: 1.0,
            label: None,
            comment: "Synthetic address/encoding contract".into(),
            evidence: "Original fixture".into(),
            views: vec![View {
                cpu,
                cpu_address,
                context,
                uploaded,
            }],
        });
    }
    map.next_id = 6;
    map.regions[3].views.push(View {
        cpu: Cpu::Main,
        cpu_address: 0xb000,
        context: Context {
            m: Some(true),
            ..Default::default()
        },
        uploaded: false,
    });
    map.validate(&image).unwrap();
    assert_eq!(
        image.rom_offset(0xc00054).unwrap(),
        image.rom_offset(0x400054).unwrap()
    );
    assert_eq!(
        decode::decode(Cpu::Main, &[0x80, 0], 0xfffe, Context::default())
            .unwrap()
            .target,
        Some(0)
    );
    assert_eq!(
        decode::decode(Cpu::Main, &[0x82, 0xfc, 0xff], 0xfffd, Context::default())
            .unwrap()
            .target,
        Some(0xfffc)
    );
    let work = std::env::temp_dir().join(format!("romatlas-snes-aliases-{}", std::process::id()));
    std::fs::create_dir(&work).unwrap();
    let file = work.join("headered.smc");
    std::fs::write(&file, bytes).unwrap();
    Backend
        .generate(&file, &map, &work.join("generated"), 30000)
        .unwrap();
    Backend
        .verify(&file, &work.join("generated"), 30000)
        .unwrap();
    let text = std::fs::read_to_string(work.join("generated/game.asm")).unwrap();
    assert!(text.contains("$c00054") && text.contains("$400054"));
    std::fs::remove_dir_all(work).unwrap();
}

#[test]
fn uploaded_traversal_is_bounded_and_not_coverage() {
    let mut bytes = rom();
    bytes[0x1000..0x1004].copy_from_slice(&[0xe8, 0x42, 0xea, 0xea]);
    let image = Rom::from_bytes(bytes).unwrap();
    let map = Mapping::new(&image);
    let view = serde_json::json!({"cpu":"spc700","cpu_address":512,"uploaded":true});
    let roots = std::collections::BTreeMap::new();
    let edges = std::collections::BTreeMap::new();
    assert!(
        romatlas_snes::analysis::discover(
            &image,
            &map,
            &serde_json::json!({"entrypoints":[{"offset":4096,"view":view}]}),
            &roots,
            &edges
        )
        .is_err()
    );
    let r=romatlas_snes::analysis::discover(&image,&map,&serde_json::json!({"entrypoints":[{"offset":4096,"length":2,"view":view}],"max_instructions":10}),&roots,&edges).unwrap();
    assert_eq!(r["candidates"].as_array().unwrap().len(), 1);
    assert_eq!(r["candidates"][0]["execution_coverage"], false);
}

#[test]
fn references_preserve_operand_roles() {
    let context = Context {
        direct_page: Some(0x100),
        ..Default::default()
    };
    let store = decode::decode(Cpu::Spc700, &[0x8f, 0x34, 0x12], 0x200, context).unwrap();
    assert_eq!(store.references[0]["cpu_address"], 0x112);
    let pair = decode::decode(Cpu::Spc700, &[0xfa, 0x34, 0x12], 0x200, context).unwrap();
    assert_eq!(pair.references[0]["cpu_address"], 0x134);
    assert_eq!(pair.references[1]["cpu_address"], 0x112);
    let jump = decode::decode(Cpu::Main, &[0x4c, 0x00, 0x90], 0x8000, Context::default()).unwrap();
    assert!(
        jump.references
            .iter()
            .all(|r| r["kind"] == "control-target")
    );
}
