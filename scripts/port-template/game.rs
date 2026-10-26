//! Replace this demonstration with the game's readable, native rules and state.
//! This tile is arbitrary ROM data, not an identified gameplay asset.
use anyhow::{Result, ensure};
use romatlas_port::{Game, assets::{Asset, MemoryAssets, Origin, Rom, RomAsset}, gb,
    graphics::{Frame, TileStyle, tile}, input::InputState};
#[cfg(not(target_arch = "wasm32"))]
use std::path::Path;

#[derive(Clone)]
pub struct Port { tile: [u8;64] }
impl Port {
    #[cfg(not(target_arch = "wasm32"))]
    pub fn load(rom: &Rom, overrides: Option<&Path>) -> Result<Self> {
        let memory;
        let overrides = if let Some(root) = overrides {
            memory = MemoryAssets::from_directory(root, &["tiles.json"])?;
            &memory
        } else { &MemoryAssets::default() };
        Self::load_assets(rom, overrides)
    }
    /// Shared startup extraction for desktop and browser, including overrides.
    pub fn load_assets(rom: &Rom, overrides: &MemoryAssets) -> Result<Self> {
        let defaults = [RomAsset { id: "demo-tile".into(), offset: 0, length: 16 }];
        let assets = defaults.iter().map(|entry| Ok(Asset {
            id: entry.id.clone(), bytes: rom.read(entry.offset, entry.length)?.to_vec(),
            origin: Origin::Rom { offset: entry.offset, length: entry.length },
        })).collect::<Result<Vec<_>>>()?;
        let tiles = overrides.collection("tiles.json", assets, |asset| {
            let bytes = &asset.bytes;
            ensure!(bytes.len() == 16,"demo tiles must be 16-byte Game Boy 2bpp data"); Ok(())
        })?;
        let tile = gb::tile_2bpp(&tiles.get("demo-tile").unwrap().bytes)?;
        Ok(Self { tile })
    }
}
impl Game for Port {
    fn display_aspect_ratio(&self) -> Option<(u32,u32)> { Some(crate::config::DISPLAY_ASPECT) }
    // This demonstration has no mutable gameplay state. Replace these hooks
    // with versioned serialization of all game/audio state when implementing it.
    fn snapshot_json(&self) -> Result<String> { Ok("{}".into()) }
    fn restore_snapshot_json(&mut self, json: &str) -> Result<()> {
        ensure!(json == "{}", "invalid demonstration snapshot"); Ok(())
    }
    fn reset(&mut self) -> Result<()> { Ok(()) }
    fn tick(&mut self, _input: &InputState) {
        // Implement gameplay here. This scaffold implements no game behavior.
    }
    fn render(&self, frame: &mut Frame) {
        frame.clear(gb::GRAYSCALE[0]);
        tile(frame,&self.tile,&gb::GRAYSCALE,76,68,TileStyle::default()).expect("validated tile");
    }
    // Implement original music/sound behavior using ROM-backed recipes. The
    // default Game::audio is silence; leaving it silent is not port completion.
}
