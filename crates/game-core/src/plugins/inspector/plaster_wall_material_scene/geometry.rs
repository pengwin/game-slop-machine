use bevy::prelude::*;
use material_preview_geometry::{
    WallPreviewDirtSettings, WallPreviewMeshSettings, WallPreviewUvSettings,
    build_wall_preview_mesh,
};
use ui_derive::Controls;

use super::super::wall_material::apply_material_settings;
use super::{
    super::InspectorSceneState, material, root::PlasterWallMaterialSceneRoot,
    scene_sets::PlasterWallMaterialSceneSet,
};

/// Editable vertex-color dirt settings for the plaster wall preview mesh.
#[derive(Resource, Clone, Debug, PartialEq, Controls)]
pub struct PlasterWallDirtSettings {
    /// Dirt amount that accumulates upward from the floor.
    #[slider(min = 0.0, max = 1.5, step = 0.01, precision = 2, label = "Floor dirt")]
    pub floor_strength: f32,
    /// Dirt amount that accumulates in wall corner triangles.
    #[slider(
        min = 0.0,
        max = 1.5,
        step = 0.01,
        precision = 2,
        label = "Corner dirt"
    )]
    pub corner_strength: f32,
    /// Red multiplier for maximum dirt.
    #[slider(min = 0.0, max = 1.0, step = 0.01, precision = 2, label = "Dirt R")]
    pub color_r: f32,
    /// Green multiplier for maximum dirt.
    #[slider(min = 0.0, max = 1.0, step = 0.01, precision = 2, label = "Dirt G")]
    pub color_g: f32,
    /// Blue multiplier for maximum dirt.
    #[slider(min = 0.0, max = 1.0, step = 0.01, precision = 2, label = "Dirt B")]
    pub color_b: f32,
}

impl Default for PlasterWallDirtSettings {
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

impl From<&PlasterWallDirtSettings> for WallPreviewDirtSettings {
    fn from(settings: &PlasterWallDirtSettings) -> Self {
        Self {
            floor_strength: settings.floor_strength,
            corner_strength: settings.corner_strength,
            color_r: settings.color_r,
            color_g: settings.color_g,
            color_b: settings.color_b,
        }
    }
}

/// Editable UV projection settings for the plaster wall preview mesh.
#[derive(Resource, Clone, Debug, PartialEq, Controls)]
pub struct PlasterWallUvSettings {
    /// Uses old per-face local UVs with deterministic offsets instead of world-space projection.
    #[checkbox(label = "Per-face UV")]
    pub per_face_offset: bool,
    /// Texture tiles per meter on the preview wall mesh.
    #[slider(min = 0.05, max = 1.5, step = 0.01, precision = 2, label = "UV scale")]
    pub tiles_per_meter: f32,
    /// Horizontal subdivisions for each vertical wall face.
    #[slider(min = 1.0, max = 48.0, step = 1.0, precision = 0, label = "Columns")]
    pub face_columns: u32,
    /// Vertical subdivisions for each vertical wall face.
    #[slider(min = 1.0, max = 32.0, step = 1.0, precision = 0, label = "Rows")]
    pub face_rows: u32,
}

impl Default for PlasterWallUvSettings {
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

impl From<&PlasterWallUvSettings> for WallPreviewUvSettings {
    fn from(settings: &PlasterWallUvSettings) -> Self {
        Self {
            per_face_offset: settings.per_face_offset,
            tiles_per_meter: settings.tiles_per_meter,
            face_columns: settings.face_columns,
            face_rows: settings.face_rows,
        }
    }
}

#[derive(Component)]
struct PlasterWallDebugWall;

pub fn plugin(app: &mut App) {
    app.init_resource::<PlasterWallDirtSettings>()
        .init_resource::<PlasterWallUvSettings>()
        .add_systems(
            OnEnter(InspectorSceneState::PlasterWallMaterial),
            spawn_plaster_wall_geometry.in_set(PlasterWallMaterialSceneSet::Content),
        )
        .add_systems(
            Update,
            update_plaster_wall_mesh
                .run_if(in_state(InspectorSceneState::PlasterWallMaterial))
                .run_if(
                    resource_changed::<PlasterWallDirtSettings>
                        .or_else(resource_changed::<PlasterWallUvSettings>),
                ),
        );
}

fn spawn_plaster_wall_geometry(
    mut commands: Commands<'_, '_>,
    root: Query<'_, '_, Entity, With<PlasterWallMaterialSceneRoot>>,
    dirt_settings: Res<'_, PlasterWallDirtSettings>,
    uv_settings: Res<'_, PlasterWallUvSettings>,
    mut meshes: ResMut<'_, Assets<Mesh>>,
    mut materials: ResMut<'_, Assets<StandardMaterial>>,
) {
    let Ok(root) = root.single() else {
        return;
    };
    let dirt_settings = dirt_settings.into_inner();
    let uv_settings = uv_settings.into_inner();

    let mut wall_material = StandardMaterial {
        base_color: Color::srgb(0.72, 0.68, 0.58),
        perceptual_roughness: 1.0,
        ..default()
    };
    apply_material_settings(
        &mut wall_material,
        &material::PlasterWallMaterialSettings::default(),
    );
    let material = materials.add(wall_material);
    let wall = commands
        .spawn((
            Name::new("Plaster Material Debug Wall"),
            PlasterWallDebugWall,
            Mesh3d(meshes.add(wall_mesh(dirt_settings, uv_settings))),
            MeshMaterial3d(material.clone()),
        ))
        .id();
    let ground = commands
        .spawn((
            Name::new("Plaster Material Debug Ground"),
            Mesh3d(meshes.add(Plane3d::default().mesh().size(9.0, 9.0))),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: Color::srgb(0.24, 0.27, 0.25),
                perceptual_roughness: 0.95,
                ..default()
            })),
        ))
        .id();

    commands.entity(root).add_children(&[wall, ground]);
    material::start_plaster_generation(&mut commands, material, material::default_plaster_params());

    info!("Spawned Plaster wall material scene geometry");
}

fn update_plaster_wall_mesh(
    dirt_settings: Res<'_, PlasterWallDirtSettings>,
    uv_settings: Res<'_, PlasterWallUvSettings>,
    mut meshes: ResMut<'_, Assets<Mesh>>,
    mut walls: Query<'_, '_, &mut Mesh3d, With<PlasterWallDebugWall>>,
) {
    let dirt_settings = dirt_settings.into_inner();
    let uv_settings = uv_settings.into_inner();
    for mut mesh in &mut walls {
        *mesh = Mesh3d(meshes.add(wall_mesh(dirt_settings, uv_settings)));
    }
}

fn wall_mesh(dirt_settings: &PlasterWallDirtSettings, uv_settings: &PlasterWallUvSettings) -> Mesh {
    build_wall_preview_mesh(
        &WallPreviewMeshSettings::default(),
        &WallPreviewDirtSettings::from(dirt_settings),
        &WallPreviewUvSettings::from(uv_settings),
    )
}
