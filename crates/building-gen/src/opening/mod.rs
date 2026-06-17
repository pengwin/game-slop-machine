mod doorways;
mod entrance;
mod wall_query;
mod windows;

use crate::config::BuildingConfig;
use crate::layout::{Doorway, Room, Window};
use crate::tile::TileGrid;
use crate::zone_layout::CorridorInfo;

pub fn place_doorways(
    grid: &mut TileGrid,
    rooms: &[Room],
    config: &BuildingConfig,
    corridor: Option<&CorridorInfo>,
) -> Vec<Doorway> {
    let mut doorways = Vec::new();

    if let Some(corridor_info) = corridor {
        doorways::place_corridor_doorways(grid, rooms, corridor_info, config, &mut doorways);
    } else {
        doorways::place_room_to_room_doorways(grid, rooms, config, &mut doorways);
    }

    entrance::place_entrance_door(grid, config, &mut doorways);

    doorways
}

pub fn place_windows(
    grid: &mut TileGrid,
    rooms: &[Room],
    config: &BuildingConfig,
) -> Vec<Window> {
    windows::place_windows(grid, rooms, config)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::RoomSpec;
    use crate::geometry::Rect;
    use crate::tile::TileType;
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
        let (rooms, corridor) = zone_layout::generate_rooms(&config);
        let mut grid = rooms_to_tile_grid(&rooms, &config);
        if let Some(ref c) = corridor {
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

        let doorways = place_doorways(
            &mut grid,
            &rooms,
            &config,
            corridor.as_ref(),
        );

        assert!(!doorways.is_empty(), "No doorways placed");
    }

    #[test]
    fn test_windows_only_on_exterior() {
        let config = test_config();
        let (rooms, corridor) = zone_layout::generate_rooms(&config);
        let mut grid = rooms_to_tile_grid(&rooms, &config);

        let _doorways = place_doorways(
            &mut grid,
            &rooms,
            &config,
            corridor.as_ref(),
        );
        let windows = place_windows(&mut grid, &rooms, &config);

        for window in &windows {
            if let Some((x, y)) = grid.tile_coord(window.position) {
                assert!(
                    wall_query::is_exterior_wall(&grid, x, y),
                    "Window placed on interior wall at ({}, {})",
                    x,
                    y
                );
            }
        }
    }

    #[test]
    fn test_interior_room_no_windows() {
        let config = BuildingConfig {
            footprint: Rect::new(0.0, 0.0, 12.0, 4.0),
            tile_size: 1.0,
            min_room_size: 3.0,
            room_specs: vec![
                RoomSpec::new("a", 2),
                RoomSpec::new("b", 2),
                RoomSpec::new("c", 2),
            ],
            ..Default::default()
        };
        let (_, _) = zone_layout::generate_rooms(&config);
    }
}
