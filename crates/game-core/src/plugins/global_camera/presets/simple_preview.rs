use bevy::prelude::*;

use crate::plugins::global_camera::SceneCameraSettings;

/// Returns the simple scene preview camera.
#[must_use]
pub const fn settings() -> SceneCameraSettings {
    SceneCameraSettings {
        clear_color: Color::srgb(0.12, 0.13, 0.15),
        orthographic_viewport_height: 10.0,
        translation: Vec3::new(6.0, 6.0, 6.0),
        target: Vec3::ZERO,
    }
}
