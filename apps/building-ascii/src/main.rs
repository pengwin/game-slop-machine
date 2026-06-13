use building_gen::config::BuildingConfig;
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

    let config = BuildingConfig {
        footprint: Rect::new(0.0, 0.0, width, height),
        ..Default::default()
    };

    let layout = building_gen::generate_layout(&config, seed);

    println!("Building Generator");
    println!("==================");
    println!(
        "Footprint: {}x{}, Seed: {}, Rooms: {}",
        width, height, seed, layout.rooms.len()
    );
    println!();

    println!("Tile Statistics:");
    println!("  Floor:    {}", layout.tile_grid.count_tiles(TileType::Floor));
    println!("  Wall:     {}", layout.tile_grid.count_tiles(TileType::Wall));
    println!("  Corner:   {}", layout.tile_grid.count_tiles(TileType::WallCorner));
    println!("  Doorway:  {}", layout.tile_grid.count_tiles(TileType::Doorway));
    println!("  Door:     {}", layout.tile_grid.count_tiles(TileType::Door));
    println!("  Window:   {}", layout.tile_grid.count_tiles(TileType::Window));
    println!("  Empty:    {}", layout.tile_grid.count_tiles(TileType::Empty));
    println!();

    println!("Rooms:");
    for room in &layout.rooms {
        println!(
            "  Room {:?}: ({:.1}, {:.1}) to ({:.1}, {:.1})",
            room.id,
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
            let c = match tile {
                TileType::Empty => '.',
                TileType::Floor => ' ',
                TileType::Wall => '#',
                TileType::WallCorner => '+',
                TileType::Doorway => 'D',
                TileType::Door => 'd',
                TileType::Window => 'w',
            };
            print!("{}", c);
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
    println!("  # = Wall");
    println!("  + = Wall corner");
    println!("  D = Doorway (between rooms)");
    println!("  d = Door (exterior)");
    println!("  w = Window");
    println!("    = Floor (inside room)");
}
