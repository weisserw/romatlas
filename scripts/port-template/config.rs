//! Shared game identity and frontend configuration. Both backends use these values.
pub const ROM_SHA256: &str = "__ROM_SHA256__";
pub const SIZE: (u32,u32) = (160,144);
pub const RATE: romatlas_port::timing::TickRate = romatlas_port::timing::TickRate::DMG;
pub fn settings() -> romatlas_port::settings::Settings {
    romatlas_port::settings::Settings::default()
}
