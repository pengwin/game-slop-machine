use bevy::prelude::*;
use material_preview_geometry::{
    WallPreviewDirtSettings, WallPreviewMeshSettings, WallPreviewUvSettings,
    build_wall_preview_mesh,
};

use super::{
    super::InspectorSceneState, material, root::ConcreteWallMaterialSceneRoot,
    scene_sets::ConcreteWallMaterialSceneSet,
};

/// Editable vertex-color dirt settings for the concrete wall preview mesh.
#[derive(Resource, Clone, Debug, PartialEq)]
pub struct ConcreteWallDirtSettings {
    /// Dirt amount that accumulates upward from the floor.
    pub floor_strength: f32,
    /// Dirt amount that accumulates in wall corner triangles.
    pub corner_strength: f32,
    /// Red multiplier for maximum dirt.
    pub color_r: f32,
    /// Green multiplier for maximum dirt.
    pub color_g: f32,
    /// Blue multiplier for maximum dirt.
    pub color_b: f32,
}

impl Default for ConcreteWallDirtSettings {
    fn default() -> Self {
        let defaults = WallPreviewDirtSettings::default();
        Self {
            floor_strength: defaults.floor_strength,
            corner_strength: defaults.corner_strength,
            color_r: defaults.color_r,
            color_g: defaults.color_g,
            color_b: defaults.color_b,
        }
    }
}

impl From<&ConcreteWallDirtSettings> for WallPreviewDirtSettings {
    fn from(settings: &ConcreteWallDirtSettings) -> Self {
        Self {
            floor_strength: settings.floor_strength,
            corner_strength: settings.corner_strength,
            color_r: settings.color_r,
            color_g: settings.color_g,
            color_b: settings.color_b,
        }
    }
}

/// Editable UV projection settings for the concrete wall preview mesh.
#[derive(Resource, Clone, Debug, PartialEq)]
pub struct ConcreteWallUvSettings {
    /// Uses old per-face local UVs with deterministic offsets instead of world-space projection.
    pub per_face_offset: bool,
    /// Texture tiles per meter on the preview wall mesh.
    pub tiles_per_meter: f32,
    /// Horizontal subdivisions for each vertical wall face.
    pub face_columns: u32,
    /// Vertical subdivisions for each vertical wall face.
    pub face_rows: u32,
}

impl Default for ConcreteWallUvSettings {
    fn default() -> Self {
        let defaults = WallPreviewUvSettings::default();
        Self {
            per_face_offset: defaults.per_face_offset,
            tiles_per_meter: defaults.tiles_per_meter,
            face_columns: defaults.face_columns,
            face_rows: defaults.face_rows,
        }
    }
}

impl From<&ConcreteWallUvSettings> for WallPreviewUvSettings {
    fn from(settings: &ConcreteWallUvSettings) -> Self {
        Self {
            per_face_offset: settings.per_face_offset,
            tiles_per_meter: settings.tiles_per_meter,
            face_columns: settings.face_columns,
            face_rows: settings.face_rows,
        }
    }
}

#[derive(Component)]
struct ConcreteWallDebugWall;

pub fn plugin(app: &mut App) {
    app.init_resource::<ConcreteWallDirtSettings>()
        .init_resource::<ConcreteWallUvSettings>()
        .add_systems(
            OnEnter(InspectorSceneState::ConcreteWallMaterial),
            spawn_concrete_wall_geometry.in_set(ConcreteWallMaterialSceneSet::Content),
        )
        .add_systems(
            Update,
            update_concrete_wall_mesh
                .run_if(in_state(InspectorSceneState::ConcreteWallMaterial))
                .run_if(
                    resource_changed::<ConcreteWallDirtSettings>
                        .or_else(resource_changed::<ConcreteWallUvSettings>),
                ),
        );
}

fn spawn_concrete_wall_geometry(
    mut commands: Commands<'_, '_>,
    root: Query<'_, '_, Entity, With<ConcreteWallMaterialSceneRoot>>,
    dirt_settings: Res<'_, ConcreteWallDirtSettings>,
    uv_settings: Res<'_, ConcreteWallUvSettings>,
    mut meshes: ResMut<'_, Assets<Mesh>>,
    mut materials: ResMut<'_, Assets<StandardMaterial>>,
) {
    let Ok(root) = root.single() else {
        return;
    };
    let dirt_settings = dirt_settings.into_inner();
    let uv_settings = uv_settings.into_inner();

    let mut wall_material = StandardMaterial {
        base_color: Color::srgb(0.58, 0.55, 0.48),
        perceptual_roughness: 1.0,
        ..default()
    };
    material::apply_material_settings(
        &mut wall_material,
        &material::ConcreteWallMaterialSettings::default(),
    );
    let material = materials.add(wall_material);
    let wall = commands
        .spawn((
            Name::new("Concrete Material Debug Wall"),
            ConcreteWallDebugWall,
            Mesh3d(meshes.add(wall_mesh(dirt_settings, uv_settings))),
            MeshMaterial3d(material.clone()),
        ))
        .id();
    let ground = commands
        .spawn((
            Name::new("Concrete Material Debug Ground"),
            Mesh3d(meshes.add(Plane3d::default().mesh().size(9.0, 9.0))),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: Color::srgb(0.24, 0.27, 0.25),
                perceptual_roughness: 0.95,
                ..default()
            })),
        ))
        .id();

    commands.entity(root).add_children(&[wall, ground]);
    material::start_concrete_generation(
        &mut commands,
        material,
        material::default_concrete_params(),
    );

    info!("Spawned Concrete wall material scene geometry");
}

fn update_concrete_wall_mesh(
    dirt_settings: Res<'_, ConcreteWallDirtSettings>,
    uv_settings: Res<'_, ConcreteWallUvSettings>,
    mut meshes: ResMut<'_, Assets<Mesh>>,
    mut walls: Query<'_, '_, &mut Mesh3d, With<ConcreteWallDebugWall>>,
) {
    let dirt_settings = dirt_settings.into_inner();
    let uv_settings = uv_settings.into_inner();
    for mut mesh in &mut walls {
        *mesh = Mesh3d(meshes.add(wall_mesh(dirt_settings, uv_settings)));
    }
}

fn wall_mesh(
    dirt_settings: &ConcreteWallDirtSettings,
    uv_settings: &ConcreteWallUvSettings,
) -> Mesh {
    build_wall_preview_mesh(
        &WallPreviewMeshSettings::default(),
        &WallPreviewDirtSettings::from(dirt_settings),
        &WallPreviewUvSettings::from(uv_settings),
    )
}
