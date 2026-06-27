use bevy::prelude::*;

use super::super::InspectorSceneState;

use super::scene_sets::SimpleSceneSet;

pub fn plugin(app: &mut App) {
    app.add_systems(
        OnEnter(InspectorSceneState::Simple),
        spawn_simple_scene_root.in_set(SimpleSceneSet::Root),
    )
    .add_systems(OnExit(InspectorSceneState::Simple), despawn_simple_scene);
}

/// Marker applied to entities owned by the simple preview scene.
#[derive(Component)]
pub struct SimpleSceneRoot;

fn spawn_simple_scene_root(mut commands: Commands<'_, '_>) {
    commands.spawn((
        Name::new("Simple Scene"),
        SimpleSceneRoot,
        Transform::default(),
        Visibility::default(),
    ));
}

fn despawn_simple_scene(
    mut commands: Commands<'_, '_>,
    roots: Query<'_, '_, Entity, With<SimpleSceneRoot>>,
) {
    for root in &roots {
        commands.entity(root).despawn_children().despawn();
    }

    info!("Despawned Simple scene");
}
