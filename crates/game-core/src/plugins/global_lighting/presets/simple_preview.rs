use bevy::prelude::*;

use crate::plugins::global_lighting::{SceneLightingSettings, SceneShadowCascadeSettings};

/// Returns bright inspector preview lighting.
#[must_use]
pub fn settings() -> SceneLightingSettings {
    SceneLightingSettings {
        ambient_color: Color::WHITE,
        ambient_brightness: 250.0,
        ambient_affects_lightmapped_meshes: true,
        sun_illuminance: 10_000.0,
        sun_rotation: Quat::from_euler(EulerRot::XYZ, -1.0, -0.7, 0.0),
        shadows_enabled: true,
        shadow_depth_bias: DirectionalLight::DEFAULT_SHADOW_DEPTH_BIAS,
        shadow_normal_bias: DirectionalLight::DEFAULT_SHADOW_NORMAL_BIAS,
        soft_shadow_size: Some(10.0),
        shadow_cascades: SceneShadowCascadeSettings {
            num_cascades: 1,
            minimum_distance: 0.1,
            first_cascade_far_bound: 60.0,
            maximum_distance: 80.0,
            overlap_proportion: 0.0,
        },
    }
}
