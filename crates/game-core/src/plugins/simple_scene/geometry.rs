use bevy::prelude::*;

use crate::plugins::inspector_scene::InspectorScene;

use super::{layout, root::SimpleSceneRoot, scene_sets::SimpleSceneSet};

pub fn plugin(app: &mut App) {
    app.add_systems(
        OnEnter(InspectorScene::Simple),
        spawn_simple_scene_geometry.in_set(SimpleSceneSet::Content),
    );
}

#[derive(Component, Clone, Default)]
struct SimpleSceneGeometry;

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
    let [red_box, green_box, blue_box, gold_box] = layout::boxes();

    bsn_list![
        (
            Name::new("Simple Scene Plane")
            SimpleSceneGeometry
            Mesh3d(asset_value(
                Plane3d::default()
                    .mesh()
                    .size(layout::PLANE_SIZE, layout::PLANE_SIZE)
            ))
            MeshMaterial3d::<StandardMaterial>(asset_value(StandardMaterial {
                base_color: Color::srgb(0.30, 0.36, 0.32),
                perceptual_roughness: 0.9,
                ..default()
            }))
        ),
        (
            Name::new(red_box.name)
            SimpleSceneGeometry
            Mesh3d(asset_value(Cuboid::from_size(layout::BOX_SIZE)))
            MeshMaterial3d::<StandardMaterial>(asset_value(box_material(red_box.color)))
            Transform::from_translation(red_box.position)
        ),
        (
            Name::new(green_box.name)
            SimpleSceneGeometry
            Mesh3d(asset_value(Cuboid::from_size(layout::BOX_SIZE)))
            MeshMaterial3d::<StandardMaterial>(asset_value(box_material(green_box.color)))
            Transform::from_translation(green_box.position)
        ),
        (
            Name::new(blue_box.name)
            SimpleSceneGeometry
            Mesh3d(asset_value(Cuboid::from_size(layout::BOX_SIZE)))
            MeshMaterial3d::<StandardMaterial>(asset_value(box_material(blue_box.color)))
            Transform::from_translation(blue_box.position)
        ),
        (
            Name::new(gold_box.name)
            SimpleSceneGeometry
            Mesh3d(asset_value(Cuboid::from_size(layout::BOX_SIZE)))
            MeshMaterial3d::<StandardMaterial>(asset_value(box_material(gold_box.color)))
            Transform::from_translation(gold_box.position)
        ),
    ]
}

fn box_material(color: Color) -> StandardMaterial {
    StandardMaterial {
        base_color: color,
        perceptual_roughness: 0.65,
        ..default()
    }
}
