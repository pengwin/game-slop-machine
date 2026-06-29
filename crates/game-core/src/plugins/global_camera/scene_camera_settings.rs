use bevy::prelude::*;

/// Projection mode used by a resolved camera preset.
#[derive(Clone)]
pub enum SceneCameraProjection {
    /// Orthographic projection with a fixed vertical viewport height.
    Orthographic {
        /// Vertical viewport height in world units.
        viewport_height: f32,
    },
    /// Perspective projection with a vertical field of view in radians.
    Perspective {
        /// Vertical field of view in radians.
        fov: f32,
    },
}

/// Resolved camera values applied to the global 3D camera.
pub struct SceneCameraSettings {
    /// Camera clear color.
    pub clear_color: Color,
    /// Camera projection.
    pub projection: SceneCameraProjection,
    /// Camera position.
    pub translation: Vec3,
    /// Camera look-at target.
    pub target: Vec3,
}
