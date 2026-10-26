#![cfg(feature = "snapshots")]
use romatlas_port::{
    Game,
    browser::Runtime,
    graphics::Frame,
    input::{InputState, PhysicalInput},
    nes::apu::Apu,
    settings::Settings,
    timing::TickRate,
};

#[derive(Default)]
struct Probe;
impl Game for Probe {
    fn tick(&mut self, _: &InputState) {}
    fn render(&self, _: &mut Frame) {}
}
fn runtime() -> Runtime<Probe> {
    Runtime::new(
        Probe,
        (1, 1),
        TickRate::NES_NTSC,
        Settings::default(),
        48000,
        "snapshot-test".into(),
    )
    .unwrap()
}
#[test]
fn driver_snapshot_round_trip_and_invalid_input_are_atomic() {
    let mut original = runtime();
    original
        .advance(21.125, std::array::from_fn(|_| PhysicalInput::default()))
        .unwrap();
    let saved = original.driver_snapshot_json().unwrap();
    let mut restored = runtime();
    restored.restore_driver_snapshot_json(&saved).unwrap();
    for elapsed in [0.125, 13.25, 35.375, 8.5] {
        assert_eq!(
            original.advance(elapsed, Default::default()).unwrap(),
            restored.advance(elapsed, Default::default()).unwrap()
        );
        assert_eq!(
            original.driver_snapshot_json().unwrap(),
            restored.driver_snapshot_json().unwrap()
        );
    }
    let before = restored.driver_snapshot_json().unwrap();
    for (field, value) in [
        ("rom_sha256", serde_json::json!("wrong")),
        ("sample_rate", serde_json::json!(44100)),
        (
            "clock",
            serde_json::json!({"rate":{"numerator":0,"denominator":0},"remainder":0,"max_catch_up":8}),
        ),
    ] {
        let mut invalid: serde_json::Value = serde_json::from_str(&saved).unwrap();
        invalid[field] = value;
        assert!(
            restored
                .restore_driver_snapshot_json(&invalid.to_string())
                .is_err()
        );
        assert_eq!(before, restored.driver_snapshot_json().unwrap());
    }
}
#[test]
fn apu_snapshot_preserves_bit_exact_sample_continuation() {
    let mut apu = Apu::default();
    apu.write(0x4015, 0x0f).unwrap();
    apu.write(0x4000, 0x9f).unwrap();
    apu.write(0x4002, 0x70).unwrap();
    apu.write(0x4003, 0x08).unwrap();
    apu.render(713, 48000, |_| 0, &mut Vec::new()).unwrap();
    let saved = serde_json::to_string(&apu).unwrap();
    let mut restored: Apu = serde_json::from_str(&saved).unwrap();
    let (mut a, mut b) = (Vec::new(), Vec::new());
    apu.render(1024, 48000, |_| 0, &mut a).unwrap();
    restored.render(1024, 48000, |_| 0, &mut b).unwrap();
    assert_eq!(
        a.iter().map(|v| v.to_bits()).collect::<Vec<_>>(),
        b.iter().map(|v| v.to_bits()).collect::<Vec<_>>()
    );
    assert_eq!(
        serde_json::to_string(&apu).unwrap(),
        serde_json::to_string(&restored).unwrap()
    );
}
