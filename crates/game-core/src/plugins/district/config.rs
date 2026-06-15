use bevy::prelude::*;
use building_gen::district::config::TradeDistrictConfig;

/// Bevy resource wrapping the trade district generation configuration.
#[derive(Resource)]
pub struct DistrictGenConfig(pub TradeDistrictConfig);

impl Default for DistrictGenConfig {
    fn default() -> Self {
        Self(TradeDistrictConfig::default())
    }
}
