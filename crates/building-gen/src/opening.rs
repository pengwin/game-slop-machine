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
//! 3. Skip walls that already have doors or windows nearby
//! 4. Windows occupy multiple tiles based on window_width

use crate::config::BuildingConfig;
use crate::layout::{Doorway, Room, Wall, WallId, WallType, Window};
use crate::random::SeededRng;
use crate::tile::{CardinalDir, TileGrid, TileType, WallKind, WallOpening, WallShape};
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
            let candidates: Vec<_> = positions
                .into_iter()
                .filter(|&(x, y)| is_straight_wall(grid.get(x, y)))
                .collect();

            if candidates.is_empty() {
                continue;
            }

            let mid = candidates.len() / 2;
            let (wx, wy) = candidates[mid];

            grid.set_wall_opening(
                wx,
                wy,
                WallOpening::Door {
                    render_panel: config.interior_door_render_panel,
                },
            );

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
            if is_straight_wall(grid.get(x, y)) {
                grid.set_wall_opening(x, y, WallOpening::Door { render_panel: true });

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
/// - Skip walls that already have doors or windows nearby
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
            if is_straight_wall(tile) {
                if tile.is_opening() {
                    continue;
                }

                // Check if there's a door or window nearby
                let has_door = grid
                    .neighbors(x, y)
                    .iter()
                    .any(|(_, _, t)| is_door_like(*t));
                let has_window = has_window_near(grid, x, y, config);

                // 30% chance to place window, skip if door nearby
                if !has_door && !has_window && rng.gen_bool(0.3) {
                    let positions = window_positions(grid, x, y, window_tiles);
                    if positions.len() != window_tiles {
                        continue;
                    }
                    if window_near_corner_or_junction(grid, &positions, config) {
                        continue;
                    }

                    for (wx, wy) in positions {
                        if !is_straight_wall(grid.get(wx, wy)) || grid.get(wx, wy).is_opening() {
                            continue;
                        }
                        let render_glass = match grid.get(wx, wy).wall().map(|wall| wall.kind) {
                            Some(WallKind::Interior) => config.interior_window_render_glass,
                            _ => config.exterior_window_render_glass,
                        };
                        grid.set_wall_opening(wx, wy, WallOpening::Window { render_glass });
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

fn is_straight_wall(tile: TileType) -> bool {
    matches!(
        tile.wall().map(|wall| wall.shape),
        Some(WallShape::Straight(_))
    )
}

fn window_near_corner_or_junction(
    grid: &TileGrid,
    positions: &[(usize, usize)],
    _config: &BuildingConfig,
) -> bool {
    positions
        .iter()
        .any(|&(x, y)| near_non_straight_wall(grid, x, y, 1))
}

fn near_non_straight_wall(grid: &TileGrid, x: usize, y: usize, margin: isize) -> bool {
    let Some(axis_x) = straight_wall_axis_x(grid.get(x, y)) else {
        return true;
    };

    for offset in -margin..=margin {
        if offset == 0 {
            continue;
        }

        let coord = if axis_x {
            x.checked_add_signed(offset)
                .filter(|&nx| nx < grid.width)
                .map(|nx| (nx, y))
        } else {
            y.checked_add_signed(offset)
                .filter(|&ny| ny < grid.height)
                .map(|ny| (x, ny))
        };

        let Some((nx, ny)) = coord else {
            return true;
        };
        let tile = grid.get(nx, ny);
        if tile.is_wall() && !is_straight_wall(tile) {
            return true;
        }
    }

    false
}

fn straight_wall_axis_x(tile: TileType) -> Option<bool> {
    match tile.wall().map(|wall| wall.shape) {
        Some(WallShape::Straight(CardinalDir::Bottom | CardinalDir::Top)) => Some(true),
        Some(WallShape::Straight(CardinalDir::Left | CardinalDir::Right)) => Some(false),
        _ => None,
    }
}

fn window_positions(
    grid: &TileGrid,
    x: usize,
    y: usize,
    window_tiles: usize,
) -> Vec<(usize, usize)> {
    let horizontal =
        has_wall(grid, x.wrapping_sub(1), y) || (x + 1 < grid.width && has_wall(grid, x + 1, y));
    let vertical =
        has_wall(grid, x, y.wrapping_sub(1)) || (y + 1 < grid.height && has_wall(grid, x, y + 1));
    let axis_x = horizontal || !vertical;
    let half = window_tiles / 2;
    let mut result = Vec::new();

    for i in 0..window_tiles {
        let offset = i as isize - half as isize;
        let wx = if axis_x {
            x.checked_add_signed(offset)
        } else {
            Some(x)
        };
        let wy = if axis_x {
            Some(y)
        } else {
            y.checked_add_signed(offset)
        };

        if let (Some(wx), Some(wy)) = (wx, wy) {
            if wx < grid.width && wy < grid.height && grid.get(wx, wy).is_wall() {
                result.push((wx, wy));
            }
        }
    }

    result
}

fn has_wall(grid: &TileGrid, x: usize, y: usize) -> bool {
    x < grid.width && y < grid.height && is_straight_wall(grid.get(x, y))
}

fn has_window_near(grid: &TileGrid, x: usize, y: usize, config: &BuildingConfig) -> bool {
    let spacing_tiles = (config.window_spacing / config.tile_size).ceil() as i32;
    let x = x as i32;
    let y = y as i32;

    for dy in -spacing_tiles..=spacing_tiles {
        for dx in -spacing_tiles..=spacing_tiles {
            let nx = x + dx;
            let ny = y + dy;

            if nx < 0 || ny < 0 || nx as usize >= grid.width || ny as usize >= grid.height {
                continue;
            }

            if matches!(
                grid.get(nx as usize, ny as usize)
                    .wall()
                    .and_then(|wall| wall.opening),
                Some(WallOpening::Window { .. })
            ) {
                return true;
            }
        }
    }

    false
}

fn is_door_like(tile: TileType) -> bool {
    matches!(
        tile.wall().and_then(|wall| wall.opening),
        Some(WallOpening::Door { .. } | WallOpening::Doorway)
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bsp::{bsp_subdivide, collect_rooms};
    use crate::geometry::Rect;
    use crate::tile::{CornerDir, WallTile};
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

    #[test]
    fn test_windows_not_placed_near_corners() {
        let config = BuildingConfig {
            footprint: Rect::new(0.0, 0.0, 5.0, 3.0),
            tile_size: 1.0,
            window_width: 1.0,
            ..Default::default()
        };
        let mut grid = TileGrid::new(5, 3, config.tile_size, config.footprint.min);

        grid.set(
            0,
            0,
            TileType::Wall(WallTile::exterior(WallShape::Corner(CornerDir::BottomLeft))),
        );
        for x in 1..4 {
            grid.set(
                x,
                0,
                TileType::Wall(WallTile::exterior(WallShape::Straight(CardinalDir::Top))),
            );
        }
        grid.set(
            4,
            0,
            TileType::Wall(WallTile::exterior(WallShape::Corner(
                CornerDir::BottomRight,
            ))),
        );
        for x in 1..4 {
            grid.set(x, 1, TileType::Floor);
        }

        assert!(
            window_near_corner_or_junction(&grid, &[(1, 0)], &config),
            "a window adjacent to a corner should be rejected"
        );
        assert!(
            !window_near_corner_or_junction(&grid, &[(2, 0)], &config),
            "a window with a straight wall tile on both sides should be allowed"
        );
    }
}
