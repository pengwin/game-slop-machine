use bevy::prelude::*;

use super::scene_config::SceneConfig;

#[derive(Component)]
pub struct Ground;

#[derive(Component)]
pub struct DemoCube;

pub fn spawn_objects(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    config: Res<SceneConfig>,
) {
    commands.spawn((
        Ground,
        Mesh3d(meshes.add(
            Plane3d::default()
                .mesh()
                .size(config.ground_size, config.ground_size),
        )),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.3, 0.5, 0.3),
            ..default()
        })),
    ));

    commands.spawn((
        DemoCube,
        Mesh3d(meshes.add(Cuboid::new(
            config.cube_size,
            config.cube_size,
            config.cube_size,
        ))),
        MeshMaterial3d(materials.add(StandardMaterial::from_color(Color::srgb(
            0.8, 0.2, 0.2,
        )))),
        Transform::from_xyz(0.0, config.cube_size / 2.0, 0.0),
    ));
}
