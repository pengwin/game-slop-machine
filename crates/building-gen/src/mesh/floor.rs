use super::MeshData;
use super::math_util::{Quad, append_colored_quad, append_colored_quad_vertices};
use super::wall::building_base_y;
use crate::config::{BuildingConfig, FloorAmbientOcclusionSettings, FloorGroutSettings};
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

            append_shaded_floor_tile(
                &mut mesh,
                grid,
                x,
                y,
                (x0, z0, x1, z1),
                floor_y,
                config.tile_size,
                config.visual_style.floor_ao,
            );
        }
    }

    mesh
}

fn append_shaded_floor_tile(
    mesh: &mut MeshData,
    grid: &TileGrid,
    tile_x: usize,
    tile_y: usize,
    bounds: (f32, f32, f32, f32),
    floor_y: f32,
    tile_size: f32,
    ao: FloorAmbientOcclusionSettings,
) {
    let (x0, z0, x1, z1) = bounds;
    let split_count = ao.subdivisions.max(1);
    for iy in 0..split_count {
        for ix in 0..split_count {
            let sx0 = lerp(x0, x1, ix as f32 / split_count as f32);
            let sx1 = lerp(x0, x1, (ix + 1) as f32 / split_count as f32);
            let sz0 = lerp(z0, z1, iy as f32 / split_count as f32);
            let sz1 = lerp(z0, z1, (iy + 1) as f32 / split_count as f32);

            let tl = [sx0, floor_y, sz1];
            let tr = [sx1, floor_y, sz1];
            let bl = [sx0, floor_y, sz0];
            let br = [sx1, floor_y, sz0];
            append_colored_quad_vertices(
                mesh,
                Quad {
                    tl,
                    tr,
                    bl,
                    br,
                    normal: [0.0, 1.0, 0.0],
                    uv_min: [sx0, sz0],
                    uv_max: [sx1, sz1],
                },
                [
                    floor_vertex_color(tl, grid, tile_x, tile_y, tile_size, ao),
                    floor_vertex_color(tr, grid, tile_x, tile_y, tile_size, ao),
                    floor_vertex_color(bl, grid, tile_x, tile_y, tile_size, ao),
                    floor_vertex_color(br, grid, tile_x, tile_y, tile_size, ao),
                ],
                crate::mesh::SurfaceMaterial::Colored,
            );
        }
    }
}

fn floor_vertex_color(
    pos: [f32; 3],
    grid: &TileGrid,
    tile_x: usize,
    tile_y: usize,
    tile_size: f32,
    ao: FloorAmbientOcclusionSettings,
) -> [f32; 4] {
    if ao.edge_strength <= 0.0 || ao.width_tiles <= 0.0 {
        return [1.0, 1.0, 1.0, 1.0];
    }

    let edge_width = tile_size * ao.width_tiles;
    let wall_fades = nearby_wall_fades(pos, grid, tile_x, tile_y, edge_width, ao.falloff);
    let side_ao = (wall_fades.left + wall_fades.right + wall_fades.bottom + wall_fades.top)
        .min(1.8)
        * ao.edge_strength;
    let corner_ao = (wall_fades.left * wall_fades.bottom
        + wall_fades.left * wall_fades.top
        + wall_fades.right * wall_fades.bottom
        + wall_fades.right * wall_fades.top)
        .min(1.0)
        * ao.corner_strength;

    let tint = (1.0 - side_ao - corner_ao).clamp(0.0, 1.0);
    [tint, tint, tint, 1.0]
}

#[derive(Debug, Clone, Copy, Default)]
struct WallFades {
    left: f32,
    right: f32,
    bottom: f32,
    top: f32,
}

fn nearby_wall_fades(
    pos: [f32; 3],
    grid: &TileGrid,
    tile_x: usize,
    tile_y: usize,
    edge_width: f32,
    falloff: f32,
) -> WallFades {
    let radius = (edge_width / grid.tile_size).ceil() as isize + 1;
    let mut fades = WallFades::default();

    for dy in -radius..=radius {
        for dx in -radius..=radius {
            let Some((wall_x, wall_y)) = offset_coord(grid, tile_x, tile_y, dx, dy) else {
                continue;
            };
            if !grid.get(wall_x, wall_y).is_wall() {
                continue;
            }

            let wall_bounds = tile_xz_bounds(grid, wall_x, wall_y);
            let fade = wall_tile_fade(pos, wall_bounds, edge_width, falloff);
            if fade <= 0.0 {
                continue;
            }

            let (min_x, max_x, min_z, max_z) = wall_bounds;
            if max_x <= pos[0] {
                fades.left = fades.left.max(fade);
            }
            if min_x >= pos[0] {
                fades.right = fades.right.max(fade);
            }
            if max_z <= pos[2] {
                fades.bottom = fades.bottom.max(fade);
            }
            if min_z >= pos[2] {
                fades.top = fades.top.max(fade);
            }
        }
    }

    fades
}

fn wall_tile_fade(pos: [f32; 3], bounds: (f32, f32, f32, f32), width: f32, falloff: f32) -> f32 {
    let (min_x, max_x, min_z, max_z) = bounds;
    let nearest_x = pos[0].clamp(min_x, max_x);
    let nearest_z = pos[2].clamp(min_z, max_z);
    let dx = pos[0] - nearest_x;
    let dz = pos[2] - nearest_z;
    edge_fade((dx * dx + dz * dz).sqrt(), width, falloff)
}

fn edge_fade(distance: f32, width: f32, falloff: f32) -> f32 {
    (1.0 - distance.max(0.0) / width.max(f32::EPSILON))
        .clamp(0.0, 1.0)
        .powf(falloff.max(f32::EPSILON))
}

fn offset_coord(
    grid: &TileGrid,
    x: usize,
    y: usize,
    dx: isize,
    dy: isize,
) -> Option<(usize, usize)> {
    let nx = x as isize + dx;
    let ny = y as isize + dy;
    if nx >= 0 && ny >= 0 && (nx as usize) < grid.width && (ny as usize) < grid.height {
        Some((nx as usize, ny as usize))
    } else {
        None
    }
}

fn tile_xz_bounds(grid: &TileGrid, x: usize, y: usize) -> (f32, f32, f32, f32) {
    let min_x = grid.origin.x + x as f32 * grid.tile_size;
    let min_z = grid.origin.y + y as f32 * grid.tile_size;
    (min_x, min_x + grid.tile_size, min_z, min_z + grid.tile_size)
}

fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t
}

pub fn generate_floor_grout_mesh(grid: &TileGrid, config: &BuildingConfig) -> MeshData {
    let mut mesh = MeshData::default();
    let ts = config.tile_size;
    let grout = config.visual_style.floor_grout;
    let min_line = grout.min_line_width.max(0.0);
    let max_line = grout.max_line_width.max(min_line);
    if grout.line_width_factor <= 0.0 || max_line <= f32::EPSILON {
        return mesh;
    }

    let line = (ts * grout.line_width_factor).clamp(min_line, max_line);
    if line <= f32::EPSILON {
        return mesh;
    }

    let center_line_half = line * grout.center_line_scale.max(0.0) * 0.5;
    let origin_x = grid.origin.x;
    let origin_z = grid.origin.y;
    let floor_y = building_base_y(config) + grout.height_offset;

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

            append_floor_strip(&mut mesh, x0, z0, x1, z0 + line, floor_y, grout);
            append_floor_strip(&mut mesh, x0, z1 - line, x1, z1, floor_y, grout);
            if center_line_half > f32::EPSILON {
                append_floor_strip(
                    &mut mesh,
                    x0,
                    zm - center_line_half,
                    x1,
                    zm + center_line_half,
                    floor_y,
                    grout,
                );
            }
            append_floor_strip(&mut mesh, x0, z0, x0 + line, z1, floor_y, grout);
            append_floor_strip(&mut mesh, x1 - line, z0, x1, z1, floor_y, grout);
            if center_line_half > f32::EPSILON {
                append_floor_strip(
                    &mut mesh,
                    xm - center_line_half,
                    z0,
                    xm + center_line_half,
                    z1,
                    floor_y,
                    grout,
                );
            }
        }
    }

    mesh
}

fn append_floor_strip(
    mesh: &mut MeshData,
    min_x: f32,
    min_z: f32,
    max_x: f32,
    max_z: f32,
    y: f32,
    grout: FloorGroutSettings,
) {
    let dx = max_x - min_x;
    let dz = max_z - min_z;
    let split_count = grout.strip_subdivisions.max(1);

    for i in 0..split_count {
        let t0 = i as f32 / split_count as f32;
        let t1 = (i + 1) as f32 / split_count as f32;
        let (sx0, sz0, sx1, sz1) = if dx >= dz {
            (min_x + dx * t0, min_z, min_x + dx * t1, max_z)
        } else {
            (min_x, min_z + dz * t0, max_x, min_z + dz * t1)
        };
        let n = strip_noise((sx0 + sx1) * 0.5, (sz0 + sz1) * 0.5, i as u32);
        let warmth = grout.warmth_base + n * grout.warmth_noise;
        let alpha = (grout.alpha_base + n * grout.alpha_noise).clamp(0.0, 1.0);

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
            [
                (grout.color[0] * warmth).clamp(0.0, 1.0),
                (grout.color[1] * warmth).clamp(0.0, 1.0),
                (grout.color[2] * warmth).clamp(0.0, 1.0),
                alpha,
            ],
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::geometry::{Rect, Vec2};
    use crate::tile::{CardinalDir, WallShape, WallTile};

    fn wall() -> TileType {
        TileType::Wall(WallTile::interior(WallShape::Straight(CardinalDir::Left)))
    }

    #[test]
    fn floor_mesh_has_vertex_colors_for_shading() {
        let config = BuildingConfig {
            footprint: Rect::new(0.0, 0.0, 1.0, 1.0),
            tile_size: 1.0,
            ..Default::default()
        };
        let mut grid = TileGrid::new(1, 1, config.tile_size, Vec2::ZERO);
        grid.set(0, 0, TileType::Floor);

        let mesh = generate_floor_mesh(&grid, &config);

        assert_eq!(mesh.colors.len(), mesh.vertices.len());
        assert!(!mesh.colors.is_empty());
    }

    #[test]
    fn floor_corner_near_two_walls_is_darker_than_edge_and_center() {
        let mut grid = TileGrid::new(8, 8, 1.0, Vec2::ZERO);
        grid.set(4, 4, TileType::Floor);
        grid.set(3, 4, wall());
        grid.set(4, 3, wall());
        grid.set(4, 6, TileType::Floor);
        grid.set(3, 6, wall());

        let ao = FloorAmbientOcclusionSettings::default();
        let corner = floor_vertex_color([4.0, 0.0, 4.0], &grid, 4, 4, 1.0, ao)[0];
        let edge = floor_vertex_color([4.0, 0.0, 6.5], &grid, 4, 6, 1.0, ao)[0];
        let center = floor_vertex_color([6.6, 0.0, 6.6], &grid, 4, 4, 1.0, ao)[0];

        assert!(corner < edge);
        assert!(edge < center);
        assert!(corner < 0.80);
        assert!(edge < 0.94);
    }

    #[test]
    fn floor_edge_shading_falls_off_as_gradient() {
        let mut grid = TileGrid::new(8, 8, 1.0, Vec2::ZERO);
        grid.set(4, 4, TileType::Floor);
        grid.set(3, 4, wall());

        let ao = FloorAmbientOcclusionSettings::default();
        let near_wall = floor_vertex_color([4.0, 0.0, 4.5], &grid, 4, 4, 1.0, ao)[0];
        let mid_fade = floor_vertex_color([4.5, 0.0, 4.5], &grid, 4, 4, 1.0, ao)[0];
        let far_edge = floor_vertex_color([5.0, 0.0, 4.5], &grid, 4, 4, 1.0, ao)[0];

        assert!(near_wall < mid_fade);
        assert!(mid_fade < far_edge);
    }

    #[test]
    fn floor_grout_zero_width_disables_mesh() {
        let mut config = BuildingConfig {
            footprint: Rect::new(0.0, 0.0, 1.0, 1.0),
            tile_size: 1.0,
            ..Default::default()
        };
        config.visual_style.floor_grout.line_width_factor = 0.0;

        let mut grid = TileGrid::new(1, 1, config.tile_size, Vec2::ZERO);
        grid.set(0, 0, TileType::Floor);

        let mesh = generate_floor_grout_mesh(&grid, &config);

        assert!(mesh.is_empty());
    }
}
