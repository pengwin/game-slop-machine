use bevy::prelude::*;

use crate::plugins::global_camera::{SceneCameraProjection, SceneCameraSettings};

/// Returns the default perspective gameplay camera.
#[must_use]
pub const fn settings() -> SceneCameraSettings {
    SceneCameraSettings {
        clear_color: Color::srgb(0.10, 0.11, 0.13),
        projection: SceneCameraProjection::Perspective {
            fov: std::f32::consts::FRAC_PI_4,
        },
        translation: Vec3::new(8.0, 8.0, 8.0),
        target: Vec3::ZERO,
    }
}
