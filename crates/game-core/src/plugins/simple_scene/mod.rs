//! A small preview scene for validating rendering and inspector scene selection.

mod camera;
mod lighting;

use bevy::prelude::*;

/// Adds systems for spawning and despawning the simple preview scene.
pub struct SimpleScenePlugin;

impl Plugin for SimpleScenePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<InspectorScene>()
            .configure_sets(
                OnEnter(InspectorScene::Simple),
                (SimpleSceneSet::Root, SimpleSceneSet::Content).chain(),
            )
            .add_systems(
                OnEnter(InspectorScene::Simple),
                spawn_simple_scene_root.in_set(SimpleSceneSet::Root),
            )
            .add_systems(
                OnEnter(InspectorScene::Simple),
                spawn_simple_scene_geometry.in_set(SimpleSceneSet::Content),
            )
            .add_systems(OnExit(InspectorScene::Simple), despawn_simple_scene);

        camera::plugin(app);
        lighting::plugin(app);
    }
}

#[derive(SystemSet, Debug, Clone, Copy, Eq, PartialEq, Hash)]
enum SimpleSceneSet {
    Root,
    Content,
}

/// Active scene selected in the inspector.
#[derive(States, Debug, Clone, Copy, Default, Eq, PartialEq, Hash)]
pub enum InspectorScene {
    /// No inspector scene is currently active.
    #[default]
    None,
    /// The simple preview scene is currently active.
    Simple,
}

impl InspectorScene {
    /// Returns the human-readable label shown in inspector UI.
    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::None => "None",
            Self::Simple => "Simple scene",
        }
    }
}

/// Marker applied to entities owned by the simple preview scene.
#[derive(Component)]
pub struct SimpleSceneRoot;

#[derive(Component)]
struct SimpleSceneGeometry;

fn spawn_simple_scene_root(mut commands: Commands<'_, '_>) {
    commands.spawn((
        Name::new("Simple Scene"),
        SimpleSceneRoot,
        Transform::default(),
        Visibility::default(),
    ));
}

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

fn despawn_simple_scene(
    mut commands: Commands<'_, '_>,
    roots: Query<'_, '_, Entity, With<SimpleSceneRoot>>,
) {
    for root in &roots {
        commands.entity(root).despawn();
    }

    info!("Despawned Simple scene");
}
