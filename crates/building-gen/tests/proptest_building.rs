use proptest::prelude::*;
use building_gen::config::{BuildingConfig, RoomSpec};
use building_gen::geometry::Rect;

fn valid_config() -> impl Strategy<Value = BuildingConfig> {
    (3.0f32..20.0, 3.0f32..20.0, 1usize..=6, 0.3f32..1.0)
        .prop_map(|(width, height, room_count, tile_size)| {
            let tile_size = tile_size.max(0.3);
            let room_specs: Vec<RoomSpec> = (0..room_count)
                .map(|i| RoomSpec::new(&format!("room_{}", i), 0))
                .collect();
            BuildingConfig {
                footprint: Rect::new(0.0, 0.0, width, height),
                tile_size,
                room_specs,
                min_room_size: tile_size * 3.0,
                wall_thickness: tile_size,
                interior_wall_thickness: tile_size * 0.4,
                ..Default::default()
            }
        })
}

proptest! {
    #[test]
    fn rooms_never_overlap(config in valid_config()) {
        let layout = building_gen::generate_layout(&config);

        for i in 0..layout.rooms.len() {
            for j in (i + 1)..layout.rooms.len() {
                let a = &layout.rooms[i];
                let b = &layout.rooms[j];
                let overlaps = a.bounds.min.x < b.bounds.max.x
                    && a.bounds.max.x > b.bounds.min.x
                    && a.bounds.min.y < b.bounds.max.y
                    && a.bounds.max.y > b.bounds.min.y;
                assert!(
                    !overlaps,
                    "Rooms {} ({}) and {} ({}) overlap: {:?} vs {:?}",
                    i, a.label, j, b.label, a.bounds, b.bounds
                );
            }
        }
    }

    #[test]
    fn room_count_matches_specs(config in valid_config()) {
        let layout = building_gen::generate_layout(&config);
        assert_eq!(
            layout.rooms.len(),
            config.room_specs.len(),
            "Expected {} rooms, got {}",
            config.room_specs.len(),
            layout.rooms.len()
        );
    }

    #[test]
    fn rooms_within_footprint(config in valid_config()) {
        let layout = building_gen::generate_layout(&config);
        let fp = config.footprint;

        for room in &layout.rooms {
            assert!(
                room.bounds.min.x >= fp.min.x - 0.01,
                "Room {} extends left of footprint: {} < {}",
                room.label, room.bounds.min.x, fp.min.x
            );
            assert!(
                room.bounds.min.y >= fp.min.y - 0.01,
                "Room {} extends below footprint: {} < {}",
                room.label, room.bounds.min.y, fp.min.y
            );
            assert!(
                room.bounds.max.x <= fp.max.x + 0.01,
                "Room {} extends right of footprint: {} > {}",
                room.label, room.bounds.max.x, fp.max.x
            );
            assert!(
                room.bounds.max.y <= fp.max.y + 0.01,
                "Room {} extends above footprint: {} > {}",
                room.label, room.bounds.max.y, fp.max.y
            );
        }
    }

    #[test]
    fn rooms_have_positive_area(config in valid_config()) {
        let layout = building_gen::generate_layout(&config);

        for room in &layout.rooms {
            let area = room.bounds.area();
            assert!(
                area > 0.0,
                "Room {} has non-positive area: {}",
                room.label, area
            );
        }
    }

    #[test]
    fn tile_grid_dimensions_match_footprint(config in valid_config()) {
        let layout = building_gen::generate_layout(&config);
        let expected_x = config.tiles_x();
        let expected_y = config.tiles_y();

        assert_eq!(layout.tile_grid.width, expected_x);
        assert_eq!(layout.tile_grid.height, expected_y);
    }

    #[test]
    fn all_floor_tiles_reachable(config in valid_config()) {
        let layout = building_gen::generate_layout(&config);
        let grid = &layout.tile_grid;

        // Find first floor tile
        let start = (0..grid.height)
            .flat_map(|y| (0..grid.width).map(move |x| (x, y)))
            .find(|&(x, y)| grid.get(x, y) == building_gen::tile::TileType::Floor);

        let Some(start) = start else {
            // No floor tiles — that's valid for degenerate configs
            return Ok(());
        };

        // BFS flood fill
        let mut visited = vec![vec![false; grid.width]; grid.height];
        let mut queue = std::collections::VecDeque::new();
        visited[start.1][start.0] = true;
        queue.push_back(start);

        while let Some((x, y)) = queue.pop_front() {
            for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                let nx = x as isize + dx;
                let ny = y as isize + dy;
                if nx < 0 || ny < 0 || nx as usize >= grid.width || ny as usize >= grid.height {
                    continue;
                }
                let (nx, ny) = (nx as usize, ny as usize);
                if visited[ny][nx] {
                    continue;
                }
                let tile = grid.get(nx, ny);
                if tile == building_gen::tile::TileType::Floor
                    || tile.is_wall()
                    || tile.is_opening()
                {
                    visited[ny][nx] = true;
                    queue.push_back((nx, ny));
                }
            }
        }

        // Check all floor tiles were reached
        for y in 0..grid.height {
            for x in 0..grid.width {
                if grid.get(x, y) == building_gen::tile::TileType::Floor {
                    assert!(
                        visited[y][x],
                        "Floor tile at ({}, {}) is not reachable from ({}, {})",
                        x, y, start.0, start.1
                    );
                }
            }
        }
    }

    #[test]
    fn layout_is_deterministic(config in valid_config()) {
        let layout1 = building_gen::generate_layout(&config);
        let layout2 = building_gen::generate_layout(&config);

        assert_eq!(layout1.rooms.len(), layout2.rooms.len());
        for (a, b) in layout1.rooms.iter().zip(layout2.rooms.iter()) {
            assert_eq!(a.label, b.label);
            assert!((a.bounds.min.x - b.bounds.min.x).abs() < 0.01);
            assert!((a.bounds.min.y - b.bounds.min.y).abs() < 0.01);
            assert!((a.bounds.max.x - b.bounds.max.x).abs() < 0.01);
            assert!((a.bounds.max.y - b.bounds.max.y).abs() < 0.01);
        }
    }
}
