use bevy::prelude::*;

use crate::plugins::global_lighting::{SceneLightingSettings, SceneShadowCascadeSettings};

/// Returns default gameplay lighting.
#[must_use]
pub fn settings() -> SceneLightingSettings {
    SceneLightingSettings {
        ambient_color: Color::WHITE,
        ambient_brightness: 80.0,
        ambient_affects_lightmapped_meshes: true,
        sun_illuminance: 6_000.0,
        sun_rotation: Quat::from_euler(EulerRot::XYZ, -0.8, -0.5, 0.0),
        shadows_enabled: true,
        shadow_depth_bias: DirectionalLight::DEFAULT_SHADOW_DEPTH_BIAS,
        shadow_normal_bias: DirectionalLight::DEFAULT_SHADOW_NORMAL_BIAS,
        soft_shadow_size: Some(10.0),
        shadow_cascades: SceneShadowCascadeSettings {
            num_cascades: 4,
            minimum_distance: 0.1,
            first_cascade_far_bound: 10.0,
            maximum_distance: 150.0,
            overlap_proportion: 0.2,
        },
    }
}
