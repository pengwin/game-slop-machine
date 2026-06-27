use bevy::prelude::*;

use crate::plugins::global_camera::SceneCameraSettings;

/// Returns the default gameplay camera.
#[must_use]
pub const fn settings() -> SceneCameraSettings {
    SceneCameraSettings {
        clear_color: Color::srgb(0.10, 0.11, 0.13),
        orthographic_viewport_height: 12.0,
        translation: Vec3::new(8.0, 8.0, 8.0),
        target: Vec3::ZERO,
    }
}
