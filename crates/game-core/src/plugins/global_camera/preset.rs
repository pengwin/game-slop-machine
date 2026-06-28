use bevy::prelude::*;

use super::{presets, SceneCameraSettings};

/// The currently requested global 3D camera preset.
#[derive(Resource, Copy, Clone, Default, Eq, PartialEq)]
pub enum CameraPreset {
    /// Default orthographic gameplay camera.
    #[default]
    DefaultGame,
    /// Default perspective gameplay camera.
    DefaultGamePerspective,
    /// Orthographic-looking perspective gameplay camera.
    DefaultGameIsometricPerspective,
}

impl CameraPreset {
    /// All built-in camera presets.
    pub const ALL: [Self; 3] = [
        Self::DefaultGame,
        Self::DefaultGamePerspective,
        Self::DefaultGameIsometricPerspective,
    ];

    /// Human-readable preset label.
    #[must_use]
    pub const fn label(&self) -> &'static str {
        match self {
            Self::DefaultGame => "Default game orthographic",
            Self::DefaultGamePerspective => "Default game perspective",
            Self::DefaultGameIsometricPerspective => "Default game isometric perspective",
        }
    }

    /// Resolves this preset into concrete camera values.
    #[must_use]
    pub const fn settings(&self) -> SceneCameraSettings {
        match self {
            Self::DefaultGame => presets::default_game::settings(),
            Self::DefaultGamePerspective => presets::default_game_perspective::settings(),
            Self::DefaultGameIsometricPerspective => {
                presets::default_game_isometric_perspective::settings()
            }
        }
    }
}
