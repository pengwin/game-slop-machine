use super::super::config::TradeDistrictConfig;
use super::super::layout::Lot;
use crate::geometry::Vec2;
use crate::random::SeededRng;

pub struct Block {
    pub mid_angle: f32,
    pub angle_span: f32,
    pub inner_radius: f32,
    pub outer_radius: f32,
}

pub fn compute_blocks(config: &TradeDistrictConfig) -> Vec<Block> {
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

pub fn try_place_lot_in_block(
    block: &Block,
    config: &TradeDistrictConfig,
    rng: &mut SeededRng,
    center: Vec2,
) -> Option<Lot> {
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

    let dist_from_center = (position.x * position.x + position.y * position.y).sqrt();
    if dist_from_center < f32::EPSILON {
        return None;
    }
    let away_from_center = Vec2::new(position.x / dist_from_center, position.y / dist_from_center);
    let to_center = Vec2::new(-away_from_center.x, -away_from_center.y);

    let entrance_dir = away_from_center;

    let entrance = Vec2::new(
        position.x + to_center.x * depth / 2.0,
        position.y + to_center.y * depth / 2.0,
    );

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

pub fn randomized_fill(base: f32, randomness: f32, rng: &mut SeededRng) -> f32 {
    if randomness <= 0.0 {
        return base.clamp(0.0, 1.0);
    }

    let random_offset = rng.gen_range(0.0, randomness.max(0.0));
    (base + random_offset).clamp(0.0, 1.0)
}

pub fn bounding_radius(width: f32, depth: f32) -> f32 {
    ((width * width + depth * depth).sqrt()) / 2.0
}
