use super::MeshData;
use super::math_util::{Quad, append_quad};
use super::wall::building_base_y;
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
                Quad {
                    tl: [x0, floor_y, z1],
                    tr: [x1, floor_y, z1],
                    bl: [x0, floor_y, z0],
                    br: [x1, floor_y, z0],
                    normal: [0.0, 1.0, 0.0],
                    uv_min: [x0, z0],
                    uv_max: [x1, z1],
                },
            );
        }
    }

    mesh
}

pub fn generate_floor_grout_mesh(grid: &TileGrid, config: &BuildingConfig) -> MeshData {
    let mut mesh = MeshData::default();
    let ts = config.tile_size;
    let line = (ts * 0.016).clamp(0.006, 0.014);
    let origin_x = grid.origin.x;
    let origin_z = grid.origin.y;
    let floor_y = building_base_y(config) + 0.004;

    for y in 0..grid.height {
        for x in 0..grid.width {
            if !needs_floor_surface(grid.get(x, y)) {
                continue;
            }

            let x0 = origin_x + x as f32 * ts;
            let z0 = origin_z + y as f32 * ts;
            let x1 = x0 + ts;
            let z1 = z0 + ts;

            append_floor_strip(&mut mesh, x0, z0, x1, z0 + line, floor_y);
            append_floor_strip(&mut mesh, x0, z1 - line, x1, z1, floor_y);
            append_floor_strip(&mut mesh, x0, z0, x0 + line, z1, floor_y);
            append_floor_strip(&mut mesh, x1 - line, z0, x1, z1, floor_y);
        }
    }

    mesh
}

fn append_floor_strip(mesh: &mut MeshData, min_x: f32, min_z: f32, max_x: f32, max_z: f32, y: f32) {
    append_quad(
        mesh,
        Quad {
            tl: [min_x, y, max_z],
            tr: [max_x, y, max_z],
            bl: [min_x, y, min_z],
            br: [max_x, y, min_z],
            normal: [0.0, 1.0, 0.0],
            uv_min: [min_x, min_z],
            uv_max: [max_x, max_z],
        },
    );
}

fn needs_floor_surface(tile: TileType) -> bool {
    match tile {
        TileType::Floor => true,
        TileType::Wall(wall) => wall.kind == WallKind::Interior,
        TileType::Empty => false,
    }
}
