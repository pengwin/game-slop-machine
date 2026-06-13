use bevy::prelude::*;

#[derive(Resource)]
pub struct CameraConfig {
    pub position: Vec3,
    pub target: Vec3,
    pub viewport_height: f32,
}

impl Default for CameraConfig {
    fn default() -> Self {
        Self {
            position: Vec3::new(10.0, 10.0, 10.0),
            target: Vec3::ZERO,
            viewport_height: 5.0,
        }
    }
}
