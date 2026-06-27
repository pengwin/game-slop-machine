use super::MeshData;
use super::math_util::{Quad, append_colored_quad_vertices};
use super::wall::building_base_y;
use crate::config::{BuildingConfig, FloorAmbientOcclusionSettings};
use crate::tile::{TileGrid, TileType, WallKind};

pub fn generate_floor_mesh(grid: &TileGrid, config: &BuildingConfig) -> MeshData {
    let mut mesh = MeshData::default();
    let ts = config.tile_size;
    let origin_x = grid.origin.x;
    let origin_z = grid.origin.y;
    let floor_y = building_base_y(config);
    let ao = config.visual_style.floor_ao;
    let mut mergeable = vec![false; grid.width * grid.height];
    let mut visited = vec![false; grid.width * grid.height];

    for y in 0..grid.height {
        for x in 0..grid.width {
            if !needs_floor_surface(grid.get(x, y)) {
                continue;
            }
            if !floor_tile_needs_subdivision(grid, x, y, ao) {
                mergeable[tile_index(grid, x, y)] = true;
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
                ao,
            );
        }
    }

    for y in 0..grid.height {
        for x in 0..grid.width {
            let index = tile_index(grid, x, y);
            if !mergeable[index] || visited[index] {
                continue;
            }

            let width = merged_rect_width(grid, &mergeable, &visited, x, y);
            let height = merged_rect_height(grid, &mergeable, &visited, x, y, width);
            for ry in y..y + height {
                for rx in x..x + width {
                    visited[tile_index(grid, rx, ry)] = true;
                }
            }

            let x0 = origin_x + x as f32 * ts;
            let z0 = origin_z + y as f32 * ts;
            let x1 = origin_x + (x + width) as f32 * ts;
            let z1 = origin_z + (y + height) as f32 * ts;
            append_flat_floor_rect(&mut mesh, (x0, z0, x1, z1), floor_y);
        }
    }

    mesh
}

fn tile_index(grid: &TileGrid, x: usize, y: usize) -> usize {
    y * grid.width + x
}

fn merged_rect_width(
    grid: &TileGrid,
    mergeable: &[bool],
    visited: &[bool],
    start_x: usize,
    y: usize,
) -> usize {
    let mut width = 0;
    for x in start_x..grid.width {
        let index = tile_index(grid, x, y);
        if !mergeable[index] || visited[index] {
            break;
        }
        width += 1;
    }
    width
}

fn merged_rect_height(
    grid: &TileGrid,
    mergeable: &[bool],
    visited: &[bool],
    start_x: usize,
    start_y: usize,
    width: usize,
) -> usize {
    let mut height = 1;
    'rows: for y in start_y + 1..grid.height {
        for x in start_x..start_x + width {
            let index = tile_index(grid, x, y);
            if !mergeable[index] || visited[index] {
                break 'rows;
            }
        }
        height += 1;
    }
    height
}

fn append_flat_floor_rect(mesh: &mut MeshData, bounds: (f32, f32, f32, f32), floor_y: f32) {
    let (x0, z0, x1, z1) = bounds;
    append_colored_quad_vertices(
        mesh,
        Quad {
            tl: [x0, floor_y, z1],
            tr: [x1, floor_y, z1],
            bl: [x0, floor_y, z0],
            br: [x1, floor_y, z0],
            normal: [0.0, 1.0, 0.0],
            uv_min: [x0, z0],
            uv_max: [x1, z1],
        },
        [[1.0, 1.0, 1.0, 1.0]; 4],
        crate::mesh::SurfaceMaterial::Colored,
    );
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
    let split_count = if floor_tile_needs_subdivision(grid, tile_x, tile_y, ao) {
        ao.subdivisions.max(1)
    } else {
        1
    };

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

fn floor_tile_needs_subdivision(
    grid: &TileGrid,
    tile_x: usize,
    tile_y: usize,
    ao: FloorAmbientOcclusionSettings,
) -> bool {
    if ao.edge_strength <= 0.0 || ao.width_tiles <= 0.0 || ao.subdivisions <= 1 {
        return false;
    }

    let radius = ao.width_tiles.ceil() as isize;
    for dy in -radius..=radius {
        for dx in -radius..=radius {
            if let Some((nx, ny)) = offset_coord(grid, tile_x, tile_y, dx, dy) {
                if grid.get(nx, ny).is_wall() {
                    return true;
                }
            }
        }
    }
    false
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
    let radius = (edge_width / grid.tile_size).ceil() as isize;
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
    fn floor_mesh_merges_center_tiles_without_wall_ao() {
        let config = BuildingConfig {
            footprint: Rect::new(0.0, 0.0, 4.0, 3.0),
            tile_size: 1.0,
            ..Default::default()
        };
        let mut grid = TileGrid::new(4, 3, config.tile_size, Vec2::ZERO);
        for y in 0..grid.height {
            for x in 0..grid.width {
                grid.set(x, y, TileType::Floor);
            }
        }

        let mesh = generate_floor_mesh(&grid, &config);

        assert_eq!(mesh.indices.len() / 3, 2);
        assert_eq!(mesh.vertices.len(), 4);
        assert_eq!(mesh.uvs[1], [4.0, 3.0]);
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
    fn floor_ao_default_uses_low_subdivision_near_walls() {
        let config = BuildingConfig {
            footprint: Rect::new(0.0, 0.0, 1.0, 1.0),
            tile_size: 1.0,
            ..Default::default()
        };
        let mut grid = TileGrid::new(2, 1, config.tile_size, Vec2::ZERO);
        grid.set(
            0,
            0,
            TileType::Wall(WallTile::exterior(WallShape::Straight(CardinalDir::Left))),
        );
        grid.set(1, 0, TileType::Floor);

        let mesh = generate_floor_mesh(&grid, &config);

        assert_eq!(mesh.indices.len() / 3, 8);
    }
}
