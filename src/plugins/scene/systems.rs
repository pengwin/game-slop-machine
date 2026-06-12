use bevy::prelude::*;
use bevy::camera::ScalingMode;

use super::components::*;
use super::resources::*;

pub fn setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    config: Res<SceneConfig>,
) {
    commands.spawn((
        Camera3d::default(),
        Projection::Orthographic(OrthographicProjection {
            scaling_mode: ScalingMode::FixedVertical { viewport_height: 5.0 },
            ..OrthographicProjection::default_3d()
        }),
        Transform::from_xyz(10.0, 10.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    commands.spawn((
        DirectionalLight {
            illuminance: 10_000.0,
            shadows_enabled: true,
            ..default()
        },
        Transform::from_rotation(Quat::from_euler(
            EulerRot::XYZ,
            -std::f32::consts::FRAC_PI_4,
            std::f32::consts::FRAC_PI_4,
            0.0,
        )),
    ));

    commands.insert_resource(GlobalAmbientLight {
        color: Color::WHITE,
        brightness: 0.2,
        ..default()
    });

    commands.spawn((
        Ground,
        Mesh3d(meshes.add(Plane3d::default().mesh().size(config.ground_size, config.ground_size))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.3, 0.5, 0.3),
            ..default()
        })),
    ));

    commands.spawn((
        DemoCube,
        Mesh3d(meshes.add(Cuboid::new(config.cube_size, config.cube_size, config.cube_size))),
        MeshMaterial3d(materials.add(StandardMaterial::from_color(
            Color::srgb(0.8, 0.2, 0.2),
        ))),
        Transform::from_xyz(0.0, config.cube_size / 2.0, 0.0),
    ));
}
