pub mod camera;
pub mod camera_config;
mod light;
mod objects;
pub mod scene_config;

use bevy::prelude::*;

use camera::spawn_camera;
use camera_config::CameraConfig;
use light::spawn_light;
use objects::spawn_objects;
use scene_config::SceneConfig;

pub struct ScenePlugin;

impl Plugin for ScenePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CameraConfig>()
            .init_resource::<SceneConfig>()
            .add_systems(Startup, (spawn_camera, spawn_light, spawn_objects));
    }
}
