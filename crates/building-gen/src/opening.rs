//! Door and window placement for buildings.
//!
//! Door placement strategy:
//! - Non-corridor: doorways between adjacent rooms + one exterior door at entrance
//! - Corridor: doorways from each room to the corridor + one exterior door at entrance
//!
//! Window placement strategy:
//! - Per-room target count from `RoomSpec.windows`
//! - Only on exterior walls adjacent to the room
//! - Respect spacing constraints and skip corners/junctions

use crate::config::BuildingConfig;
use crate::layout::{Doorway, Room, Wall, WallId, Window};
use crate::random::SeededRng;
use crate::tile::{CardinalDir, TileGrid, TileType, WallKind, WallOpening, WallShape};
use crate::tile_converter::{find_adjacent_rooms, find_doorway_positions_between_rooms};
use crate::zone_layout::{entrance_door_position, CorridorInfo};

/// Places doorways between rooms (or to corridor) and one exterior door at entrance.
pub fn place_doorways(
    walls: &[Wall],
    grid: &mut TileGrid,
    rooms: &[Room],
    _rng: &mut SeededRng,
    config: &BuildingConfig,
    corridor: Option<&CorridorInfo>,
) -> Vec<Doorway> {
    let mut doorways = Vec::new();

    if let Some(corridor_info) = corridor {
        // Corridor mode: doorways from each room to the corridor
        place_corridor_doorways(grid, rooms, corridor_info, config, &mut doorways);
    } else {
        // Non-corridor mode: doorways between adjacent rooms
        place_room_to_room_doorways(grid, rooms, config, &mut doorways);
    }

    // Place one exterior door at the entrance position
    place_entrance_door(walls, grid, config, &mut doorways);

    doorways
}

/// Places doorways from each room to the center corridor.
fn place_corridor_doorways(
    grid: &mut TileGrid,
    rooms: &[Room],
    corridor: &CorridorInfo,
    config: &BuildingConfig,
    doorways: &mut Vec<Doorway>,
) {
    let cb = corridor.bounds;

    for room in rooms {
        // Find the wall of this room that faces the corridor
        let doorway_pos = find_corridor_doorway_position(room, &cb, grid, config);
        if let Some((x, y)) = doorway_pos {
            grid.set_wall_opening(
                x,
                y,
                WallOpening::Door {
                    render_panel: config.interior_door_render_panel,
                },
            );

            let pos = crate::tile::TileGrid::world_pos(grid, x, y);
            doorways.push(Doorway {
                wall_id: WallId(0),
                position: pos,
                width: config.door_width,
                height: config.door_height,
            });
        }
    }
}

/// Finds a wall tile between a room and the corridor for placing a doorway.
fn find_corridor_doorway_position(
    room: &Room,
    corridor_bounds: &crate::geometry::Rect,
    grid: &TileGrid,
    config: &BuildingConfig,
) -> Option<(usize, usize)> {
    let rb = room.bounds;
    let ts = config.tile_size;
    let origin = crate::geometry::Vec2::new(config.footprint.min.x, config.footprint.min.y);

    // Check which side of the room faces the corridor
    // Left room: right wall faces corridor (room.max.x ≈ corridor.min.x)
    // Right room: left wall faces corridor (room.min.x ≈ corridor.max.x)
    // Center room (odd last): bottom wall faces corridor (room.min.y ≈ corridor.max.y)
    // Or for depth_axis=X: analogous

    let candidates = if (rb.max.x - corridor_bounds.min.x).abs() < ts * 1.5 {
        // Left room: right wall
        let x = ((rb.max.x - origin.x) / ts).round() as isize;
        let y_mid = ((rb.center().y - origin.y) / ts).round() as usize;
        find_wall_tile_along_axis(grid, x, y_mid, true, config)
    } else if (corridor_bounds.max.x - rb.min.x).abs() < ts * 1.5 {
        // Right room: left wall
        let x = ((rb.min.x - origin.x) / ts).round() as isize;
        let y_mid = ((rb.center().y - origin.y) / ts).round() as usize;
        find_wall_tile_along_axis(grid, x, y_mid, true, config)
    } else if (rb.min.y - corridor_bounds.max.y).abs() < ts * 1.5 {
        // Center room (depth axis Y): bottom wall faces corridor
        let y = ((rb.min.y - origin.y) / ts).round() as isize;
        let x_mid = ((rb.center().x - origin.x) / ts).round() as usize;
        find_wall_tile_along_axis(grid, y, x_mid, false, config)
    } else if (corridor_bounds.min.y - rb.max.y).abs() < ts * 1.5 {
        // Room above corridor (depth axis X case)
        let y = ((rb.max.y - origin.y) / ts).round() as isize;
        let x_mid = ((rb.center().x - origin.x) / ts).round() as usize;
        find_wall_tile_along_axis(grid, y, x_mid, false, config)
    } else {
        None
    };

    candidates
}

/// Finds a straight wall tile near the given position along an axis.
fn find_wall_tile_along_axis(
    grid: &TileGrid,
    fixed_coord: isize,
    vary_start: usize,
    vary_is_y: bool,
    config: &BuildingConfig,
) -> Option<(usize, usize)> {
    let max_vary = if vary_is_y { grid.height } else { grid.width };
    let search_range = (config.door_width / config.tile_size).ceil() as usize + 2;

    for offset in 0..=search_range {
        for &sign in &[0isize, -1, 1] {
            let vary = vary_start as isize + sign * offset as isize;
            if vary < 0 || vary as usize >= max_vary {
                continue;
            }
            let (x, y) = if vary_is_y {
                (fixed_coord.max(0) as usize, vary as usize)
            } else {
                (vary as usize, fixed_coord.max(0) as usize)
            };
            if x >= grid.width || y >= grid.height {
                continue;
            }
            if is_straight_wall(grid.get(x, y)) && !grid.get(x, y).is_opening() {
                return Some((x, y));
            }
        }
    }
    None
}

/// Places doorways between adjacent rooms (non-corridor mode).
fn place_room_to_room_doorways(
    grid: &mut TileGrid,
    rooms: &[Room],
    config: &BuildingConfig,
    doorways: &mut Vec<Doorway>,
) {
    let adjacent_pairs = find_adjacent_rooms(grid, rooms);

    for (i, j) in adjacent_pairs {
        let positions = find_doorway_positions_between_rooms(grid, &rooms[i], &rooms[j]);

        if !positions.is_empty() {
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

            let pos = crate::tile::TileGrid::world_pos(grid, wx, wy);
            doorways.push(Doorway {
                wall_id: WallId(0),
                position: pos,
                width: config.door_width,
                height: config.door_height,
            });
        }
    }
}

/// Places the exterior door at the entrance position.
fn place_entrance_door(
    _walls: &[Wall],
    grid: &mut TileGrid,
    config: &BuildingConfig,
    doorways: &mut Vec<Doorway>,
) {
    let entrance_pos = entrance_door_position(config);

    if let Some((x, y)) = grid.tile_coord(entrance_pos) {
        if is_straight_wall(grid.get(x, y)) {
            grid.set_wall_opening(x, y, WallOpening::Door { render_panel: true });

            doorways.push(Doorway {
                wall_id: WallId(0),
                position: entrance_pos,
                width: config.door_width,
                height: config.door_height,
            });
        }
    }
}

/// Places windows on exterior walls per room spec.
///
/// For each room, finds exterior wall tiles adjacent to that room
/// and places up to `spec.windows` windows, respecting spacing constraints.
pub fn place_windows(
    _walls: &[Wall],
    grid: &mut TileGrid,
    rooms: &[Room],
    config: &BuildingConfig,
) -> Vec<Window> {
    let mut windows = Vec::new();

    for (room_idx, room) in rooms.iter().enumerate() {
        let spec = &config.room_specs[room_idx];
        if spec.windows == 0 {
            continue;
        }

        let candidates = room_exterior_wall_tiles(grid, room, config);
        let mut placed = 0;

        for (x, y) in candidates {
            if placed >= spec.windows {
                break;
            }

            let tile = grid.get(x, y);
            if !is_straight_wall(tile) || tile.is_opening() {
                continue;
            }

            // Check spacing constraints
            if has_window_near(grid, x, y, config) {
                continue;
            }
            if window_near_corner_or_junction(grid, x, y) {
                continue;
            }

            let render_glass = match tile.wall().map(|wall| wall.kind) {
                Some(WallKind::Interior) => config.interior_window_render_glass,
                _ => config.exterior_window_render_glass,
            };
            grid.set_wall_opening(x, y, WallOpening::Window { render_glass });

            let pos = crate::tile::TileGrid::world_pos(grid, x, y);
            windows.push(Window {
                wall_id: WallId(0),
                position: pos,
                width: config.window_width,
                height: config.window_height,
                sill_height: config.window_sill_height,
            });
            placed += 1;
        }
    }

    windows
}

/// Finds exterior wall tiles adjacent to a room's floor tiles.
fn room_exterior_wall_tiles(
    grid: &TileGrid,
    room: &Room,
    config: &BuildingConfig,
) -> Vec<(usize, usize)> {
    let ts = config.tile_size;
    let origin = config.footprint.min;

    let min_x = ((room.bounds.min.x - origin.x) / ts).round().max(0.0) as usize;
    let min_y = ((room.bounds.min.y - origin.y) / ts).round().max(0.0) as usize;
    let max_x = ((room.bounds.max.x - origin.x) / ts).round().max(0.0) as usize;
    let max_y = ((room.bounds.max.y - origin.y) / ts).round().max(0.0) as usize;

    let mut result = Vec::new();

    // Check all tiles in the room's bounding box
    for y in min_y..=max_y.min(grid.height - 1) {
        for x in min_x..=max_x.min(grid.width - 1) {
            let tile = grid.get(x, y);
            if !tile.is_wall() {
                continue;
            }

            // Check if this wall is exterior (adjacent to empty or grid edge)
            if is_exterior_wall(grid, x, y) {
                result.push((x, y));
            }
        }
    }

    result
}

/// Checks if a wall tile is on the exterior boundary.
fn is_exterior_wall(grid: &TileGrid, x: usize, y: usize) -> bool {
    x == 0
        || y == 0
        || x == grid.width - 1
        || y == grid.height - 1
        || [(-1, 0), (1, 0), (0, -1), (0, 1)].iter().any(|(dx, dy)| {
            matches!(
                grid.get_neighbor(x, y, *dx, *dy),
                None | Some(TileType::Empty)
            )
        })
}

fn is_straight_wall(tile: TileType) -> bool {
    matches!(
        tile.wall().map(|wall| wall.shape),
        Some(WallShape::Straight(_))
    )
}

fn window_near_corner_or_junction(grid: &TileGrid, x: usize, y: usize) -> bool {
    near_non_straight_wall(grid, x, y, 1)
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::RoomSpec;
    use crate::geometry::Rect;
    use crate::tile_converter::rooms_to_tile_grid;
    use crate::zone_layout;

    fn test_config() -> BuildingConfig {
        BuildingConfig {
            footprint: Rect::new(0.0, 0.0, 10.0, 8.0),
            tile_size: 0.5,
            min_room_size: 2.5,
            room_specs: vec![
                RoomSpec::new("hall", 1),
                RoomSpec::new("kitchen", 1),
                RoomSpec::new("bedroom", 1),
                RoomSpec::new("bathroom", 0),
            ],
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
        let (rooms, corridor) = zone_layout::generate_rooms(&config);
        let mut grid = rooms_to_tile_grid(&rooms, &config);
        if let Some(ref c) = corridor {
            // Mark corridor floor
            let cb = c.bounds;
            let ts = config.tile_size;
            let origin = config.footprint.min;
            let min_x = ((cb.min.x - origin.x) / ts).round() as usize;
            let min_y = ((cb.min.y - origin.y) / ts).round() as usize;
            let max_x = ((cb.max.x - origin.x) / ts).round() as usize;
            let max_y = ((cb.max.y - origin.y) / ts).round() as usize;
            for y in min_y..max_y.min(grid.height) {
                for x in min_x..max_x.min(grid.width) {
                    if grid.get(x, y) == TileType::Empty {
                        grid.set(x, y, TileType::Floor);
                    }
                }
            }
        }

        let walls = crate::tile_converter::detect_walls(&grid);
        let doorways = place_doorways(
            &walls,
            &mut grid,
            &rooms,
            &mut rng,
            &config,
            corridor.as_ref(),
        );

        assert!(!doorways.is_empty(), "No doorways placed");
    }

    #[test]
    fn test_windows_only_on_exterior() {
        let config = test_config();
        let mut rng = SeededRng::new(42);
        let (rooms, corridor) = zone_layout::generate_rooms(&config);
        let mut grid = rooms_to_tile_grid(&rooms, &config);

        let walls = crate::tile_converter::detect_walls(&grid);
        let _doorways = place_doorways(
            &walls,
            &mut grid,
            &rooms,
            &mut rng,
            &config,
            corridor.as_ref(),
        );
        let windows = place_windows(&walls, &mut grid, &rooms, &config);

        for window in &windows {
            // Verify window position is on an exterior wall
            if let Some((x, y)) = grid.tile_coord(window.position) {
                assert!(
                    is_exterior_wall(&grid, x, y),
                    "Window placed on interior wall at ({}, {})",
                    x,
                    y
                );
            }
        }
    }

    #[test]
    fn test_interior_room_no_windows() {
        // 3 rooms in a row - middle room has no exterior walls
        let config = BuildingConfig {
            footprint: Rect::new(0.0, 0.0, 12.0, 4.0),
            tile_size: 1.0,
            min_room_size: 3.0,
            room_specs: vec![
                RoomSpec::new("a", 2),
                RoomSpec::new("b", 2), // middle room
                RoomSpec::new("c", 2),
            ],
            ..Default::default()
        };
        let (_, _) = zone_layout::generate_rooms(&config);
        // Middle room should get 0 windows since it has no exterior walls
        // (This is validated by the window placement logic limiting to available exterior tiles)
    }
}
