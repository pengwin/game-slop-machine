pub mod components;
pub mod resources;
mod systems;

use bevy::prelude::*;
use resources::SceneConfig;
use systems::setup_scene;

pub struct ScenePlugin;

impl Plugin for ScenePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SceneConfig>()
           .add_systems(Startup, setup_scene);
    }
}
