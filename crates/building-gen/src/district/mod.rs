pub mod config;
pub mod layout;
pub mod placement;
pub mod road;

use crate::random::SeededRng;
use config::TradeDistrictConfig;
use layout::{RoadSegment, TradeDistrictLayout};

use crate::geometry::{Rect, Vec2};

/// Generates a complete trade district with lots and roads.
///
/// 1. Builds the road skeleton (radials + rings)
/// 2. Places lots in blocks between roads
/// 3. Generates connector roads from lot entrances to the nearest road
pub fn generate_district(config: &TradeDistrictConfig) -> TradeDistrictLayout {
    let mut rng = SeededRng::new(config.seed);

    // Step 1: Road skeleton (hex road + radials + rings)
    let mut roads = road::generate_road_network(config);
    let base_roads = roads.clone();

    // Step 2: Place lots in blocks, avoiding overlap with the road skeleton
    let lots = placement::place_lots(config, &mut rng, &base_roads);

    // Step 3: Connector roads from lot entrances to nearest base road
    for lot in &lots {
        if let Some(connector) =
            placement::connector_road_for_lot(lot, &base_roads, config.road_width)
        {
            roads.push(connector);
        }
    }

    // Step 4: Compute overall bounds
    let bounds = compute_bounds(config, &lots, &roads);

    TradeDistrictLayout {
        lots,
        roads,
        town_square_center: Vec2::ZERO,
        bounds,
    }
}

/// Computes the overall bounding rectangle of the district.
fn compute_bounds(
    config: &TradeDistrictConfig,
    lots: &[layout::Lot],
    roads: &[RoadSegment],
) -> Rect {
    let mut min_x = f32::MAX;
    let mut max_x = f32::MIN;
    let mut min_y = f32::MAX;
    let mut max_y = f32::MIN;

    let expand =
        |min_x: &mut f32, max_x: &mut f32, min_y: &mut f32, max_y: &mut f32, x: f32, y: f32| {
            *min_x = (*min_x).min(x);
            *max_x = (*max_x).max(x);
            *min_y = (*min_y).min(y);
            *max_y = (*max_y).max(y);
        };

    // Town square
    let sq = config.town_square_radius + config.road_width;
    expand(&mut min_x, &mut max_x, &mut min_y, &mut max_y, -sq, -sq);
    expand(&mut min_x, &mut max_x, &mut min_y, &mut max_y, sq, sq);

    // Lots
    for lot in lots {
        let margin = ((lot.width * lot.width + lot.depth * lot.depth).sqrt()) / 2.0 + 2.0;
        expand(
            &mut min_x,
            &mut max_x,
            &mut min_y,
            &mut max_y,
            lot.position.x - margin,
            lot.position.y - margin,
        );
        expand(
            &mut min_x,
            &mut max_x,
            &mut min_y,
            &mut max_y,
            lot.position.x + margin,
            lot.position.y + margin,
        );
    }

    // Roads
    for r in roads {
        expand(
            &mut min_x, &mut max_x, &mut min_y, &mut max_y, r.start.x, r.start.y,
        );
        expand(
            &mut min_x, &mut max_x, &mut min_y, &mut max_y, r.end.x, r.end.y,
        );
    }

    Rect::new(min_x, min_y, max_x, max_y)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_district_produces_lots() {
        let config = TradeDistrictConfig::default();
        let district = generate_district(&config);
        assert_eq!(district.lots.len(), config.lot_count);
    }

    #[test]
    fn test_generate_district_produces_roads() {
        let config = TradeDistrictConfig::default();
        let district = generate_district(&config);
        // hex ring + radials + outer rings + connectors
        let expected_base = 6 + config.radial_count + config.ring_count * config.radial_count;
        assert!(district.roads.len() >= expected_base);
    }

    #[test]
    fn test_generate_district_deterministic() {
        let config = TradeDistrictConfig::default();
        let d1 = generate_district(&config);
        let d2 = generate_district(&config);

        assert_eq!(d1.lots.len(), d2.lots.len());
        assert_eq!(d1.roads.len(), d2.roads.len());

        for (a, b) in d1.lots.iter().zip(d2.lots.iter()) {
            assert!((a.position.x - b.position.x).abs() < 0.01);
            assert!((a.position.y - b.position.y).abs() < 0.01);
        }
    }

    #[test]
    fn test_generate_district_bounds_contain_everything() {
        let config = TradeDistrictConfig::default();
        let district = generate_district(&config);

        for lot in &district.lots {
            assert!(district.bounds.contains(lot.position));
            assert!(district.bounds.contains(lot.entrance));
        }
        for r in &district.roads {
            assert!(district.bounds.contains(r.start));
            assert!(district.bounds.contains(r.end));
        }
    }

    #[test]
    fn test_connector_roads_use_inward_ray_to_nearest_base_road() {
        let config = TradeDistrictConfig::default();
        let district = generate_district(&config);
        let base_roads = road::generate_road_network(&config);
        let base_count = base_roads.len();

        for (lot, connector) in district
            .lots
            .iter()
            .zip(district.roads[base_count..].iter())
        {
            let to_center = Vec2::new(-lot.entrance_dir.x, -lot.entrance_dir.y);
            let nearest =
                road::nearest_road_intersection_along_ray(&base_roads, lot.entrance, to_center)
                    .unwrap();
            assert!(
                connector.end.distance_to(nearest) < 0.01,
                "Connector should target nearest inward road from entrance"
            );
        }
    }

    #[test]
    fn test_no_road_crosses_any_lot_interior() {
        let config = TradeDistrictConfig::default();
        let district = generate_district(&config);

        for (road_idx, road) in district.roads.iter().enumerate() {
            for (lot_idx, lot) in district.lots.iter().enumerate() {
                assert!(
                    !placement::road_crosses_lot_interior(road, lot),
                    "Road {} crosses lot {} interior",
                    road_idx,
                    lot_idx,
                );
            }
        }
    }
}
