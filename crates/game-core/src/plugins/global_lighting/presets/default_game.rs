use bevy::prelude::*;

use crate::plugins::global_lighting::SceneLightingSettings;

/// Returns default gameplay lighting.
#[must_use]
pub fn scene_lighting() -> SceneLightingSettings {
    SceneLightingSettings {
        ambient_color: Color::WHITE,
        ambient_brightness: 80.0,
        ambient_affects_lightmapped_meshes: true,
        sun_illuminance: 6_000.0,
        sun_rotation: Quat::from_euler(EulerRot::XYZ, -0.8, -0.5, 0.0),
        shadows_enabled: true,
    }
}
