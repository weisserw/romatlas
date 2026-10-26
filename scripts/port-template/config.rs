//! Shared game identity and frontend configuration. Both backends use these values.
pub const ROM_SHA256: &str = "__ROM_SHA256__";
pub const DISPLAY_ASPECT: (u32,u32) = (4,3);
pub const SIZE: (u32,u32) = (160,144);
pub const RATE: romatlas_port::timing::TickRate = romatlas_port::timing::TickRate::DMG;
pub fn settings() -> romatlas_port::settings::Settings {
    let defaults = romatlas_port::settings::Settings::default();
    romatlas_port::settings::Settings {
        width: (defaults.height * DISPLAY_ASPECT.0 + DISPLAY_ASPECT.1 / 2) / DISPLAY_ASPECT.1,
        ..defaults
    }
}
