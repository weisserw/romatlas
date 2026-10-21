use anyhow::{Result, ensure};
use romatlas_port::{
    Game,
    assets::{Asset, MemoryAssets, Origin},
    browser::Runtime,
    graphics::Frame,
    input::{InputState, PhysicalInput},
    saves::{self, BatterySave},
    settings::Settings,
    timing::TickRate,
};

#[derive(Default)]
struct Battery {
    value: u8,
    dirty: bool,
}
impl Game for Battery {
    fn tick(&mut self, input: &InputState) {
        if input.held("a") {
            self.value = self.value.wrapping_add(1);
            self.dirty = true;
        }
    }
    fn render(&self, frame: &mut Frame) {
        frame.clear([self.value, 0, 0, 255]);
    }
    fn save_version(&self) -> Option<u32> {
        Some(1)
    }
    fn load_save(&mut self, bytes: &[u8]) -> Result<()> {
        ensure!(bytes.len() == 1, "invalid battery payload");
        self.value = bytes[0];
        Ok(())
    }
    fn save_data(&self) -> Result<Vec<u8>> {
        Ok(vec![self.value])
    }
    fn save_dirty(&self) -> bool {
        self.dirty
    }
    fn save_acknowledged(&mut self) {
        self.dirty = false;
    }
}
#[test]
fn browser_boundaries_clock_and_battery_validation() -> Result<()> {
    let mut runtime = Runtime::new(
        Battery::default(),
        (8, 8),
        TickRate::DMG,
        Settings::default(),
        48000,
        "owned".into(),
    )?;
    let mut input: [PhysicalInput; 4] = Default::default();
    input[0].keys.insert("Z".into());
    assert_eq!(runtime.advance(1000.0, input.clone())?, 8); // Bounded catch-up.
    assert_eq!(runtime.game.value, 8);
    assert!(runtime.samples.len() > 100);
    assert_eq!(runtime.frame.pixels[0], 8);
    let saved = runtime.save_json()?;
    assert!(!saved.is_empty());
    assert!(runtime.game.save_dirty()); // Export is not acknowledgement.
    runtime.game.save_acknowledged();
    assert!(runtime.save_json()?.is_empty());
    runtime.boundary(&input);
    runtime.advance(20.0, input.clone())?;
    assert_eq!(runtime.game.value, 8); // Held input suppressed until neutral.
    input[0].keys.clear();
    runtime.advance(20.0, input.clone())?;
    input[0].keys.insert("Z".into());
    runtime.advance(20.0, input)?;
    assert!(runtime.game.value > 8);
    runtime.restore(&saved)?;
    assert_eq!(runtime.game.value, 8);
    let mut save: BatterySave = serde_json::from_str(&saved)?;
    save.rom_sha256 = "foreign".into();
    assert!(save.restore(&mut runtime.game, "owned").is_err());
    save.rom_sha256 = "owned".into();
    save.version = 2;
    assert!(save.restore(&mut runtime.game, "owned").is_err());
    save.version = 1;
    save.bytes = vec![1, 2];
    assert!(save.restore(&mut runtime.game, "owned").is_err());
    assert_eq!(runtime.game.value, 8);
    assert!(runtime.advance(f64::NAN, Default::default()).is_err());
    Ok(())
}
#[test]
fn desktop_battery_persistence_acknowledges_only_success() -> Result<()> {
    let base = std::env::temp_dir().join(format!("romatlas-battery-{}", std::process::id()));
    std::fs::create_dir_all(&base)?;
    let path = base.join("save.json");
    let mut game = Battery {
        value: 42,
        dirty: true,
    };
    assert!(saves::flush(&mut game, "owned", &base).is_err());
    assert!(game.dirty);
    saves::flush(&mut game, "owned", &path)?;
    assert!(!game.dirty);
    let mut restored = Battery::default();
    saves::load(&mut restored, "owned", &path)?;
    assert_eq!(restored.value, 42);
    assert!(saves::load(&mut restored, "foreign", &path).is_err());
    std::fs::remove_file(path)?;
    std::fs::remove_dir(base)?;
    Ok(())
}
#[test]
fn fetched_overrides_preserve_order_and_reject_escaping_paths() -> Result<()> {
    let defaults = || {
        vec![Asset {
            id: "original".into(),
            bytes: vec![0],
            origin: Origin::DerivedFromRom {
                recipe: "test".into(),
            },
        }]
    };
    let json = r#"{"manifests":{"tiles.json":{"version":1,"entries":[{"id":"original","file":"one.bin"},{"id":"new","file":"two.bin"}],"order":["new","original"]}},"files":{"one.bin":[1],"two.bin":[2]}}"#;
    let enabled: MemoryAssets =
        serde_json::from_str(r#"{"enabled":true,"manifests":{},"files":{}}"#)?;
    assert!(
        enabled
            .collection("tiles.json", defaults(), |_| Ok(()))
            .is_err()
    );
    let assets: MemoryAssets = serde_json::from_str(json)?;
    let collection = assets.collection("tiles.json", defaults(), |asset| {
        ensure!(asset.bytes.len() == 1, "invalid schema");
        Ok(())
    })?;
    assert_eq!(collection.assets[0].id, "new");
    assert_eq!(collection.assets[1].bytes, vec![1]);
    let invalid: MemoryAssets = serde_json::from_str(&json.replace("one.bin", "../one.bin"))?;
    assert!(
        invalid
            .collection("tiles.json", defaults(), |_| Ok(()))
            .is_err()
    );
    let missing: MemoryAssets = serde_json::from_str(&json.replace("\"one.bin\":[1],", ""))?;
    assert!(
        missing
            .collection("tiles.json", defaults(), |_| Ok(()))
            .is_err()
    );
    Ok(())
}
