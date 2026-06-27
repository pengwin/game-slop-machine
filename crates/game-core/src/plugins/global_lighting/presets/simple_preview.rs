use bevy::prelude::*;

use crate::plugins::global_lighting::SceneLightingSettings;

/// Returns bright inspector preview lighting.
#[must_use]
pub fn scene_lighting() -> SceneLightingSettings {
    SceneLightingSettings {
        ambient_color: Color::WHITE,
        ambient_brightness: 250.0,
        ambient_affects_lightmapped_meshes: true,
        sun_illuminance: 10_000.0,
        sun_rotation: Quat::from_euler(EulerRot::XYZ, -1.0, -0.7, 0.0),
        shadows_enabled: true,
    }
}
