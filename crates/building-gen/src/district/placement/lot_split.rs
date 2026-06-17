use super::super::config::TradeDistrictConfig;
use super::super::layout::Lot;
use crate::geometry::Vec2;
use crate::random::deterministic_lot_unit;

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
    if deterministic_lot_unit(lot.position.x, lot.position.y, lot.width, lot.depth, config.seed)
        > config.building_lot_split_chance.clamp(0.0, 1.0)
    {
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

    let landmark_pick =
        deterministic_lot_unit(lot.position.x, lot.position.y, lot.width, lot.depth, config.seed ^ 0xD1B5_4A32_D192_ED03);
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

    let pick = deterministic_lot_unit(lot.position.x, lot.position.y, lot.width, lot.depth, config.seed ^ 0xA24B_AED4_963E_E407) * total_weight;
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
        let unit = deterministic_lot_unit(
            lot.position.x,
            lot.position.y,
            lot.width,
            lot.depth,
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

pub fn scaled_lot(lot: &Lot, scale: f32) -> Lot {
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
