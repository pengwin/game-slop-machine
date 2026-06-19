mod catalog;
pub mod placement;

pub mod bed;
pub mod barrel;
pub mod bench;
pub mod chair;
pub mod counter;
pub mod crate_mesh;
pub mod desk;
pub mod shelf;
pub mod stove;
pub mod table;

use crate::config::BuildingConfig;
use crate::geometry::Vec3;
use crate::layout::{Doorway, Room};
use crate::mesh::MeshData;
use crate::tile::TileGrid;

pub use catalog::single_item;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FurnitureType {
    Table,
    Chair,
    Bed,
    Shelf,
    Counter,
    Desk,
    Stove,
    Barrel,
    Crate,
    Bench,
}

#[derive(Debug, Clone)]
pub struct FurnitureItem {
    pub position: Vec3,
    pub rotation: f32,
    pub item_type: FurnitureType,
    pub width: f32,
    pub height: f32,
    pub depth: f32,
    pub color: [f32; 3],
    pub mesh: MeshData,
}

/// Places furniture in rooms based on room labels.
///
/// Can be called independently of `generate_layout`. Returns an empty vec
/// if `config.furniture` is false.
pub fn place_furniture(
    rooms: &[Room],
    grid: &TileGrid,
    config: &BuildingConfig,
    doorways: &[Doorway],
) -> Vec<FurnitureItem> {
    if !config.furniture {
        return Vec::new();
    }

    let floor_y = crate::mesh::building_base_y(config);
    let mut all_items = Vec::new();
    let mut occupied = placement::OccupiedTiles::new(grid.width, grid.height);

    // Mark doorway tiles as occupied
    for doorway in doorways {
        if let Some((x, y)) = grid.tile_coord(doorway.position) {
            occupied.mark_doorway(x, y);
        }
    }

    for room in rooms {
        let items = catalog::furniture_for_room(room, grid, config, floor_y, &mut occupied);
        all_items.extend(items);
    }

    all_items
}
