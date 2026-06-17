use crate::geometry::Rect;
use crate::tile::{TileGrid, TileType};
use std::collections::HashSet;

/// Tracks which grid tiles are occupied by furniture or doorways.
pub struct OccupiedTiles {
    tiles: HashSet<(usize, usize)>,
    grid_width: usize,
    grid_height: usize,
}

impl OccupiedTiles {
    pub fn new(grid_width: usize, grid_height: usize) -> Self {
        Self {
            tiles: HashSet::new(),
            grid_width,
            grid_height,
        }
    }

    /// Marks a doorway tile and its immediate neighbors as occupied.
    pub fn mark_doorway(&mut self, x: usize, y: usize) {
        for dy in -1isize..=1 {
            for dx in -1isize..=1 {
                let nx = x as isize + dx;
                let ny = y as isize + dy;
                if nx >= 0 && ny >= 0 && (nx as usize) < self.grid_width && (ny as usize) < self.grid_height {
                    self.tiles.insert((nx as usize, ny as usize));
                }
            }
        }
    }

    /// Marks a tile as occupied by furniture.
    pub fn mark(&mut self, x: usize, y: usize) {
        self.tiles.insert((x, y));
    }

    /// Marks a rectangular area as occupied.
    #[allow(dead_code)]
    pub fn mark_rect(&mut self, x: usize, y: usize, w: usize, h: usize) {
        for dy in 0..h {
            for dx in 0..w {
                self.tiles.insert((x + dx, y + dy));
            }
        }
    }

    pub fn is_occupied(&self, x: usize, y: usize) -> bool {
        self.tiles.contains(&(x, y))
    }
}

/// Finds floor tiles adjacent to walls within a room, suitable for wall-mounted furniture.
/// Returns (tile_x, tile_y, rotation) where rotation faces away from the wall.
pub fn find_wall_positions(
    room_bounds: Rect,
    grid: &TileGrid,
    occupied: &OccupiedTiles,
) -> Vec<(usize, usize, f32)> {
    let ts = grid.tile_size;
    let origin = grid.origin;
    let mut positions = Vec::new();

    let min_x = ((room_bounds.min.x - origin.x) / ts).round().max(0.0) as usize;
    let min_y = ((room_bounds.min.y - origin.y) / ts).round().max(0.0) as usize;
    let max_x = ((room_bounds.max.x - origin.x) / ts).round().max(0.0) as usize;
    let max_y = ((room_bounds.max.y - origin.y) / ts).round().max(0.0) as usize;

    let max_x = max_x.min(grid.width.saturating_sub(1));
    let max_y = max_y.min(grid.height.saturating_sub(1));

    for y in min_y..=max_y {
        for x in min_x..=max_x {
            if grid.get(x, y) != TileType::Floor {
                continue;
            }
            if occupied.is_occupied(x, y) {
                continue;
            }

            // Check if adjacent to a wall (interior or exterior)
            let has_wall_north = y > 0 && grid.get(x, y - 1).is_wall();
            let has_wall_south = y < grid.height - 1 && grid.get(x, y + 1).is_wall();
            let has_wall_west = x > 0 && grid.get(x - 1, y).is_wall();
            let has_wall_east = x < grid.width - 1 && grid.get(x + 1, y).is_wall();

            if has_wall_north {
                positions.push((x, y, std::f32::consts::PI)); // face south
            }
            if has_wall_south {
                positions.push((x, y, 0.0)); // face north
            }
            if has_wall_west {
                positions.push((x, y, std::f32::consts::FRAC_PI_2)); // face east
            }
            if has_wall_east {
                positions.push((x, y, -std::f32::consts::FRAC_PI_2)); // face west
            }
        }
    }

    positions
}

/// Finds floor tiles in the center of a room, away from walls.
/// Returns (tile_x, tile_y).
pub fn find_center_positions(
    room_bounds: Rect,
    grid: &TileGrid,
    occupied: &OccupiedTiles,
    min_distance_from_wall: usize,
) -> Vec<(usize, usize)> {
    let ts = grid.tile_size;
    let origin = grid.origin;
    let mut positions = Vec::new();

    let min_x = ((room_bounds.min.x - origin.x) / ts).round().max(0.0) as usize;
    let min_y = ((room_bounds.min.y - origin.y) / ts).round().max(0.0) as usize;
    let max_x = ((room_bounds.max.x - origin.x) / ts).round().max(0.0) as usize;
    let max_y = ((room_bounds.max.y - origin.y) / ts).round().max(0.0) as usize;

    let max_x = max_x.min(grid.width.saturating_sub(1));
    let max_y = max_y.min(grid.height.saturating_sub(1));

    // Need enough room for wall distance
    if max_x < min_x + min_distance_from_wall * 2
        || max_y < min_y + min_distance_from_wall * 2
    {
        return positions;
    }

    let inner_min_x = min_x + min_distance_from_wall;
    let inner_min_y = min_y + min_distance_from_wall;
    let inner_max_x = max_x.saturating_sub(min_distance_from_wall);
    let inner_max_y = max_y.saturating_sub(min_distance_from_wall);

    for y in inner_min_y..=inner_max_y {
        for x in inner_min_x..=inner_max_x {
            if grid.get(x, y) != TileType::Floor {
                continue;
            }
            if occupied.is_occupied(x, y) {
                continue;
            }
            positions.push((x, y));
        }
    }

    positions
}

/// Converts a tile position to a world-space center position.
pub fn tile_to_world(x: usize, y: usize, grid: &TileGrid) -> (f32, f32) {
    let ts = grid.tile_size;
    (
        grid.origin.x + (x as f32 + 0.5) * ts,
        grid.origin.y + (y as f32 + 0.5) * ts,
    )
}
