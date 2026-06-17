use bevy::prelude::*;
use building_gen::district::generate_district;
use building_gen::district::layout::TradeDistrictLayout;
use building_gen::geometry::Vec2;
use game_core::plugins::building::mesh_util::{local_to_world, make_ground_quad};
use game_core::plugins::building::render::spawn_building_layout;
use game_core::plugins::scene::scene_config::SceneConfig;

use super::fixtures;

pub fn spawn_district(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    fixture: &str,
) {
    commands.insert_resource(fixtures::district_camera_for_fixture(fixture));
    commands.insert_resource(SceneConfig {
        ground_size: fixtures::district_ground_size_for_fixture(fixture),
    });

    let district_config = fixtures::district_config_for_fixture(fixture);
    let district = generate_district(&district_config);
    let mut entity_count = 0;
    let show_lots = fixtures::is_district_lots_fixture(fixture);

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
        entity_count += spawn_lot_debug_view(commands, meshes, materials, &district);
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
                commands,
                meshes,
                materials,
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
                commands,
                meshes,
                materials,
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
}

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
