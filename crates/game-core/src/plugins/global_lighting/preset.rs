use bevy::prelude::*;

use super::{presets, SceneLightingSettings};

/// The currently requested lighting preset.
#[derive(Resource, Clone, Default, Eq, PartialEq)]
pub enum LightingPreset {
    /// Default gameplay lighting.
    #[default]
    DefaultGame,
    /// Bright preview lighting for simple inspector scenes.
    SimplePreview,
}

impl LightingPreset {
    /// Resolves this preset into concrete lighting values.
    #[must_use]
    pub fn settings(&self) -> SceneLightingSettings {
        match self {
            Self::DefaultGame => presets::default_game::settings(),
            Self::SimplePreview => presets::simple_preview::settings(),
        }
    }
}
