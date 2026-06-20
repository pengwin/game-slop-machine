mod catalog;
pub mod placement;

pub mod barrel;
pub mod bed;
pub mod bench;
pub mod chair;
pub mod counter;
pub mod crate_mesh;
pub mod desk;
pub mod shelf;
pub mod stove;
pub mod table;

use crate::config::BuildingConfig;
use crate::layout::Window;
use crate::layout::{Doorway, Room};
use crate::scene::{SceneObject, SceneObjectKind};
use crate::tile::TileGrid;

pub use catalog::single_item;

pub type FurnitureType = SceneObjectKind;
pub type FurnitureItem = SceneObject;

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
    place_scene_objects(rooms, grid, config, doorways, &[])
}

pub fn place_scene_objects(
    rooms: &[Room],
    grid: &TileGrid,
    config: &BuildingConfig,
    doorways: &[Doorway],
    windows: &[Window],
) -> Vec<SceneObject> {
    crate::scenelet::generate_scene_objects(rooms, grid, config, doorways, windows)
}
