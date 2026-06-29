use bevy::prelude::*;

use super::{super::InspectorSceneState, scene_sets::PlasterWallMaterialSceneSet};

pub fn plugin(app: &mut App) {
    app.add_systems(
        OnEnter(InspectorSceneState::PlasterWallMaterial),
        spawn_plaster_wall_material_scene_root.in_set(PlasterWallMaterialSceneSet::Root),
    )
    .add_systems(
        OnExit(InspectorSceneState::PlasterWallMaterial),
        despawn_plaster_wall_material_scene,
    );
}

/// Marker applied to entities owned by the plaster wall material debug scene.
#[derive(Component)]
pub struct PlasterWallMaterialSceneRoot;

fn spawn_plaster_wall_material_scene_root(mut commands: Commands<'_, '_>) {
    commands.spawn((
        Name::new("Plaster Wall Material Scene"),
        PlasterWallMaterialSceneRoot,
        Transform::default(),
        Visibility::default(),
    ));
}

fn despawn_plaster_wall_material_scene(
    mut commands: Commands<'_, '_>,
    roots: Query<'_, '_, Entity, With<PlasterWallMaterialSceneRoot>>,
) {
    for root in &roots {
        commands.entity(root).despawn_children().despawn();
    }

    commands.remove_resource::<super::material::PlasterWallGeneration>();
    commands.remove_resource::<super::material::PlasterWallGenerationProgress>();
    commands.remove_resource::<super::material::PlasterWallGenerationRequest>();

    info!("Despawned Plaster wall material scene");
}
