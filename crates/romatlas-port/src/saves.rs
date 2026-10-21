//! Versioned, ROM-bound battery persistence. No emulator or save-state dependency.
use crate::{Game, assets::read_bounded};
use anyhow::{Result, ensure};
use serde::{Deserialize, Serialize};
use std::path::Path;

pub const MAX_SAVE_BYTES: usize = 1024 * 1024;
#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BatterySave {
    pub rom_sha256: String,
    pub version: u32,
    pub bytes: Vec<u8>,
}
impl BatterySave {
    pub fn capture(game: &impl Game, rom_sha256: &str) -> Result<Option<Self>> {
        let Some(version) = game.save_version() else {
            return Ok(None);
        };
        let bytes = game.save_data()?;
        ensure!(bytes.len() <= MAX_SAVE_BYTES, "battery save exceeds 1 MiB");
        Ok(Some(Self {
            rom_sha256: rom_sha256.into(),
            version,
            bytes,
        }))
    }
    pub fn restore(&self, game: &mut impl Game, rom_sha256: &str) -> Result<()> {
        ensure!(
            self.rom_sha256 == rom_sha256,
            "battery save ROM identity mismatch"
        );
        ensure!(
            game.save_version() == Some(self.version),
            "battery save version mismatch"
        );
        ensure!(
            self.bytes.len() <= MAX_SAVE_BYTES,
            "battery save exceeds 1 MiB"
        );
        // Game implementations validate fully before changing persistent state.
        game.load_save(&self.bytes)
    }
}
pub fn load(game: &mut impl Game, rom_sha256: &str, path: &Path) -> Result<()> {
    if game.save_version().is_some() && path.try_exists()? {
        let save: BatterySave =
            serde_json::from_slice(&read_bounded(path, MAX_SAVE_BYTES * 4 + 1024)?)?;
        save.restore(game, rom_sha256)?;
    }
    Ok(())
}
pub fn flush(game: &mut impl Game, rom_sha256: &str, path: &Path) -> Result<()> {
    if !game.save_dirty() {
        return Ok(());
    }
    if let Some(save) = BatterySave::capture(game, rom_sha256)? {
        crate::settings::write_atomic(path, &serde_json::to_vec(&save)?)?;
        game.save_acknowledged();
    }
    Ok(())
}
