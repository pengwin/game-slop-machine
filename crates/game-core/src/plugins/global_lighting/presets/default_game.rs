use crate::plugins::global_lighting::{GlobalLightControls, SceneLightingSettings};

/// Returns default gameplay lighting.
#[must_use]
pub fn settings() -> SceneLightingSettings {
    GlobalLightControls::default().to_settings()
}
