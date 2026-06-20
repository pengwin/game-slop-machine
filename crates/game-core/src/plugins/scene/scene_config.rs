use bevy::prelude::*;

#[derive(Resource)]
pub struct SceneConfig {
    pub ground_size: f32,
    pub ground_color: Color,
}

impl Default for SceneConfig {
    fn default() -> Self {
        Self {
            ground_size: 20.0,
            ground_color: Color::srgb(0.72, 0.72, 0.70),
        }
    }
}
