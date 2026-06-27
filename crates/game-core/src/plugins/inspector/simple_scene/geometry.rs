use bevy::prelude::*;

use super::super::InspectorSceneState;

use super::{boxes, root::SimpleSceneRoot, scene_sets::SimpleSceneSet};

pub fn plugin(app: &mut App) {
    app.add_systems(
        OnEnter(InspectorSceneState::Simple),
        spawn_simple_scene_geometry.in_set(SimpleSceneSet::Content),
    );
}

fn spawn_simple_scene_geometry(
    mut commands: Commands<'_, '_>,
    root: Query<'_, '_, Entity, With<SimpleSceneRoot>>,
) {
    let Ok(root) = root.single() else {
        return;
    };

    commands
        .entity(root)
        .queue_spawn_related_scenes::<Children>(geometry_scene());

    info!("Spawned Simple scene geometry");
}

fn geometry_scene() -> impl SceneList {
    let mut scenes: Vec<Box<dyn Scene>> = Vec::with_capacity(5);

    scenes.push(Box::new(bsn!(
        Name::new("Simple Scene Plane")
        Mesh3d(asset_value(
            Plane3d::default()
                .mesh()
                .size(boxes::PLANE_SIZE, boxes::PLANE_SIZE)
        ))
        MeshMaterial3d::<StandardMaterial>(asset_value(StandardMaterial {
            base_color: Color::srgb(0.30, 0.36, 0.32),
            perceptual_roughness: 0.9,
            ..default()
        }))
    )));

    for box_item in boxes::boxes() {
        let boxed_scene = Box::new(box_item.scene());
        scenes.push(boxed_scene);
    }

    scenes
}
