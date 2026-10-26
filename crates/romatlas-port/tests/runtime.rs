use anyhow::{Result, ensure};
use romatlas_port::{
    Game,
    assets::{Collection, FileAsset, Manifest, Origin, Rom, RomAsset},
    audio::{Pulse, SampleClock},
    gb,
    graphics::{Frame, TileStyle, tile, tilemap},
    input::{Binding, InputLatch, InputMapper, InputState, PhysicalInput},
    settings::{Menu, MenuInput, Settings},
    timing::{Clock, TickRate},
};
use sha2::{Digest, Sha256};
use std::{
    fs,
    path::PathBuf,
    sync::atomic::{AtomicU64, Ordering},
    time::Duration,
};

struct Temporary(PathBuf);
impl Temporary {
    fn new() -> Self {
        static SERIAL: AtomicU64 = AtomicU64::new(0);
        let path = std::env::temp_dir().join(format!(
            "romatlas-port-test-{}-{}",
            std::process::id(),
            SERIAL.fetch_add(1, Ordering::Relaxed)
        ));
        fs::create_dir(&path).unwrap();
        Self(path)
    }
}
impl Drop for Temporary {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}
fn rom(bytes: Vec<u8>) -> Rom {
    let hash = format!("{:x}", Sha256::digest(&bytes));
    Rom::from_bytes(bytes, &hash).unwrap()
}

#[test]
fn identity_and_reads_reject_wrong_images_and_overflow() {
    let image = rom(vec![0, 1, 2, 3]);
    assert_eq!(image.read(1, 2).unwrap(), [1, 2]);
    assert!(image.read(4, 1).is_err());
    assert!(image.read(usize::MAX, 2).is_err());
    assert!(Rom::from_bytes(vec![4, 1, 2, 3], &image.sha256).is_err());
    assert!(Rom::from_bytes(Vec::new(), &image.sha256).is_err());
}

#[test]
fn original_fifteen_levels_survive_replacement_and_added_sixteenth() -> Result<()> {
    let temp = Temporary::new();
    let image = rom((1..=15).collect());
    let defaults: Vec<_> = (0..15)
        .map(|i| RomAsset {
            id: format!("level-{:02}", i + 1),
            offset: i,
            length: 1,
        })
        .collect();
    fs::write(temp.0.join("replacement"), [42])?;
    fs::write(temp.0.join("new"), [99])?;
    let mut manifest = Manifest {
        version: 1,
        entries: vec![
            FileAsset {
                id: "level-01".into(),
                file: "replacement".into(),
            },
            FileAsset {
                id: "level-16".into(),
                file: "new".into(),
            },
        ],
        order: vec![],
    };
    let validate = |_: &str, bytes: &[u8]| {
        ensure!(bytes.len() == 1, "level size");
        Ok(())
    };
    let loaded = Collection::load(&image, &defaults, Some((&manifest, &temp.0)), validate)?;
    assert_eq!(loaded.assets.len(), 16);
    assert_eq!(loaded.assets[0].bytes, [42]);
    assert_eq!(loaded.assets[14].bytes, [15]);
    assert_eq!(loaded.assets[15].bytes, [99]);
    assert!(matches!(
        loaded.assets[14].origin,
        Origin::Rom {
            offset: 14,
            length: 1
        }
    ));
    manifest.order = loaded.assets.iter().rev().map(|a| a.id.clone()).collect();
    assert_eq!(
        Collection::load(&image, &defaults, Some((&manifest, &temp.0)), validate)?.assets[0].id,
        "level-16"
    );
    manifest.order[0] = manifest.order[1].clone();
    assert!(Collection::load(&image, &defaults, Some((&manifest, &temp.0)), validate).is_err());
    manifest.order.clear();
    fs::write(temp.0.join("new"), [1, 2])?;
    assert!(Collection::load(&image, &defaults, Some((&manifest, &temp.0)), validate).is_err());
    Ok(())
}

#[test]
fn overrides_reject_duplicates_traversal_and_escaping_symlinks() -> Result<()> {
    let temp = Temporary::new();
    let image = rom(vec![1]);
    fs::write(temp.0.join("one"), [1])?;
    let mut manifest = Manifest {
        version: 1,
        entries: vec![FileAsset {
            id: "one".into(),
            file: "one".into(),
        }],
        order: vec![],
    };
    manifest.entries.push(manifest.entries[0].clone());
    assert!(Collection::load(&image, &[], Some((&manifest, &temp.0)), |_, _| Ok(())).is_err());
    manifest.entries.pop();
    manifest.entries[0].file = "../one".into();
    assert!(Collection::load(&image, &[], Some((&manifest, &temp.0)), |_, _| Ok(())).is_err());
    #[cfg(unix)]
    {
        let outside = Temporary::new();
        fs::write(outside.0.join("outside"), [2])?;
        std::os::unix::fs::symlink(outside.0.join("outside"), temp.0.join("link"))?;
        manifest.entries[0].file = "link".into();
        assert!(Collection::load(&image, &[], Some((&manifest, &temp.0)), |_, _| Ok(())).is_err());
    }
    Ok(())
}

#[test]
fn decoded_rom_recipes_and_modern_file_levels_share_a_schema() -> Result<()> {
    use romatlas_port::assets::Asset;
    let temp = Temporary::new();
    let image = rom(vec![3, 7]);
    // The game's recipe converts its original encoding to editor-friendly JSON
    // at startup. No decoded original is stored in the port's source.
    let bytes = serde_json::to_vec(
        &serde_json::json!({"width":image.read(0,1)?[0],"height":image.read(1,1)?[0]}),
    )?;
    let defaults = vec![Asset {
        id: "level-01".into(),
        bytes,
        origin: Origin::DerivedFromRom {
            recipe: "level-table".into(),
        },
    }];
    fs::write(temp.0.join("new.json"), br#"{"width":4,"height":8}"#)?;
    let manifest = Manifest {
        version: 1,
        entries: vec![FileAsset {
            id: "level-02".into(),
            file: "new.json".into(),
        }],
        order: vec![],
    };
    let levels = Collection::with_overrides(defaults, Some((&manifest, &temp.0)), |asset| {
        let value: serde_json::Value = serde_json::from_slice(&asset.bytes)?;
        ensure!(
            value["width"].as_u64().is_some_and(|v| v > 0),
            "invalid width"
        );
        Ok(())
    })?;
    assert_eq!(levels.assets.len(), 2);
    assert_eq!(
        serde_json::from_slice::<serde_json::Value>(&levels.assets[0].bytes)?["height"],
        7
    );
    assert_eq!(
        serde_json::from_slice::<serde_json::Value>(&levels.assets[1].bytes)?["height"],
        8
    );
    Ok(())
}

#[test]
fn sparse_oversized_input_is_rejected_before_loading() -> Result<()> {
    let temp = Temporary::new();
    let path = temp.0.join("oversized");
    let file = fs::File::create(&path)?;
    file.set_len(romatlas_port::assets::MAX_ROM_BYTES as u64 + 1)?;
    assert!(
        Rom::open(&path, &"0".repeat(64))
            .err()
            .unwrap()
            .to_string()
            .contains("exceeds")
    );
    Ok(())
}

#[test]
fn console_clock_is_exact_across_partitions_and_discards_stall_backlog() -> Result<()> {
    let rate = TickRate {
        numerator: 60,
        denominator: 1,
    };
    let mut clock = Clock::new(rate, 8)?;
    let ticks: u32 = (0..100)
        .map(|_| clock.advance(Duration::from_millis(10)).ticks)
        .sum();
    assert_eq!(ticks, 60);
    let stall = clock.advance(Duration::from_secs(10));
    assert_eq!(stall.ticks, 8);
    assert_eq!(stall.dropped_ticks, 592);
    assert_eq!(clock.advance(Duration::ZERO).ticks, 0);
    clock.advance(Duration::from_millis(10));
    clock.reset();
    assert_eq!(clock.advance(Duration::from_millis(10)).ticks, 0);
    let mut dmg = Clock::new(TickRate::DMG, 1000)?;
    assert_eq!(dmg.advance(Duration::from_secs(1)).ticks, 59);
    assert!(
        Clock::new(
            TickRate {
                numerator: 60,
                denominator: 0
            },
            8
        )
        .is_err()
    );
    Ok(())
}

#[test]
fn audio_sample_budget_has_no_per_frame_rounding_drift() -> Result<()> {
    let mut clock = SampleClock::new(TickRate::DMG, 48000)?;
    let frames: usize = (0..10000).map(|_| clock.next_tick()).sum();
    assert_eq!(frames, (48000_u64 * 70224 * 10000 / 4194304) as usize);
    let mut pulse = Pulse::default();
    let samples: Vec<_> = (0..100).map(|_| pulse.sample(1000.0, 0.5, 48000)).collect();
    assert!(samples.contains(&1.0) && samples.contains(&-1.0));
    assert_eq!(pulse.sample(f64::NAN, 0.5, 48000), 0.0);
    Ok(())
}

#[test]
fn tile_decoding_palette_flips_clipping_and_map_composition() -> Result<()> {
    let mut bytes = [0; 16];
    bytes[0] = 0x80;
    bytes[1] = 0x40;
    let indices = gb::tile_2bpp(&bytes)?;
    assert_eq!(&indices[..3], &[1, 2, 0]);
    assert!(gb::tile_2bpp(&bytes[..15]).is_err());
    let palette = gb::dmg_palette(0b00011011);
    assert_eq!(palette[0], gb::GRAYSCALE[3]);
    assert_eq!(gb::cgb_color(0x001f), [255, 0, 0, 255]);
    let mut frame = Frame::new(16, 8)?;
    frame.clear([9; 4]);
    tile(
        &mut frame,
        &indices,
        &gb::GRAYSCALE,
        -6,
        0,
        TileStyle {
            flip_x: true,
            transparent: Some(0),
            ..Default::default()
        },
    )?;
    assert_eq!(&frame.pixels[..8], &[85, 85, 85, 255, 170, 170, 170, 255]);
    assert_eq!(&frame.pixels[8..12], &[9; 4]);
    tilemap(&mut frame, &[indices], &[0, 0], 2, &gb::GRAYSCALE, (0, 0))?;
    assert_eq!(&frame.pixels[32..36], &gb::GRAYSCALE[1]);
    assert!(tilemap(&mut frame, &[indices], &[1], 1, &gb::GRAYSCALE, (0, 0)).is_err());
    Ok(())
}

#[test]
fn input_edges_dead_zones_and_focus_rearm_are_independent_of_devices() {
    let settings = Settings::default();
    let mut mapper = InputMapper::default();
    let mut physical = PhysicalInput::default();
    physical.keys.insert("Z".into());
    physical.axes.insert("leftx".into(), 0.1);
    let first = mapper.sample(&physical, &settings);
    assert!(first.pressed("a") && !first.held("right"));
    assert!(mapper.sample(&physical, &settings).pressed.is_empty());
    physical.axes.insert("leftx".into(), 0.8);
    assert!(mapper.sample(&physical, &settings).pressed("right"));
    let mut latch = InputLatch::default();
    latch.suppress(&physical);
    mapper.clear();
    let filtered = latch.filter(physical.clone(), settings.dead_zone);
    assert!(mapper.sample(&filtered, &settings).held.is_empty());
    physical.keys.clear();
    physical.axes.insert("leftx".into(), 0.0);
    latch.filter(physical.clone(), settings.dead_zone);
    physical.keys.insert("Z".into());
    assert!(
        mapper
            .sample(&latch.filter(physical, settings.dead_zone), &settings)
            .pressed("a")
    );
}

#[test]
fn persisted_settings_rebinding_cancellation_reset_and_rejection_preserve_data() -> Result<()> {
    let temp = Temporary::new();
    let path = temp.0.join("nested/settings.json");
    let defaults = Settings::default();
    let mut settings = defaults.clone();
    settings.volume = 0.4;
    settings.save(&path)?;
    assert_eq!(Settings::load(&path)?, settings);
    let saved = fs::read(&path)?;
    settings.volume = f32::NAN;
    assert!(settings.save(&path).is_err());
    assert_eq!(fs::read(&path)?, saved);
    settings = Settings::load(&path)?;
    let mut menu = Menu {
        open: true,
        selected: 10,
        ..Default::default()
    };
    menu.handle(MenuInput::Accept, &mut settings, &defaults, &[]);
    assert_eq!(menu.rebinding.as_deref(), Some("a"));
    menu.handle(MenuInput::Cancel, &mut settings, &defaults, &[]);
    assert!(menu.rebinding.is_none() && menu.open);
    menu.handle(MenuInput::Accept, &mut settings, &defaults, &[]);
    assert!(menu.capture(Binding::Key { name: "K".into() }, &mut settings));
    assert!(settings.bindings["a"].contains(&Binding::Key { name: "K".into() }));
    menu.handle(MenuInput::ClearBinding, &mut settings, &defaults, &[]);
    assert!(settings.bindings["a"].is_empty());
    menu.selected = 9;
    menu.handle(MenuInput::Accept, &mut settings, &defaults, &[]);
    assert_eq!(settings, defaults);
    menu.selected = 4;
    let pads = [("0".repeat(32), "PAD".into())];
    menu.handle(MenuInput::Right, &mut settings, &defaults, &pads);
    menu.handle(MenuInput::Right, &mut settings, &defaults, &pads);
    assert_eq!(settings.players[0], "0".repeat(32));
    assert!(menu.labels(&settings, &[])[4].contains("DISCONNECTED"));
    settings.bindings.get_mut("a").unwrap().push(Binding::Axis {
        name: "leftx".into(),
        direction: 0,
    });
    assert!(settings.validate().is_err());
    Ok(())
}

#[test]
fn headless_uses_scripted_game_logic_without_initializing_frontend() -> Result<()> {
    struct Example {
        position: u8,
    }
    impl Game for Example {
        fn tick(&mut self, input: &InputState) {
            if input.held("right") {
                self.position += 1;
            }
        }
        fn render(&self, frame: &mut Frame) {
            frame.pixel(i32::from(self.position), 0, [255, 0, 0, 255]);
        }
    }
    let mut game = Example { position: 0 };
    let frame = romatlas_port::headless(
        &mut game,
        5,
        |tick| {
            let mut input = InputState::default();
            if tick % 2 == 0 {
                input.held.insert("right".into());
            }
            input
        },
        (8, 1),
    )?;
    assert_eq!(&frame.pixels[12..16], &[255, 0, 0, 255]);
    assert!(
        romatlas_port::headless(&mut game, 1_000_001, |_| InputState::default(), (8, 1)).is_err()
    );
    Ok(())
}

#[test]
fn headless_players_delivers_independent_secondary_input() -> Result<()> {
    use romatlas_port::input::{MultiInputState, PlayerInput};
    struct Example {
        secondary: u8,
    }
    impl Game for Example {
        fn tick(&mut self, _: &InputState) {}
        fn tick_players(&mut self, input: &MultiInputState) {
            if input.players[1].held("right") {
                self.secondary += 1;
            }
        }
        fn render(&self, frame: &mut Frame) {
            frame.pixel(i32::from(self.secondary), 0, [255, 0, 0, 255]);
        }
    }
    let mut game = Example { secondary: 0 };
    romatlas_port::headless_players(
        &mut game,
        3,
        |_| {
            let mut input = MultiInputState::default();
            input.players[1] = PlayerInput {
                held: ["right".into()].into(),
                ..Default::default()
            };
            input
        },
        (4, 1),
    )?;
    assert_eq!(game.secondary, 3);
    Ok(())
}

#[cfg(feature = "desktop")]
#[test]
fn display_fit_calculations_need_no_display_server() {
    use romatlas_port::desktop::viewport;
    let fit = viewport((160, 144), (1920, 1080), true);
    assert_eq!(
        (fit.width(), fit.height(), fit.x(), fit.y()),
        (1120, 1008, 400, 36)
    );
    let small = viewport((160, 144), (80, 72), true);
    assert_eq!((small.width(), small.height()), (80, 72));
}

#[cfg(feature = "desktop")]
#[test]
fn stretched_native_pixels_keep_the_requested_display_aspect() {
    use romatlas_port::desktop::viewport_with_aspect;
    let fit = viewport_with_aspect((720, 240), (1920, 1080), false, Some((16, 9)));
    assert_eq!(
        (fit.width(), fit.height(), fit.x(), fit.y()),
        (1920, 1080, 0, 0)
    );
    let integer = viewport_with_aspect((720, 240), (1920, 1080), true, Some((16, 9)));
    assert_eq!(
        (integer.width(), integer.height(), integer.y()),
        (1707, 960, 60)
    );
    let small = viewport_with_aspect((720, 240), (320, 180), true, Some((16, 9)));
    assert_eq!((small.width(), small.height()), (320, 180));
}

#[test]
fn legacy_integer_scale_is_accepted_but_not_emitted_or_shown() -> Result<()> {
    let mut json = serde_json::to_value(Settings::default())?;
    assert!(json.get("integer_scale").is_none());
    json["integer_scale"] = true.into();
    json.as_object_mut().unwrap().remove("players");
    let settings: Settings = serde_json::from_value(json)?;
    settings.validate()?;
    assert_eq!(settings.players, ["auto", "auto", "auto", "auto"]);
    assert!(
        serde_json::to_value(&settings)?
            .get("integer_scale")
            .is_none()
    );
    assert!(
        !Menu::default()
            .labels(&settings, &[])
            .iter()
            .any(|label| label.contains("INTEGER"))
    );
    Ok(())
}
