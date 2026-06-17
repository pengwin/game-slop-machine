use building_gen::config::{BuildingConfig, RoomSpec};
use building_gen::geometry::Rect;
use building_gen::tile::TileType;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let seed = args
        .get(1)
        .and_then(|s| s.parse::<u64>().ok())
        .unwrap_or(42);

    let width = args
        .get(2)
        .and_then(|s| s.parse::<f32>().ok())
        .unwrap_or(10.0);

    let height = args
        .get(3)
        .and_then(|s| s.parse::<f32>().ok())
        .unwrap_or(8.0);

    let room_specs = args
        .get(4)
        .map(|s| parse_room_specs(s))
        .unwrap_or_else(|| vec![RoomSpec::new("room", 0)]);

    let corridor_mode = args.get(5).map(String::as_str).unwrap_or("none");
    let has_corridor = matches!(corridor_mode, "corridor" | "true" | "1");
    let auto_corridor = matches!(corridor_mode, "auto");

    let corridor_width_tiles = args
        .get(6)
        .and_then(|s| s.parse::<usize>().ok())
        .unwrap_or_else(|| BuildingConfig::default().corridor_width_tiles);

    let config = BuildingConfig {
        footprint: Rect::new(0.0, 0.0, width, height),
        room_specs,
        has_corridor,
        auto_corridor,
        corridor_width_tiles,
        ..Default::default()
    };

    let layout = building_gen::generate_layout(&config);

    println!("Building Generator");
    println!("==================");
    println!(
        "Footprint: {}x{}, Seed: {}, Rooms: {}, Corridor: {}, Corridor Width: {} tiles",
        width,
        height,
        seed,
        layout.rooms.len(),
        layout.corridor.is_some(),
        config.corridor_width_tiles
    );
    println!();

    println!("Tile Statistics:");
    println!(
        "  Floor:    {}",
        layout.tile_grid.count_tiles(TileType::Floor)
    );
    println!(
        "  Wall:     {}",
        layout.tile_grid.count_matching_tiles(TileType::is_wall)
    );
    println!(
        "  Opening:  {}",
        layout.tile_grid.count_matching_tiles(TileType::is_opening)
    );
    println!(
        "  Empty:    {}",
        layout.tile_grid.count_tiles(TileType::Empty)
    );
    println!();

    println!("Rooms:");
    for room in &layout.rooms {
        println!(
            "  [{:?}] {}: ({:.1}, {:.1}) to ({:.1}, {:.1})",
            room.id,
            room.label,
            room.bounds.min.x,
            room.bounds.min.y,
            room.bounds.max.x,
            room.bounds.max.y
        );
    }
    println!();

    println!("ASCII Map (y=0 at bottom):");
    println!();

    for y in (0..layout.tile_grid.height).rev() {
        print!("{:3} ", y);
        for x in 0..layout.tile_grid.width {
            let tile = layout.tile_grid.get(x, y);
            let marker = layout
                .tile_grid
                .room_label(x, y)
                .and_then(|label| label.chars().next())
                .filter(|_| tile == TileType::Floor)
                .unwrap_or_else(|| tile.ascii_char());
            print!("{}", marker);
        }
        println!();
    }

    print!("    ");
    for x in 0..layout.tile_grid.width {
        print!("{}", x % 10);
    }
    println!();

    println!();
    println!("Legend:");
    println!("  . = Empty (outside)");
    println!("  -/| = Wall");
    println!("  + = Wall corner");
    println!("  T = T-junction");
    println!("  D = Open doorway");
    println!("  d = Door (exterior)");
    println!("  w = Window");
    println!("  letters = Floor room marker (first letter of room name)");
}

/// Parses room specs from a comma-separated string like "hall:1,kitchen:2,bedroom:0"
fn parse_room_specs(input: &str) -> Vec<RoomSpec> {
    input
        .split(',')
        .map(|s| {
            let parts: Vec<&str> = s.split(':').collect();
            let name = parts[0].trim().to_string();
            let windows = parts
                .get(1)
                .and_then(|w| w.trim().parse::<usize>().ok())
                .unwrap_or(0);
            RoomSpec::new(&name, windows)
        })
        .collect()
}
