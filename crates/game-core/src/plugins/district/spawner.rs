use bevy::prelude::*;
use building_gen::district::generate_district;

use super::config::DistrictGenConfig;
use crate::plugins::building::render::spawn_building_layout;

/// Tracks entities spawned for the current district.
#[derive(Resource)]
pub struct CurrentDistrict {
    pub entities: Vec<Entity>,
}

/// Spawns a trade district when the T key is pressed.
pub fn spawn_district_on_command(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    input: Res<ButtonInput<KeyCode>>,
    config: Res<DistrictGenConfig>,
    existing: Option<ResMut<CurrentDistrict>>,
) {
    if !input.just_pressed(KeyCode::KeyT) {
        return;
    }

    if let Some(mut existing) = existing {
        for entity in existing.entities.drain(..) {
            commands.entity(entity).despawn();
        }
    }

    let district = generate_district(&config.0);
    let mut entities = Vec::new();

    // Town square
    let sq = config.0.town_square_radius;
    let sq_mesh = make_ground_quad(Vec3::new(0.0, 0.005, 0.0), sq * 2.0, sq * 2.0);
    entities.push(
        commands
            .spawn((
                Mesh3d(meshes.add(sq_mesh)),
                MeshMaterial3d(materials.add(StandardMaterial {
                    base_color: Color::srgb(0.76, 0.70, 0.50),
                    perceptual_roughness: 0.95,
                    ..default()
                })),
                Transform::default(),
                Name::new("Town Square"),
            ))
            .id(),
    );

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
        entities.push(
            commands
                .spawn((
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
                ))
                .id(),
        );
    }

    // Buildings
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
        entities.extend(spawn_building_layout(
            &mut commands,
            &mut meshes,
            &mut materials,
            &building.config,
            &building.layout,
            Transform {
                translation: Vec3::new(building.world_position.x, 0.0, building.world_position.y),
                rotation: Quat::from_rotation_y(building.rotation),
                ..default()
            },
            &format!("District Building {}", building.lot_index),
        ));
        if let Some(approach) = spawn_entrance_approach(
            &mut commands,
            &mut meshes,
            &mut materials,
            lot.entrance,
            door_world_position,
            building.lot_index,
        ) {
            entities.push(approach);
        }
    }

    println!(
        "District spawned: {} lots, {} buildings, {} roads",
        district.lots.len(),
        district.buildings.len(),
        district.roads.len()
    );

    commands.insert_resource(CurrentDistrict { entities });
}

/// Creates a flat quad mesh at the given position.
fn make_ground_quad(center: Vec3, width: f32, depth: f32) -> Mesh {
    let hw = width / 2.0;
    let hd = depth / 2.0;
    let mut mesh = Mesh::new(
        bevy::render::render_resource::PrimitiveTopology::TriangleList,
        bevy::asset::RenderAssetUsages::MAIN_WORLD | bevy::asset::RenderAssetUsages::RENDER_WORLD,
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
    mesh.insert_indices(bevy::mesh::Indices::U32(vec![0, 2, 1, 0, 3, 2]));

    mesh
}

fn local_to_world(
    origin: building_gen::geometry::Vec2,
    rotation: f32,
    local: building_gen::geometry::Vec2,
) -> building_gen::geometry::Vec2 {
    let sin = rotation.sin();
    let cos = rotation.cos();
    building_gen::geometry::Vec2::new(
        origin.x + local.x * cos + local.y * sin,
        origin.y - local.x * sin + local.y * cos,
    )
}

fn spawn_entrance_approach(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    start: building_gen::geometry::Vec2,
    end: building_gen::geometry::Vec2,
    lot_index: usize,
) -> Option<Entity> {
    let dx = end.x - start.x;
    let dz = end.y - start.y;
    let length = (dx * dx + dz * dz).sqrt();
    if length < 0.05 {
        return None;
    }
    let angle = dz.atan2(dx);
    let center =
        building_gen::geometry::Vec2::new((start.x + end.x) / 2.0, (start.y + end.y) / 2.0);

    Some(
        commands
            .spawn((
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
            ))
            .id(),
    )
}
