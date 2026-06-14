use crate::config::BuildingConfig;
use crate::tile::{CardinalDir, TileType, WallOpening, WallShape};

/// A unit cube mesh (1x1x1) centered at origin.
///
/// This is the base for all wall tiles. It gets scaled by the
/// tile's dimensions (tile_size, wall_height, wall_thickness).
pub fn unit_cube() -> Vec<[f32; 3]> {
    vec![
        // Front face
        [-0.5, 0.0, -0.5],
        [0.5, 0.0, -0.5],
        [0.5, 1.0, -0.5],
        [-0.5, 1.0, -0.5],
        // Back face
        [-0.5, 0.0, 0.5],
        [0.5, 0.0, 0.5],
        [0.5, 1.0, 0.5],
        [-0.5, 1.0, 0.5],
        // Top face
        [-0.5, 1.0, -0.5],
        [0.5, 1.0, -0.5],
        [0.5, 1.0, 0.5],
        [-0.5, 1.0, 0.5],
        // Bottom face
        [-0.5, 0.0, -0.5],
        [0.5, 0.0, -0.5],
        [0.5, 0.0, 0.5],
        [-0.5, 0.0, 0.5],
        // Left face
        [-0.5, 0.0, -0.5],
        [-0.5, 1.0, -0.5],
        [-0.5, 1.0, 0.5],
        [-0.5, 0.0, 0.5],
        // Right face
        [0.5, 0.0, -0.5],
        [0.5, 1.0, -0.5],
        [0.5, 1.0, 0.5],
        [0.5, 0.0, 0.5],
    ]
}

/// Normals for the unit cube (4 per face, 6 faces).
pub fn unit_cube_normals() -> Vec<[f32; 3]> {
    vec![
        [0.0, 0.0, -1.0],
        [0.0, 0.0, -1.0],
        [0.0, 0.0, -1.0],
        [0.0, 0.0, -1.0],
        [0.0, 0.0, 1.0],
        [0.0, 0.0, 1.0],
        [0.0, 0.0, 1.0],
        [0.0, 0.0, 1.0],
        [0.0, 1.0, 0.0],
        [0.0, 1.0, 0.0],
        [0.0, 1.0, 0.0],
        [0.0, 1.0, 0.0],
        [0.0, -1.0, 0.0],
        [0.0, -1.0, 0.0],
        [0.0, -1.0, 0.0],
        [0.0, -1.0, 0.0],
        [-1.0, 0.0, 0.0],
        [-1.0, 0.0, 0.0],
        [-1.0, 0.0, 0.0],
        [-1.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
    ]
}

/// UV coordinates for the unit cube.
pub fn unit_cube_uvs() -> Vec<[f32; 2]> {
    vec![
        [0.0, 0.0],
        [1.0, 0.0],
        [1.0, 1.0],
        [0.0, 1.0],
        [0.0, 0.0],
        [1.0, 0.0],
        [1.0, 1.0],
        [0.0, 1.0],
        [0.0, 0.0],
        [1.0, 0.0],
        [1.0, 1.0],
        [0.0, 1.0],
        [0.0, 0.0],
        [1.0, 0.0],
        [1.0, 1.0],
        [0.0, 1.0],
        [0.0, 0.0],
        [1.0, 0.0],
        [1.0, 1.0],
        [0.0, 1.0],
        [0.0, 0.0],
        [1.0, 0.0],
        [1.0, 1.0],
        [0.0, 1.0],
    ]
}

/// Indices for the unit cube (2 triangles per face, 6 faces).
pub fn unit_cube_indices() -> Vec<u32> {
    vec![
        0, 2, 1, 0, 3, 2, 4, 5, 6, 4, 6, 7, 8, 9, 10, 8, 10, 11, 12, 14, 13, 12, 15, 14, 16,
        17, 18, 16, 18, 19, 20, 22, 21, 20, 23, 22,
    ]
}

/// A floor quad (1x1 at y=0).
pub fn floor_quad() -> Vec<[f32; 3]> {
    vec![
        [-0.5, 0.0, -0.5],
        [0.5, 0.0, -0.5],
        [0.5, 0.0, 0.5],
        [-0.5, 0.0, 0.5],
    ]
}

/// Normals for the floor quad (all pointing up).
pub fn floor_quad_normals() -> Vec<[f32; 3]> {
    vec![
        [0.0, 1.0, 0.0],
        [0.0, 1.0, 0.0],
        [0.0, 1.0, 0.0],
        [0.0, 1.0, 0.0],
    ]
}

/// UVs for the floor quad.
pub fn floor_quad_uvs() -> Vec<[f32; 2]> {
    vec![[0.0, 0.0], [1.0, 0.0], [1.0, 1.0], [0.0, 1.0]]
}

/// Indices for the floor quad.
pub fn floor_quad_indices() -> Vec<u32> {
    vec![0, 1, 2, 0, 2, 3]
}

/// Gets the scale transform for a tile type.
///
/// Returns (x, y, z) scale factors to apply to the unit cube.
/// The unit cube is 1x1x1, so scaling by these values gives the
/// correct dimensions for each tile type.
pub fn tile_scale(tile_type: TileType, config: &BuildingConfig) -> (f32, f32, f32) {
    let s = config.tile_size;
    let h = config.wall_height;
    let t = config.wall_thickness;

    match tile_type {
        TileType::Wall(wall) => match wall.shape {
            WallShape::Straight(CardinalDir::Left | CardinalDir::Right) => (t, h, s),
            WallShape::Straight(CardinalDir::Bottom | CardinalDir::Top) => (s, h, t),
            WallShape::Corner(_) | WallShape::TJunction(_) | WallShape::Cross => (s, h, s),
        },
        // Floor: full tile width, small height for visibility, full tile depth
        TileType::Floor => (s, 0.1, s),
        // Empty: no scale (shouldn't be rendered)
        TileType::Empty => (0.0, 0.0, 0.0),
    }
}

/// Gets the color hint for a tile type.
///
/// Returns (r, g, b) color values for visual distinction.
/// The actual rendering should use materials, but this helps
/// with debugging and ASCII visualization.
pub fn tile_color(tile_type: TileType) -> (f32, f32, f32) {
    match tile_type {
        TileType::Floor => (0.6, 0.6, 0.6),
        TileType::Wall(wall) => match wall.opening {
            Some(WallOpening::Door { .. } | WallOpening::Doorway) => (0.4, 0.2, 0.0),
            Some(WallOpening::Window { .. }) => (0.5, 0.7, 1.0),
            None => (0.8, 0.8, 0.8),
        },
        TileType::Empty => (0.0, 0.0, 0.0),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tile::WallTile;

    fn exterior_wall(shape: WallShape) -> TileType {
        TileType::Wall(WallTile::exterior(shape))
    }

    #[test]
    fn test_unit_cube_vertices() {
        let vertices = unit_cube();
        assert_eq!(vertices.len(), 24);
    }

    #[test]
    fn test_unit_cube_indices() {
        let indices = unit_cube_indices();
        assert_eq!(indices.len(), 36);
    }

    #[test]
    fn test_floor_quad_vertices() {
        let vertices = floor_quad();
        assert_eq!(vertices.len(), 4);
    }

    #[test]
    fn test_tile_scale_wall() {
        let config = BuildingConfig::default();
        let (x, y, z) = tile_scale(
            exterior_wall(WallShape::Straight(CardinalDir::Top)),
            &config,
        );
        assert_eq!(x, config.tile_size);
        assert_eq!(y, config.wall_height);
        assert_eq!(z, config.wall_thickness);
    }

    #[test]
    fn test_tile_scale_floor() {
        let config = BuildingConfig::default();
        let (x, y, z) = tile_scale(TileType::Floor, &config);
        assert_eq!(x, config.tile_size);
        assert!(y > 0.0);
        assert_eq!(z, config.tile_size);
    }
}
