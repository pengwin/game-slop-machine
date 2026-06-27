use bevy::prelude::*;

use super::{presets, SceneCameraSettings};

/// The currently requested global 3D camera preset.
#[derive(Resource, Clone, Copy, Default, Eq, PartialEq)]
pub enum CameraPreset {
    /// Default gameplay camera.
    #[default]
    DefaultGame,
    /// Orthographic preview camera for simple inspector scenes.
    SimplePreview,
}

impl CameraPreset {
    /// Resolves this preset into concrete camera values.
    #[must_use]
    pub const fn scene_camera(self) -> SceneCameraSettings {
        match self {
            Self::DefaultGame => presets::default_game::scene_camera(),
            Self::SimplePreview => presets::simple_preview::scene_camera(),
        }
    }
}
