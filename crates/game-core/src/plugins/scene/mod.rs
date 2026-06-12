mod camera;
mod light;
mod objects;
mod scene_config;

use bevy::prelude::*;

use camera::spawn_camera;
use light::spawn_light;
use objects::spawn_objects;
use scene_config::SceneConfig;

pub struct ScenePlugin;

impl Plugin for ScenePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SceneConfig>()
            .add_systems(Startup, (spawn_camera, spawn_light, spawn_objects));
    }
}
