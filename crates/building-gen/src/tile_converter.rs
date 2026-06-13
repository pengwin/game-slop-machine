//! Converts BSP room layout into a tile grid with walls, floors, and openings.
//!
//! The conversion process:
//! 1. Flood each room's interior with Floor tiles (offset inward from room bounds)
//! 2. Mark empty tiles adjacent to Floor as Wall tiles (boundary detection)
//! 3. Identify corner tiles where diagonal floor exists but no adjacent floor
//!
//! Design decisions:
//! - Wall offset (0.5 * tile_size) creates a 1-tile gap between adjacent rooms' floors
//! - This gap becomes the shared wall between rooms
//! - At room boundaries, two wall tiles may appear (one from each room) - this is expected
//! - Walls are detected by checking 4-connected neighbors, not 8-connected

use crate::config::BuildingConfig;
use crate::geometry::Vec2;
use crate::layout::{Room, Wall, WallType};
use crate::tile::{TileGrid, TileType};

/// Converts BSP rooms into a tile grid with walls and floors.
///
/// The wall_offset shrinks each room inward by half a tile, creating space
/// for walls at the boundaries. Adjacent rooms will have their walls meet
/// at the shared boundary, forming a single wall thickness visually.
pub fn rooms_to_tile_grid(rooms: &[Room], config: &BuildingConfig) -> TileGrid {
    let tiles_x = config.tiles_x();
    let tiles_y = config.tiles_y();
    let mut grid = TileGrid::new(tiles_x, tiles_y, config.tile_size, config.footprint.min);

    // Shrink each room inward by half a tile to leave space for walls
    let wall_offset = config.tile_size * 0.5;

    for room in rooms {
        flood_room_with_offset(&mut grid, room, config, wall_offset);
    }

    // Place walls at the boundary between floor and empty tiles
    mark_boundary_walls(&mut grid);

    grid
}

/// Fills a room's interior with Floor tiles, offset inward from the room bounds.
///
/// The offset ensures walls can be placed at the room boundary without
/// overlapping the floor. The room bounds are in world coordinates,
/// converted to tile coordinates with the offset applied.
fn flood_room_with_offset(grid: &mut TileGrid, room: &Room, config: &BuildingConfig, offset: f32) {
    // Convert world coordinates to tile indices, applying the inward offset
    let min_x = ((room.bounds.min.x - config.footprint.min.x + offset) / config.tile_size).ceil() as usize;
    let min_y = ((room.bounds.min.y - config.footprint.min.y + offset) / config.tile_size).ceil() as usize;
    let max_x = ((room.bounds.max.x - config.footprint.min.x - offset) / config.tile_size).floor() as usize;
    let max_y = ((room.bounds.max.y - config.footprint.min.y - offset) / config.tile_size).floor() as usize;

    // Clamp to grid bounds to prevent out-of-bounds access
    let min_x = min_x.min(grid.width);
    let min_y = min_y.min(grid.height);
    let max_x = max_x.min(grid.width).max(min_x);
    let max_y = max_y.min(grid.height).max(min_y);

    // Fill the room interior with floor tiles
    for y in min_y..max_y {
        for x in min_x..max_x {
            grid.set(x, y, TileType::Floor);
        }
    }
}

/// Marks empty tiles adjacent to Floor tiles as Wall tiles.
///
/// This uses a two-pass approach:
/// 1. Collect all wall positions (empty tiles next to floor)
/// 2. Mark those positions as walls
///
/// The deduplication step ensures we don't process the same tile twice
/// when multiple floor tiles share the same empty neighbor.
fn mark_boundary_walls(grid: &mut TileGrid) {
    let width = grid.width;
    let height = grid.height;
    let mut wall_positions = Vec::new();

    // Pass 1: Find all empty tiles adjacent to floor tiles (4-connected)
    for y in 0..height {
        for x in 0..width {
            if grid.get(x, y) == TileType::Floor {
                let neighbors = grid.neighbors(x, y);
                for (nx, ny, tile) in neighbors {
                    if tile == TileType::Empty {
                        wall_positions.push((nx, ny));
                    }
                }
            }
        }
    }

    // Deduplicate to avoid processing the same tile multiple times
    wall_positions.sort();
    wall_positions.dedup();

    // Pass 2: Mark wall positions
    for (x, y) in wall_positions {
        if grid.get(x, y) == TileType::Empty {
            grid.set(x, y, TileType::Wall);
        }
    }

    // Pass 3: Identify and mark corner tiles
    // A corner is a wall tile that has floor diagonally but not adjacently
    for y in 0..height {
        for x in 0..width {
            if grid.get(x, y) == TileType::Wall && is_corner(grid, x, y) {
                grid.set(x, y, TileType::WallCorner);
            }
        }
    }
}

/// Determines if a wall tile is a corner piece.
///
/// A corner is defined as a wall tile where:
/// - At least one diagonal neighbor is Floor
/// - No adjacent (4-connected) neighbor is Floor
///
/// This identifies walls at building corners where two walls meet at 90 degrees.
fn is_corner(grid: &TileGrid, x: usize, y: usize) -> bool {
    // Check diagonal neighbors for floor
    let has_floor_diag = (x > 0 && y > 0 && grid.get(x - 1, y - 1) == TileType::Floor)
        || (x > 0 && y < grid.height - 1 && grid.get(x - 1, y + 1) == TileType::Floor)
        || (x < grid.width - 1 && y > 0 && grid.get(x + 1, y - 1) == TileType::Floor)
        || (x < grid.width - 1 && y < grid.height - 1 && grid.get(x + 1, y + 1) == TileType::Floor);

    // Check adjacent neighbors for floor
    let has_floor_adj = (x > 0 && grid.get(x - 1, y) == TileType::Floor)
        || (x < grid.width - 1 && grid.get(x + 1, y) == TileType::Floor)
        || (y > 0 && grid.get(x, y - 1) == TileType::Floor)
        || (y < grid.height - 1 && grid.get(x, y + 1) == TileType::Floor);

    has_floor_diag && !has_floor_adj
}

/// Converts wall tiles into Wall structs for the building layout.
///
/// Each wall tile becomes a Wall with:
/// - A horizontal line segment (for mesh generation)
/// - Classification as Interior or Exterior based on adjacent floor count
pub fn detect_walls(grid: &TileGrid) -> Vec<Wall> {
    let mut walls = Vec::new();
    let mut wall_id = 0u32;

    for y in 0..grid.height {
        for x in 0..grid.width {
            let tile = grid.get(x, y);
            if tile == TileType::Wall || tile == TileType::WallCorner {
                let wall_type = classify_wall(grid, x, y);
                let world_pos = grid.world_pos(x, y);
                let half = grid.tile_size / 2.0;

                // Create a horizontal line segment for this wall tile
                let segment = crate::geometry::LineSegment2D::new(
                    Vec2::new(world_pos.x - half, world_pos.y),
                    Vec2::new(world_pos.x + half, world_pos.y),
                );

                walls.push(Wall::new(wall_id, segment, wall_type));
                wall_id += 1;
            }
        }
    }

    walls
}

/// Classifies a wall as Interior or Exterior based on its neighbors.
///
/// - Exterior: At grid edge, or adjacent to only 1 floor tile
/// - Interior: Adjacent to 2+ floor tiles (shared between rooms)
///
/// Note: At room boundaries, walls may be classified as Interior even if
/// they're on the building's outer edge, because they're adjacent to
/// floor tiles from two different rooms.
fn classify_wall(grid: &TileGrid, x: usize, y: usize) -> WallType {
    // Walls at the grid edge are always exterior
    if x == 0 || y == 0 || x == grid.width - 1 || y == grid.height - 1 {
        return WallType::Exterior;
    }

    let neighbors = grid.neighbors(x, y);
    let floor_count = neighbors
        .iter()
        .filter(|(_, _, t)| *t == TileType::Floor)
        .count();

    // Walls adjacent to 2+ floor tiles are interior (shared between rooms)
    if floor_count >= 2 {
        WallType::Interior
    } else {
        WallType::Exterior
    }
}

/// Finds pairs of room indices that are adjacent (share a wall).
///
/// Two rooms are adjacent if their tile coordinate ranges touch along one axis
/// and overlap along the other. This is used to determine where to place
/// doorways between rooms.
pub fn find_adjacent_rooms(grid: &TileGrid, rooms: &[Room]) -> Vec<(usize, usize)> {
    let mut pairs = Vec::new();

    for i in 0..rooms.len() {
        for j in (i + 1)..rooms.len() {
            if rooms_are_adjacent(grid, &rooms[i], &rooms[j]) {
                pairs.push((i, j));
            }
        }
    }

    pairs
}

/// Checks if two rooms are adjacent by comparing their tile coordinate ranges.
///
/// Two rooms are adjacent if:
/// - Their X ranges overlap AND their Y ranges touch (horizontal neighbors)
/// - Their Y ranges overlap AND their X ranges touch (vertical neighbors)
fn rooms_are_adjacent(grid: &TileGrid, room_a: &Room, room_b: &Room) -> bool {
    // Convert room bounds to tile coordinates
    let min_x_a = ((room_a.bounds.min.x) / grid.tile_size).floor() as i32;
    let min_y_a = ((room_a.bounds.min.y) / grid.tile_size).floor() as i32;
    let max_x_a = ((room_a.bounds.max.x) / grid.tile_size).ceil() as i32;
    let max_y_a = ((room_a.bounds.max.y) / grid.tile_size).ceil() as i32;

    let min_x_b = ((room_b.bounds.min.x) / grid.tile_size).floor() as i32;
    let min_y_b = ((room_b.bounds.min.y) / grid.tile_size).floor() as i32;
    let max_x_b = ((room_b.bounds.max.x) / grid.tile_size).ceil() as i32;
    let max_y_b = ((room_b.bounds.max.y) / grid.tile_size).ceil() as i32;

    // Check for overlap on one axis and touching on the other
    let overlap_x = min_x_a < max_x_b && max_x_a > min_x_b;
    let overlap_y = min_y_a < max_y_b && max_y_a > min_y_b;

    // Horizontal neighbors: X overlaps, Y touches
    if overlap_x && (max_y_a == min_y_b || max_y_b == min_y_a) {
        return true;
    }

    // Vertical neighbors: Y overlaps, X touches
    if overlap_y && (max_x_a == min_x_b || max_x_b == min_x_a) {
        return true;
    }

    false
}

/// Finds wall tiles that are between two adjacent rooms (candidate doorway positions).
///
/// A wall tile is "between" two rooms if it's near both rooms' tile coordinate ranges.
/// The "near" check uses a 1-tile margin to account for walls being at the boundary.
///
/// These positions are used to place doorways connecting adjacent rooms.
pub fn find_doorway_positions_between_rooms(
    grid: &TileGrid,
    room_a: &Room,
    room_b: &Room,
) -> Vec<(usize, usize)> {
    let mut positions = Vec::new();

    // Convert room bounds to tile coordinates
    let min_x_a = ((room_a.bounds.min.x) / grid.tile_size).floor() as i32;
    let min_y_a = ((room_a.bounds.min.y) / grid.tile_size).floor() as i32;
    let max_x_a = ((room_a.bounds.max.x) / grid.tile_size).ceil() as i32;
    let max_y_a = ((room_a.bounds.max.y) / grid.tile_size).ceil() as i32;

    let min_x_b = ((room_b.bounds.min.x) / grid.tile_size).floor() as i32;
    let min_y_b = ((room_b.bounds.min.y) / grid.tile_size).floor() as i32;
    let max_x_b = ((room_b.bounds.max.x) / grid.tile_size).ceil() as i32;
    let max_y_b = ((room_b.bounds.max.y) / grid.tile_size).ceil() as i32;

    // Find wall tiles that are near both rooms
    for y in 0..grid.height as i32 {
        for x in 0..grid.width as i32 {
            let tile = grid.get(x as usize, y as usize);
            if tile != TileType::Wall && tile != TileType::WallCorner {
                continue;
            }

            // Check if this wall tile is near both rooms (with 1-tile margin)
            let near_a = x >= min_x_a - 1 && x <= max_x_a && y >= min_y_a - 1 && y <= max_y_a;
            let near_b = x >= min_x_b - 1 && x <= max_x_b && y >= min_y_b - 1 && y <= max_y_b;

            if near_a && near_b {
                positions.push((x as usize, y as usize));
            }
        }
    }

    positions
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bsp::{bsp_subdivide, collect_rooms};
    use crate::geometry::Rect;
    use crate::random::SeededRng;

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
    fn test_rooms_flood_to_grid() {
        let config = test_config();
        let mut rng = SeededRng::new(42);
        let tree = bsp_subdivide(&config, &mut rng);
        let rooms = collect_rooms(&tree);
        let grid = rooms_to_tile_grid(&rooms, &config);

        assert!(grid.count_tiles(TileType::Floor) > 0);
    }

    #[test]
    fn test_walls_detected() {
        let config = test_config();
        let mut rng = SeededRng::new(42);
        let tree = bsp_subdivide(&config, &mut rng);
        let rooms = collect_rooms(&tree);
        let grid = rooms_to_tile_grid(&rooms, &config);

        let wall_count = grid.count_tiles(TileType::Wall) + grid.count_tiles(TileType::WallCorner);
        assert!(wall_count > 0, "No walls detected");
    }

    #[test]
    fn test_grid_dimensions() {
        let config = test_config();
        let mut rng = SeededRng::new(42);
        let tree = bsp_subdivide(&config, &mut rng);
        let rooms = collect_rooms(&tree);
        let grid = rooms_to_tile_grid(&rooms, &config);

        assert_eq!(grid.width, 20);
        assert_eq!(grid.height, 16);
    }

    #[test]
    fn test_adjacent_rooms_detected() {
        let config = test_config();
        let mut rng = SeededRng::new(42);
        let tree = bsp_subdivide(&config, &mut rng);
        let rooms = collect_rooms(&tree);
        let grid = rooms_to_tile_grid(&rooms, &config);

        let pairs = find_adjacent_rooms(&grid, &rooms);
        assert!(!pairs.is_empty(), "No adjacent rooms found");
    }
}
