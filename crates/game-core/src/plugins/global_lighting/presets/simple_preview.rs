use crate::plugins::global_lighting::{GlobalLightControls, SceneLightingSettings};

/// Returns bright inspector preview lighting.
#[must_use]
pub fn settings() -> SceneLightingSettings {
    GlobalLightControls::default().to_settings()
}
