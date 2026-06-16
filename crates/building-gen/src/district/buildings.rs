use super::config::{BuildingDescription, TradeDistrictConfig};
use super::layout::{DistrictBuilding, Lot};
use crate::config::BuildingConfig;
use crate::generate_layout;
use crate::geometry::{Rect, Vec2};

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
    let description = select_building_description_for_lot(
        lot,
        &district_config.building_descriptions,
        district_config.seed + lot_index as u64,
    );
    let config = building_config_for_lot(lot, description, district_config);
    let layout = generate_layout(&config, district_config.seed + lot_index as u64);
    let world_position = building_origin_for_lot(lot, &config);
    let rotation = building_rotation_for_lot(lot);

    DistrictBuilding {
        lot_index,
        description_name: description.name.clone(),
        config,
        layout,
        world_position,
        rotation,
    }
}

fn select_building_description_for_lot<'a>(
    lot: &Lot,
    descriptions: &'a [BuildingDescription],
    seed: u64,
) -> &'a BuildingDescription {
    let area = lot.width * lot.depth;
    let mut scored: Vec<(f32, &'a BuildingDescription)> = descriptions
        .iter()
        .filter(|description| area >= description.min_lot_area && area <= description.max_lot_area)
        .map(|description| (description_score(lot, description), description))
        .collect();

    if scored.is_empty() {
        scored = descriptions
            .iter()
            .map(|description| (fallback_description_score(lot, description), description))
            .collect();
    }

    scored.sort_by(|a, b| {
        a.0.total_cmp(&b.0)
            .then_with(|| a.1.rooms.len().cmp(&b.1.rooms.len()))
    });

    let best_score = scored[0].0;
    let candidate_count = scored
        .iter()
        .take_while(|(score, _)| *score <= best_score + 0.45)
        .count()
        .clamp(1, 4);
    let candidates = &scored[..candidate_count];
    let total_weight: f32 = candidates
        .iter()
        .map(|(score, _)| 1.0 / (0.15 + score - best_score))
        .sum();
    let mut pick = deterministic_unit(lot, seed) * total_weight;

    for (score, description) in candidates {
        pick -= 1.0 / (0.15 + score - best_score);
        if pick <= 0.0 {
            return description;
        }
    }

    candidates[0].1
}

pub fn select_building_description<'a>(
    lot: &Lot,
    descriptions: &'a [BuildingDescription],
) -> &'a BuildingDescription {
    let area = lot.width * lot.depth;

    descriptions
        .iter()
        .filter(|description| area >= description.min_lot_area && area <= description.max_lot_area)
        .min_by(|a, b| {
            description_score(lot, a)
                .total_cmp(&description_score(lot, b))
                .then_with(|| a.rooms.len().cmp(&b.rooms.len()))
        })
        .or_else(|| {
            descriptions.iter().min_by(|a, b| {
                fallback_description_score(lot, a)
                    .total_cmp(&fallback_description_score(lot, b))
                    .then_with(|| a.rooms.len().cmp(&b.rooms.len()))
            })
        })
        .expect("building description list is non-empty")
}

fn description_score(lot: &Lot, description: &BuildingDescription) -> f32 {
    let area = lot.width * lot.depth;
    let area_mid = if description.max_lot_area.is_finite() {
        (description.min_lot_area + description.max_lot_area) / 2.0
    } else {
        description.min_lot_area
    };
    let area_score = (area - area_mid).abs() / area.max(1.0);
    let room_score = (description.rooms.len() as f32 - desired_room_count(area) as f32).abs();
    let program_score = program_fit_score(lot, description);
    area_score * 0.35 + room_score * 0.2 + program_score
}

fn fallback_description_score(lot: &Lot, description: &BuildingDescription) -> f32 {
    let area = lot.width * lot.depth;
    let range_distance = if area < description.min_lot_area {
        description.min_lot_area - area
    } else if area > description.max_lot_area {
        area - description.max_lot_area
    } else {
        0.0
    };
    range_distance / area.max(1.0)
        + (description.rooms.len() as f32 - desired_room_count(area) as f32).abs() * 0.25
        + program_fit_score(lot, description)
}

fn program_fit_score(lot: &Lot, description: &BuildingDescription) -> f32 {
    let lot_area = (lot.width * lot.depth).max(1.0);
    let program_area = preferred_program_area(description);
    let area_score = if program_area > lot_area {
        (program_area - lot_area) / lot_area
    } else {
        (lot_area - program_area) / lot_area * 0.25
    };
    let lot_aspect = lot.width / lot.depth.max(0.1);
    let aspect_score =
        (lot_aspect - preferred_program_aspect(description)).abs() / lot_aspect.max(0.1) * 0.2;

    area_score + aspect_score
}

fn desired_room_count(area: f32) -> usize {
    if area < 55.0 {
        4
    } else if area < 110.0 {
        6
    } else {
        8
    }
}

fn building_config_for_lot(
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

fn best_footprint_for_lot(
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

fn preferred_program_area(description: &BuildingDescription) -> f32 {
    let room_area: f32 = description
        .rooms
        .iter()
        .map(|room| room.preferred_area.max(room.min_area))
        .sum();
    let wall_and_circulation = 1.18;
    let corridor_area = if description.auto_corridor && description.rooms.len() >= 6 {
        description.rooms.len() as f32 * 1.5
    } else {
        0.0
    };

    (room_area * wall_and_circulation + corridor_area).max(12.0)
}

fn preferred_program_aspect(description: &BuildingDescription) -> f32 {
    match description.rooms.len() {
        0..=3 => 1.0,
        4..=5 => 1.25,
        6..=7 => 1.55,
        _ => 1.85,
    }
}

fn snap_size(size: f32, tile_size: f32) -> f32 {
    if tile_size <= f32::EPSILON {
        return size;
    }
    (size / tile_size).round().max(1.0) * tile_size
}

fn building_origin_for_lot(lot: &Lot, config: &BuildingConfig) -> Vec2 {
    let front_center = lot.entrance;
    let right = lot_right_axis(lot);
    front_center - right * (config.footprint.width() / 2.0)
}

fn lot_right_axis(lot: &Lot) -> Vec2 {
    Vec2::new(lot.entrance_dir.y, -lot.entrance_dir.x)
}

fn building_rotation_for_lot(lot: &Lot) -> f32 {
    lot.entrance_dir.x.atan2(lot.entrance_dir.y)
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::district::config::default_building_descriptions;

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
                select_building_description_for_lot(&lot, &descriptions, seed)
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
        let config = building_config_for_lot(&lot, large, &district_config);

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
        let config = building_config_for_lot(&lot, description, &district_config);

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
        let small = building_config_for_lot(&lot, &descriptions[0], &district_config);
        let large = building_config_for_lot(&lot, &descriptions[2], &district_config);

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
        let config = building_config_for_lot(&lot, &descriptions[1], &district_config);

        assert!(config.footprint.width() <= lot.width - 2.0);
        assert!(config.footprint.height() <= lot.depth - 1.0);
        assert!(config.footprint.width() < lot.width - 2.0);
    }

    #[test]
    fn test_building_entrance_is_front_local_edge() {
        let lot = test_lot(12.0, 8.0);
        let district_config = TradeDistrictConfig::default();
        let description = select_building_description(&lot, &district_config.building_descriptions);
        let config = building_config_for_lot(&lot, description, &district_config);

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
        let config = building_config_for_lot(&lot, description, &district_config);
        let origin = building_origin_for_lot(&lot, &config);
        let right = Vec2::new(lot.entrance_dir.y, -lot.entrance_dir.x);
        let mapped_entrance = origin + right * config.entrance.x;
        let expected = lot.entrance;

        assert!(mapped_entrance.distance_to(expected) < 0.01);
    }
}
