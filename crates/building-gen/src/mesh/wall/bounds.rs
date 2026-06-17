use crate::config::BuildingConfig;
use crate::tile::{CardinalDir, TileGrid, WallKind, WallShape, WallTile};

pub fn wall_bounds_for_tile(
    grid: &TileGrid,
    x: usize,
    y: usize,
    wall: WallTile,
    config: &BuildingConfig,
) -> ([f32; 3], [f32; 3]) {
    let (tile_min_x, tile_max_x, tile_min_z, tile_max_z) = tile_xz_bounds(grid, x, y, config);
    let ext = config.wall_thickness.min(config.tile_size);
    let int = config.interior_wall_thickness.min(config.tile_size);

    let (min_x, max_x, min_z, max_z) = match wall.shape {
        WallShape::Straight(CardinalDir::Left) if wall.kind == WallKind::Exterior => {
            (tile_min_x, tile_min_x + ext, tile_min_z, tile_max_z)
        }
        WallShape::Straight(CardinalDir::Right) if wall.kind == WallKind::Exterior => {
            (tile_max_x - ext, tile_max_x, tile_min_z, tile_max_z)
        }
        WallShape::Straight(CardinalDir::Bottom) if wall.kind == WallKind::Exterior => {
            (tile_min_x, tile_max_x, tile_min_z, tile_min_z + ext)
        }
        WallShape::Straight(CardinalDir::Top) if wall.kind == WallKind::Exterior => {
            (tile_min_x, tile_max_x, tile_max_z - ext, tile_max_z)
        }
        WallShape::Straight(CardinalDir::Left | CardinalDir::Right) => {
            let cx = (tile_min_x + tile_max_x) / 2.0;
            (cx - int / 2.0, cx + int / 2.0, tile_min_z, tile_max_z)
        }
        WallShape::Straight(CardinalDir::Bottom | CardinalDir::Top) => {
            let cz = (tile_min_z + tile_max_z) / 2.0;
            (tile_min_x, tile_max_x, cz - int / 2.0, cz + int / 2.0)
        }
        WallShape::Corner(_) | WallShape::TJunction(_) | WallShape::Cross => {
            (tile_min_x, tile_max_x, tile_min_z, tile_max_z)
        }
    };

    (
        [min_x, building_base_y(config), min_z],
        [max_x, building_top_y(config), max_z],
    )
}

pub fn building_base_y(config: &BuildingConfig) -> f32 {
    config.foundation_height.max(0.0)
}

pub fn building_top_y(config: &BuildingConfig) -> f32 {
    building_base_y(config) + config.wall_height
}

pub fn tile_xz_bounds(
    grid: &TileGrid,
    x: usize,
    y: usize,
    config: &BuildingConfig,
) -> (f32, f32, f32, f32) {
    let ts = config.tile_size;
    let min_x = grid.origin.x + x as f32 * ts;
    let min_z = grid.origin.y + y as f32 * ts;
    (min_x, min_x + ts, min_z, min_z + ts)
}
