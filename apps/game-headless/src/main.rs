mod screenshot;

use bevy::{app::ScheduleRunnerPlugin, prelude::*, window::ExitCondition, winit::WinitPlugin};
use bevy::asset::RenderAssetUsages;
use bevy::mesh::Indices;
use bevy::render::render_resource::PrimitiveTopology;
use building_gen::config::BuildingConfig;
use building_gen::geometry::{Rect, Vec2};
use building_gen::mesh::{generate_building_mesh, MeshData};
use building_gen::tile::{TileGrid, TileType};
use game_core::plugins::GamePlugin;
use std::time::Duration;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let output_path = args
        .get(1)
        .cloned()
        .unwrap_or_else(|| "output.png".to_string());

    App::new()
        .add_plugins((
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: None,
                    exit_condition: ExitCondition::DontExit,
                    ..default()
                })
                .disable::<WinitPlugin>(),
            ScheduleRunnerPlugin::run_loop(Duration::from_millis(100)),
            GamePlugin,
        ))
        .insert_resource(screenshot::ScreenshotConfig {
            path: output_path,
            width: 1280,
            height: 1024,
        })
        .add_systems(
            Startup,
            (generate_building, screenshot::setup_screenshot).chain(),
        )
        .add_systems(Update, screenshot::capture_and_exit)
        .run();
}

fn generate_building(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let config = BuildingConfig {
        footprint: Rect::new(0.0, 0.0, 5.0, 5.0),
        tile_size: 0.5,
        wall_thickness: 0.5,
        wall_height: 3.0,
        ..Default::default()
    };
    let grid = build_corner_grid(&config);

    // Debug: print wall orientations for window tiles
    for y in 0..grid.height {
        for x in 0..grid.width {
            if grid.get(x, y) == TileType::Window {
                let along_z = grid.wall_runs_along_z(x, y);
                println!("  Window at ({}, {}): runs_along_z={}", x, y, along_z);
            }
        }
    }

    // Debug: print tile grid as ASCII
    println!("Tile grid ({}x{}):", grid.width, grid.height);
    for y in (0..grid.height).rev() {
        print!("{:2} ", y);
        for x in 0..grid.width {
            let ch = match grid.get(x, y) {
                TileType::Empty => '.',
                TileType::Floor => ' ',
                TileType::Wall => '#',
                TileType::WallCorner => '+',
                TileType::Doorway => 'D',
                TileType::Door => 'd',
                TileType::Window => 'w',
            };
            print!("{}", ch);
        }
        println!();
    }
    print!("   ");
    for x in 0..grid.width {
        print!("{}", x % 10);
    }
    println!();

    let roof = building_gen::roof::generate_roof(config.footprint, &config);
    let bmesh = generate_building_mesh(&grid, &config, &roof);

    println!("Mesh stats:");
    println!("  wall:   {} verts, {} tris", bmesh.wall_mesh.vertices.len(), bmesh.wall_mesh.indices.len() / 3);
    println!("  floor:  {} verts, {} tris", bmesh.floor_mesh.vertices.len(), bmesh.floor_mesh.indices.len() / 3);
    println!("  roof:   {} verts, {} tris", bmesh.roof_mesh.vertices.len(), bmesh.roof_mesh.indices.len() / 3);
    println!("  door:   {} verts, {} tris", bmesh.door_mesh.vertices.len(), bmesh.door_mesh.indices.len() / 3);
    println!("  window: {} verts, {} tris", bmesh.window_mesh.vertices.len(), bmesh.window_mesh.indices.len() / 3);

    if !bmesh.wall_mesh.is_empty() {
        commands.spawn((
            Mesh3d(meshes.add(convert_mesh(&bmesh.wall_mesh))),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: Color::srgb(0.8, 0.8, 0.8),
                cull_mode: None,
                ..default()
            })),
            Transform::default(),
            Name::new("Walls"),
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

    if !bmesh.roof_mesh.is_empty() {
        commands.spawn((
            Mesh3d(meshes.add(convert_mesh(&bmesh.roof_mesh))),
            MeshMaterial3d(materials.add(Color::srgb(0.55, 0.35, 0.2))),
            Transform::default(),
            Name::new("Roof"),
        ));
    }

    if !bmesh.door_mesh.is_empty() {
        commands.spawn((
            Mesh3d(meshes.add(convert_mesh(&bmesh.door_mesh))),
            MeshMaterial3d(materials.add(Color::srgb(0.4, 0.2, 0.0))),
            Transform::default(),
            Name::new("Doors"),
        ));
    }

    if !bmesh.window_mesh.is_empty() {
        commands.spawn((
            Mesh3d(meshes.add(convert_mesh(&bmesh.window_mesh))),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: Color::srgb(0.3, 0.5, 1.0),
                cull_mode: None,
                ..default()
            })),
            Transform::default(),
            Name::new("Windows"),
        ));
    }

    println!("Corner test building generated");
}

fn build_corner_grid(config: &BuildingConfig) -> TileGrid {
    let w = config.tiles_x();
    let h = config.tiles_y();
    let origin = Vec2::new(config.footprint.min.x, config.footprint.min.y);
    let mut grid = TileGrid::new(w, h, config.tile_size, origin);

    for y in 0..h {
        for x in 0..w {
            if x == 0 || x == w - 1 || y == 0 || y == h - 1 {
                grid.set(x, y, TileType::Wall);
            } else {
                grid.set(x, y, TileType::Floor);
            }
        }
    }

    // Place one window per wall at the middle tile.
    let mid_x = w / 2;
    let mid_y = h / 2;
    grid.set(0, mid_y, TileType::Window);    // left wall
    grid.set(w - 1, mid_y, TileType::Window); // right wall
    grid.set(mid_x, 0, TileType::Window);    // bottom wall
    grid.set(mid_x, h - 1, TileType::Window); // top wall

    grid
}

fn convert_mesh(data: &MeshData) -> Mesh {
    let mut mesh = Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD,
    );

    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, data.vertices.clone());
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, data.normals.clone());
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, data.uvs.clone());
    mesh.insert_indices(Indices::U32(data.indices.clone()));

    mesh
}
