pub mod config;
pub mod mesh_util;
pub mod render;
pub mod procedural_texture;
pub mod spawner;

use bevy::prelude::*;
use config::BuildingGenConfig;
use spawner::spawn_building_on_command;

pub struct BuildingPlugin;

impl Plugin for BuildingPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<BuildingGenConfig>()
            .add_systems(Startup, setup_procedural_textures)
            .add_systems(Update, spawn_building_on_command);
    }
}

fn setup_procedural_textures(mut commands: Commands, mut images: ResMut<Assets<Image>>) {
    let textures = procedural_texture::generate_textures(&mut images);
    commands.insert_resource(textures);
}
