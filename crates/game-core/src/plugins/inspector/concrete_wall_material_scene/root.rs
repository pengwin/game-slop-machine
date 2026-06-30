use bevy::prelude::*;

use super::{super::InspectorSceneState, scene_sets::ConcreteWallMaterialSceneSet};

pub fn plugin(app: &mut App) {
    app.add_systems(
        OnEnter(InspectorSceneState::ConcreteWallMaterial),
        spawn_concrete_wall_material_scene_root.in_set(ConcreteWallMaterialSceneSet::Root),
    )
    .add_systems(
        OnExit(InspectorSceneState::ConcreteWallMaterial),
        despawn_concrete_wall_material_scene,
    );
}

/// Marker applied to entities owned by the concrete wall material debug scene.
#[derive(Component)]
pub struct ConcreteWallMaterialSceneRoot;

fn spawn_concrete_wall_material_scene_root(mut commands: Commands<'_, '_>) {
    commands.spawn((
        Name::new("Concrete Wall Material Scene"),
        ConcreteWallMaterialSceneRoot,
        Transform::default(),
        Visibility::default(),
    ));
}

fn despawn_concrete_wall_material_scene(
    mut commands: Commands<'_, '_>,
    roots: Query<'_, '_, Entity, With<ConcreteWallMaterialSceneRoot>>,
) {
    for root in &roots {
        commands.entity(root).despawn_children().despawn();
    }

    commands.remove_resource::<super::material::ConcreteWallGeneration>();
    commands.remove_resource::<super::material::ConcreteWallGenerationProgress>();
    commands.remove_resource::<super::material::ConcreteWallGenerationRequest>();

    info!("Despawned Concrete wall material scene");
}
