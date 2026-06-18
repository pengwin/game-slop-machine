use bevy::prelude::*;
use building_gen::config::{BuildingConfig, RoomSpec};
use building_gen::district::config::TradeDistrictConfig;
use building_gen::geometry::Rect;
use game_core::plugins::scene::camera_config::CameraConfig;

pub fn is_district_fixture(fixture: &str) -> bool {
    matches!(
        fixture,
        "district"
            | "district-lots"
            | "district-no-roof"
            | "huge-trade-district"
            | "huge-trade-district-lots"
            | "huge-trade-district-no-roof"
    )
}

pub fn is_furniture_fixture(fixture: &str) -> bool {
    matches!(
        fixture,
        "table"
            | "chair"
            | "bed"
            | "stove"
            | "counter"
            | "desk"
            | "barrel"
            | "crate"
            | "bench"
            | "shelf"
            | "all-furniture"
    )
}

pub fn furniture_camera_for_fixture(fixture: &str) -> CameraConfig {
    let (vh, pos) = if fixture == "all-furniture" {
        (8.0, Vec3::new(6.0, 6.0, 6.0))
    } else {
        (4.0, Vec3::new(3.0, 3.0, 3.0))
    };
    CameraConfig {
        position: pos,
        target: Vec3::new(0.0, 0.2, 0.0),
        viewport_height: vh,
    }
}

pub fn is_district_lots_fixture(fixture: &str) -> bool {
    matches!(fixture, "district-lots" | "huge-trade-district-lots")
}

pub fn district_camera_for_fixture(fixture: &str) -> CameraConfig {
    if fixture.starts_with("huge-trade-district") {
        CameraConfig {
            position: Vec3::new(86.0, 86.0, 86.0),
            target: Vec3::new(0.0, 0.0, 0.0),
            viewport_height: 145.0,
        }
    } else {
        CameraConfig {
            position: Vec3::new(42.0, 42.0, 42.0),
            target: Vec3::new(0.0, 0.0, 0.0),
            viewport_height: 70.0,
        }
    }
}

pub fn district_ground_size_for_fixture(fixture: &str) -> f32 {
    if fixture.starts_with("huge-trade-district") {
        210.0
    } else {
        100.0
    }
}

pub fn district_config_for_fixture(fixture: &str) -> TradeDistrictConfig {
    if fixture.starts_with("huge-trade-district") {
        let mut config = TradeDistrictConfig {
            seed: 42,
            ring_count: 4,
            ring_spacing: 20.0,
            lot_count: 36,
            radial_count: 6,
            lot_width: 1.0,
            lot_height: 0.42,
            lot_depth: 0.08,
            lot_width_randomness: 0.0,
            lot_height_randomness: 0.14,
            lot_depth_randomness: 0.05,
            building_lot_inset: 0.08,
            max_buildings_per_lot: 3,
            building_gap: 0.9,
            preserve_large_lot_area: 300.0,
            landmark_lot_chance: 0.36,
            standalone_lot_width_scale: 0.5,
            standalone_lot_depth_scale: 0.7,
            building_lot_split_chance: 0.9,
            one_building_lot_weight: 0.22,
            two_building_lot_weight: 0.43,
            three_building_lot_weight: 0.35,
            building_lot_split_jitter: 0.32,
            ..Default::default()
        };
        if fixture == "huge-trade-district-no-roof" {
            for desc in &mut config.building_descriptions {
                desc.render_roof = false;
            }
        }
        config
    } else {
        let mut config = TradeDistrictConfig {
            seed: 42,
            ring_spacing: 22.0,
            lot_gap: 0.55,
            lot_width: 1.0,
            lot_height: 0.46,
            lot_depth: 0.08,
            lot_width_randomness: 0.0,
            lot_height_randomness: 0.12,
            lot_depth_randomness: 0.05,
            building_lot_inset: 0.08,
            max_buildings_per_lot: 3,
            building_gap: 0.9,
            preserve_large_lot_area: 260.0,
            landmark_lot_chance: 0.42,
            standalone_lot_width_scale: 0.52,
            standalone_lot_depth_scale: 0.72,
            building_lot_split_chance: 0.86,
            one_building_lot_weight: 0.24,
            two_building_lot_weight: 0.43,
            three_building_lot_weight: 0.33,
            building_lot_split_jitter: 0.32,
            ..Default::default()
        };
        if fixture == "district-no-roof" {
            for desc in &mut config.building_descriptions {
                desc.render_roof = false;
            }
        }
        config
    }
}

pub fn config_for_fixture(fixture: &str) -> BuildingConfig {
    match fixture {
        "procedural" => BuildingConfig {
            room_specs: vec![
                RoomSpec::new("hall", 1),
                RoomSpec::new("kitchen", 2),
                RoomSpec::new("bedroom", 1),
                RoomSpec::new("bathroom", 0),
            ],
            ..Default::default()
        },
        "with-roof" => BuildingConfig {
            room_specs: vec![
                RoomSpec::new("hall", 1),
                RoomSpec::new("kitchen", 2),
                RoomSpec::new("bedroom", 1),
                RoomSpec::new("bathroom", 0),
            ],
            render_roof: true,
            ..Default::default()
        },
        "corridor" => BuildingConfig {
            room_specs: vec![
                RoomSpec::new("hall", 1),
                RoomSpec::new("kitchen", 2),
                RoomSpec::new("bedroom", 1),
                RoomSpec::new("bathroom", 0),
            ],
            has_corridor: true,
            corridor_width: 1.0,
            render_roof: false,
            ..Default::default()
        },
        _ => BuildingConfig {
            footprint: Rect::new(0.0, 0.0, 8.0, 6.0),
            tile_size: 1.0,
            wall_thickness: 1.0,
            interior_wall_thickness: 0.18,
            wall_height: 3.0,
            door_width: 0.8,
            room_specs: vec![RoomSpec::new("room", 0)],
            ..Default::default()
        },
    }
}
