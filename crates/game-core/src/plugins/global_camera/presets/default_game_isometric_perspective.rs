use bevy::prelude::*;

use crate::plugins::global_camera::{SceneCameraProjection, SceneCameraSettings};

/// Returns an orthographic-looking perspective gameplay camera.
#[must_use]
pub const fn settings() -> SceneCameraSettings {
    SceneCameraSettings {
        clear_color: Color::srgb(0.10, 0.11, 0.13),
        projection: SceneCameraProjection::Perspective { fov: 0.18 },
        translation: Vec3::new(38.0, 38.0, 38.0),
        target: Vec3::ZERO,
    }
}
