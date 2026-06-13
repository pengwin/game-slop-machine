//! Door and window placement for buildings.
//!
//! Door placement strategy:
//! 1. Find adjacent rooms (rooms that share a wall boundary)
//! 2. For each pair, find wall tiles between them
//! 3. Place a doorway at the middle of those wall tiles
//! 4. Also place one exterior door on a random exterior wall
//!
//! Window placement strategy:
//! 1. Only on exterior walls (not between rooms)
//! 2. 30% chance per wall tile (randomized)
//! 3. Skip walls that already have doors nearby
//! 4. Windows occupy multiple tiles based on window_width

use crate::config::BuildingConfig;
use crate::layout::{Doorway, Wall, WallType, Window, Room, WallId};
use crate::random::SeededRng;
use crate::tile::{TileGrid, TileType};
use crate::tile_converter::{find_adjacent_rooms, find_doorway_positions_between_rooms};

/// Places doorways between adjacent rooms and one exterior door.
///
/// For each pair of adjacent rooms:
/// 1. Find wall tiles that are between both rooms
/// 2. Place a doorway at the middle position
///
/// Also places one exterior door on a random exterior wall.
pub fn place_doorways(
    walls: &[Wall],
    grid: &mut TileGrid,
    rooms: &[Room],
    rng: &mut SeededRng,
    config: &BuildingConfig,
) -> Vec<Doorway> {
    let mut doorways = Vec::new();

    // Find all pairs of adjacent rooms
    let adjacent_pairs = find_adjacent_rooms(grid, rooms);

    // Place doorways between adjacent rooms
    for (i, j) in adjacent_pairs {
        let positions = find_doorway_positions_between_rooms(grid, &rooms[i], &rooms[j]);

        if !positions.is_empty() {
            // Place doorway at the middle of the shared wall
            let mid = positions.len() / 2;
            let (wx, wy) = positions[mid];

            // Mark the tile as a doorway
            grid.set(wx, wy, TileType::Doorway);

            let pos = grid.world_pos(wx, wy);
            doorways.push(Doorway {
                wall_id: WallId(0),
                position: pos,
                width: config.door_width,
                height: config.door_height,
            });
        }
    }

    // Place one exterior door on a random exterior wall
    let exterior_walls: Vec<_> = walls
        .iter()
        .filter(|w| w.wall_type == WallType::Exterior)
        .collect();

    if !exterior_walls.is_empty() {
        let wall = &exterior_walls[rng.gen_range_usize(0, exterior_walls.len())];
        let mid = wall.segment.midpoint();

        if let Some((x, y)) = grid.tile_coord(mid) {
            let tile = grid.get(x, y);
            if tile == TileType::Wall || tile == TileType::WallCorner {
                // Mark the tile as a door
                grid.set(x, y, TileType::Door);

                doorways.push(Doorway {
                    wall_id: wall.id,
                    position: mid,
                    width: config.door_width,
                    height: config.door_height,
                });
            }
        }
    }

    doorways
}

/// Places windows on exterior walls.
///
/// Strategy:
/// - Only on exterior walls (not between rooms)
/// - 30% chance per wall tile (randomized)
/// - Skip walls that already have doors nearby (within 1 tile)
/// - Windows can span multiple tiles based on window_width config
pub fn place_windows(
    walls: &[Wall],
    grid: &mut TileGrid,
    rng: &mut SeededRng,
    config: &BuildingConfig,
) -> Vec<Window> {
    let mut windows = Vec::new();
    let window_tiles = ((config.window_width / config.tile_size).ceil() as usize).max(1);

    for wall in walls {
        // Only place windows on exterior walls
        if wall.wall_type != WallType::Exterior {
            continue;
        }

        let mid = wall.segment.midpoint();
        if let Some((x, y)) = grid.tile_coord(mid) {
            let tile = grid.get(x, y);
            if tile == TileType::Wall || tile == TileType::WallCorner {
                // Check if there's a door nearby (within 1 tile)
                let has_door = grid
                    .neighbors(x, y)
                    .iter()
                    .any(|(_, _, t)| *t == TileType::Door || *t == TileType::Doorway);

                // 30% chance to place window, skip if door nearby
                if !has_door && rng.gen_bool(0.3) {
                    // Place window tiles (may span multiple tiles)
                    for dx in 0..window_tiles {
                        let nx = x + dx;
                        if nx < grid.width {
                            let t = grid.get(nx, y);
                            if t == TileType::Wall || t == TileType::WallCorner {
                                grid.set(nx, y, TileType::Window);
                            }
                        }
                    }

                    windows.push(Window {
                        wall_id: wall.id,
                        position: mid,
                        width: config.window_width,
                        height: config.window_height,
                        sill_height: config.window_sill_height,
                    });
                }
            }
        }
    }

    windows
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bsp::{bsp_subdivide, collect_rooms};
    use crate::geometry::Rect;
    use crate::tile_converter::rooms_to_tile_grid;

    fn test_config() -> BuildingConfig {
        BuildingConfig {
            footprint: Rect::new(0.0, 0.0, 10.0, 8.0),
            tile_size: 0.5,
            min_room_size: 2.5,
            target_rooms: 4,
            door_width: 0.9,
            window_width: 1.0,
            window_spacing: 1.5,
            ..Default::default()
        }
    }

    #[test]
    fn test_doorways_placed() {
        let config = test_config();
        let mut rng = SeededRng::new(42);
        let tree = bsp_subdivide(&config, &mut rng);
        let rooms = collect_rooms(&tree);
        let mut grid = rooms_to_tile_grid(&rooms, &config);

        let walls = crate::tile_converter::detect_walls(&grid);
        let doorways = place_doorways(&walls, &mut grid, &rooms, &mut rng, &config);

        assert!(!doorways.is_empty(), "No doorways placed");
    }

    #[test]
    fn test_windows_only_on_exterior() {
        let config = test_config();
        let mut rng = SeededRng::new(42);
        let tree = bsp_subdivide(&config, &mut rng);
        let rooms = collect_rooms(&tree);
        let mut grid = rooms_to_tile_grid(&rooms, &config);

        let walls = crate::tile_converter::detect_walls(&grid);
        let _doorways = place_doorways(&walls, &mut grid, &rooms, &mut rng, &config);
        let windows = place_windows(&walls, &mut grid, &mut rng, &config);

        for window in &windows {
            let wall = walls.iter().find(|w| w.id == window.wall_id).unwrap();
            assert_eq!(
                wall.wall_type,
                WallType::Exterior,
                "Window placed on interior wall"
            );
        }
    }
}
