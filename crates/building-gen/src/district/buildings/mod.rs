mod config_apply;
mod footprint;
mod selection;

use super::config::TradeDistrictConfig;
use super::layout::{DistrictBuilding, Lot};
use crate::generate_layout;

pub use selection::select_building_description;

pub fn generate_buildings_for_lots(
    lots: &[Lot],
    config: &TradeDistrictConfig,
) -> Vec<DistrictBuilding> {
    if !config.generate_buildings || config.building_descriptions.is_empty() {
        return Vec::new();
    }

    lots.iter()
        .enumerate()
        .map(|(lot_index, lot)| building_for_lot(lot_index, lot, config))
        .collect()
}

fn building_for_lot(
    lot_index: usize,
    lot: &Lot,
    district_config: &TradeDistrictConfig,
) -> DistrictBuilding {
    let description = selection::select_building_description_for_lot(
        lot,
        &district_config.building_descriptions,
        district_config.seed + lot_index as u64,
    );
    let config = config_apply::building_config_for_lot(lot, description, district_config);
    let layout = generate_layout(&config);
    let world_position = footprint::building_origin_for_lot(lot, &config);
    let rotation = footprint::building_rotation_for_lot(lot);

    DistrictBuilding {
        lot_index,
        description_name: description.name.clone(),
        config,
        layout,
        world_position,
        rotation,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::district::config::default_building_descriptions;
    use crate::geometry::Vec2;

    fn test_lot(width: f32, depth: f32) -> Lot {
        Lot {
            position: Vec2::new(10.0, 4.0),
            width,
            depth,
            rotation: 0.0,
            entrance: Vec2::new(10.0, 0.0),
            entrance_dir: Vec2::new(0.0, 1.0),
        }
    }

    #[test]
    fn test_description_selection_by_size() {
        let descriptions = default_building_descriptions();

        assert_eq!(
            select_building_description(&test_lot(7.0, 6.0), &descriptions).name,
            "small shop/home"
        );
        assert_eq!(
            select_building_description(&test_lot(11.0, 8.0), &descriptions).name,
            "medium hall house"
        );
        assert_eq!(
            select_building_description(&test_lot(16.0, 10.0), &descriptions).name,
            "large inn"
        );
        assert_eq!(
            select_building_description(&test_lot(24.0, 12.0), &descriptions).name,
            "great warehouse"
        );
    }

    #[test]
    fn test_seeded_description_selection_varies_room_program_for_same_lot_size() {
        let descriptions = default_building_descriptions();
        let lot = test_lot(13.0, 8.0);
        let mut selected = Vec::new();

        for seed in 0..16 {
            selected.push(
                selection::select_building_description_for_lot(&lot, &descriptions, seed)
                    .name
                    .clone(),
            );
        }
        selected.sort();
        selected.dedup();

        assert!(
            selected.len() > 1,
            "same-sized lots should have more than one viable room program"
        );
    }

    #[test]
    fn test_building_description_overrides_config() {
        let lot = test_lot(16.0, 10.0);
        let district_config = TradeDistrictConfig::default();
        let descriptions = default_building_descriptions();
        let large = descriptions
            .iter()
            .find(|description| description.name == "large inn")
            .unwrap();
        let config = config_apply::building_config_for_lot(&lot, large, &district_config);

        assert_eq!(config.wall_height, 3.4);
        assert_eq!(config.roof_height, 2.3);
        assert_eq!(config.foundation_width, 0.28);
        assert_eq!(config.window_spacing, 1.7);
    }

    #[test]
    fn test_building_config_fits_inside_lot_after_inset() {
        let lot = test_lot(12.0, 8.0);
        let district_config = TradeDistrictConfig {
            building_lot_inset: 1.0,
            ..Default::default()
        };
        let description = select_building_description(&lot, &district_config.building_descriptions);
        let config = config_apply::building_config_for_lot(&lot, description, &district_config);

        assert!(config.footprint.width() <= lot.width - 2.0);
        assert!(config.footprint.height() <= lot.depth - 1.0);
    }

    #[test]
    fn test_building_footprint_depends_on_room_program() {
        let lot = test_lot(20.0, 12.0);
        let district_config = TradeDistrictConfig {
            building_lot_inset: 1.0,
            ..Default::default()
        };
        let descriptions = default_building_descriptions();
        let small = config_apply::building_config_for_lot(&lot, &descriptions[0], &district_config);
        let large = config_apply::building_config_for_lot(&lot, &descriptions[2], &district_config);

        assert!(
            large.footprint.width() * large.footprint.height()
                > small.footprint.width() * small.footprint.height()
        );
        assert!(small.footprint.width() < lot.width - 2.0);
        assert!(small.footprint.height() < lot.depth - 1.0);
    }

    #[test]
    fn test_building_footprint_uses_best_fit_for_long_lot() {
        let lot = test_lot(24.0, 7.0);
        let district_config = TradeDistrictConfig {
            building_lot_inset: 1.0,
            ..Default::default()
        };
        let descriptions = default_building_descriptions();
        let config =
            config_apply::building_config_for_lot(&lot, &descriptions[1], &district_config);

        assert!(config.footprint.width() <= lot.width - 2.0);
        assert!(config.footprint.height() <= lot.depth - 1.0);
        assert!(config.footprint.width() < lot.width - 2.0);
    }

    #[test]
    fn test_building_entrance_is_front_local_edge() {
        let lot = test_lot(12.0, 8.0);
        let district_config = TradeDistrictConfig::default();
        let description = select_building_description(&lot, &district_config.building_descriptions);
        let config = config_apply::building_config_for_lot(&lot, description, &district_config);

        assert!((config.entrance.x - config.footprint.width() / 2.0).abs() < 0.01);
        assert!((config.entrance.y - 0.0).abs() < 0.01);
        assert_eq!(config.entrance_dir, Vec2::new(0.0, 1.0));
    }

    #[test]
    fn test_building_origin_aligns_local_entrance_to_lot_entrance() {
        let lot = test_lot(12.0, 8.0);
        let district_config = TradeDistrictConfig {
            building_lot_inset: 1.0,
            ..Default::default()
        };
        let description = select_building_description(&lot, &district_config.building_descriptions);
        let config = config_apply::building_config_for_lot(&lot, description, &district_config);
        let origin = footprint::building_origin_for_lot(&lot, &config);
        let right = Vec2::new(lot.entrance_dir.y, -lot.entrance_dir.x);
        let mapped_entrance = origin + right * config.entrance.x;
        let expected = lot.entrance;

        assert!(mapped_entrance.distance_to(expected) < 0.01);
    }
}
