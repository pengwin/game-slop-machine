use super::config::TradeDistrictConfig;
use super::layout::{Lot, RoadSegment};
use crate::geometry::Vec2;
use crate::random::SeededRng;

/// A block defined by two adjacent radials and two ring levels.
struct Block {
    /// Angular midpoint of the block (radians).
    mid_angle: f32,
    /// Angular span of the block (radians).
    angle_span: f32,
    /// Inner ring radius.
    inner_radius: f32,
    /// Outer ring radius.
    outer_radius: f32,
}

/// Places lots in blocks between radial and ring roads.
///
/// All lot entrances face toward the town square center.
/// Lots are rejected if they overlap any road skeleton segment.
pub fn place_lots(
    config: &TradeDistrictConfig,
    rng: &mut SeededRng,
    roads: &[RoadSegment],
) -> Vec<Lot> {
    let center = Vec2::ZERO;
    let blocks = compute_blocks(config);
    let mut lots: Vec<Lot> = Vec::new();

    let mut remaining = config.lot_count;
    let max_per_block = 1;
    let mut placed_per_block: Vec<usize> = vec![0; blocks.len()];

    // Multiple passes across all blocks
    for _pass in 0..max_per_block {
        for (block_idx, block) in blocks.iter().enumerate() {
            if remaining == 0 {
                break;
            }
            if placed_per_block[block_idx] >= max_per_block {
                continue;
            }

            // Try multiple times in this block with different random offsets
            for _attempt in 0..30 {
                let Some(lot) = try_place_lot_in_block(block, config, rng, center) else {
                    break;
                };

                // Try the requested fill first, then shrink locally if this
                // trapezoid segment needs extra clearance near road chords or neighboring lots.
                let mut fitted_lot = None;
                for scale in [1.0, 0.92, 0.84, 0.76, 0.68, 0.6, 0.52, 0.44, 0.36] {
                    let candidate = scaled_lot(&lot, scale);
                    let overlaps_roads = lot_overlaps_roads(&candidate, roads, config.road_width);
                    let overlaps_lots = lots
                        .iter()
                        .any(|existing| lots_overlap(&candidate, existing, 0.0));
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

/// Splits large parcel lots into building lots, each with its own frontage entrance.
pub fn split_lots_for_buildings(lots: &[Lot], config: &TradeDistrictConfig) -> Vec<Lot> {
    if config.max_buildings_per_lot <= 1 {
        return lots.to_vec();
    }

    lots.iter()
        .flat_map(|lot| split_lot_for_buildings(lot, config))
        .collect()
}

fn split_lot_for_buildings(lot: &Lot, config: &TradeDistrictConfig) -> Vec<Lot> {
    let area = lot.width * lot.depth;
    let is_large_parcel = area >= config.preserve_large_lot_area;
    if deterministic_unit(lot, config.seed) > config.building_lot_split_chance.clamp(0.0, 1.0) {
        return vec![single_building_lot(lot, config, is_large_parcel)];
    }

    let count = choose_building_lot_count(lot, config);
    if count <= 1 {
        return vec![single_building_lot(lot, config, is_large_parcel)];
    }
    split_lot_evenly_with_jitter(lot, config, count).unwrap_or_else(|| vec![lot.clone()])
}

fn single_building_lot(lot: &Lot, config: &TradeDistrictConfig, is_large_parcel: bool) -> Lot {
    if !is_large_parcel {
        return lot.clone();
    }

    let landmark_pick = deterministic_unit(lot, config.seed ^ 0xD1B5_4A32_D192_ED03);
    if landmark_pick <= config.landmark_lot_chance.clamp(0.0, 1.0) {
        return lot.clone();
    }

    let width = (lot.width * config.standalone_lot_width_scale.clamp(0.25, 1.0)).max(3.0);
    let depth = (lot.depth * config.standalone_lot_depth_scale.clamp(0.25, 1.0)).max(3.0);
    scaled_lot_around_entrance(lot, width.min(lot.width), depth.min(lot.depth))
}

fn choose_building_lot_count(lot: &Lot, config: &TradeDistrictConfig) -> usize {
    let max_buildings = config.max_buildings_per_lot.clamp(1, 3);
    if max_buildings <= 1 {
        return 1;
    }

    let weights = [
        config.one_building_lot_weight.max(0.0),
        if max_buildings >= 2 {
            config.two_building_lot_weight.max(0.0)
        } else {
            0.0
        },
        if max_buildings >= 3 {
            config.three_building_lot_weight.max(0.0)
        } else {
            0.0
        },
    ];
    let total_weight: f32 = weights.iter().sum();
    if total_weight <= f32::EPSILON {
        return 1;
    }

    let pick = deterministic_unit(lot, config.seed ^ 0xA24B_AED4_963E_E407) * total_weight;
    if pick < weights[0] {
        1
    } else if pick < weights[0] + weights[1] {
        2
    } else {
        3
    }
}

fn split_lot_evenly_with_jitter(
    lot: &Lot,
    config: &TradeDistrictConfig,
    count: usize,
) -> Option<Vec<Lot>> {
    let min_building_width = 3.0;
    let side_inset = config.building_lot_inset.min(lot.width * 0.2).max(0.0);
    let gap = config.building_gap.max(0.0);
    let usable_width = lot.width - side_inset * 2.0 - gap * (count.saturating_sub(1) as f32);
    if usable_width <= 0.0 {
        return None;
    }

    let split_jitter = config.building_lot_split_jitter.clamp(0.0, 0.35);
    let mut weights = Vec::with_capacity(count);
    for index in 0..count {
        let unit = deterministic_unit(
            lot,
            config.seed ^ (0x9E37_79B9_u64.wrapping_mul(index as u64 + 1)),
        );
        weights.push((1.0 + (unit - 0.5) * split_jitter).max(0.2));
    }
    let total_weight: f32 = weights.iter().sum();
    let widths: Vec<f32> = weights
        .into_iter()
        .map(|weight| usable_width * weight / total_weight)
        .collect();
    if widths.iter().any(|width| *width < min_building_width) {
        return None;
    }

    let right = lot_right_axis(lot);
    let total_frontage = usable_width + gap * (count.saturating_sub(1) as f32);
    let mut cursor = -total_frontage / 2.0;
    let mut lots = Vec::with_capacity(count);
    for width in widths {
        let center_offset = cursor + width / 2.0;
        lots.push(sub_lot_from_center(
            lot,
            lot.position + right * center_offset,
            width,
        ));
        cursor += width + gap;
    }

    Some(lots)
}

fn sub_lot_from_center(parent: &Lot, position: Vec2, width: f32) -> Lot {
    let to_center = Vec2::new(-parent.entrance_dir.x, -parent.entrance_dir.y);
    let entrance = position + to_center * (parent.depth / 2.0);

    Lot {
        position,
        width,
        depth: parent.depth,
        rotation: parent.rotation,
        entrance,
        entrance_dir: parent.entrance_dir,
    }
}

fn scaled_lot_around_entrance(parent: &Lot, width: f32, depth: f32) -> Lot {
    let interior = parent.entrance_dir;
    let position = parent.entrance + interior * (depth / 2.0);

    Lot {
        position,
        width,
        depth,
        rotation: parent.rotation,
        entrance: parent.entrance,
        entrance_dir: parent.entrance_dir,
    }
}

fn lot_right_axis(lot: &Lot) -> Vec2 {
    Vec2::new(lot.entrance_dir.y, -lot.entrance_dir.x)
}

fn deterministic_unit(lot: &Lot, seed: u64) -> f32 {
    let mut hash = seed
        ^ (lot.position.x.to_bits() as u64).wrapping_mul(0x9E37_79B1_85EB_CA87)
        ^ (lot.position.y.to_bits() as u64).wrapping_mul(0xC2B2_AE3D_27D4_EB4F)
        ^ (lot.width.to_bits() as u64).wrapping_mul(0x1656_67B1_9E37_79F9)
        ^ (lot.depth.to_bits() as u64).wrapping_mul(0x85EB_CA77_C2B2_AE63);
    hash ^= hash >> 33;
    hash = hash.wrapping_mul(0xff51_afd7_ed55_8ccd);
    hash ^= hash >> 33;
    hash = hash.wrapping_mul(0xc4ce_b9fe_1a85_ec53);
    hash ^= hash >> 33;
    (hash as f64 / u64::MAX as f64) as f32
}

fn scaled_lot(lot: &Lot, scale: f32) -> Lot {
    let width = lot.width * scale;
    let depth = lot.depth * scale;
    let to_center = Vec2::new(-lot.entrance_dir.x, -lot.entrance_dir.y);
    let entrance = Vec2::new(
        lot.position.x + to_center.x * depth / 2.0,
        lot.position.y + to_center.y * depth / 2.0,
    );

    Lot {
        position: lot.position,
        width,
        depth,
        rotation: lot.rotation,
        entrance,
        entrance_dir: lot.entrance_dir,
    }
}

/// Computes all blocks between radials and rings.
fn compute_blocks(config: &TradeDistrictConfig) -> Vec<Block> {
    let hex_radius = config.town_square_radius + config.road_width;
    let angle_step = std::f32::consts::TAU / (config.radial_count as f32);
    let mut blocks = Vec::new();

    for ring_idx in 0..config.ring_count {
        let inner_radius = if ring_idx == 0 {
            hex_radius
        } else {
            config.town_square_radius + config.ring_spacing * ring_idx as f32
        };
        let outer_radius = config.town_square_radius + config.ring_spacing * (ring_idx + 1) as f32;

        for radial_idx in 0..config.radial_count {
            let mid_angle = angle_step * (radial_idx as f32 + 0.5);
            blocks.push(Block {
                mid_angle,
                angle_span: angle_step,
                inner_radius,
                outer_radius,
            });
        }
    }

    blocks
}

/// Tries to place a single lot inside a block.
fn try_place_lot_in_block(
    block: &Block,
    config: &TradeDistrictConfig,
    rng: &mut SeededRng,
    center: Vec2,
) -> Option<Lot> {
    // Compute available space in this block, reserving clearance from the road centerlines.
    let radial_available = block.outer_radius - block.inner_radius;
    let road_clearance = config.road_width / 2.0 + config.lot_gap;
    let usable_radial = radial_available - road_clearance * 2.0;
    if usable_radial <= 0.0 {
        return None;
    }

    let width_fill = randomized_fill(config.lot_width, config.lot_width_randomness, rng);
    let height_fill = randomized_fill(config.lot_height, config.lot_height_randomness, rng);
    let depth_setback = randomized_fill(config.lot_depth, config.lot_depth_randomness, rng);

    let max_height = usable_radial * height_fill;
    if max_height <= 0.0 {
        return None;
    }
    let depth = rng.gen_range(max_height * 0.95, max_height);
    if depth > usable_radial {
        return None;
    }

    let inner_lot_edge = block.inner_radius + road_clearance;
    let outer_lot_edge = block.outer_radius - road_clearance;
    let setback_space = (outer_lot_edge - inner_lot_edge - depth).max(0.0);
    let setback = setback_space * depth_setback;
    let r = inner_lot_edge + setback + depth / 2.0;

    let inner_face_radius = r - depth / 2.0;
    let chord_width =
        2.0 * (inner_face_radius * (block.angle_span / 2.0).tan() - road_clearance) * width_fill;
    let angular_width = 2.0 * ((block.angle_span / 2.0) * r - road_clearance) * width_fill;
    let max_width = chord_width.min(angular_width);
    if max_width <= 0.0 {
        return None;
    }
    let width = rng.gen_range(max_width * 0.95, max_width);

    let half_angle_space = block.angle_span / 2.0;
    let half_lot_angle = (width / 2.0 + road_clearance) / r;
    if half_lot_angle > half_angle_space {
        return None;
    }
    let angle = block.mid_angle;

    let position = Vec2::new(center.x + r * angle.cos(), center.y + r * angle.sin());

    // Entrance faces toward center: compute unit vectors
    let dist_from_center = (position.x * position.x + position.y * position.y).sqrt();
    if dist_from_center < f32::EPSILON {
        return None;
    }
    // direction from center to lot = away from center = INTO the lot from entrance
    let away_from_center = Vec2::new(position.x / dist_from_center, position.y / dist_from_center);
    // direction from lot to center = toward center
    let to_center = Vec2::new(-away_from_center.x, -away_from_center.y);

    // entrance_dir points INTO the lot from the entrance (away from center)
    let entrance_dir = away_from_center;

    // Entrance is on the center-facing edge of the lot (position moved toward center by depth/2)
    let entrance = Vec2::new(
        position.x + to_center.x * depth / 2.0,
        position.y + to_center.y * depth / 2.0,
    );

    // Rotation: align lot so its depth axis points toward center
    let rotation = to_center.y.atan2(to_center.x);

    Some(Lot {
        position,
        width,
        depth,
        rotation,
        entrance,
        entrance_dir,
    })
}

fn randomized_fill(base: f32, randomness: f32, rng: &mut SeededRng) -> f32 {
    if randomness <= 0.0 {
        return base.clamp(0.0, 1.0);
    }

    let random_offset = rng.gen_range(0.0, randomness.max(0.0));
    (base + random_offset).clamp(0.0, 1.0)
}

/// Returns the bounding circle radius for a lot.
pub fn bounding_radius(width: f32, depth: f32) -> f32 {
    ((width * width + depth * depth).sqrt()) / 2.0
}

pub(crate) fn lots_overlap(a: &Lot, b: &Lot, gap: f32) -> bool {
    let a_corners = lot_corners(a);
    let b_corners = lot_corners(b);
    let axes = [
        normalized(a_corners[1] - a_corners[0]),
        normalized(a_corners[3] - a_corners[0]),
        normalized(b_corners[1] - b_corners[0]),
        normalized(b_corners[3] - b_corners[0]),
    ];

    for axis in axes {
        let (a_min, a_max) = project_polygon(&a_corners, axis);
        let (b_min, b_max) = project_polygon(&b_corners, axis);
        if a_max + gap < b_min || b_max + gap < a_min {
            return false;
        }
    }

    true
}

fn normalized(v: Vec2) -> Vec2 {
    let length = v.length();
    if length <= f32::EPSILON {
        Vec2::ZERO
    } else {
        v / length
    }
}

fn project_polygon(corners: &[Vec2; 4], axis: Vec2) -> (f32, f32) {
    let mut min = dot(corners[0], axis);
    let mut max = min;

    for corner in corners.iter().skip(1) {
        let projection = dot(*corner, axis);
        min = min.min(projection);
        max = max.max(projection);
    }

    (min, max)
}

fn dot(a: Vec2, b: Vec2) -> f32 {
    a.x * b.x + a.y * b.y
}

/// Checks whether a lot rectangle crosses any road segment.
///
/// Computes the lot's 4 corners and checks if any road segment
/// (expanded by road_width/2) intersects the lot rectangle.
pub(crate) fn lot_overlaps_roads(lot: &Lot, roads: &[RoadSegment], road_width: f32) -> bool {
    let corners = lot_corners(lot);
    let edges = [
        (corners[0], corners[1]),
        (corners[1], corners[2]),
        (corners[2], corners[3]),
        (corners[3], corners[0]),
    ];
    let half_road = road_width / 2.0;

    for road in roads {
        // Check if road segment intersects any lot edge (with road width padding)
        for &(edge_a, edge_b) in &edges {
            if segments_intersect_with_width(road.start, road.end, edge_a, edge_b, half_road) {
                return true;
            }
        }

        // Check if road midpoint is inside the lot rectangle
        let mid = Vec2::new(
            (road.start.x + road.end.x) / 2.0,
            (road.start.y + road.end.y) / 2.0,
        );
        if point_in_rect(mid, &corners) {
            return true;
        }
    }

    false
}

#[cfg(test)]
pub(crate) fn road_crosses_lot_interior(road: &RoadSegment, lot: &Lot) -> bool {
    let corners = lot_corners(lot);
    let samples = 12;

    for i in 1..samples {
        let t = i as f32 / samples as f32;
        let p = road.start + (road.end - road.start) * t;
        if point_in_rect(p, &corners) {
            return true;
        }
    }

    false
}

/// Returns the 4 corners of a lot rectangle.
fn lot_corners(lot: &Lot) -> [Vec2; 4] {
    let hw = lot.width / 2.0;
    let hd = lot.depth / 2.0;
    let cos_r = lot.rotation.cos();
    let sin_r = lot.rotation.sin();

    // Local axes: depth along rotation direction, width perpendicular
    let dx = cos_r;
    let dz = sin_r;
    let wx = -sin_r;
    let wz = cos_r;

    [
        Vec2::new(
            lot.position.x + (-dx * hd - wx * hw),
            lot.position.y + (-dz * hd - wz * hw),
        ),
        Vec2::new(
            lot.position.x + (-dx * hd + wx * hw),
            lot.position.y + (-dz * hd + wz * hw),
        ),
        Vec2::new(
            lot.position.x + (dx * hd + wx * hw),
            lot.position.y + (dz * hd + wz * hw),
        ),
        Vec2::new(
            lot.position.x + (dx * hd - wx * hw),
            lot.position.y + (dz * hd - wz * hw),
        ),
    ]
}

/// Checks if two segments intersect, with an added width tolerance.
fn segments_intersect_with_width(a1: Vec2, a2: Vec2, b1: Vec2, b2: Vec2, width: f32) -> bool {
    // Check exact intersection
    if segments_intersect(a1, a2, b1, b2) {
        return true;
    }
    // Check if any endpoint of segment A is within `width` of segment B
    if point_to_segment_dist(a1, b1, b2) < width || point_to_segment_dist(a2, b1, b2) < width {
        return true;
    }
    // Check if any endpoint of segment B is within `width` of segment A
    if point_to_segment_dist(b1, a1, a2) < width || point_to_segment_dist(b2, a1, a2) < width {
        return true;
    }
    false
}

/// Returns the distance from a point to a line segment.
fn point_to_segment_dist(p: Vec2, a: Vec2, b: Vec2) -> f32 {
    let closest = super::road::closest_point_on_segment(a, b, p);
    p.distance_to(closest)
}

/// Checks if two line segments intersect (exact, no width).
fn segments_intersect(a1: Vec2, a2: Vec2, b1: Vec2, b2: Vec2) -> bool {
    let d1 = direction(a1, a2, b1);
    let d2 = direction(a1, a2, b2);
    let d3 = direction(b1, b2, a1);
    let d4 = direction(b1, b2, a2);

    if ((d1 > 0.0 && d2 < 0.0) || (d1 < 0.0 && d2 > 0.0))
        && ((d3 > 0.0 && d4 < 0.0) || (d3 < 0.0 && d4 > 0.0))
    {
        return true;
    }

    if d1.abs() < f32::EPSILON && on_segment(a1, a2, b1) {
        return true;
    }
    if d2.abs() < f32::EPSILON && on_segment(a1, a2, b2) {
        return true;
    }
    if d3.abs() < f32::EPSILON && on_segment(b1, b2, a1) {
        return true;
    }
    if d4.abs() < f32::EPSILON && on_segment(b1, b2, a2) {
        return true;
    }

    false
}

fn direction(a: Vec2, b: Vec2, c: Vec2) -> f32 {
    (b.x - a.x) * (c.y - a.y) - (b.y - a.y) * (c.x - a.x)
}

fn on_segment(a: Vec2, b: Vec2, p: Vec2) -> bool {
    p.x >= a.x.min(b.x) - f32::EPSILON
        && p.x <= a.x.max(b.x) + f32::EPSILON
        && p.y >= a.y.min(b.y) - f32::EPSILON
        && p.y <= a.y.max(b.y) + f32::EPSILON
}

/// Checks if a point is inside a convex polygon (4 corners) using cross products.
fn point_in_rect(p: Vec2, corners: &[Vec2; 4]) -> bool {
    for i in 0..4 {
        let a = corners[i];
        let b = corners[(i + 1) % 4];
        if direction(a, b, p) < -f32::EPSILON {
            return false;
        }
    }
    true
}

/// Connector road from a lot entrance to the nearest road segment.
pub fn connector_road_for_lot(
    lot: &Lot,
    all_roads: &[super::layout::RoadSegment],
    road_width: f32,
) -> Option<super::layout::RoadSegment> {
    super::road::connector_road_from_entrance(
        lot.entrance,
        Vec2::new(-lot.entrance_dir.x, -lot.entrance_dir.y),
        all_roads,
        road_width,
    )
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
        assert!(lots
            .windows(2)
            .all(|pair| pair[0].position.distance_to(pair[1].position) > config.building_gap));
        assert!(lots.iter().all(|lot| lot.width >= 3.0));
        assert!(lots
            .iter()
            .all(|lot| lot.entrance_dir == Vec2::new(0.0, 1.0)));
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
        let blocks = compute_blocks(&config);
        let mut rng = SeededRng::new(config.seed);
        let lots = place_lots(&config, &mut rng, &roads);

        for lot in lots {
            let lot_angle = lot.position.y.atan2(lot.position.x);
            let block = blocks
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

        let block = &compute_blocks(&narrow_config)[0];
        let narrow_lot = try_place_lot_in_block(block, &narrow_config, &mut narrow_rng, Vec2::ZERO)
            .expect("narrow lot should fit");
        let wide_lot = try_place_lot_in_block(block, &wide_config, &mut wide_rng, Vec2::ZERO)
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

        let block = &compute_blocks(&shallow_config)[0];
        let shallow_lot =
            try_place_lot_in_block(block, &shallow_config, &mut shallow_rng, Vec2::ZERO)
                .expect("shallow lot should fit");
        let deep_lot = try_place_lot_in_block(block, &deep_config, &mut deep_rng, Vec2::ZERO)
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

        let block = &compute_blocks(&close_config)[0];
        let close_lot = try_place_lot_in_block(block, &close_config, &mut close_rng, Vec2::ZERO)
            .expect("close lot should fit");
        let far_lot = try_place_lot_in_block(block, &far_config, &mut far_rng, Vec2::ZERO)
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
            let value = randomized_fill(0.4, 0.1, &mut rng);
            assert!((0.4..=0.5).contains(&value));
        }

        let mut rng = SeededRng::new(7);
        for _ in 0..16 {
            let value = randomized_fill(0.95, 0.2, &mut rng);
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
            // entrance_dir should point away from center (into the lot)
            // Verify: entrance_dir dot (position - entrance) > 0
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
