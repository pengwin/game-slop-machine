//! # Building Generator
//!
//! A procedural building generation library using zone-row layout.
//!
//! ## Architecture
//!
//! The generation pipeline:
//!
//! ```text
//! BuildingConfig + Seed
//!        ↓
//!    Zone Layout (zone_layout.rs)
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
//! use building_gen::config::{BuildingConfig, RoomSpec};
//! use building_gen::tile::{CardinalDir, TileType, WallShape, WallTile};
//!
//! let config = BuildingConfig {
//!     room_specs: vec![
//!         RoomSpec::new("hall", 1),
//!         RoomSpec::new("kitchen", 2),
//!         RoomSpec::new("bedroom", 1),
//!     ],
//!     ..Default::default()
//! };
//! let layout = generate_layout(&config, 42);
//!
//! // layout.rooms contains the room list with labels
//! // layout.tile_grid contains the 2D tile representation
//! ```

pub mod config;
pub mod district;
pub mod geometry;
pub mod layout;
pub mod mesh;
pub mod opening;
pub mod random;
pub mod roof;
pub mod tile;
pub mod tile_converter;
pub mod zone_layout;

use config::BuildingConfig;
use layout::BuildingLayout;

/// Generates a complete building layout from config and seed.
///
/// This is the main entry point for building generation. It:
/// 1. Uses zone-row algorithm to create room rectangles
/// 2. Converts rooms to a tile grid with walls
/// 3. Places doorways between rooms (or to corridor)
/// 4. Places windows on exterior walls per room spec
/// 5. Generates roof geometry
///
/// The seed ensures deterministic output - same seed always produces same building.
pub fn generate_layout(config: &BuildingConfig, _seed: u64) -> BuildingLayout {
    // Step 1: Zone-row layout to create room rectangles
    let (rooms, corridor) = zone_layout::generate_rooms(config);

    // Step 2: Convert rooms to tile grid with walls
    let mut grid = tile_converter::rooms_to_tile_grid(&rooms, config);

    // Step 2b: Mark corridor floor tiles if present
    if let Some(ref corridor_info) = corridor {
        mark_corridor_floor(&mut grid, corridor_info, config);
        tile_converter::classify_wall_tiles(&mut grid);
    }

    // Step 3: Detect wall tiles for doorway/window placement
    let walls = tile_converter::detect_walls(&grid);

    // Step 4: Place openings (doors between rooms, windows per room spec)
    let doorways = opening::place_doorways(
        &mut grid,
        &rooms,
        config,
        corridor.as_ref(),
    );
    let windows = opening::place_windows(&mut grid, &rooms, config);

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
        corridor,
    }
}

/// Marks the corridor strip as enclosed floor tiles in the grid.
fn mark_corridor_floor(
    grid: &mut tile::TileGrid,
    corridor: &zone_layout::CorridorInfo,
    config: &BuildingConfig,
) {
    let bounds = corridor.bounds;
    let ts = config.tile_size;
    let origin = config.footprint.min;

    let min_x = ((bounds.min.x - origin.x) / ts).round().max(0.0) as usize;
    let min_y = ((bounds.min.y - origin.y) / ts).round().max(0.0) as usize;
    let max_x = ((bounds.max.x - origin.x) / ts).round().max(0.0) as usize;
    let max_y = ((bounds.max.y - origin.y) / ts).round().max(0.0) as usize;

    let max_x = max_x.min(grid.width.saturating_sub(1));
    let max_y = max_y.min(grid.height.saturating_sub(1));

    for y in min_y..=max_y {
        for x in min_x..=max_x {
            let on_perimeter = x == min_x || x == max_x || y == min_y || y == max_y;
            if on_perimeter {
                if grid.get(x, y) == tile::TileType::Empty {
                    grid.set(x, y, corridor_raw_wall());
                }
            } else if grid.get(x, y) == tile::TileType::Empty {
                grid.set(x, y, tile::TileType::Floor);
            }
        }
    }
}

fn corridor_raw_wall() -> tile::TileType {
    tile::TileType::Wall(tile::WallTile::exterior(tile::WallShape::Straight(
        tile::CardinalDir::Top,
    )))
}

/// Gets the scale transform for a tile type.
pub fn tile_scale(tile_type: tile::TileType, config: &BuildingConfig) -> (f32, f32, f32) {
    mesh::tile_scale(tile_type, config)
}

/// Gets the color hint for a tile type.
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

/// Generates a complete merged building mesh from the tile grid.
pub fn generate_building_mesh(
    grid: &tile::TileGrid,
    config: &config::BuildingConfig,
    roof: &layout::RoofGeometry,
) -> mesh::BuildingMesh {
    mesh::generate_building_mesh(grid, config, roof)
}

#[cfg(test)]
mod tests {
    use super::*;
    use config::RoomSpec;
    use geometry::Rect;
    use tile::{CardinalDir, TileType, WallShape, WallTile};

    fn test_config() -> BuildingConfig {
        BuildingConfig {
            footprint: Rect::new(0.0, 0.0, 10.0, 8.0),
            tile_size: 0.5,
            min_room_size: 2.5,
            room_specs: vec![
                RoomSpec::new("hall", 1),
                RoomSpec::new("kitchen", 2),
                RoomSpec::new("bedroom", 1),
                RoomSpec::new("bathroom", 0),
            ],
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
        let wall_count = layout.tile_grid.count_matching_tiles(TileType::is_wall);
        assert!(wall_count > 0);
    }

    #[test]
    fn test_generate_layout_has_doorways() {
        let config = test_config();
        let layout = generate_layout(&config, 42);
        assert!(!layout.doorways.is_empty(), "No doorways generated");
    }

    #[test]
    fn test_generate_layout_connected() {
        let config = test_config();
        let layout = generate_layout(&config, 42);
        assert!(layout.is_connected(), "Building layout is not connected");
    }

    #[test]
    fn test_room_labels_preserved() {
        let config = test_config();
        let layout = generate_layout(&config, 42);
        assert_eq!(layout.rooms[0].label, "hall");
        assert_eq!(layout.rooms[1].label, "kitchen");
        assert_eq!(layout.rooms[2].label, "bathroom");
        assert_eq!(layout.rooms[3].label, "bedroom");
    }

    #[test]
    fn test_different_seeds_same_layout() {
        let config = test_config();
        let layout1 = generate_layout(&config, 42);
        let layout2 = generate_layout(&config, 43);

        // Zone-row is deterministic from config, not seed
        // (rooms are the same, only window/door randomness differs)
        assert_eq!(layout1.rooms.len(), layout2.rooms.len());
        for (r1, r2) in layout1.rooms.iter().zip(layout2.rooms.iter()) {
            assert_eq!(r1.bounds, r2.bounds);
            assert_eq!(r1.label, r2.label);
        }
    }

    #[test]
    fn test_tile_scale_and_color() {
        let config = test_config();
        let wall = TileType::Wall(WallTile::exterior(WallShape::Straight(CardinalDir::Top)));
        let (x, y, z) = tile_scale(wall, &config);
        assert_eq!(x, config.tile_size);
        assert_eq!(y, config.wall_height);
        assert_eq!(z, config.wall_thickness);

        let (r, g, b) = tile_color(wall);
        assert!(r > 0.0);
        assert!(g > 0.0);
        assert!(b > 0.0);
    }
}
