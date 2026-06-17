use bevy::prelude::*;
use super::scene::ScenePlugin;
use super::building::BuildingPlugin;
use super::district::DistrictPlugin;
use super::seed::{GenerationSeed, cycle_seed_on_command};

pub fn game_plugin(app: &mut App) {
    app.add_plugins(ScenePlugin);
    app.add_plugins(BuildingPlugin);
    app.add_plugins(DistrictPlugin);
    app.init_resource::<GenerationSeed>()
        .add_systems(Update, cycle_seed_on_command);
}
