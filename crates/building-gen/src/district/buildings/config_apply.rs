use super::footprint::best_footprint_for_lot;
use crate::config::BuildingConfig;
use crate::district::config::{BuildingDescription, TradeDistrictConfig};
use crate::district::layout::Lot;
use crate::geometry::{Rect, Vec2};

pub fn building_config_for_lot(
    lot: &Lot,
    description: &BuildingDescription,
    district_config: &TradeDistrictConfig,
) -> BuildingConfig {
    let defaults = BuildingConfig::default();
    let min_size = defaults.min_room_size + defaults.tile_size;
    let inset = district_config
        .building_lot_inset
        .min(lot.width * 0.35)
        .min(lot.depth * 0.35)
        .max(0.0);
    let available_width = (lot.width - inset * 2.0).max(min_size);
    let available_depth = (lot.depth - inset).max(min_size);
    let (width, depth) = best_footprint_for_lot(
        description,
        available_width,
        available_depth,
        min_size,
        defaults.tile_size,
    );

    let mut config = BuildingConfig {
        footprint: Rect::new(0.0, 0.0, width, depth),
        entrance: Vec2::new(width / 2.0, 0.0),
        entrance_dir: Vec2::new(0.0, 1.0),
        room_specs: description.rooms.clone(),
        auto_corridor: description.auto_corridor,
        corridor_width_tiles: description.corridor_width_tiles,
        render_roof: description.render_roof,
        ..Default::default()
    };
    apply_building_overrides(&mut config, description);
    config
}

fn apply_building_overrides(config: &mut BuildingConfig, description: &BuildingDescription) {
    let overrides = &description.config_overrides;

    if let Some(value) = overrides.wall_height {
        config.wall_height = value;
    }
    if let Some(value) = overrides.roof_height {
        config.roof_height = value;
    }
    if let Some(value) = overrides.roof_overhang {
        config.roof_overhang = value;
    }
    if let Some(value) = overrides.foundation_width {
        config.foundation_width = value;
    }
    if let Some(value) = overrides.opening_trim_thickness {
        config.opening_trim_thickness = value;
    }
    if let Some(value) = overrides.opening_trim_depth {
        config.opening_trim_depth = value;
    }
    if let Some(value) = overrides.window_width {
        config.window_width = value;
    }
    if let Some(value) = overrides.window_height {
        config.window_height = value;
    }
    if let Some(value) = overrides.window_spacing {
        config.window_spacing = value;
    }
    if let Some(value) = overrides.visual_style.clone() {
        config.visual_style = value;
    }
}
