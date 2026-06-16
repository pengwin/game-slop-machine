pub mod config;
pub mod mesh_util;
pub mod render;
pub mod spawner;

use bevy::prelude::*;
use config::BuildingGenConfig;
use spawner::spawn_building_on_command;

pub struct BuildingPlugin;

impl Plugin for BuildingPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<BuildingGenConfig>()
            .add_systems(Update, spawn_building_on_command);
    }
}
