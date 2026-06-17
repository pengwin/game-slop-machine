mod screenshot;

use bevy::asset::RenderAssetUsages;
use bevy::mesh::Indices;
use bevy::render::render_resource::PrimitiveTopology;
use bevy::{app::ScheduleRunnerPlugin, prelude::*, window::ExitCondition, winit::WinitPlugin};
use building_gen::config::{BuildingConfig, RoomSpec};
use building_gen::district::config::TradeDistrictConfig;
use building_gen::district::generate_district;
use building_gen::district::layout::TradeDistrictLayout;
use building_gen::geometry::{Rect, Vec2};
use building_gen::mesh::{generate_building_mesh, MeshData};
use building_gen::tile::{CardinalDir, TileGrid, TileType, WallOpening, WallShape, WallTile};
use building_gen::tile_converter::classify_wall_tiles;
use game_core::plugins::building::render::spawn_building_layout;
use game_core::plugins::scene::camera_config::CameraConfig;
use game_core::plugins::scene::scene_config::SceneConfig;
use game_core::plugins::GamePlugin;
use std::time::Duration;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let output_path = args
        .get(1)
        .cloned()
        .unwrap_or_else(|| "output.png".to_string());
    let fixture = args
        .get(2)
        .cloned()
        .unwrap_or_else(|| "procedural".to_string());

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
        .insert_resource(HeadlessFixture(fixture))
        .add_systems(
            Startup,
            (generate_building, screenshot::setup_screenshot).chain(),
        )
        .add_systems(Update, screenshot::capture_and_exit)
        .run();
}

#[derive(Resource)]
struct HeadlessFixture(String);

fn spawn_lot_debug_view(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    district: &TradeDistrictLayout,
) -> usize {
    let mut entity_count = 0;

    for (i, lot) in district.lots.iter().enumerate() {
        let width_axis_rotation = lot.rotation + std::f32::consts::FRAC_PI_2;

        let lot_mesh = make_ground_quad(Vec3::ZERO, lot.width, lot.depth);
        commands.spawn((
            Mesh3d(meshes.add(lot_mesh)),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: Color::srgb(0.82, 0.75, 0.55),
                perceptual_roughness: 0.95,
                ..default()
            })),
            Transform {
                translation: Vec3::new(lot.position.x, 0.01, lot.position.y),
                rotation: Quat::from_rotation_y(-width_axis_rotation),
                ..default()
            },
            Name::new(format!("Lot {}", i)),
        ));
        entity_count += 1;

        let marker_width = (lot.width * 0.18).clamp(0.8, 1.8);
        let marker_depth = 0.45;
        let marker_mesh = make_ground_quad(Vec3::ZERO, marker_width, marker_depth);
        commands.spawn((
            Mesh3d(meshes.add(marker_mesh)),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: Color::srgb(0.45, 0.38, 0.28),
                perceptual_roughness: 0.95,
                ..default()
            })),
            Transform {
                translation: Vec3::new(lot.entrance.x, 0.015, lot.entrance.y),
                rotation: Quat::from_rotation_y(-width_axis_rotation),
                ..default()
            },
            Name::new(format!("Lot {} Entrance", i)),
        ));
        entity_count += 1;
    }

    entity_count
}

fn generate_building(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    fixture: Res<HeadlessFixture>,
) {
    if is_district_fixture(&fixture.0) {
        commands.insert_resource(district_camera_for_fixture(&fixture.0));
        commands.insert_resource(SceneConfig {
            ground_size: district_ground_size_for_fixture(&fixture.0),
            ..Default::default()
        });

        let district_config = district_config_for_fixture(&fixture.0);
        let district = generate_district(&district_config);
        let mut entity_count = 0;
        let show_lots = is_district_lots_fixture(&fixture.0);

        // Town square
        let sq = district_config.town_square_radius;
        let sq_mesh = make_ground_quad(Vec3::new(0.0, 0.005, 0.0), sq * 2.0, sq * 2.0);
        commands.spawn((
            Mesh3d(meshes.add(sq_mesh)),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: Color::srgb(0.76, 0.70, 0.50),
                perceptual_roughness: 0.95,
                ..default()
            })),
            Transform::default(),
            Name::new("Town Square"),
        ));
        entity_count += 1;

        // Roads
        for (i, road) in district.roads.iter().enumerate() {
            let dx = road.end.x - road.start.x;
            let dz = road.end.y - road.start.y;
            let length = (dx * dx + dz * dz).sqrt();
            if length < 0.01 {
                continue;
            }
            let angle = dz.atan2(dx);
            let cx = (road.start.x + road.end.x) / 2.0;
            let cz = (road.start.y + road.end.y) / 2.0;

            let road_mesh = make_ground_quad(Vec3::ZERO, length, road.width);
            commands.spawn((
                Mesh3d(meshes.add(road_mesh)),
                MeshMaterial3d(materials.add(StandardMaterial {
                    base_color: Color::srgb(0.55, 0.45, 0.35),
                    perceptual_roughness: 0.95,
                    ..default()
                })),
                Transform {
                    translation: Vec3::new(cx, 0.005, cz),
                    rotation: Quat::from_rotation_y(-angle),
                    ..default()
                },
                Name::new(format!("Road {}", i)),
            ));
            entity_count += 1;
        }

        if show_lots {
            entity_count +=
                spawn_lot_debug_view(&mut commands, &mut meshes, &mut materials, &district);
        } else {
            for building in &district.buildings {
                let lot = &district.lots[building.lot_index];
                let door_local_position = building
                    .exterior_door_position()
                    .unwrap_or(building.config.entrance);
                let door_world_position = local_to_world(
                    building.world_position,
                    building.rotation,
                    door_local_position,
                );
                let spawned = spawn_building_layout(
                    &mut commands,
                    &mut meshes,
                    &mut materials,
                    &building.config,
                    &building.layout,
                    Transform {
                        translation: Vec3::new(
                            building.world_position.x,
                            0.0,
                            building.world_position.y,
                        ),
                        rotation: Quat::from_rotation_y(building.rotation),
                        ..default()
                    },
                    &format!("District Building {}", building.lot_index),
                );
                entity_count += spawned.len();
                if spawn_entrance_approach(
                    &mut commands,
                    &mut meshes,
                    &mut materials,
                    lot.entrance,
                    door_world_position,
                    building.lot_index,
                ) {
                    entity_count += 1;
                }
            }
        }

        println!(
            "District generated: {} lots, {} buildings, {} roads, {} entities",
            district.lots.len(),
            district.buildings.len(),
            district.roads.len(),
            entity_count
        );
        return;
    }

    let config = config_for_fixture(&fixture.0);
    let grid = match fixture.0.as_str() {
        "procedural" | "with-roof" | "corridor" => {
            building_gen::generate_layout(&config, 42).tile_grid
        }
        "four-doors" => {
            build_perimeter_opening_grid(&config, WallOpening::Door { render_panel: true })
        }
        "four-windows" => build_perimeter_opening_grid(
            &config,
            WallOpening::Window {
                render_glass: config.exterior_window_render_glass,
            },
        ),
        _ => build_two_room_grid(&config),
    };

    // Debug: print opening tiles
    for y in 0..grid.height {
        for x in 0..grid.width {
            if let TileType::Wall(wall) = grid.get(x, y) {
                if let Some(opening) = wall.opening {
                    println!("  Opening at ({}, {}): {:?}", x, y, opening);
                }
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

    let roof = building_gen::roof::generate_roof(config.footprint, &config);
    let bmesh = generate_building_mesh(&grid, &config, &roof);

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

    println!("{} test building generated", fixture.0);
}

fn config_for_fixture(fixture: &str) -> BuildingConfig {
    match fixture {
        "procedural" => BuildingConfig {
            room_specs: vec![
                RoomSpec::new("hall", 1),
                RoomSpec::new("kitchen", 2),
                RoomSpec::new("bedroom", 1),
                RoomSpec::new("bathroom", 0),
            ],
            ..Default::default()
        },
        "with-roof" => BuildingConfig {
            room_specs: vec![
                RoomSpec::new("hall", 1),
                RoomSpec::new("kitchen", 2),
                RoomSpec::new("bedroom", 1),
                RoomSpec::new("bathroom", 0),
            ],
            render_roof: true,
            ..Default::default()
        },
        "corridor" => BuildingConfig {
            room_specs: vec![
                RoomSpec::new("hall", 1),
                RoomSpec::new("kitchen", 2),
                RoomSpec::new("bedroom", 1),
                RoomSpec::new("bathroom", 0),
            ],
            has_corridor: true,
            corridor_width: 1.0,
            render_roof: false,
            ..Default::default()
        },
        _ => BuildingConfig {
            footprint: Rect::new(0.0, 0.0, 8.0, 6.0),
            tile_size: 1.0,
            wall_thickness: 1.0,
            interior_wall_thickness: 0.18,
            wall_height: 3.0,
            door_width: 0.8,
            room_specs: vec![RoomSpec::new("room", 0)],
            ..Default::default()
        },
    }
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

    // Interior door connecting the two rooms.
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

fn make_ground_quad(center: Vec3, width: f32, depth: f32) -> Mesh {
    let hw = width / 2.0;
    let hd = depth / 2.0;
    let mut mesh = Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD,
    );

    let cx = center.x;
    let cy = center.y;
    let cz = center.z;

    mesh.insert_attribute(
        Mesh::ATTRIBUTE_POSITION,
        vec![
            [cx - hw, cy, cz - hd],
            [cx + hw, cy, cz - hd],
            [cx + hw, cy, cz + hd],
            [cx - hw, cy, cz + hd],
        ],
    );
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, vec![[0.0, 1.0, 0.0]; 4]);
    mesh.insert_attribute(
        Mesh::ATTRIBUTE_UV_0,
        vec![[0.0, 0.0], [1.0, 0.0], [1.0, 1.0], [0.0, 1.0]],
    );
    mesh.insert_indices(Indices::U32(vec![0, 2, 1, 0, 3, 2]));

    mesh
}

fn local_to_world(origin: Vec2, rotation: f32, local: Vec2) -> Vec2 {
    let sin = rotation.sin();
    let cos = rotation.cos();
    Vec2::new(
        origin.x + local.x * cos + local.y * sin,
        origin.y - local.x * sin + local.y * cos,
    )
}

fn is_district_fixture(fixture: &str) -> bool {
    matches!(
        fixture,
        "district"
            | "district-lots"
            | "district-no-roof"
            | "huge-trade-district"
            | "huge-trade-district-lots"
            | "huge-trade-district-no-roof"
    )
}

fn is_district_lots_fixture(fixture: &str) -> bool {
    matches!(fixture, "district-lots" | "huge-trade-district-lots")
}

fn district_camera_for_fixture(fixture: &str) -> CameraConfig {
    if fixture.starts_with("huge-trade-district") {
        CameraConfig {
            position: Vec3::new(86.0, 86.0, 86.0),
            target: Vec3::new(0.0, 0.0, 0.0),
            viewport_height: 145.0,
        }
    } else {
        CameraConfig {
            position: Vec3::new(42.0, 42.0, 42.0),
            target: Vec3::new(0.0, 0.0, 0.0),
            viewport_height: 70.0,
        }
    }
}

fn district_ground_size_for_fixture(fixture: &str) -> f32 {
    if fixture.starts_with("huge-trade-district") {
        210.0
    } else {
        100.0
    }
}

fn district_config_for_fixture(fixture: &str) -> TradeDistrictConfig {
    if fixture.starts_with("huge-trade-district") {
        let mut config = TradeDistrictConfig {
            seed: 42,
            ring_count: 4,
            ring_spacing: 20.0,
            lot_count: 36,
            radial_count: 6,
            lot_width: 1.0,
            lot_height: 0.42,
            lot_depth: 0.08,
            lot_width_randomness: 0.0,
            lot_height_randomness: 0.14,
            lot_depth_randomness: 0.05,
            building_lot_inset: 0.08,
            max_buildings_per_lot: 3,
            building_gap: 0.9,
            preserve_large_lot_area: 300.0,
            landmark_lot_chance: 0.36,
            standalone_lot_width_scale: 0.5,
            standalone_lot_depth_scale: 0.7,
            building_lot_split_chance: 0.9,
            one_building_lot_weight: 0.22,
            two_building_lot_weight: 0.43,
            three_building_lot_weight: 0.35,
            building_lot_split_jitter: 0.32,
            ..Default::default()
        };
        if fixture == "huge-trade-district-no-roof" {
            for desc in &mut config.building_descriptions {
                desc.render_roof = false;
            }
        }
        config
    } else {
        let mut config = TradeDistrictConfig {
            seed: 42,
            ring_spacing: 22.0,
            lot_gap: 0.55,
            lot_width: 1.0,
            lot_height: 0.46,
            lot_depth: 0.08,
            lot_width_randomness: 0.0,
            lot_height_randomness: 0.12,
            lot_depth_randomness: 0.05,
            building_lot_inset: 0.08,
            max_buildings_per_lot: 3,
            building_gap: 0.9,
            preserve_large_lot_area: 260.0,
            landmark_lot_chance: 0.42,
            standalone_lot_width_scale: 0.52,
            standalone_lot_depth_scale: 0.72,
            building_lot_split_chance: 0.86,
            one_building_lot_weight: 0.24,
            two_building_lot_weight: 0.43,
            three_building_lot_weight: 0.33,
            building_lot_split_jitter: 0.32,
            ..Default::default()
        };
        if fixture == "district-no-roof" {
            for desc in &mut config.building_descriptions {
                desc.render_roof = false;
            }
        }
        config
    }
}

fn spawn_entrance_approach(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    start: Vec2,
    end: Vec2,
    lot_index: usize,
) -> bool {
    let dx = end.x - start.x;
    let dz = end.y - start.y;
    let length = (dx * dx + dz * dz).sqrt();
    if length < 0.05 {
        return false;
    }

    let angle = dz.atan2(dx);
    let center = Vec2::new((start.x + end.x) / 2.0, (start.y + end.y) / 2.0);
    commands.spawn((
        Mesh3d(meshes.add(make_ground_quad(Vec3::ZERO, length, 0.75))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.24, 0.18, 0.12),
            perceptual_roughness: 0.95,
            ..default()
        })),
        Transform {
            translation: Vec3::new(center.x, 0.035, center.y),
            rotation: Quat::from_rotation_y(-angle),
            ..default()
        },
        Name::new(format!("District Building {} Entrance Approach", lot_index)),
    ));

    true
}
