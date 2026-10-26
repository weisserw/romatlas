//! Browser entry point; the same game module is used by the desktop executable.
#[cfg(all(feature = "web", target_arch = "wasm32"))]
mod game;
#[cfg(all(feature = "web", target_arch = "wasm32"))]
mod config;
#[cfg(all(feature = "web", target_arch = "wasm32"))]
mod web {
    use super::{game::Port, config};
    use romatlas_port::{Game, assets::{MemoryAssets, Rom}, browser::Runtime};
    use wasm_bindgen::prelude::*;
    fn error(e: anyhow::Error) -> JsValue { JsValue::from_str(&format!("{e:#}")) }
    /// Transfer binary files directly into Rust instead of expanding bytes into JSON arrays.
    #[wasm_bindgen]
    pub struct BrowserAssets { assets: MemoryAssets }
    #[wasm_bindgen]
    impl BrowserAssets {
        #[wasm_bindgen(constructor)]
        pub fn new(metadata: &str) -> Result<BrowserAssets, JsValue> {
            if metadata.len() > 4*1024*1024 { return Err(JsValue::from_str("asset metadata exceeds 4 MiB")) }
            let assets: MemoryAssets = serde_json::from_str(metadata).map_err(|e| error(e.into()))?;
            assets.validate().map_err(error)?;
            Ok(Self { assets })
        }
        pub fn add_file(&mut self, name: String, bytes: Vec<u8>) -> Result<(), JsValue> {
            if self.assets.files.contains_key(&name) || name.len()>4096 { return Err(JsValue::from_str("invalid or duplicate asset path")) }
            let total: usize = self.assets.files.values().map(Vec::len).sum();
            if bytes.len()>romatlas_port::assets::MAX_ASSET_BYTES || total+bytes.len()>romatlas_port::assets::MAX_ROM_BYTES { return Err(JsValue::from_str("asset size limit exceeded")) }
            self.assets.files.insert(name,bytes); Ok(())
        }
    }
    #[wasm_bindgen]
    pub struct BrowserGame { runtime: Runtime<Port> }
    #[wasm_bindgen]
    impl BrowserGame {
        #[wasm_bindgen(constructor)]
        pub fn new(bytes: Vec<u8>, assets: BrowserAssets, sample_rate: u32) -> Result<BrowserGame, JsValue> {
            let rom = Rom::from_bytes(bytes,config::ROM_SHA256).map_err(error)?;
            let game = Port::load_assets(&rom, &assets.assets).map_err(error)?;
            let runtime = Runtime::new(game,config::SIZE,config::RATE,
                config::settings(),sample_rate,rom.sha256).map_err(error)?;
            Ok(Self { runtime })
        }
        pub fn snapshots_supported(&self) -> bool { self.runtime.game.snapshot_json().is_ok() }
        pub fn snapshot_json(&self) -> Result<String, JsValue> {
            serde_json::to_string(&serde_json::json!({
                "version": 1,
                "game": self.runtime.game.snapshot_json().map_err(error)?,
                "driver": self.runtime.driver_snapshot_json().map_err(error)?,
            }))
            .map_err(|e| error(e.into()))
        }
        pub fn restore_snapshot_json(&mut self, json: &str) -> Result<(), JsValue> {
            if json.len() > 4 * 1024 * 1024 {
                return Err(JsValue::from_str("snapshot too large"));
            }
            let saved: serde_json::Value =
                serde_json::from_str(json).map_err(|e| error(e.into()))?;
            if saved["version"] != 1 {
                return Err(JsValue::from_str("unsupported snapshot version"));
            }
            let game_json = saved["game"]
                .as_str()
                .ok_or_else(|| JsValue::from_str("missing game snapshot"))?;
            let driver_json = saved["driver"]
                .as_str()
                .ok_or_else(|| JsValue::from_str("missing driver snapshot"))?;
            self.runtime.restore_state_json(game_json, driver_json).map_err(error)?;
            self.runtime.game.render(&mut self.runtime.frame);
            Ok(())
        }
        pub fn display_aspect(&self) -> f64 {
            f64::from(config::DISPLAY_ASPECT.0) / f64::from(config::DISPLAY_ASPECT.1)
        }
        pub fn settings(&self) -> String { serde_json::to_string(&self.runtime.settings).expect("validated settings") }
        pub fn configure(&mut self, settings: &str) -> Result<(), JsValue> { self.runtime.configure(settings).map_err(error) }
        pub fn width(&self) -> u32 { self.runtime.frame.width }
        pub fn height(&self) -> u32 { self.runtime.frame.height }
        pub fn advance(&mut self, ms: f64, physical: &str) -> Result<u32, JsValue> {
            if physical.len() > 65536 { return Err(JsValue::from_str("input too large")) }
            let physical = serde_json::from_str(physical).map_err(|e| error(e.into()))?;
            self.runtime.advance(ms,physical).map_err(error)
        }
        pub fn boundary(&mut self, physical: &str) -> Result<(), JsValue> {
            if physical.len() > 65536 { return Err(JsValue::from_str("input too large")) }
            let physical = serde_json::from_str(physical).map_err(|e| error(e.into()))?;
            self.runtime.boundary(&physical); Ok(())
        }
        pub fn pixels(&self) -> Vec<u8> { self.runtime.frame.pixels.clone() }
        pub fn audio(&self) -> Vec<f32> { self.runtime.samples.clone() }
        pub fn save_version(&self) -> Option<u32> { self.runtime.game.save_version() }
        pub fn save_json(&self) -> Result<String, JsValue> { self.runtime.save_json().map_err(error) }
        pub fn restore(&mut self, json: &str) -> Result<(), JsValue> { self.runtime.restore(json).map_err(error) }
        pub fn acknowledge_save(&mut self) { self.runtime.game.save_acknowledged(); }
    }
}
