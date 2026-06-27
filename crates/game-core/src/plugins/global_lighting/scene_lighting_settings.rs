use bevy::prelude::*;

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
}
