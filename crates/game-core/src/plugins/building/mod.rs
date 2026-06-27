pub mod config;
pub mod mesh_util;
pub mod procedural_texture;
pub mod render;
pub mod spawner;

use bevy::prelude::*;
use config::BuildingGenConfig;
use procedural_texture::{ProceduralTextures, update_procedural_textures};
use spawner::{spawn_building_on_command, spawn_texture_plaster_wall_on_command};

pub struct BuildingPlugin;

impl Plugin for BuildingPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<BuildingGenConfig>()
            .init_resource::<ProceduralTextures>()
            .add_systems(Update, update_procedural_textures)
            .add_systems(
                Update,
                (
                    spawn_building_on_command,
                    spawn_texture_plaster_wall_on_command,
                ),
            );
    }
}
