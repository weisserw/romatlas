use romatlas_core::{Provenance, RegionKind};
use romatlas_nes::{
    Rom,
    decode::{self, Context, Cpu},
    mapping::{Entry, Mapping, View},
};
fn rom(mapper: u8) -> Rom {
    let mut data = vec![0; 16 + 32768];
    data[..4].copy_from_slice(b"NES\x1a");
    data[4] = 2;
    data[6] = mapper << 4;
    Rom::from_bytes(data).unwrap()
}
#[test]
fn decoding_keeps_operand_width_and_indirect_uncertainty() {
    let ctx = Context::default();
    let abs = decode::decode(Cpu::Main, &[0xad, 0x01, 0], 0x8000, ctx).unwrap();
    assert_eq!(abs.assembly.as_deref(), Some("lda a:$0001"));
    assert_eq!(abs.length, 3);
    let branch = decode::decode(Cpu::Main, &[0xd0, 0xfc], 0x8000, ctx).unwrap();
    assert_eq!(branch.successors, vec![0x8002, 0x7ffe]);
    let jmp = decode::decode(Cpu::Main, &[0x6c, 0xff, 0x80], 0x8000, ctx).unwrap();
    assert!(jmp.successors.is_empty());
    assert_eq!(jmp.references[0]["indirect"], true);
    assert!(decode::decode(Cpu::Main, &[0x20, 0], 0x8000, ctx).is_err());
}
#[test]
fn code_mapping_checks_operand_bank_boundaries_and_rejected_edit_is_transactional() {
    let r = rom(2);
    let mut m = Mapping::new(&r);
    let entry = Entry {
        id: 0,
        offset: 16 + 8191,
        length: 3,
        kind: RegionKind::Code,
        provenance: Provenance::Static,
        confidence: 1.0,
        label: Some("Boundary".into()),
        comment: String::new(),
        evidence: String::new(),
        views: vec![View {
            cpu: Cpu::Main,
            cpu_address: 0x9fff,
            context: Context {
                prg_banks: [Some(0), Some(2), Some(2), Some(3)],
            },
            uploaded: false,
        }],
    };
    // A three-byte JSR would cross the CPU slot into a nonadjacent physical bank.
    let mut r = r;
    r.bytes[16 + 8191] = 0x20;
    let before = serde_json::to_value(&m).unwrap();
    assert!(m.annotate(&r, entry.clone()).is_err());
    assert_eq!(serde_json::to_value(&m).unwrap(), before);
    let mut valid = entry;
    valid.views[0].context.prg_banks[1] = Some(1);
    assert!(m.annotate(&r, valid).is_ok());
}
#[test]
fn static_aliases_and_banked_identity_are_distinct() {
    let mut r = rom(0);
    r.bytes.truncate(16 + 16384);
    r.bytes[4] = 1;
    r = Rom::from_bytes(r.bytes).unwrap();
    assert_eq!(r.translate(0x8000), r.translate(0xc000));
    let banked = rom(2);
    assert!(banked.translate(0x8000).is_none());
    assert!(banked.translate(0x4000).is_none());
    assert_eq!(banked.translate(0x1800).unwrap().offset, 0);
}
