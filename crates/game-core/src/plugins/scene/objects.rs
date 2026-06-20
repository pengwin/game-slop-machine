use bevy::prelude::*;

use super::scene_config::SceneConfig;

#[derive(Component)]
pub struct Ground;

pub fn spawn_objects(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    config: Res<SceneConfig>,
) {
    commands.spawn((
        Ground,
        Name::new("Ground"),
        Mesh3d(
            meshes.add(
                Plane3d::default()
                    .mesh()
                    .size(config.ground_size, config.ground_size),
            ),
        ),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: config.ground_color,
            ..default()
        })),
    ));
}
