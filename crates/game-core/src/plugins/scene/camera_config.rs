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
            position: Vec3::new(15.0, 15.0, 15.0),
            target: Vec3::new(5.0, 0.0, 4.0),
            viewport_height: 15.0,
        }
    }
}
