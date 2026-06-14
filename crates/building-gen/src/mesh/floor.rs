use super::math_util::append_quad;
use super::wall::building_base_y;
use super::MeshData;
use crate::config::BuildingConfig;
use crate::tile::{TileGrid, TileType, WallKind};

pub fn generate_floor_mesh(grid: &TileGrid, config: &BuildingConfig) -> MeshData {
    let mut mesh = MeshData::default();
    let ts = config.tile_size;
    let origin_x = grid.origin.x;
    let origin_z = grid.origin.y;
    let floor_y = building_base_y(config);

    for y in 0..grid.height {
        for x in 0..grid.width {
            if !needs_floor_surface(grid.get(x, y)) {
                continue;
            }
            let x0 = origin_x + x as f32 * ts;
            let z0 = origin_z + y as f32 * ts;
            let x1 = x0 + ts;
            let z1 = z0 + ts;

            append_quad(
                &mut mesh,
                [x0, floor_y, z1],
                [x1, floor_y, z1],
                [x0, floor_y, z0],
                [x1, floor_y, z0],
                [0.0, 1.0, 0.0],
                [x0, z0],
                [x1, z1],
            );
        }
    }

    mesh
}

fn needs_floor_surface(tile: TileType) -> bool {
    match tile {
        TileType::Floor => true,
        TileType::Wall(wall) => wall.kind == WallKind::Interior,
        TileType::Empty => false,
    }
}
