use bevy::prelude::*;

/// Resolved camera values applied to the global 3D camera.
#[derive(Clone)]
pub struct SceneCameraSettings {
    /// Camera clear color.
    pub clear_color: Color,
    /// Orthographic viewport height in world units.
    pub orthographic_viewport_height: f32,
    /// Camera position.
    pub translation: Vec3,
    /// Camera look-at target.
    pub target: Vec3,
}
