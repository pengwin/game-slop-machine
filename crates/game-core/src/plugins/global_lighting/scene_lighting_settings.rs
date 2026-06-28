use bevy::prelude::*;

/// Directional light cascade shadow map bounds.
pub struct SceneShadowCascadeSettings {
    /// Number of shadow cascades.
    pub num_cascades: usize,
    /// Minimum distance from the camera receiving shadows.
    pub minimum_distance: f32,
    /// Far bound of the first cascade.
    pub first_cascade_far_bound: f32,
    /// Maximum distance from the camera receiving shadows.
    pub maximum_distance: f32,
    /// Proportion of overlap between shadow cascades.
    pub overlap_proportion: f32,
}

/// Resolved lighting values applied to Bevy lighting resources and entities.
pub struct SceneLightingSettings {
    /// Ambient light color.
    pub ambient_color: Color,
    /// Ambient light brightness.
    pub ambient_brightness: f32,
    /// Whether ambient light affects lightmapped meshes.
    pub ambient_affects_lightmapped_meshes: bool,
    /// Directional sun illuminance.
    pub sun_illuminance: f32,
    /// Directional sun rotation.
    pub sun_rotation: Quat,
    /// Whether the sun casts shadow maps.
    pub shadows_enabled: bool,
    /// Directional shadow depth bias.
    pub shadow_depth_bias: f32,
    /// Directional shadow normal bias.
    pub shadow_normal_bias: f32,
    /// Soft shadow radius for PCSS. `None` disables soft shadows.
    pub soft_shadow_size: Option<f32>,
    /// Directional shadow cascade bounds.
    pub shadow_cascades: SceneShadowCascadeSettings,
}
