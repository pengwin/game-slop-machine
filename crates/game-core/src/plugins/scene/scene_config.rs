use bevy::prelude::*;

#[derive(Resource)]
pub struct SceneConfig {
    pub ground_size: f32,
    pub cube_size: f32,
}

impl Default for SceneConfig {
    fn default() -> Self {
        Self {
            ground_size: 20.0,
            cube_size: 1.0,
        }
    }
}
