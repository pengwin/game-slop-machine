use super::wall_query::{find_wall_tile_along_axis, is_straight_wall};
use crate::config::BuildingConfig;
use crate::geometry::{Rect, Vec2};
use crate::layout::{Doorway, Room, WallId};
use crate::tile::{TileGrid, WallOpening};
use crate::tile_converter::{find_adjacent_rooms, find_doorway_positions_between_rooms};
use crate::zone_layout::CorridorInfo;

pub fn place_corridor_doorways(
    grid: &mut TileGrid,
    rooms: &[Room],
    corridor: &CorridorInfo,
    config: &BuildingConfig,
    doorways: &mut Vec<Doorway>,
) {
    let cb = corridor.bounds;

    for room in rooms {
        let doorway_pos = find_corridor_doorway_position(room, &cb, grid, config);
        if let Some((x, y)) = doorway_pos {
            grid.set_wall_opening(
                x,
                y,
                WallOpening::Door {
                    render_panel: config.interior_door_render_panel,
                },
            );

            let pos = TileGrid::world_pos(grid, x, y);
            doorways.push(Doorway {
                wall_id: WallId(0),
                position: pos,
                width: config.door_width,
                height: config.door_height,
            });
        }
    }
}

fn find_corridor_doorway_position(
    room: &Room,
    corridor_bounds: &Rect,
    grid: &TileGrid,
    config: &BuildingConfig,
) -> Option<(usize, usize)> {
    let rb = room.bounds;
    let ts = config.tile_size;
    let origin = Vec2::new(config.footprint.min.x, config.footprint.min.y);

    let candidates = if (rb.max.x - corridor_bounds.min.x).abs() < ts * 1.5 {
        let x = ((rb.max.x - origin.x) / ts).round() as isize;
        let y_mid = ((rb.center().y - origin.y) / ts).round() as usize;
        find_wall_tile_along_axis(grid, x, y_mid, true, config)
    } else if (corridor_bounds.max.x - rb.min.x).abs() < ts * 1.5 {
        let x = ((rb.min.x - origin.x) / ts).round() as isize;
        let y_mid = ((rb.center().y - origin.y) / ts).round() as usize;
        find_wall_tile_along_axis(grid, x, y_mid, true, config)
    } else if (rb.min.y - corridor_bounds.max.y).abs() < ts * 1.5 {
        let y = ((rb.min.y - origin.y) / ts).round() as isize;
        let x_mid = ((rb.center().x - origin.x) / ts).round() as usize;
        find_wall_tile_along_axis(grid, y, x_mid, false, config)
    } else if (corridor_bounds.min.y - rb.max.y).abs() < ts * 1.5 {
        let y = ((rb.max.y - origin.y) / ts).round() as isize;
        let x_mid = ((rb.center().x - origin.x) / ts).round() as usize;
        find_wall_tile_along_axis(grid, y, x_mid, false, config)
    } else {
        None
    };

    candidates
}

pub fn place_room_to_room_doorways(
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

            let pos = TileGrid::world_pos(grid, wx, wy);
            doorways.push(Doorway {
                wall_id: WallId(0),
                position: pos,
                width: config.door_width,
                height: config.door_height,
            });
        }
    }
}
