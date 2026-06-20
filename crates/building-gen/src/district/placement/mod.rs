mod blocks;
mod collision;
mod connector;
mod lot_split;

use super::config::TradeDistrictConfig;
use super::layout::{Lot, RoadSegment};
use crate::geometry::Vec2;
use crate::random::SeededRng;

pub use blocks::bounding_radius;
pub use collision::{lot_overlaps_roads, lots_overlap};
pub use connector::connector_road_for_lot;
pub use lot_split::split_lots_for_buildings;

pub fn place_lots(
    config: &TradeDistrictConfig,
    rng: &mut SeededRng,
    roads: &[RoadSegment],
) -> Vec<Lot> {
    let center = Vec2::ZERO;
    let block_list = blocks::compute_blocks(config);
    let mut lots: Vec<Lot> = Vec::new();

    let mut remaining = config.lot_count;
    let max_per_block = 1;
    let mut placed_per_block: Vec<usize> = vec![0; block_list.len()];

    for _pass in 0..max_per_block {
        for (block_idx, block) in block_list.iter().enumerate() {
            if remaining == 0 {
                break;
            }
            if placed_per_block[block_idx] >= max_per_block {
                continue;
            }

            for _attempt in 0..30 {
                let Some(lot) = blocks::try_place_lot_in_block(block, config, rng, center) else {
                    break;
                };

                let mut fitted_lot = None;
                for scale in [1.0, 0.92, 0.84, 0.76, 0.68, 0.6, 0.52, 0.44, 0.36] {
                    let candidate = lot_split::scaled_lot(&lot, scale);
                    let overlaps_roads =
                        collision::lot_overlaps_roads(&candidate, roads, config.road_width);
                    let overlaps_lots = lots
                        .iter()
                        .any(|existing| collision::lots_overlap(&candidate, existing, 0.0));
                    if !overlaps_roads && !overlaps_lots {
                        fitted_lot = Some(candidate);
                        break;
                    }
                }

                let Some(lot) = fitted_lot else {
                    continue;
                };

                lots.push(lot);
                placed_per_block[block_idx] += 1;
                remaining -= 1;
                break;
            }
        }
        if remaining == 0 {
            break;
        }
    }

    lots
}

#[cfg(test)]
pub fn road_crosses_lot_interior(road: &RoadSegment, lot: &Lot) -> bool {
    collision::road_crosses_lot_interior(road, lot)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_roads() -> Vec<RoadSegment> {
        super::super::road::generate_road_network(&TradeDistrictConfig::default())
    }

    #[test]
    fn test_place_lots_count() {
        let config = TradeDistrictConfig::default();
        let roads = test_roads();
        let mut rng = SeededRng::new(config.seed);
        let lots = place_lots(&config, &mut rng, &roads);
        assert_eq!(lots.len(), config.lot_count);
    }

    #[test]
    fn test_place_lots_no_overlap() {
        let config = TradeDistrictConfig::default();
        let roads = test_roads();
        let mut rng = SeededRng::new(config.seed);
        let lots = place_lots(&config, &mut rng, &roads);

        for i in 0..lots.len() {
            for j in (i + 1)..lots.len() {
                assert!(
                    !lots_overlap(&lots[i], &lots[j], 0.0),
                    "Lots {} and {} overlap",
                    i,
                    j,
                );
            }
        }
    }

    #[test]
    fn test_lot_width_one_still_places_requested_lots() {
        let config = TradeDistrictConfig {
            lot_width: 1.0,
            lot_width_randomness: 0.0,
            lot_height_randomness: 0.0,
            lot_depth_randomness: 0.0,
            ..Default::default()
        };
        let roads = super::super::road::generate_road_network(&config);
        let mut rng = SeededRng::new(config.seed);
        let lots = place_lots(&config, &mut rng, &roads);

        assert_eq!(lots.len(), config.lot_count);
    }

    #[test]
    fn test_wide_lot_splits_into_building_lots_with_own_entrances() {
        let lot = Lot {
            position: Vec2::new(10.0, 4.0),
            width: 24.0,
            depth: 9.0,
            rotation: 0.0,
            entrance: Vec2::new(10.0, -0.5),
            entrance_dir: Vec2::new(0.0, 1.0),
        };
        let config = TradeDistrictConfig {
            building_lot_inset: 1.0,
            max_buildings_per_lot: 2,
            building_gap: 1.0,
            preserve_large_lot_area: f32::MAX,
            building_lot_split_chance: 1.0,
            one_building_lot_weight: 0.0,
            two_building_lot_weight: 1.0,
            three_building_lot_weight: 0.0,
            building_lot_split_jitter: 0.3,
            ..Default::default()
        };
        let lots = split_lots_for_buildings(&[lot.clone()], &config);

        assert_eq!(lots.len(), 2);
        assert!(lots[0].position.distance_to(lots[1].position) > config.building_gap);
        assert!(lots[0].entrance.distance_to(lots[1].entrance) > config.building_gap);
        assert!((lots[0].width - lots[1].width).abs() > 0.01);
        assert_eq!(lots[0].entrance_dir, Vec2::new(0.0, 1.0));
        assert_eq!(lots[1].entrance_dir, Vec2::new(0.0, 1.0));
    }

    #[test]
    fn test_wide_lot_can_split_into_three_small_building_lots() {
        let lot = Lot {
            position: Vec2::new(10.0, 4.0),
            width: 24.0,
            depth: 9.0,
            rotation: 0.0,
            entrance: Vec2::new(10.0, -0.5),
            entrance_dir: Vec2::new(0.0, 1.0),
        };
        let config = TradeDistrictConfig {
            building_lot_inset: 1.0,
            max_buildings_per_lot: 3,
            building_gap: 1.0,
            preserve_large_lot_area: f32::MAX,
            building_lot_split_chance: 1.0,
            one_building_lot_weight: 0.0,
            two_building_lot_weight: 0.0,
            three_building_lot_weight: 1.0,
            building_lot_split_jitter: 0.3,
            ..Default::default()
        };
        let lots = split_lots_for_buildings(&[lot.clone()], &config);

        assert_eq!(lots.len(), 3);
        assert!(
            lots.windows(2)
                .all(|pair| pair[0].position.distance_to(pair[1].position) > config.building_gap)
        );
        assert!(lots.iter().all(|lot| lot.width >= 3.0));
        assert!(
            lots.iter()
                .all(|lot| lot.entrance_dir == Vec2::new(0.0, 1.0))
        );
    }

    #[test]
    fn test_large_lot_can_be_preserved_for_landmark_building() {
        let lot = Lot {
            position: Vec2::new(10.0, 4.0),
            width: 24.0,
            depth: 9.0,
            rotation: 0.0,
            entrance: Vec2::new(10.0, -0.5),
            entrance_dir: Vec2::new(0.0, 1.0),
        };
        let config = TradeDistrictConfig {
            max_buildings_per_lot: 2,
            preserve_large_lot_area: 100.0,
            building_lot_split_chance: 1.0,
            one_building_lot_weight: 1.0,
            two_building_lot_weight: 0.0,
            three_building_lot_weight: 0.0,
            landmark_lot_chance: 1.0,
            ..Default::default()
        };
        let lots = split_lots_for_buildings(&[lot], &config);

        assert_eq!(lots.len(), 1);
        assert_eq!(lots[0].width, 24.0);
        assert_eq!(lots[0].depth, 9.0);
    }

    #[test]
    fn test_large_lot_can_hold_one_smaller_standalone_building() {
        let lot = Lot {
            position: Vec2::new(10.0, 4.0),
            width: 24.0,
            depth: 9.0,
            rotation: 0.0,
            entrance: Vec2::new(10.0, -0.5),
            entrance_dir: Vec2::new(0.0, 1.0),
        };
        let original_width = lot.width;
        let original_depth = lot.depth;
        let original_entrance = lot.entrance;
        let config = TradeDistrictConfig {
            max_buildings_per_lot: 3,
            preserve_large_lot_area: 100.0,
            building_lot_split_chance: 1.0,
            one_building_lot_weight: 1.0,
            two_building_lot_weight: 0.0,
            three_building_lot_weight: 0.0,
            landmark_lot_chance: 0.0,
            standalone_lot_width_scale: 0.5,
            standalone_lot_depth_scale: 0.75,
            ..Default::default()
        };
        let lots = split_lots_for_buildings(&[lot], &config);

        assert_eq!(lots.len(), 1);
        assert!(lots[0].width < original_width);
        assert!(lots[0].depth < original_depth);
        assert_eq!(lots[0].entrance, original_entrance);
    }

    #[test]
    fn test_max_one_building_per_lot_disables_lot_splitting() {
        let lot = Lot {
            position: Vec2::new(10.0, 4.0),
            width: 24.0,
            depth: 9.0,
            rotation: 0.0,
            entrance: Vec2::new(10.0, -0.5),
            entrance_dir: Vec2::new(0.0, 1.0),
        };
        let config = TradeDistrictConfig {
            max_buildings_per_lot: 1,
            ..Default::default()
        };
        let lots = split_lots_for_buildings(&[lot], &config);

        assert_eq!(lots.len(), 1);
    }

    #[test]
    fn test_lots_dont_overlap_roads() {
        let config = TradeDistrictConfig::default();
        let roads = test_roads();
        let mut rng = SeededRng::new(config.seed);
        let lots = place_lots(&config, &mut rng, &roads);

        for (i, lot) in lots.iter().enumerate() {
            assert!(
                !lot_overlaps_roads(lot, &roads, config.road_width),
                "Lot {} overlaps a road",
                i,
            );
        }
    }

    #[test]
    fn test_lot_wide_side_is_parallel_to_ring_edge() {
        let config = TradeDistrictConfig::default();
        let roads = test_roads();
        let block_list = blocks::compute_blocks(&config);
        let mut rng = SeededRng::new(config.seed);
        let lots = place_lots(&config, &mut rng, &roads);

        for lot in lots {
            let lot_angle = lot.position.y.atan2(lot.position.x);
            let block = block_list
                .iter()
                .min_by(|a, b| {
                    angular_distance(a.mid_angle, lot_angle)
                        .partial_cmp(&angular_distance(b.mid_angle, lot_angle))
                        .unwrap()
                })
                .unwrap();
            let width_angle = lot.rotation + std::f32::consts::FRAC_PI_2;
            let tangent_angle = block.mid_angle + std::f32::consts::FRAC_PI_2;
            assert!(
                angular_distance(width_angle, tangent_angle) < 0.01,
                "Lot wide side should be parallel to its ring road edge"
            );
        }
    }

    #[test]
    fn test_lot_width_controls_cross_segment_size() {
        let mut narrow_config = TradeDistrictConfig::default();
        narrow_config.lot_width_randomness = 0.0;
        narrow_config.lot_height_randomness = 0.0;
        narrow_config.lot_depth_randomness = 0.0;
        narrow_config.lot_width = 0.35;
        let mut narrow_rng = SeededRng::new(narrow_config.seed);

        let mut wide_config = TradeDistrictConfig::default();
        wide_config.lot_width_randomness = 0.0;
        wide_config.lot_height_randomness = 0.0;
        wide_config.lot_depth_randomness = 0.0;
        wide_config.lot_width = 1.0;
        let mut wide_rng = SeededRng::new(wide_config.seed);

        let block = &blocks::compute_blocks(&narrow_config)[0];
        let narrow_lot =
            blocks::try_place_lot_in_block(block, &narrow_config, &mut narrow_rng, Vec2::ZERO)
                .expect("narrow lot should fit");
        let wide_lot =
            blocks::try_place_lot_in_block(block, &wide_config, &mut wide_rng, Vec2::ZERO)
                .expect("wide lot should fit");

        assert!(
            narrow_lot.width < wide_lot.width,
            "Lower lot_width should create narrower lots"
        );
    }

    #[test]
    fn test_lot_height_controls_radial_size() {
        let mut shallow_config = TradeDistrictConfig::default();
        shallow_config.lot_width_randomness = 0.0;
        shallow_config.lot_height_randomness = 0.0;
        shallow_config.lot_depth_randomness = 0.0;
        shallow_config.lot_height = 0.3;
        let mut shallow_rng = SeededRng::new(shallow_config.seed);

        let mut deep_config = TradeDistrictConfig::default();
        deep_config.lot_width_randomness = 0.0;
        deep_config.lot_height_randomness = 0.0;
        deep_config.lot_depth_randomness = 0.0;
        deep_config.lot_height = 0.8;
        let mut deep_rng = SeededRng::new(deep_config.seed);

        let block = &blocks::compute_blocks(&shallow_config)[0];
        let shallow_lot =
            blocks::try_place_lot_in_block(block, &shallow_config, &mut shallow_rng, Vec2::ZERO)
                .expect("shallow lot should fit");
        let deep_lot =
            blocks::try_place_lot_in_block(block, &deep_config, &mut deep_rng, Vec2::ZERO)
                .expect("deep lot should fit");

        assert!(
            shallow_lot.depth < deep_lot.depth,
            "Lower lot_height should create shorter lots"
        );
    }

    #[test]
    fn test_lot_depth_controls_entrance_setback() {
        let mut close_config = TradeDistrictConfig::default();
        close_config.lot_width_randomness = 0.0;
        close_config.lot_height_randomness = 0.0;
        close_config.lot_depth_randomness = 0.0;
        close_config.lot_depth = 0.0;
        let mut close_rng = SeededRng::new(close_config.seed);

        let mut far_config = TradeDistrictConfig::default();
        far_config.lot_width_randomness = 0.0;
        far_config.lot_height_randomness = 0.0;
        far_config.lot_depth_randomness = 0.0;
        far_config.lot_depth = 1.0;
        let mut far_rng = SeededRng::new(far_config.seed);

        let block = &blocks::compute_blocks(&close_config)[0];
        let close_lot =
            blocks::try_place_lot_in_block(block, &close_config, &mut close_rng, Vec2::ZERO)
                .expect("close lot should fit");
        let far_lot = blocks::try_place_lot_in_block(block, &far_config, &mut far_rng, Vec2::ZERO)
            .expect("far lot should fit");

        assert!(
            close_lot.entrance.length() < far_lot.entrance.length(),
            "Lower lot_depth should place the entrance closer to the center-facing road"
        );
    }

    #[test]
    fn test_randomized_fill_adds_positive_clamped_offset() {
        let mut rng = SeededRng::new(7);
        for _ in 0..16 {
            let value = blocks::randomized_fill(0.4, 0.1, &mut rng);
            assert!((0.4..=0.5).contains(&value));
        }

        let mut rng = SeededRng::new(7);
        for _ in 0..16 {
            let value = blocks::randomized_fill(0.95, 0.2, &mut rng);
            assert!((0.95..=1.0).contains(&value));
        }
    }

    #[test]
    fn test_lot_randomness_changes_generated_dimensions() {
        let mut fixed_config = TradeDistrictConfig::default();
        fixed_config.lot_width = 0.5;
        fixed_config.lot_height = 0.3;
        fixed_config.lot_depth = 0.0;
        fixed_config.lot_width_randomness = 0.0;
        fixed_config.lot_height_randomness = 0.0;
        fixed_config.lot_depth_randomness = 0.0;
        let fixed_roads = super::super::road::generate_road_network(&fixed_config);
        let mut fixed_rng = SeededRng::new(fixed_config.seed);
        let fixed_lots = place_lots(&fixed_config, &mut fixed_rng, &fixed_roads);

        let mut varied_config = fixed_config.clone();
        varied_config.lot_width_randomness = 0.1;
        varied_config.lot_height_randomness = 0.1;
        varied_config.lot_depth_randomness = 0.1;
        let varied_roads = super::super::road::generate_road_network(&varied_config);
        let mut varied_rng = SeededRng::new(varied_config.seed);
        let varied_lots = place_lots(&varied_config, &mut varied_rng, &varied_roads);

        assert_eq!(fixed_lots.len(), fixed_config.lot_count);
        assert_eq!(varied_lots.len(), varied_config.lot_count);
        assert!(
            fixed_lots
                .iter()
                .zip(varied_lots.iter())
                .any(|(fixed, varied)| {
                    (fixed.width - varied.width).abs() > 0.01
                        || (fixed.depth - varied.depth).abs() > 0.01
                        || fixed.entrance.distance_to(varied.entrance) > 0.01
                }),
            "Nonzero lot randomness should change at least one generated lot"
        );
    }

    #[test]
    fn test_entrances_face_center() {
        let config = TradeDistrictConfig::default();
        let roads = test_roads();
        let mut rng = SeededRng::new(config.seed);
        let lots = place_lots(&config, &mut rng, &roads);

        for (i, lot) in lots.iter().enumerate() {
            let to_interior = Vec2::new(
                lot.position.x - lot.entrance.x,
                lot.position.y - lot.entrance.y,
            );
            let dot = lot.entrance_dir.x * to_interior.x + lot.entrance_dir.y * to_interior.y;
            assert!(
                dot >= -0.01,
                "Lot {} entrance_dir doesn't point into lot (dot={:.2})",
                i,
                dot,
            );
        }
    }

    #[test]
    fn test_place_lots_deterministic() {
        let config = TradeDistrictConfig::default();
        let roads = test_roads();
        let mut rng1 = SeededRng::new(config.seed);
        let mut rng2 = SeededRng::new(config.seed);
        let a = place_lots(&config, &mut rng1, &roads);
        let b = place_lots(&config, &mut rng2, &roads);

        assert_eq!(a.len(), b.len());
        for (la, lb) in a.iter().zip(b.iter()) {
            assert!((la.position.x - lb.position.x).abs() < 0.01);
            assert!((la.position.y - lb.position.y).abs() < 0.01);
        }
    }

    fn angular_distance(a: f32, b: f32) -> f32 {
        let mut diff = (a - b).abs() % std::f32::consts::PI;
        if diff > std::f32::consts::FRAC_PI_2 {
            diff = std::f32::consts::PI - diff;
        }
        diff
    }
}
