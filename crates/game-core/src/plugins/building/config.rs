use bevy::prelude::*;
use building_gen::config::BuildingConfig;

#[derive(Resource)]
pub struct BuildingGenConfig(pub BuildingConfig);

impl Default for BuildingGenConfig {
    fn default() -> Self {
        Self(BuildingConfig::default())
    }
}
