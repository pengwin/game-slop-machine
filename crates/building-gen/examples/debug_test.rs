use building_gen::config::BuildingConfig;
use building_gen::geometry::Rect;
use building_gen::tile::TileType;
use building_gen::*;

fn main() {
    let config = BuildingConfig {
        footprint: Rect::new(0.0, 0.0, 10.0, 8.0),
        tile_size: 0.5,
        min_room_size: 2.5,
        target_rooms: 4,
        ..Default::default()
    };

    let layout = generate_layout(&config, 42);

    println!("Rooms: {}", layout.rooms.len());
    println!(
        "Floor tiles: {}",
        layout.tile_grid.count_tiles(TileType::Floor)
    );
    println!(
        "Wall tiles: {}",
        layout.tile_grid.count_matching_tiles(TileType::is_wall)
    );
    println!(
        "Empty tiles: {}",
        layout.tile_grid.count_tiles(TileType::Empty)
    );
    println!(
        "Opening tiles: {}",
        layout.tile_grid.count_matching_tiles(TileType::is_opening)
    );

    println!("\nGrid visualization (y=0 at bottom):");
    for y in (0..layout.tile_grid.height).rev() {
        print!("{:2} ", y);
        for x in 0..layout.tile_grid.width {
            let tile = layout.tile_grid.get(x, y);
            print!("{}", tile.ascii_char());
        }
        println!();
    }
    print!("   ");
    for x in 0..layout.tile_grid.width {
        print!("{}", x % 10);
    }
    println!();

    println!("\nRooms:");
    for room in &layout.rooms {
        println!(
            "  Room {:?}: min=({}, {}), max=({}, {})",
            room.id, room.bounds.min.x, room.bounds.min.y, room.bounds.max.x, room.bounds.max.y
        );
    }
}
