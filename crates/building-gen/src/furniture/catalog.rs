use super::placement::{self, OccupiedTiles};
use super::{FurnitureItem, FurnitureType};
use crate::config::BuildingConfig;
use crate::geometry::Vec3;
use crate::layout::Room;
use crate::tile::TileGrid;

use super::barrel::BarrelConfig;
use super::bed::BedConfig;
use super::chair::ChairConfig;
use super::counter::CounterConfig;
use super::shelf::ShelfConfig;
use super::table::TableConfig;

pub fn furniture_for_room(
    room: &Room,
    grid: &TileGrid,
    config: &BuildingConfig,
    floor_y: f32,
    occupied: &mut OccupiedTiles,
) -> Vec<FurnitureItem> {
    match room.label.as_str() {
        "kitchen" => kitchen_items(room, grid, config, floor_y, occupied),
        "bedroom" => bedroom_items(room, grid, config, floor_y, occupied),
        "hall" | "foyer" | "entry" => hall_items(room, grid, config, floor_y, occupied),
        "storage" | "closet" | "pantry" => storage_items(room, grid, config, floor_y, occupied),
        _ => generic_items(room, grid, config, floor_y, occupied),
    }
}

fn kitchen_items(
    room: &Room,
    grid: &TileGrid,
    _config: &BuildingConfig,
    floor_y: f32,
    occupied: &mut OccupiedTiles,
) -> Vec<FurnitureItem> {
    let mut items = Vec::new();
    let wall_positions = placement::find_wall_positions(room.bounds, grid, occupied);

    // Stove (wall-attached)
    if !wall_positions.is_empty() {
        let (tx, ty, rot) = wall_positions[0];
        let (wx, wz) = placement::tile_to_world(tx, ty, grid);
        let w = 1.4;
        let h = 2.5;
        let d = 0.8;
        items.push(FurnitureItem {
            position: Vec3::new(wx, floor_y, wz),
            rotation: rot,
            item_type: FurnitureType::Stove,
            width: w,
            height: h,
            depth: d,
            color: [0.25, 0.25, 0.25],
            mesh: super::stove::generate_stove_mesh(w, h, d, &super::stove::StoveConfig::default()),
        });
        occupied.mark(tx, ty);
    }

    // Counter (wall-attached)
    if wall_positions.len() >= 2 {
        let (tx, ty, rot) = wall_positions[1];
        let (wx, wz) = placement::tile_to_world(tx, ty, grid);
        let w = 0.9;
        let h = 0.9;
        let d = 0.5;
        items.push(FurnitureItem {
            position: Vec3::new(wx, floor_y, wz),
            rotation: rot,
            item_type: FurnitureType::Counter,
            width: w,
            height: h,
            depth: d,
            color: [0.55, 0.4, 0.25],
            mesh: super::counter::generate_counter_mesh(w, h, d, &CounterConfig::default()),
        });
        occupied.mark(tx, ty);
    }

    // Table (center)
    let center_positions = placement::find_center_positions(room.bounds, grid, occupied, 1);
    if !center_positions.is_empty() {
        let (tx, ty) = center_positions[0];
        let (wx, wz) = placement::tile_to_world(tx, ty, grid);
        let table_config = TableConfig {
            width: 0.7,
            ..Default::default()
        };
        let w = table_config.width;
        let h = table_config.height;
        let d = table_config.depth;
        items.push(FurnitureItem {
            position: Vec3::new(wx, floor_y, wz),
            rotation: 0.0,
            item_type: FurnitureType::Table,
            width: w,
            height: h,
            depth: d,
            color: [0.6, 0.45, 0.25],
            mesh: super::table::generate_table_mesh(w, h, d, &table_config),
        });
        occupied.mark(tx, ty);

        // Chairs around table
        let chair_offsets = [(0, -1), (0, 1), (-1, 0), (1, 0)];
        let mut chairs_placed = 0;
        for (dx, dy) in chair_offsets {
            if chairs_placed >= 2 {
                break;
            }
            let cx = tx.wrapping_add_signed(dx);
            let cy = ty.wrapping_add_signed(dy);
            if cx < grid.width
                && cy < grid.height
                && grid.get(cx, cy) == crate::tile::TileType::Floor
                && !occupied.is_occupied(cx, cy)
            {
                let (cwx, cwz) = placement::tile_to_world(cx, cy, grid);
                let rot = if dx == 0 && dy < 0 {
                    std::f32::consts::PI
                } else if dx == 0 && dy > 0 {
                    0.0
                } else if dx < 0 {
                    std::f32::consts::FRAC_PI_2
                } else {
                    -std::f32::consts::FRAC_PI_2
                };
                let chair_config = ChairConfig {
                    width: 0.35,
                    depth: 0.35,
                    ..Default::default()
                };
                items.push(FurnitureItem {
                    position: Vec3::new(cwx, floor_y, cwz),
                    rotation: rot,
                    item_type: FurnitureType::Chair,
                    width: chair_config.width,
                    height: chair_config.height,
                    depth: chair_config.depth,
                    color: [0.5, 0.35, 0.2],
                    mesh: super::chair::generate_chair_mesh(
                        chair_config.width,
                        chair_config.height,
                        chair_config.depth,
                        &chair_config,
                    ),
                });
                occupied.mark(cx, cy);
                chairs_placed += 1;
            }
        }
    }

    items
}

fn bedroom_items(
    room: &Room,
    grid: &TileGrid,
    _config: &BuildingConfig,
    floor_y: f32,
    occupied: &mut OccupiedTiles,
) -> Vec<FurnitureItem> {
    let mut items = Vec::new();
    let wall_positions = placement::find_wall_positions(room.bounds, grid, occupied);

    // Bed (wall-attached, takes 2 tiles)
    if !wall_positions.is_empty() {
        let (tx, ty, rot) = wall_positions[0];
        let (wx, wz) = placement::tile_to_world(tx, ty, grid);
        let w = 1.0;
        let h = 0.45;
        let d = 0.9;
        items.push(FurnitureItem {
            position: Vec3::new(wx, floor_y, wz),
            rotation: rot,
            item_type: FurnitureType::Bed,
            width: w,
            height: h,
            depth: d,
            color: [0.9, 0.9, 0.85],
            mesh: super::bed::generate_bed_mesh(w, h, d, &BedConfig::default()),
        });
        occupied.mark(tx, ty);
    }

    // Desk (wall-attached)
    if wall_positions.len() >= 2 {
        let (tx, ty, rot) = wall_positions[1];
        let (wx, wz) = placement::tile_to_world(tx, ty, grid);
        let w = 0.7;
        let h = 0.75;
        let d = 0.45;
        items.push(FurnitureItem {
            position: Vec3::new(wx, floor_y, wz),
            rotation: rot,
            item_type: FurnitureType::Desk,
            width: w,
            height: h,
            depth: d,
            color: [0.5, 0.35, 0.2],
            mesh: super::desk::generate_desk_mesh(w, h, d, [0.5, 0.35, 0.2]),
        });
        occupied.mark(tx, ty);
    }

    // Chair (center, near desk)
    let center_positions = placement::find_center_positions(room.bounds, grid, occupied, 1);
    if !center_positions.is_empty() {
        let (tx, ty) = center_positions[0];
        let (wx, wz) = placement::tile_to_world(tx, ty, grid);
        let chair_config = ChairConfig {
            width: 0.35,
            depth: 0.35,
            ..Default::default()
        };
        items.push(FurnitureItem {
            position: Vec3::new(wx, floor_y, wz),
            rotation: 0.0,
            item_type: FurnitureType::Chair,
            width: chair_config.width,
            height: chair_config.height,
            depth: chair_config.depth,
            color: [0.5, 0.35, 0.2],
            mesh: super::chair::generate_chair_mesh(
                chair_config.width,
                chair_config.height,
                chair_config.depth,
                &chair_config,
            ),
        });
        occupied.mark(tx, ty);
    }

    items
}

fn hall_items(
    room: &Room,
    grid: &TileGrid,
    _config: &BuildingConfig,
    floor_y: f32,
    occupied: &mut OccupiedTiles,
) -> Vec<FurnitureItem> {
    let mut items = Vec::new();
    let wall_positions = placement::find_wall_positions(room.bounds, grid, occupied);

    // Bench (wall-attached)
    if let Some(&(tx, ty, rot)) = wall_positions.first() {
        let (wx, wz) = placement::tile_to_world(tx, ty, grid);
        let w = 0.8;
        let h = 0.45;
        let d = 0.35;
        items.push(FurnitureItem {
            position: Vec3::new(wx, floor_y, wz),
            rotation: rot,
            item_type: FurnitureType::Bench,
            width: w,
            height: h,
            depth: d,
            color: [0.45, 0.32, 0.18],
            mesh: super::bench::generate_bench_mesh(w, h, d, [0.45, 0.32, 0.18]),
        });
        occupied.mark(tx, ty);
    }

    items
}

fn storage_items(
    room: &Room,
    grid: &TileGrid,
    _config: &BuildingConfig,
    floor_y: f32,
    occupied: &mut OccupiedTiles,
) -> Vec<FurnitureItem> {
    let mut items = Vec::new();
    let center_positions = placement::find_center_positions(room.bounds, grid, occupied, 0);

    // Barrels and crates
    for (i, &(tx, ty)) in center_positions.iter().enumerate().take(4) {
        let (wx, wz) = placement::tile_to_world(tx, ty, grid);
        if i % 2 == 0 {
            items.push(FurnitureItem {
                position: Vec3::new(wx, floor_y, wz),
                rotation: 0.0,
                item_type: FurnitureType::Barrel,
                width: 0.4,
                height: 0.6,
                depth: 0.4,
                color: [0.4, 0.28, 0.15],
                mesh: super::barrel::generate_barrel_mesh(0.4, 0.6, &BarrelConfig::default()),
            });
        } else {
            items.push(FurnitureItem {
                position: Vec3::new(wx, floor_y, wz),
                rotation: 0.0,
                item_type: FurnitureType::Crate,
                width: 0.5,
                height: 0.5,
                depth: 0.5,
                color: [0.65, 0.55, 0.35],
                mesh: super::crate_mesh::generate_crate_mesh(0.5, 0.5, 0.5, [0.65, 0.55, 0.35]),
            });
        }
        occupied.mark(tx, ty);
    }

    items
}

fn generic_items(
    room: &Room,
    grid: &TileGrid,
    _config: &BuildingConfig,
    floor_y: f32,
    occupied: &mut OccupiedTiles,
) -> Vec<FurnitureItem> {
    let mut items = Vec::new();
    let center_positions = placement::find_center_positions(room.bounds, grid, occupied, 1);

    // Table
    if !center_positions.is_empty() {
        let (tx, ty) = center_positions[0];
        let (wx, wz) = placement::tile_to_world(tx, ty, grid);
        let table_config = TableConfig {
            width: 0.7,
            ..Default::default()
        };
        items.push(FurnitureItem {
            position: Vec3::new(wx, floor_y, wz),
            rotation: 0.0,
            item_type: FurnitureType::Table,
            width: table_config.width,
            height: table_config.height,
            depth: table_config.depth,
            color: [0.6, 0.45, 0.25],
            mesh: super::table::generate_table_mesh(
                table_config.width,
                table_config.height,
                table_config.depth,
                &table_config,
            ),
        });
        occupied.mark(tx, ty);
    }

    items
}

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
