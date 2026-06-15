pub mod config;
pub mod spawner;

use bevy::prelude::*;
use config::DistrictGenConfig;
use spawner::spawn_district_on_command;

pub struct DistrictPlugin;

impl Plugin for DistrictPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<DistrictGenConfig>()
            .add_systems(Update, spawn_district_on_command);
    }
}
