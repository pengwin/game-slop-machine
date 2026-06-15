use bevy::prelude::*;
use building_gen::district::generate_district;

use super::config::DistrictGenConfig;

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

    // Lots
    for (i, lot) in district.lots.iter().enumerate() {
        let width_axis_rotation = lot.rotation + std::f32::consts::FRAC_PI_2;

        let lot_mesh = make_ground_quad(Vec3::ZERO, lot.width, lot.depth);
        entities.push(
            commands
                .spawn((
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
                ))
                .id(),
        );

        // Entrance marker centered on the entrance point.
        let marker_width = (lot.width * 0.18).clamp(0.8, 1.8);
        let marker_depth = 0.45;
        let marker_mesh = make_ground_quad(Vec3::ZERO, marker_width, marker_depth);
        entities.push(
            commands
                .spawn((
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
                ))
                .id(),
        );
    }

    println!(
        "District spawned: {} lots, {} roads",
        district.lots.len(),
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
