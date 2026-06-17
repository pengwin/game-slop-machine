use super::wall_query::{is_straight_wall, wall_tile_at};
use crate::config::BuildingConfig;
use crate::layout::{Doorway, WallId};
use crate::tile::{TileGrid, WallOpening};
use crate::zone_layout::entrance_door_position;

pub fn place_entrance_door(
    grid: &mut TileGrid,
    config: &BuildingConfig,
    doorways: &mut Vec<Doorway>,
) {
    let entrance_pos = entrance_door_position(config);

    if let Some((x, y)) = find_entrance_wall_tile(grid, config, entrance_pos) {
        grid.set_wall_opening(x, y, WallOpening::Door { render_panel: true });

        doorways.push(Doorway {
            wall_id: WallId(0),
            position: TileGrid::world_pos(grid, x, y),
            width: config.door_width,
            height: config.door_height,
        });
    }
}

fn find_entrance_wall_tile(
    grid: &TileGrid,
    config: &BuildingConfig,
    entrance_pos: crate::geometry::Vec2,
) -> Option<(usize, usize)> {
    let local = entrance_pos - grid.origin;
    let approx_x = (local.x / grid.tile_size).floor() as isize;
    let approx_y = (local.y / grid.tile_size).floor() as isize;
    let search_range = (config.door_width / config.tile_size).ceil() as isize + 4;

    let exact = if config.entrance_dir.y.abs() >= config.entrance_dir.x.abs() {
        let y = if config.entrance_dir.y >= 0.0 {
            0
        } else {
            grid.height.saturating_sub(1) as isize
        };
        (approx_x, y)
    } else {
        let x = if config.entrance_dir.x >= 0.0 {
            0
        } else {
            grid.width.saturating_sub(1) as isize
        };
        (x, approx_y)
    };

    if let Some(tile) = wall_tile_at(grid, exact)
        && !tile.is_opening()
    {
        return Some((exact.0 as usize, exact.1 as usize));
    }

    let wall_candidates: Vec<(isize, isize)> =
        if config.entrance_dir.y.abs() >= config.entrance_dir.x.abs() {
            let y = if config.entrance_dir.y >= 0.0 {
                0
            } else {
                grid.height.saturating_sub(1) as isize
            };
            (-search_range..=search_range)
                .map(|offset| (approx_x + offset, y))
                .collect()
        } else {
            let x = if config.entrance_dir.x >= 0.0 {
                0
            } else {
                grid.width.saturating_sub(1) as isize
            };
            (-search_range..=search_range)
                .map(|offset| (x, approx_y + offset))
                .collect()
        };

    for (x, y) in wall_candidates {
        if let Some(tile) = wall_tile_at(grid, (x, y))
            && is_straight_wall(tile)
            && !tile.is_opening()
        {
            return Some((x as usize, y as usize));
        }
    }

    None
}
