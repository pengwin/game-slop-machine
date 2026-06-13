//! # Building Generator
//!
//! A procedural building generation library using Binary Space Partitioning (BSP).
//!
//! ## Architecture
//!
//! The generation pipeline:
//!
//! ```text
//! BuildingConfig + Seed
//!        ↓
//!    BSP Algorithm (bsp.rs)
//!        ↓
//!  Room Layout (Vec<Room>)
//!        ↓
//!  Tile Grid (tile_converter.rs)
//!        ↓
//!  Wall Detection (tile_converter.rs)
//!        ↓
//!  Door/Window Placement (opening.rs)
//!        ↓
//!  Roof Geometry (roof.rs)
//!        ↓
//!  3D Mesh Data (mesh.rs)
//! ```
//!
//! ## Usage
//!
//! ```rust
//! use building_gen::{generate_layout, tile_scale, tile_color};
//! use building_gen::config::BuildingConfig;
//! use building_gen::tile::TileType;
//!
//! let config = BuildingConfig::default();
//! let layout = generate_layout(&config, 42); // seed = 42
//!
//! // Get scale and color for a wall tile
//! let (sx, sy, sz) = tile_scale(TileType::Wall, &config);
//! let (r, g, b) = tile_color(TileType::Wall);
//!
//! // layout.rooms contains the room list
//! // layout.tile_grid contains the 2D tile representation
//! ```

pub mod bsp;
pub mod config;
pub mod geometry;
pub mod layout;
pub mod mesh;
pub mod opening;
pub mod random;
pub mod roof;
pub mod tile;
pub mod tile_converter;

use config::BuildingConfig;
use layout::BuildingLayout;
use random::SeededRng;

/// Generates a complete building layout from config and seed.
///
/// This is the main entry point for building generation. It:
/// 1. Uses BSP to create room rectangles
/// 2. Converts rooms to a tile grid with walls
/// 3. Places doorways between adjacent rooms
/// 4. Places windows on exterior walls
/// 5. Generates roof geometry
///
/// The seed ensures deterministic output - same seed always produces same building.
pub fn generate_layout(config: &BuildingConfig, seed: u64) -> BuildingLayout {
    let mut rng = SeededRng::new(seed);

    // Step 1: BSP subdivision to create room rectangles
    let tree = bsp::bsp_subdivide(config, &mut rng);
    let rooms = bsp::collect_rooms(&tree);

    // Step 2: Convert rooms to tile grid with walls
    let mut grid = tile_converter::rooms_to_tile_grid(&rooms, config);

    // Step 3: Detect wall tiles for doorway/window placement
    let walls = tile_converter::detect_walls(&grid);

    // Step 4: Place openings (doors between rooms, windows on exterior)
    let doorways = opening::place_doorways(&walls, &mut grid, &rooms, &mut rng, config);
    let windows = opening::place_windows(&walls, &mut grid, &mut rng, config);

    // Step 5: Generate roof geometry
    let roof = roof::generate_roof(config.footprint, config);

    BuildingLayout {
        rooms,
        walls,
        doorways,
        windows,
        tile_grid: grid,
        roof,
        bounds: config.footprint,
    }
}

/// Gets the scale transform for a tile type.
///
/// Returns (x, y, z) scale factors to apply to the unit cube.
pub fn tile_scale(tile_type: tile::TileType, config: &BuildingConfig) -> (f32, f32, f32) {
    mesh::tile_scale(tile_type, config)
}

/// Gets the color hint for a tile type.
///
/// Returns (r, g, b) color values for visual distinction.
pub fn tile_color(tile_type: tile::TileType) -> (f32, f32, f32) {
    mesh::tile_color(tile_type)
}

/// Gets the unit cube mesh vertices.
pub fn unit_cube_vertices() -> Vec<[f32; 3]> {
    mesh::unit_cube()
}

/// Gets the unit cube mesh normals.
pub fn unit_cube_normals() -> Vec<[f32; 3]> {
    mesh::unit_cube_normals()
}

/// Gets the unit cube mesh UVs.
pub fn unit_cube_uvs() -> Vec<[f32; 2]> {
    mesh::unit_cube_uvs()
}

/// Gets the unit cube mesh indices.
pub fn unit_cube_indices() -> Vec<u32> {
    mesh::unit_cube_indices()
}

/// Gets the floor quad mesh vertices.
pub fn floor_quad_vertices() -> Vec<[f32; 3]> {
    mesh::floor_quad()
}

/// Gets the floor quad mesh normals.
pub fn floor_quad_normals() -> Vec<[f32; 3]> {
    mesh::floor_quad_normals()
}

/// Gets the floor quad mesh UVs.
pub fn floor_quad_uvs() -> Vec<[f32; 2]> {
    mesh::floor_quad_uvs()
}

/// Gets the floor quad mesh indices.
pub fn floor_quad_indices() -> Vec<u32> {
    mesh::floor_quad_indices()
}

#[cfg(test)]
mod tests {
    use super::*;
    use geometry::Rect;
    use tile::TileType;

    fn test_config() -> BuildingConfig {
        BuildingConfig {
            footprint: Rect::new(0.0, 0.0, 10.0, 8.0),
            tile_size: 0.5,
            min_room_size: 2.5,
            target_rooms: 4,
            ..Default::default()
        }
    }

    #[test]
    fn test_generate_layout_returns_rooms() {
        let config = test_config();
        let layout = generate_layout(&config, 42);
        assert!(!layout.rooms.is_empty());
    }

    #[test]
    fn test_generate_layout_deterministic() {
        let config = test_config();
        let layout1 = generate_layout(&config, 42);
        let layout2 = generate_layout(&config, 42);

        assert_eq!(layout1.rooms.len(), layout2.rooms.len());
        assert_eq!(layout1.walls.len(), layout2.walls.len());
        assert_eq!(layout1.doorways.len(), layout2.doorways.len());
    }

    #[test]
    fn test_generate_layout_has_floor_tiles() {
        let config = test_config();
        let layout = generate_layout(&config, 42);
        assert!(layout.tile_grid.count_tiles(TileType::Floor) > 0);
    }

    #[test]
    fn test_generate_layout_has_walls() {
        let config = test_config();
        let layout = generate_layout(&config, 42);
        let wall_count = layout.tile_grid.count_tiles(TileType::Wall)
            + layout.tile_grid.count_tiles(TileType::WallCorner);
        assert!(wall_count > 0);
    }

    #[test]
    fn test_generate_layout_has_doorways() {
        let config = test_config();
        let layout = generate_layout(&config, 42);
        assert!(
            !layout.doorways.is_empty(),
            "No doorways generated"
        );
    }

    #[test]
    fn test_generate_layout_connected() {
        let config = test_config();
        let layout = generate_layout(&config, 42);
        assert!(layout.is_connected(), "Building layout is not connected");
    }

    #[test]
    fn test_different_seeds_different_layouts() {
        let config = test_config();
        let layout1 = generate_layout(&config, 42);
        let layout2 = generate_layout(&config, 43);

        let rooms_different = layout1.rooms.len() != layout2.rooms.len()
            || layout1
                .rooms
                .iter()
                .zip(layout2.rooms.iter())
                .any(|(a, b)| a.bounds != b.bounds);

        assert!(rooms_different, "Different seeds should produce different layouts");
    }

    #[test]
    fn test_tile_scale_and_color() {
        let config = test_config();
        let (x, y, z) = tile_scale(TileType::Wall, &config);
        assert_eq!(x, config.tile_size);
        assert_eq!(y, config.wall_height);
        assert_eq!(z, config.wall_thickness);

        let (r, g, b) = tile_color(TileType::Wall);
        assert!(r > 0.0);
        assert!(g > 0.0);
        assert!(b > 0.0);
    }
}
