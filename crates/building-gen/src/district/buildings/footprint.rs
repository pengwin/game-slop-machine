use super::selection::{preferred_program_area, preferred_program_aspect};
use crate::config::BuildingConfig;
use crate::district::config::BuildingDescription;
use crate::district::layout::Lot;
use crate::geometry::Vec2;

pub fn best_footprint_for_lot(
    description: &BuildingDescription,
    available_width: f32,
    available_depth: f32,
    min_size: f32,
    tile_size: f32,
) -> (f32, f32) {
    let target_area = preferred_program_area(description)
        * description
            .config_overrides
            .footprint_area_scale
            .unwrap_or(1.0)
            .max(0.25);
    let max_area = available_width * available_depth;
    let target_area = target_area.clamp(min_size * min_size, max_area);
    let lot_aspect = (available_width / available_depth).clamp(0.55, 3.25);
    let room_aspect = description
        .config_overrides
        .footprint_aspect
        .unwrap_or_else(|| preferred_program_aspect(description))
        .clamp(0.65, 2.75);
    let candidate_aspects = [
        room_aspect,
        lot_aspect.min(room_aspect * 1.35).max(room_aspect * 0.75),
        1.0,
        1.4,
        1.8,
        2.3,
        0.75,
    ];

    candidate_aspects
        .into_iter()
        .map(|aspect| {
            let mut width = (target_area * aspect).sqrt();
            let mut depth = target_area / width;

            if width > available_width {
                width = available_width;
                depth = (target_area / width).min(available_depth);
            }
            if depth > available_depth {
                depth = available_depth;
                width = (target_area / depth).min(available_width);
            }

            width = snap_size(width.max(min_size).min(available_width), tile_size);
            depth = snap_size(depth.max(min_size).min(available_depth), tile_size);
            width = width.min(available_width);
            depth = depth.min(available_depth);

            let area = width * depth;
            let area_score = ((area - target_area) / target_area.max(1.0)).abs();
            let aspect_score = ((width / depth) - room_aspect).abs() / room_aspect.max(0.1);
            let lot_fill_score = area / max_area.max(1.0) * 0.15;
            let score = area_score * 2.0 + aspect_score + lot_fill_score;
            (score, width, depth)
        })
        .min_by(|a, b| a.0.total_cmp(&b.0))
        .map(|(_, width, depth)| (width, depth))
        .unwrap_or((available_width, available_depth))
}

pub fn building_origin_for_lot(lot: &Lot, config: &BuildingConfig) -> Vec2 {
    let front_center = lot.entrance;
    let right = lot_right_axis(lot);
    front_center - right * (config.footprint.width() / 2.0)
}

pub fn lot_right_axis(lot: &Lot) -> Vec2 {
    Vec2::new(lot.entrance_dir.y, -lot.entrance_dir.x)
}

pub fn building_rotation_for_lot(lot: &Lot) -> f32 {
    lot.entrance_dir.x.atan2(lot.entrance_dir.y)
}

fn snap_size(size: f32, tile_size: f32) -> f32 {
    if tile_size <= f32::EPSILON {
        return size;
    }
    (size / tile_size).round().max(1.0) * tile_size
}
