//! Converts BSP room layout into a tile grid with walls, floors, and openings.
//!
//! The conversion process:
//! 1. Mark each room's perimeter as wall tiles
//! 2. Fill the remaining room interior with floor tiles
//! 3. Identify corner tiles where horizontal and vertical walls meet
//!
//! Design decisions:
//! - BSP split coordinates are snapped to the tile grid
//! - A boundary coordinate maps to one wall tile line, so adjacent rooms share a wall
//! - Walls are detected by checking room perimeters rather than empty neighbor gaps

use crate::config::BuildingConfig;
use crate::geometry::Vec2;
use crate::layout::{Room, Wall, WallType};
use crate::tile::{
    CardinalDir, CornerDir, TJunctionDir, TileGrid, TileType, WallKind, WallShape, WallTile,
};

/// Converts BSP rooms into a tile grid with walls and floors.
///
/// Room bounds are snapped to tile lines. Adjacent rooms that share a BSP
/// split write to the same perimeter tile, producing one shared wall.
pub fn rooms_to_tile_grid(rooms: &[Room], config: &BuildingConfig) -> TileGrid {
    let tiles_x = config.tiles_x();
    let tiles_y = config.tiles_y();
    let mut grid = TileGrid::new(tiles_x, tiles_y, config.tile_size, config.footprint.min);

    for room in rooms {
        mark_room_perimeter(&mut grid, room, config);
    }

    for room in rooms {
        fill_room_interior(&mut grid, room, config);
    }

    classify_wall_tiles(&mut grid);

    grid
}

fn room_tile_bounds(
    grid: &TileGrid,
    room: &Room,
    config: &BuildingConfig,
) -> (usize, usize, usize, usize) {
    let min_x = world_to_grid_line(room.bounds.min.x, config.footprint.min.x, config.tile_size)
        .min(grid.width.saturating_sub(1));
    let min_y = world_to_grid_line(room.bounds.min.y, config.footprint.min.y, config.tile_size)
        .min(grid.height.saturating_sub(1));
    let max_x = world_to_grid_line(room.bounds.max.x, config.footprint.min.x, config.tile_size)
        .min(grid.width.saturating_sub(1));
    let max_y = world_to_grid_line(room.bounds.max.y, config.footprint.min.y, config.tile_size)
        .min(grid.height.saturating_sub(1));

    (min_x, min_y, max_x.max(min_x), max_y.max(min_y))
}

fn world_to_grid_line(value: f32, origin: f32, tile_size: f32) -> usize {
    ((value - origin) / tile_size).round().max(0.0) as usize
}

fn mark_room_perimeter(grid: &mut TileGrid, room: &Room, config: &BuildingConfig) {
    let (min_x, min_y, max_x, max_y) = room_tile_bounds(grid, room, config);

    for x in min_x..=max_x {
        grid.set(x, min_y, raw_wall());
        grid.set(x, max_y, raw_wall());
    }

    for y in min_y..=max_y {
        grid.set(min_x, y, raw_wall());
        grid.set(max_x, y, raw_wall());
    }
}

fn fill_room_interior(grid: &mut TileGrid, room: &Room, config: &BuildingConfig) {
    let (min_x, min_y, max_x, max_y) = room_tile_bounds(grid, room, config);

    if max_x <= min_x + 1 || max_y <= min_y + 1 {
        return;
    }

    for y in (min_y + 1)..max_y {
        for x in (min_x + 1)..max_x {
            if grid.get(x, y) == TileType::Empty {
                grid.set(x, y, TileType::Floor);
            }
        }
    }
}

fn raw_wall() -> TileType {
    TileType::Wall(WallTile::exterior(WallShape::Straight(CardinalDir::Top)))
}

pub fn classify_wall_tiles(grid: &mut TileGrid) {
    let width = grid.width;
    let height = grid.height;
    let mut classified = Vec::new();

    for y in 0..height {
        for x in 0..width {
            if !grid.get(x, y).is_wall() {
                continue;
            }
            classified.push((x, y, classify_wall_tile(grid, x, y)));
        }
    }
    for (x, y, wall) in classified {
        grid.set(x, y, TileType::Wall(wall));
    }
}

fn classify_wall_tile(grid: &TileGrid, x: usize, y: usize) -> WallTile {
    let floor_left = grid.is_room_neighbor(x, y, -1, 0);
    let floor_right = grid.is_room_neighbor(x, y, 1, 0);
    let floor_bottom = grid.is_room_neighbor(x, y, 0, -1);
    let floor_top = grid.is_room_neighbor(x, y, 0, 1);

    let wall_left = is_wall_neighbor(grid, x, y, -1, 0);
    let wall_right = is_wall_neighbor(grid, x, y, 1, 0);
    let wall_bottom = is_wall_neighbor(grid, x, y, 0, -1);
    let wall_top = is_wall_neighbor(grid, x, y, 0, 1);

    let kind = if is_exterior_boundary(grid, x, y) {
        WallKind::Exterior
    } else {
        WallKind::Interior
    };

    let connections =
        (wall_left as u8) + (wall_right as u8) + (wall_bottom as u8) + (wall_top as u8);

    let shape = if connections >= 4 {
        WallShape::Cross
    } else if connections == 3 {
        if !wall_left {
            WallShape::TJunction(TJunctionDir::Left)
        } else if !wall_right {
            WallShape::TJunction(TJunctionDir::Right)
        } else if !wall_bottom {
            WallShape::TJunction(TJunctionDir::Bottom)
        } else {
            WallShape::TJunction(TJunctionDir::Top)
        }
    } else if is_corner_connection(wall_left, wall_right, wall_bottom, wall_top) {
        match (wall_left, wall_right, wall_bottom, wall_top) {
            (false, true, false, true) => WallShape::Corner(CornerDir::TopRight),
            (true, false, false, true) => WallShape::Corner(CornerDir::TopLeft),
            (false, true, true, false) => WallShape::Corner(CornerDir::BottomRight),
            (true, false, true, false) => WallShape::Corner(CornerDir::BottomLeft),
            _ => straight_shape(floor_left, floor_right, floor_bottom, floor_top),
        }
    } else {
        straight_shape(floor_left, floor_right, floor_bottom, floor_top)
    };

    WallTile::new(kind, shape)
}

fn is_exterior_boundary(grid: &TileGrid, x: usize, y: usize) -> bool {
    x == 0
        || y == 0
        || x == grid.width - 1
        || y == grid.height - 1
        || touches_empty_neighbor(grid, x, y)
}

fn touches_empty_neighbor(grid: &TileGrid, x: usize, y: usize) -> bool {
    [(-1, 0), (1, 0), (0, -1), (0, 1)]
        .into_iter()
        .any(|(dx, dy)| {
            matches!(
                grid.get_neighbor(x, y, dx, dy),
                None | Some(TileType::Empty)
            )
        })
}

fn is_wall_neighbor(grid: &TileGrid, x: usize, y: usize, dx: i32, dy: i32) -> bool {
    matches!(grid.get_neighbor(x, y, dx, dy), Some(tile) if tile.is_wall())
}

fn is_corner_connection(left: bool, right: bool, bottom: bool, top: bool) -> bool {
    ((left || right) && (bottom || top)) && !((left && right) || (bottom && top))
}

fn straight_shape(
    floor_left: bool,
    floor_right: bool,
    floor_bottom: bool,
    floor_top: bool,
) -> WallShape {
    if floor_left && !floor_right {
        WallShape::Straight(CardinalDir::Right)
    } else if floor_right && !floor_left {
        WallShape::Straight(CardinalDir::Left)
    } else if floor_bottom && !floor_top {
        WallShape::Straight(CardinalDir::Top)
    } else if floor_top && !floor_bottom {
        WallShape::Straight(CardinalDir::Bottom)
    } else if floor_left || floor_right {
        WallShape::Straight(CardinalDir::Left)
    } else {
        WallShape::Straight(CardinalDir::Bottom)
    }
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
            if tile.is_wall() {
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
    if is_exterior_boundary(grid, x, y) {
        return WallType::Exterior;
    }
    WallType::Interior
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

    let (min_x_a, min_y_a, max_x_a, max_y_a) = room_grid_lines(grid, room_a);
    let (min_x_b, min_y_b, max_x_b, max_y_b) = room_grid_lines(grid, room_b);

    if max_x_a == min_x_b || max_x_b == min_x_a {
        let x = if max_x_a == min_x_b { max_x_a } else { max_x_b };
        let start_y = min_y_a.max(min_y_b).saturating_add(1);
        let end_y = max_y_a.min(max_y_b);

        for y in start_y..end_y {
            if grid.get(x, y).is_wall() {
                positions.push((x, y));
            }
        }
    } else if max_y_a == min_y_b || max_y_b == min_y_a {
        let y = if max_y_a == min_y_b { max_y_a } else { max_y_b };
        let start_x = min_x_a.max(min_x_b).saturating_add(1);
        let end_x = max_x_a.min(max_x_b);

        for x in start_x..end_x {
            if grid.get(x, y).is_wall() {
                positions.push((x, y));
            }
        }
    }

    positions
}

fn room_grid_lines(grid: &TileGrid, room: &Room) -> (usize, usize, usize, usize) {
    let min_x = ((room.bounds.min.x - grid.origin.x) / grid.tile_size)
        .round()
        .max(0.0) as usize;
    let min_y = ((room.bounds.min.y - grid.origin.y) / grid.tile_size)
        .round()
        .max(0.0) as usize;
    let max_x = ((room.bounds.max.x - grid.origin.x) / grid.tile_size)
        .round()
        .max(0.0) as usize;
    let max_y = ((room.bounds.max.y - grid.origin.y) / grid.tile_size)
        .round()
        .max(0.0) as usize;

    (
        min_x.min(grid.width.saturating_sub(1)),
        min_y.min(grid.height.saturating_sub(1)),
        max_x.min(grid.width.saturating_sub(1)),
        max_y.min(grid.height.saturating_sub(1)),
    )
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

        let wall_count = grid.count_matching_tiles(TileType::is_wall);
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
