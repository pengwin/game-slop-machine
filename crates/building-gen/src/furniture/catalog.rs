use super::barrel::BarrelConfig;
use super::bed::BedConfig;
use super::chair::ChairConfig;
use super::counter::CounterConfig;
use super::shelf::ShelfConfig;
use super::table::TableConfig;
use super::{FurnitureItem, FurnitureType};

pub fn single_item(item_type: FurnitureType) -> FurnitureItem {
    use crate::geometry::Vec3;

    let (w, h, d, color, mesh) = match item_type {
        FurnitureType::Table => {
            let table_config = TableConfig::default();
            let (w, h, d) = (table_config.width, table_config.height, table_config.depth);
            (
                w,
                h,
                d,
                [0.6, 0.45, 0.25],
                super::table::generate_table_mesh(w, h, d, &table_config),
            )
        }
        FurnitureType::Chair => {
            let chair_config = ChairConfig::default();
            let (w, h, d) = (chair_config.width, chair_config.height, chair_config.depth);
            (
                w,
                h,
                d,
                [0.5, 0.35, 0.2],
                super::chair::generate_chair_mesh(w, h, d, &chair_config),
            )
        }
        FurnitureType::Bed => {
            let (w, h, d) = (1.0, 0.45, 0.9);
            (
                w,
                h,
                d,
                [0.9, 0.9, 0.85],
                super::bed::generate_bed_mesh(w, h, d, &BedConfig::default()),
            )
        }
        FurnitureType::Stove => {
            let (w, h, d) = (1.4, 2.5, 0.8);
            (
                w,
                h,
                d,
                [0.25, 0.25, 0.25],
                super::stove::generate_stove_mesh(w, h, d, &super::stove::StoveConfig::default()),
            )
        }
        FurnitureType::Counter => {
            let (w, h, d) = (0.9, 0.9, 0.5);
            (
                w,
                h,
                d,
                [0.55, 0.4, 0.25],
                super::counter::generate_counter_mesh(w, h, d, &CounterConfig::default()),
            )
        }
        FurnitureType::Desk => {
            let (w, h, d) = (0.7, 0.75, 0.45);
            (
                w,
                h,
                d,
                [0.5, 0.35, 0.2],
                super::desk::generate_desk_mesh(w, h, d, [0.5, 0.35, 0.2]),
            )
        }
        FurnitureType::Barrel => {
            let (d, h) = (0.4, 0.6);
            (
                d,
                h,
                d,
                [0.4, 0.28, 0.15],
                super::barrel::generate_barrel_mesh(d, h, &BarrelConfig::default()),
            )
        }
        FurnitureType::Crate => {
            let (w, h, d) = (0.5, 0.5, 0.5);
            (
                w,
                h,
                d,
                [0.65, 0.55, 0.35],
                super::crate_mesh::generate_crate_mesh(w, h, d, [0.65, 0.55, 0.35]),
            )
        }
        FurnitureType::Bench => {
            let (w, h, d) = (0.8, 0.45, 0.35);
            (
                w,
                h,
                d,
                [0.45, 0.32, 0.18],
                super::bench::generate_bench_mesh(w, h, d, [0.45, 0.32, 0.18]),
            )
        }
        FurnitureType::Shelf => {
            let (w, h, d) = (0.6, 1.2, 0.3);
            (
                w,
                h,
                d,
                [0.5, 0.35, 0.2],
                super::shelf::generate_shelf_mesh(w, h, d, &ShelfConfig::default()),
            )
        }
        FurnitureType::Stool => {
            let stool_config = super::chair::ChairConfig {
                seat_shape: super::chair::ChairSeatShape::Round,
                leg_count: 3,
                back_style: super::chair::ChairBackStyle::None,
                width: 0.35,
                depth: 0.35,
                height: 0.45,
                seat_height: 0.40,
                ..super::chair::ChairConfig::default()
            };
            let (w, h, d) = (stool_config.width, stool_config.height, stool_config.depth);
            (
                w,
                h,
                d,
                [0.5, 0.35, 0.2],
                super::chair::generate_chair_mesh(w, h, d, &stool_config),
            )
        }
    };

    let rotation = if matches!(item_type, FurnitureType::Stove) {
        std::f32::consts::PI
    } else {
        0.0
    };

    FurnitureItem {
        position: Vec3::ZERO,
        rotation,
        item_type,
        width: w,
        height: h,
        depth: d,
        color,
        mesh,
    }
}
