use super::MeshData;
use super::math_util::{Quad, append_colored_quad, append_quad};
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
    let line = (ts * 0.0025).clamp(0.0010, 0.0022);
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
            let xm = (x0 + x1) * 0.5;
            let zm = (z0 + z1) * 0.5;

            append_floor_strip(&mut mesh, x0, z0, x1, z0 + line, floor_y);
            append_floor_strip(&mut mesh, x0, z1 - line, x1, z1, floor_y);
            append_floor_strip(&mut mesh, x0, zm - line * 0.5, x1, zm + line * 0.5, floor_y);
            append_floor_strip(&mut mesh, x0, z0, x0 + line, z1, floor_y);
            append_floor_strip(&mut mesh, x1 - line, z0, x1, z1, floor_y);
            append_floor_strip(&mut mesh, xm - line * 0.5, z0, xm + line * 0.5, z1, floor_y);
        }
    }

    mesh
}

fn append_floor_strip(mesh: &mut MeshData, min_x: f32, min_z: f32, max_x: f32, max_z: f32, y: f32) {
    let dx = max_x - min_x;
    let dz = max_z - min_z;
    let split_count = 4;

    for i in 0..split_count {
        let t0 = i as f32 / split_count as f32;
        let t1 = (i + 1) as f32 / split_count as f32;
        let (sx0, sz0, sx1, sz1) = if dx >= dz {
            (min_x + dx * t0, min_z, min_x + dx * t1, max_z)
        } else {
            (min_x, min_z + dz * t0, max_x, min_z + dz * t1)
        };
        let n = strip_noise((sx0 + sx1) * 0.5, (sz0 + sz1) * 0.5, i as u32);
        let warmth = 0.88 + n * 0.18;
        let alpha = 0.004 + n * 0.012;

        append_colored_quad(
            mesh,
            Quad {
                tl: [sx0, y, sz1],
                tr: [sx1, y, sz1],
                bl: [sx0, y, sz0],
                br: [sx1, y, sz0],
                normal: [0.0, 1.0, 0.0],
                uv_min: [sx0, sz0],
                uv_max: [sx1, sz1],
            },
            [0.40 * warmth, 0.36 * warmth, 0.29 * warmth, alpha],
        );
    }
}

fn strip_noise(x: f32, z: f32, salt: u32) -> f32 {
    let ix = (x * 19.0).floor() as i32;
    let iz = (z * 19.0).floor() as i32;
    let mut n = ix
        .wrapping_mul(374_761_393)
        .wrapping_add(iz.wrapping_mul(668_265_263))
        .wrapping_add((salt as i32).wrapping_mul(97_531));
    n = (n ^ (n >> 13)).wrapping_mul(1_274_126_177);
    ((n ^ (n >> 16)) & 0xffff) as f32 / 65_535.0
}

fn needs_floor_surface(tile: TileType) -> bool {
    match tile {
        TileType::Floor => true,
        TileType::Wall(wall) => wall.kind == WallKind::Interior,
        TileType::Empty => false,
    }
}
