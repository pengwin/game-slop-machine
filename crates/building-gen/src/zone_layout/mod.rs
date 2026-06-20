mod axes;
mod corridor;
mod rows;
mod scoring;
mod splits;

use crate::config::{BuildingConfig, RoomSpec};
use crate::geometry::Vec2;
use crate::layout::Room;

pub use axes::DepthAxis;
pub use corridor::CorridorInfo;

pub fn generate_rooms(config: &BuildingConfig) -> (Vec<Room>, Option<CorridorInfo>) {
    let specs = ordered_room_specs(config);

    if specs.is_empty() {
        return (vec![Room::new(0, config.footprint, "room")], None);
    }

    if specs.len() == 1 {
        return (vec![Room::new(0, config.footprint, &specs[0].name)], None);
    }

    let layout_axes = axes::determine_layout_axes(config);

    scoring::choose_best_layout(config, layout_axes, &specs)
}

pub fn entrance_door_position(config: &BuildingConfig) -> Vec2 {
    let fp = config.footprint;
    let entrance = config.entrance;

    if config.entrance_dir.y.abs() >= config.entrance_dir.x.abs() {
        let y = if config.entrance_dir.y >= 0.0 {
            fp.min.y
        } else {
            fp.max.y
        };
        Vec2::new(entrance.x.clamp(fp.min.x, fp.max.x), y)
    } else {
        let x = if config.entrance_dir.x >= 0.0 {
            fp.min.x
        } else {
            fp.max.x
        };
        Vec2::new(x, entrance.y.clamp(fp.min.y, fp.max.y))
    }
}

fn ordered_room_specs(config: &BuildingConfig) -> Vec<&RoomSpec> {
    let mut specs: Vec<_> = config.room_specs.iter().collect();
    specs.sort_by_key(|spec| spec.placement);
    specs
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::RoomSpec;
    use crate::geometry::Rect;

    fn default_config() -> BuildingConfig {
        BuildingConfig {
            footprint: Rect::new(0.0, 0.0, 10.0, 8.0),
            tile_size: 0.5,
            min_room_size: 2.5,
            room_specs: vec![
                RoomSpec::new("hall", 1),
                RoomSpec::new("kitchen", 2),
                RoomSpec::new("bedroom", 1),
                RoomSpec::new("bathroom", 0),
            ],
            ..Default::default()
        }
    }

    #[test]
    fn test_single_room() {
        let config = BuildingConfig::default();
        let (rooms, corridor) = generate_rooms(&config);
        assert_eq!(rooms.len(), 1);
        assert_eq!(rooms[0].label, "room");
        assert!(corridor.is_none());
    }

    #[test]
    fn test_room_count_matches_specs() {
        let config = default_config();
        let (rooms, _) = generate_rooms(&config);
        assert_eq!(rooms.len(), config.room_specs.len());
    }

    #[test]
    fn test_labels_assigned_in_order() {
        let config = default_config();
        let (rooms, _) = generate_rooms(&config);
        assert_eq!(rooms[0].label, "hall");
        assert_eq!(rooms[1].label, "kitchen");
        assert_eq!(rooms[2].label, "bathroom");
        assert_eq!(rooms[3].label, "bedroom");
    }

    #[test]
    fn test_preferred_area_controls_room_size() {
        let config = BuildingConfig {
            footprint: Rect::new(0.0, 0.0, 12.0, 8.0),
            tile_size: 0.5,
            room_specs: vec![
                RoomSpec::new("bathroom", 0).with_area(3.0, 4.0),
                RoomSpec::new("kitchen", 1).with_area(8.0, 18.0),
            ],
            ..Default::default()
        };
        let (rooms, _) = generate_rooms(&config);
        let bathroom = rooms.iter().find(|room| room.label == "bathroom").unwrap();
        let kitchen = rooms.iter().find(|room| room.label == "kitchen").unwrap();

        assert!(kitchen.bounds.area() > bathroom.bounds.area());
    }

    #[test]
    fn test_candidate_scoring_avoids_skinny_room_strip() {
        let config = BuildingConfig {
            footprint: Rect::new(0.0, 0.0, 12.0, 8.0),
            tile_size: 0.5,
            room_specs: vec![
                RoomSpec::new("hall", 0),
                RoomSpec::new("kitchen", 1),
                RoomSpec::new("bathroom", 0),
                RoomSpec::new("bedroom", 1),
            ],
            ..Default::default()
        };
        let (rooms, corridor) = generate_rooms(&config);

        assert!(corridor.is_none());
        assert!(rooms.iter().all(|room| {
            let ratio = (room.bounds.width() / room.bounds.height())
                .max(room.bounds.height() / room.bounds.width());
            ratio <= 2.5
        }));
    }

    #[test]
    fn test_auto_corridor_chooses_corridor_for_large_program() {
        let config = BuildingConfig {
            footprint: Rect::new(0.0, 0.0, 14.0, 10.0),
            tile_size: 0.5,
            auto_corridor: true,
            room_specs: vec![
                RoomSpec::new("hall", 0),
                RoomSpec::new("kitchen", 1),
                RoomSpec::new("bathroom", 0),
                RoomSpec::new("bedroom", 1),
                RoomSpec::new("bedroom", 1),
                RoomSpec::new("storage", 0),
            ],
            ..Default::default()
        };
        let (_, corridor) = generate_rooms(&config);

        assert!(corridor.is_some());
    }

    #[test]
    fn test_rooms_fill_footprint() {
        let config = default_config();
        let (rooms, _) = generate_rooms(&config);

        for room in &rooms {
            assert!(room.bounds.min.x >= config.footprint.min.x - 0.01);
            assert!(room.bounds.min.y >= config.footprint.min.y - 0.01);
            assert!(room.bounds.max.x <= config.footprint.max.x + 0.01);
            assert!(room.bounds.max.y <= config.footprint.max.y + 0.01);
        }
    }

    #[test]
    fn test_rooms_no_overlap() {
        let config = default_config();
        let (rooms, _) = generate_rooms(&config);

        for i in 0..rooms.len() {
            for j in (i + 1)..rooms.len() {
                assert!(
                    !rooms[i].bounds.intersects(rooms[j].bounds),
                    "Rooms {} ({}) and {} ({}) overlap",
                    i,
                    rooms[i].label,
                    j,
                    rooms[j].label,
                );
            }
        }
    }

    #[test]
    fn test_corridor_mode_odd_rooms() {
        let config = BuildingConfig {
            footprint: Rect::new(0.0, 0.0, 10.0, 8.0),
            tile_size: 0.5,
            min_room_size: 2.5,
            room_specs: vec![
                RoomSpec::new("a", 0),
                RoomSpec::new("b", 0),
                RoomSpec::new("c", 0),
                RoomSpec::new("d", 0),
                RoomSpec::new("e", 0),
            ],
            has_corridor: true,
            corridor_width: 1.0,
            ..Default::default()
        };
        let (rooms, corridor) = generate_rooms(&config);

        assert_eq!(rooms.len(), 5);
        assert!(corridor.is_some());

        assert_eq!(rooms[0].label, "a");
        assert_eq!(rooms[1].label, "b");
        assert_eq!(rooms[2].label, "c");
        assert_eq!(rooms[3].label, "d");
        assert_eq!(rooms[4].label, "e");

        let cb = corridor.unwrap().bounds;
        assert!(cb.min.x > config.footprint.min.x);
        assert!(cb.max.x < config.footprint.max.x);
        assert!((cb.height() - 8.0).abs() < 0.1);

        assert!(rooms.iter().any(|room| room.bounds.max.x <= cb.min.x));
        assert!(rooms.iter().any(|room| room.bounds.min.x >= cb.max.x));

        let room_width = rooms[0].bounds.width();
        for room in &rooms {
            assert!((room.bounds.width() - room_width).abs() < 0.1);
        }
    }

    #[test]
    fn test_corridor_mode_even_rooms() {
        let config = BuildingConfig {
            footprint: Rect::new(0.0, 0.0, 10.0, 8.0),
            tile_size: 0.5,
            min_room_size: 2.5,
            room_specs: vec![
                RoomSpec::new("a", 0),
                RoomSpec::new("b", 0),
                RoomSpec::new("c", 0),
                RoomSpec::new("d", 0),
            ],
            has_corridor: true,
            corridor_width: 1.0,
            ..Default::default()
        };
        let (rooms, corridor) = generate_rooms(&config);

        assert_eq!(rooms.len(), 4);
        assert!(corridor.is_some());

        assert_eq!(rooms[0].label, "a");
        assert_eq!(rooms[1].label, "b");
        assert_eq!(rooms[2].label, "c");
        assert_eq!(rooms[3].label, "d");

        let cb = corridor.unwrap().bounds;
        assert!((cb.min.x - 4.5).abs() < 0.1);
        assert!((cb.max.x - 5.5).abs() < 0.1);
        assert_eq!(rooms[0].bounds.max.x, cb.min.x);
        assert_eq!(rooms[1].bounds.min.x, cb.max.x);
        assert_eq!(rooms[2].bounds.max.x, cb.min.x);
        assert_eq!(rooms[3].bounds.min.x, cb.max.x);
    }

    #[test]
    fn test_corridor_width_tiles_controls_corridor_size() {
        let config = BuildingConfig {
            footprint: Rect::new(0.0, 0.0, 12.0, 8.0),
            tile_size: 0.5,
            corridor_width_tiles: 4,
            room_specs: vec![
                RoomSpec::new("hall", 0),
                RoomSpec::new("kitchen", 0),
                RoomSpec::new("bedroom", 0),
                RoomSpec::new("bathroom", 0),
            ],
            has_corridor: true,
            ..Default::default()
        };
        let (_, corridor) = generate_rooms(&config);
        let cb = corridor.unwrap().bounds;

        assert!((cb.width() - 2.0).abs() < 0.1);
    }

    #[test]
    fn test_corridor_follows_offset_entrance_without_touching_edge() {
        let config = BuildingConfig {
            footprint: Rect::new(0.0, 0.0, 12.0, 8.0),
            entrance: Vec2::new(8.0, 0.0),
            tile_size: 0.5,
            min_room_size: 2.5,
            room_specs: vec![
                RoomSpec::new("hall", 0),
                RoomSpec::new("kitchen", 0),
                RoomSpec::new("bedroom", 0),
                RoomSpec::new("bathroom", 0),
            ],
            has_corridor: true,
            corridor_width: 1.0,
            ..Default::default()
        };
        let (_, corridor) = generate_rooms(&config);
        let cb = corridor.unwrap().bounds;

        assert!((cb.center().x - 8.0).abs() < 0.1);
        assert!(cb.min.x > config.footprint.min.x);
        assert!(cb.max.x < config.footprint.max.x);
    }

    #[test]
    fn test_entrance_door_position_south() {
        let config = BuildingConfig {
            footprint: Rect::new(0.0, 0.0, 10.0, 8.0),
            entrance: Vec2::new(2.5, 0.0),
            entrance_dir: Vec2::new(0.0, 1.0),
            ..Default::default()
        };
        let pos = entrance_door_position(&config);
        assert!((pos.x - 2.5).abs() < 0.01);
        assert!((pos.y - 0.0).abs() < 0.01);
    }

    #[test]
    fn test_entrance_door_position_west() {
        let config = BuildingConfig {
            footprint: Rect::new(0.0, 0.0, 10.0, 8.0),
            entrance: Vec2::new(0.0, 6.0),
            entrance_dir: Vec2::new(1.0, 0.0),
            ..Default::default()
        };
        let pos = entrance_door_position(&config);
        assert!((pos.x - 0.0).abs() < 0.01);
        assert!((pos.y - 6.0).abs() < 0.01);
    }

    #[test]
    fn test_room_order_starts_at_north_entrance() {
        let config = BuildingConfig {
            entrance: Vec2::new(5.0, 8.0),
            entrance_dir: Vec2::new(0.0, -1.0),
            room_specs: vec![
                RoomSpec::new("hall", 0),
                RoomSpec::new("kitchen", 0),
                RoomSpec::new("bathroom", 0),
                RoomSpec::new("bedroom", 0),
                RoomSpec::new("storage", 0),
            ],
            ..default_config()
        };
        let (rooms, _) = generate_rooms(&config);

        assert_eq!(rooms[0].label, "hall");
        assert!(rooms[0].bounds.min.y > rooms[4].bounds.min.y);
    }

    #[test]
    fn test_snap_to_grid() {
        assert_eq!(splits::snap_to_grid(1.23, 0.5), 1.0);
        assert_eq!(splits::snap_to_grid(1.26, 0.5), 1.5);
        assert_eq!(splits::snap_to_grid(2.0, 1.0), 2.0);
    }

    #[test]
    fn test_distribute_splits_even() {
        let splits_result = splits::distribute_splits(0.0, 10.0, 2, 0.5);
        assert_eq!(splits_result.len(), 3);
        assert_eq!(splits_result[0], 0.0);
        assert_eq!(splits_result[2], 10.0);
        assert_eq!(splits_result[1] % 0.5, 0.0);
    }
}
