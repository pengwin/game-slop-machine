use bevy::prelude::*;
use building_gen::config::BuildingConfig;
use building_gen::geometry::Vec2;
use building_gen::mesh::generate_building_mesh;
use building_gen::tile::{CardinalDir, TileGrid, TileType, WallOpening, WallShape, WallTile};
use building_gen::tile_converter::classify_wall_tiles;
use game_core::plugins::building::mesh_util::convert_mesh;

pub fn spawn_building_preview(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    config: &BuildingConfig,
    fixture: &str,
) {
    let grid = match fixture {
        "procedural" | "with-roof" | "corridor" => {
            building_gen::generate_layout(config).tile_grid
        }
        "four-doors" => build_perimeter_opening_grid(config, WallOpening::Door { render_panel: true }),
        "four-windows" => build_perimeter_opening_grid(
            config,
            WallOpening::Window {
                render_glass: config.exterior_window_render_glass,
            },
        ),
        _ => build_two_room_grid(config),
    };

    // Debug: print opening tiles
    for y in 0..grid.height {
        for x in 0..grid.width {
            if let TileType::Wall(wall) = grid.get(x, y)
                && let Some(opening) = wall.opening
            {
                println!("  Opening at ({}, {}): {:?}", x, y, opening);
            }
        }
    }

    // Debug: print tile grid as ASCII
    println!("Tile grid ({}x{}):", grid.width, grid.height);
    for y in (0..grid.height).rev() {
        print!("{:2} ", y);
        for x in 0..grid.width {
            print!("{}", grid.get(x, y).ascii_char());
        }
        println!();
    }
    print!("   ");
    for x in 0..grid.width {
        print!("{}", x % 10);
    }
    println!();

    let roof = building_gen::roof::generate_roof(config.footprint, config);
    let bmesh = generate_building_mesh(&grid, config, &roof);

    println!("Mesh stats:");
    println!(
        "  foundation: {} verts, {} tris",
        bmesh.foundation_mesh.vertices.len(),
        bmesh.foundation_mesh.indices.len() / 3
    );
    println!(
        "  wall:   {} verts, {} tris",
        bmesh.wall_mesh.vertices.len(),
        bmesh.wall_mesh.indices.len() / 3
    );
    println!(
        "  wall top: {} verts, {} tris",
        bmesh.wall_top_mesh.vertices.len(),
        bmesh.wall_top_mesh.indices.len() / 3
    );
    println!(
        "  exterior wall: {} verts, {} tris",
        bmesh.exterior_wall_mesh.vertices.len(),
        bmesh.exterior_wall_mesh.indices.len() / 3
    );
    println!(
        "  exterior corner: {} verts, {} tris",
        bmesh.exterior_corner_mesh.vertices.len(),
        bmesh.exterior_corner_mesh.indices.len() / 3
    );
    println!(
        "  exterior t-junction: {} verts, {} tris",
        bmesh.exterior_t_junction_mesh.vertices.len(),
        bmesh.exterior_t_junction_mesh.indices.len() / 3
    );
    println!(
        "  floor:  {} verts, {} tris",
        bmesh.floor_mesh.vertices.len(),
        bmesh.floor_mesh.indices.len() / 3
    );
    println!(
        "  roof:   {} verts, {} tris",
        bmesh.roof_mesh.vertices.len(),
        bmesh.roof_mesh.indices.len() / 3
    );
    println!(
        "  gable:  {} verts, {} tris",
        bmesh.gable_mesh.vertices.len(),
        bmesh.gable_mesh.indices.len() / 3
    );
    println!(
        "  door:   {} verts, {} tris",
        bmesh.door_mesh.vertices.len(),
        bmesh.door_mesh.indices.len() / 3
    );
    println!(
        "  opening trim: {} verts, {} tris",
        bmesh.opening_trim_mesh.vertices.len(),
        bmesh.opening_trim_mesh.indices.len() / 3
    );
    println!(
        "  window: {} verts, {} tris",
        bmesh.window_mesh.vertices.len(),
        bmesh.window_mesh.indices.len() / 3
    );

    if !bmesh.foundation_mesh.is_empty() {
        commands.spawn((
            Mesh3d(meshes.add(convert_mesh(&bmesh.foundation_mesh))),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: Color::srgb(0.42, 0.42, 0.38),
                perceptual_roughness: 0.95,
                ..default()
            })),
            Transform::default(),
            Name::new("Foundation"),
        ));
    }

    if !bmesh.wall_mesh.is_empty() {
        commands.spawn((
            Mesh3d(meshes.add(convert_mesh(&bmesh.wall_mesh))),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: Color::srgb(0.8, 0.8, 0.8),
                ..default()
            })),
            Transform::default(),
            Name::new("Walls"),
        ));
    }

    if !bmesh.wall_top_mesh.is_empty() {
        commands.spawn((
            Mesh3d(meshes.add(convert_mesh(&bmesh.wall_top_mesh))),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: Color::srgb(
                    config.visual_style.wall_top_color[0],
                    config.visual_style.wall_top_color[1],
                    config.visual_style.wall_top_color[2],
                ),
                unlit: true,
                ..default()
            })),
            Transform::default(),
            Name::new("Wall Top Faces"),
        ));
    }

    if !bmesh.exterior_wall_mesh.is_empty() {
        commands.spawn((
            Mesh3d(meshes.add(convert_mesh(&bmesh.exterior_wall_mesh))),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: Color::srgb(0.92, 0.88, 0.68),
                ..default()
            })),
            Transform::default(),
            Name::new("Exterior Wall Faces"),
        ));
    }

    if !bmesh.exterior_corner_mesh.is_empty() {
        commands.spawn((
            Mesh3d(meshes.add(convert_mesh(&bmesh.exterior_corner_mesh))),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: Color::srgb(0.96, 0.9, 0.62),
                ..default()
            })),
            Transform::default(),
            Name::new("Exterior Corner Faces"),
        ));
    }

    if !bmesh.exterior_t_junction_mesh.is_empty() {
        commands.spawn((
            Mesh3d(meshes.add(convert_mesh(&bmesh.exterior_t_junction_mesh))),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: Color::srgb(0.86, 0.78, 0.48),
                ..default()
            })),
            Transform::default(),
            Name::new("Exterior T-Junction Faces"),
        ));
    }

    if !bmesh.floor_mesh.is_empty() {
        commands.spawn((
            Mesh3d(meshes.add(convert_mesh(&bmesh.floor_mesh))),
            MeshMaterial3d(materials.add(Color::srgb(0.6, 0.6, 0.6))),
            Transform::default(),
            Name::new("Floor"),
        ));
    }

    if config.render_roof && !bmesh.roof_mesh.is_empty() {
        commands.spawn((
            Mesh3d(meshes.add(convert_mesh(&bmesh.roof_mesh))),
            MeshMaterial3d(materials.add(Color::srgb(0.55, 0.35, 0.2))),
            Transform::default(),
            Name::new("Roof"),
        ));
    }

    if config.render_roof && !bmesh.gable_mesh.is_empty() {
        commands.spawn((
            Mesh3d(meshes.add(convert_mesh(&bmesh.gable_mesh))),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: Color::srgb(0.92, 0.88, 0.68),
                cull_mode: None,
                ..default()
            })),
            Transform::default(),
            Name::new("Gables"),
        ));
    }

    if !bmesh.door_mesh.is_empty() {
        commands.spawn((
            Mesh3d(meshes.add(convert_mesh(&bmesh.door_mesh))),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: Color::srgb(0.4, 0.2, 0.0),
                cull_mode: None,
                ..default()
            })),
            Transform::default(),
            Name::new("Doors"),
        ));
    }

    if !bmesh.opening_trim_mesh.is_empty() {
        commands.spawn((
            Mesh3d(meshes.add(convert_mesh(&bmesh.opening_trim_mesh))),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: Color::srgb(0.18, 0.16, 0.13),
                cull_mode: None,
                ..default()
            })),
            Transform::default(),
            Name::new("Opening Trim"),
        ));
    }

    if !bmesh.window_mesh.is_empty() {
        commands.spawn((
            Mesh3d(meshes.add(convert_mesh(&bmesh.window_mesh))),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: Color::srgba(0.45, 0.7, 1.0, 0.45),
                alpha_mode: AlphaMode::Blend,
                cull_mode: None,
                ..default()
            })),
            Transform::default(),
            Name::new("Windows"),
        ));
    }

    println!("{} test building generated", fixture);
}

fn build_two_room_grid(config: &BuildingConfig) -> TileGrid {
    let w = config.tiles_x();
    let h = config.tiles_y();
    let origin = Vec2::new(config.footprint.min.x, config.footprint.min.y);
    let mut grid = TileGrid::new(w, h, config.tile_size, origin);
    let divider_y = h / 2;
    let door_x = w / 2;
    let wall = TileType::Wall(WallTile::exterior(WallShape::Straight(CardinalDir::Top)));

    for y in 0..h {
        for x in 0..w {
            if x == 0 || x == w - 1 || y == 0 || y == h - 1 || y == divider_y {
                grid.set(x, y, wall);
            } else {
                grid.set(x, y, TileType::Floor);
            }
        }
    }

    classify_wall_tiles(&mut grid);

    grid.set_wall_opening(
        door_x,
        divider_y,
        WallOpening::Door {
            render_panel: config.interior_door_render_panel,
        },
    );

    grid
}

fn build_perimeter_opening_grid(config: &BuildingConfig, opening: WallOpening) -> TileGrid {
    let w = config.tiles_x();
    let h = config.tiles_y();
    let origin = Vec2::new(config.footprint.min.x, config.footprint.min.y);
    let mut grid = TileGrid::new(w, h, config.tile_size, origin);
    let wall = TileType::Wall(WallTile::exterior(WallShape::Straight(CardinalDir::Top)));

    for y in 0..h {
        for x in 0..w {
            if x == 0 || x == w - 1 || y == 0 || y == h - 1 {
                grid.set(x, y, wall);
            } else {
                grid.set(x, y, TileType::Floor);
            }
        }
    }

    classify_wall_tiles(&mut grid);

    let mid_x = w / 2;
    let mid_y = h / 2;
    grid.set_wall_opening(0, mid_y, opening);
    grid.set_wall_opening(w - 1, mid_y, opening);
    grid.set_wall_opening(mid_x, 0, opening);
    grid.set_wall_opening(mid_x, h - 1, opening);

    grid
}
