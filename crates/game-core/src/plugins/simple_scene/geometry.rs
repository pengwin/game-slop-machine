use bevy::prelude::*;

use super::{root::SimpleSceneRoot, scene_sets::SimpleSceneSet, InspectorScene};

pub fn plugin(app: &mut App) {
    app.add_systems(
        OnEnter(InspectorScene::Simple),
        spawn_simple_scene_geometry.in_set(SimpleSceneSet::Content),
    );
}

#[derive(Component)]
struct SimpleSceneGeometry;

fn spawn_simple_scene_geometry(
    mut commands: Commands<'_, '_>,
    root: Query<'_, '_, Entity, With<SimpleSceneRoot>>,
    mut meshes: ResMut<'_, Assets<Mesh>>,
    mut materials: ResMut<'_, Assets<StandardMaterial>>,
) {
    let Ok(root) = root.single() else {
        return;
    };

    let plane_mesh = meshes.add(Plane3d::default().mesh().size(12.0, 12.0));
    let plane_material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.30, 0.36, 0.32),
        perceptual_roughness: 0.9,
        ..default()
    });
    let box_mesh = meshes.add(Cuboid::new(1.5, 1.5, 1.5));

    commands.entity(root).with_children(|parent| {
        parent.spawn((
            Name::new("Simple Scene Plane"),
            SimpleSceneGeometry,
            Mesh3d(plane_mesh),
            MeshMaterial3d(plane_material),
        ));
    });

    for (name, color, position) in simple_scene_boxes() {
        commands.entity(root).with_children(|parent| {
            parent.spawn((
                Name::new(name),
                SimpleSceneGeometry,
                Mesh3d(box_mesh.clone()),
                MeshMaterial3d(materials.add(StandardMaterial {
                    base_color: color,
                    perceptual_roughness: 0.65,
                    ..default()
                })),
                Transform::from_translation(position),
            ));
        });
    }

    info!("Spawned Simple scene geometry");
}

const fn simple_scene_boxes() -> [(&'static str, Color, Vec3); 4] {
    [
        (
            "Red Box",
            Color::srgb(0.90, 0.18, 0.16),
            Vec3::new(-2.0, 0.75, -2.0),
        ),
        (
            "Green Box",
            Color::srgb(0.18, 0.75, 0.30),
            Vec3::new(2.0, 0.75, -2.0),
        ),
        (
            "Blue Box",
            Color::srgb(0.18, 0.36, 0.95),
            Vec3::new(-2.0, 0.75, 2.0),
        ),
        (
            "Gold Box",
            Color::srgb(0.95, 0.68, 0.12),
            Vec3::new(2.0, 0.75, 2.0),
        ),
    ]
}
