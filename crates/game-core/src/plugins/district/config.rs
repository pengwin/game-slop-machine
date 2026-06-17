use bevy::prelude::*;
use building_gen::district::config::TradeDistrictConfig;

/// Bevy resource wrapping the trade district generation configuration.
#[derive(Resource, Default)]
pub struct DistrictGenConfig(pub TradeDistrictConfig);
