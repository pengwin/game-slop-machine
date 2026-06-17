use bevy::prelude::*;
use building_gen::config::BuildingConfig;

#[derive(Resource, Default)]
pub struct BuildingGenConfig(pub BuildingConfig);
