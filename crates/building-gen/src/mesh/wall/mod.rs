mod bounds;
mod boxes;
mod classify;
mod faces;

use super::MeshData;
use crate::config::BuildingConfig;
use crate::tile::{TileGrid, TileType};

pub use bounds::{building_base_y, building_top_y, wall_bounds_for_tile};

#[derive(Debug, Clone, Default)]
pub struct WallMeshes {
    pub wall: MeshData,
    pub top: MeshData,
    pub exterior: MeshData,
    pub exterior_corner: MeshData,
    pub exterior_t_junction: MeshData,
}

pub fn generate_wall_meshes(grid: &TileGrid, config: &BuildingConfig) -> WallMeshes {
    let mut wall_mesh = MeshData::default();
    let mut wall_top_mesh = MeshData::default();
    let mut exterior_wall_mesh = MeshData::default();
    let mut exterior_corner_mesh = MeshData::default();
    let mut exterior_t_junction_mesh = MeshData::default();

    for y in 0..grid.height {
        for x in 0..grid.width {
            let TileType::Wall(wall) = grid.get(x, y) else {
                continue;
            };
            let exterior_faces = classify::exterior_face_dirs(wall);
            for wall_box in boxes::wall_boxes(grid, x, y, wall, config) {
                faces::append_wall_box(
                    &mut wall_mesh,
                    &mut wall_top_mesh,
                    &mut exterior_wall_mesh,
                    &mut exterior_corner_mesh,
                    &mut exterior_t_junction_mesh,
                    wall_box.bounds,
                    wall_box.axis,
                    wall_box.exterior_class,
                    &exterior_faces,
                    config,
                    wall_box.cutout,
                );
            }
        }
    }

    WallMeshes {
        wall: wall_mesh,
        top: wall_top_mesh,
        exterior: exterior_wall_mesh,
        exterior_corner: exterior_corner_mesh,
        exterior_t_junction: exterior_t_junction_mesh,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::geometry::{Rect, Vec2};
    use crate::tile::{CardinalDir, WallOpening, WallShape, WallTile};

    fn exterior_wall(shape: WallShape) -> TileType {
        TileType::Wall(WallTile::exterior(shape))
    }

    #[test]
    fn test_window_tiles_cut_both_wall_faces_for_each_orientation() {
        let config = BuildingConfig {
            footprint: Rect::new(0.0, 0.0, 3.0, 3.0),
            tile_size: 1.0,
            wall_thickness: 0.25,
            window_width: 0.5,
            ..Default::default()
        };

        for (floor_x, floor_y) in [(1, 2), (2, 1)] {
            let mut plain_grid = TileGrid::new(3, 3, config.tile_size, Vec2::ZERO);
            let mut window_grid = TileGrid::new(3, 3, config.tile_size, Vec2::ZERO);
            let shape = if floor_x == 1 {
                WallShape::Straight(CardinalDir::Bottom)
            } else {
                WallShape::Straight(CardinalDir::Left)
            };

            plain_grid.set(1, 1, exterior_wall(shape));
            plain_grid.set(floor_x, floor_y, TileType::Floor);
            window_grid.set(
                1,
                1,
                exterior_wall(shape).wall().map_or(TileType::Empty, |wall| {
                    TileType::Wall(wall.with_opening(WallOpening::Window { render_glass: true }))
                }),
            );
            window_grid.set(floor_x, floor_y, TileType::Floor);

            let plain_meshes = generate_wall_meshes(&plain_grid, &config);
            let window_meshes = generate_wall_meshes(&window_grid, &config);
            let plain_vertices = wall_mesh_vertices(&plain_meshes);
            let window_vertices = wall_mesh_vertices(&window_meshes);

            assert_eq!(window_vertices - plain_vertices, 2 * 12);
        }
    }

    fn wall_mesh_vertices(meshes: &WallMeshes) -> usize {
        meshes.wall.vertices.len()
            + meshes.top.vertices.len()
            + meshes.exterior.vertices.len()
            + meshes.exterior_corner.vertices.len()
            + meshes.exterior_t_junction.vertices.len()
    }

    #[test]
    fn test_interior_wall_uses_interior_thickness() {
        let config = BuildingConfig {
            footprint: Rect::new(0.0, 0.0, 3.0, 3.0),
            tile_size: 1.0,
            wall_thickness: 0.5,
            interior_wall_thickness: 0.25,
            ..Default::default()
        };

        let mut grid = TileGrid::new(3, 3, config.tile_size, Vec2::ZERO);
        let wall = WallTile::interior(WallShape::Straight(CardinalDir::Left));
        grid.set(1, 1, TileType::Wall(wall));
        grid.set(0, 1, TileType::Floor);
        grid.set(2, 1, TileType::Floor);

        let (min, max) = wall_bounds_for_tile(&grid, 1, 1, wall, &config);
        assert_eq!(max[0] - min[0], config.interior_wall_thickness);

        let wall = WallTile::exterior(WallShape::Straight(CardinalDir::Left));
        grid.set(1, 1, TileType::Wall(wall));
        let (min, max) = wall_bounds_for_tile(&grid, 1, 1, wall, &config);
        assert_eq!(max[0] - min[0], config.wall_thickness);
    }

    #[test]
    fn test_wall_corner_uses_full_tile_bounds() {
        let config = BuildingConfig {
            footprint: Rect::new(0.0, 0.0, 1.0, 1.0),
            tile_size: 1.0,
            wall_thickness: 0.4,
            interior_wall_thickness: 0.2,
            ..Default::default()
        };
        let mut grid = TileGrid::new(1, 1, config.tile_size, Vec2::ZERO);
        let wall = WallTile::exterior(WallShape::Corner(crate::tile::CornerDir::BottomLeft));
        grid.set(0, 0, TileType::Wall(wall));

        let (min, max) = wall_bounds_for_tile(&grid, 0, 0, wall, &config);
        assert_eq!(max[0] - min[0], config.tile_size);
        assert_eq!(max[2] - min[2], config.tile_size);
    }

    #[test]
    fn test_wall_triangle_winding_matches_normals() {
        let config = BuildingConfig {
            footprint: Rect::new(0.0, 0.0, 3.0, 3.0),
            tile_size: 1.0,
            wall_thickness: 0.25,
            ..Default::default()
        };

        for shape in [
            WallShape::Straight(CardinalDir::Left),
            WallShape::Straight(CardinalDir::Right),
            WallShape::Straight(CardinalDir::Bottom),
            WallShape::Straight(CardinalDir::Top),
        ] {
            let mut grid = TileGrid::new(3, 3, config.tile_size, Vec2::ZERO);
            grid.set(1, 1, exterior_wall(shape));

            let meshes = generate_wall_meshes(&grid, &config);
            for (name, mesh) in [
                ("wall", &meshes.wall),
                ("top", &meshes.top),
                ("exterior", &meshes.exterior),
                ("corner", &meshes.exterior_corner),
                ("t_junction", &meshes.exterior_t_junction),
            ] {
                assert_mesh_winding_matches_normals(name, mesh);
            }
        }
    }

    fn assert_mesh_winding_matches_normals(name: &str, mesh: &MeshData) {
        for triangle in mesh.indices.chunks_exact(3) {
            let a_index = triangle[0] as usize;
            let b_index = triangle[1] as usize;
            let c_index = triangle[2] as usize;
            let a = mesh.vertices[a_index];
            let b = mesh.vertices[b_index];
            let c = mesh.vertices[c_index];
            let normal = mesh.normals[a_index];
            let edge_ab = super::super::math_util::sub3(b, a);
            let edge_ac = super::super::math_util::sub3(c, a);
            let winding_normal = super::super::math_util::normalize3(
                super::super::math_util::cross3(edge_ab, edge_ac),
            );
            let dot = winding_normal[0] * normal[0]
                + winding_normal[1] * normal[1]
                + winding_normal[2] * normal[2];

            assert!(
                dot > 0.99,
                "{name}: triangle winding opposes normal: dot={dot}, normal={normal:?}, winding_normal={winding_normal:?}"
            );
        }
    }
}
