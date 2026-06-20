use super::wall_query::{
    has_window_near, is_exterior_wall, is_straight_wall, window_near_corner_or_junction,
};
use crate::config::BuildingConfig;
use crate::layout::{Room, WallId, Window};
use crate::tile::{TileGrid, WallKind, WallOpening};

pub fn place_windows(grid: &mut TileGrid, rooms: &[Room], config: &BuildingConfig) -> Vec<Window> {
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

            let pos = TileGrid::world_pos(grid, x, y);
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

    for y in min_y..=max_y.min(grid.height - 1) {
        for x in min_x..=max_x.min(grid.width - 1) {
            let tile = grid.get(x, y);
            if !tile.is_wall() {
                continue;
            }

            if is_exterior_wall(grid, x, y) {
                result.push((x, y));
            }
        }
    }

    result
}
