use crate::config::BuildingConfig;
use crate::tile::{CardinalDir, TileGrid, TileType, WallOpening, WallShape};

pub fn is_straight_wall(tile: TileType) -> bool {
    matches!(
        tile.wall().map(|wall| wall.shape),
        Some(WallShape::Straight(_))
    )
}

pub fn is_exterior_wall(grid: &TileGrid, x: usize, y: usize) -> bool {
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

pub fn wall_tile_at(grid: &TileGrid, coord: (isize, isize)) -> Option<TileType> {
    let (x, y) = coord;
    if x < 0 || y < 0 || x as usize >= grid.width || y as usize >= grid.height {
        return None;
    }
    let tile = grid.get(x as usize, y as usize);
    tile.is_wall().then_some(tile)
}

pub fn straight_wall_axis_x(tile: TileType) -> Option<bool> {
    match tile.wall().map(|wall| wall.shape) {
        Some(WallShape::Straight(CardinalDir::Bottom | CardinalDir::Top)) => Some(true),
        Some(WallShape::Straight(CardinalDir::Left | CardinalDir::Right)) => Some(false),
        _ => None,
    }
}

pub fn near_non_straight_wall(grid: &TileGrid, x: usize, y: usize, margin: isize) -> bool {
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

pub fn window_near_corner_or_junction(grid: &TileGrid, x: usize, y: usize) -> bool {
    near_non_straight_wall(grid, x, y, 1)
}

pub fn has_window_near(grid: &TileGrid, x: usize, y: usize, config: &BuildingConfig) -> bool {
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

pub fn find_wall_tile_along_axis(
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
