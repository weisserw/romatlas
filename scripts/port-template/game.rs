//! Replace this demonstration with the game's readable, native rules and state.
//! This tile is arbitrary ROM data, not an identified gameplay asset.
use anyhow::{Result, ensure};
use romatlas_port::{Game, assets::{Collection, Manifest, Rom, RomAsset}, gb,
    graphics::{Frame, TileStyle, tile}, input::InputState};
use std::path::Path;

pub struct Port { tile: [u8;64] }
impl Port {
    pub fn load(rom: &Rom, overrides: Option<&Path>) -> Result<Self> {
        let defaults = [RomAsset { id: "demo-tile".into(), offset: 0, length: 16 }];
        let manifest = overrides.map(|root| Manifest::load(&root.join("tiles.json"))).transpose()?;
        let overlay = manifest.as_ref().zip(overrides);
        let tiles = Collection::load(rom,&defaults,overlay,|_,bytes| {
            ensure!(bytes.len() == 16,"demo tiles must be 16-byte Game Boy 2bpp data"); Ok(())
        })?;
        let tile = gb::tile_2bpp(&tiles.get("demo-tile").unwrap().bytes)?;
        Ok(Self { tile })
    }
}
impl Game for Port {
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
