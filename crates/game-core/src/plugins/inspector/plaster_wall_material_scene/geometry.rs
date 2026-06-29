use bevy::{
    mesh::Indices, prelude::*,
    render::render_resource::PrimitiveTopology as RenderPrimitiveTopology,
};

use super::{
    super::InspectorSceneState, material, root::PlasterWallMaterialSceneRoot,
    scene_sets::PlasterWallMaterialSceneSet,
};

const PLASTER_UV_TILES_PER_METER: f32 = 0.32;
const WALL_FACE_COLUMNS: usize = 12;
const WALL_FACE_ROWS: usize = 8;

pub fn plugin(app: &mut App) {
    app.init_resource::<PlasterWallDirtSettings>()
        .add_systems(
            OnEnter(InspectorSceneState::PlasterWallMaterial),
            spawn_plaster_wall_geometry.in_set(PlasterWallMaterialSceneSet::Content),
        )
        .add_systems(
            Update,
            update_plaster_wall_dirt
                .run_if(in_state(InspectorSceneState::PlasterWallMaterial))
                .run_if(resource_changed::<PlasterWallDirtSettings>),
        );
}

/// Editable vertex-color dirt settings for the plaster wall preview mesh.
#[derive(Resource, Clone, Debug, PartialEq)]
pub struct PlasterWallDirtSettings {
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

impl Default for PlasterWallDirtSettings {
    fn default() -> Self {
        Self {
            floor_strength: 0.62,
            corner_strength: 0.72,
            color_r: 0.66,
            color_g: 0.44,
            color_b: 0.26,
        }
    }
}

#[derive(Component)]
struct PlasterWallDebugWall;

fn spawn_plaster_wall_geometry(
    mut commands: Commands<'_, '_>,
    root: Query<'_, '_, Entity, With<PlasterWallMaterialSceneRoot>>,
    dirt_settings: Res<'_, PlasterWallDirtSettings>,
    mut meshes: ResMut<'_, Assets<Mesh>>,
    mut materials: ResMut<'_, Assets<StandardMaterial>>,
) {
    let Ok(root) = root.single() else {
        return;
    };
    let dirt_settings = dirt_settings.into_inner();

    let mut wall_material = StandardMaterial {
        base_color: Color::srgb(0.72, 0.68, 0.58),
        perceptual_roughness: 1.0,
        ..default()
    };
    material::apply_material_settings(
        &mut wall_material,
        &material::PlasterWallMaterialSettings::default(),
    );
    let material = materials.add(wall_material);
    let wall = commands
        .spawn((
            Name::new("Plaster Material Debug Wall"),
            PlasterWallDebugWall,
            Mesh3d(meshes.add(wall_mesh(dirt_settings))),
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

fn update_plaster_wall_dirt(
    dirt_settings: Res<'_, PlasterWallDirtSettings>,
    mut meshes: ResMut<'_, Assets<Mesh>>,
    mut walls: Query<'_, '_, &mut Mesh3d, With<PlasterWallDebugWall>>,
) {
    let dirt_settings = dirt_settings.into_inner();
    for mut mesh in &mut walls {
        *mesh = Mesh3d(meshes.add(wall_mesh(dirt_settings)));
    }
}

fn wall_mesh(dirt_settings: &PlasterWallDirtSettings) -> Mesh {
    let mut builder = WallMeshBuilder::default();
    let height = 2.4;
    let thickness = 0.18;
    let half_thickness = thickness * 0.5;

    let min_x = -2.5;
    let max_x = 2.5;
    let min_z = -2.5;
    let max_z = 2.5;
    let top_opening_x = -0.7;
    let left_wall_end_z = 1.25;
    let outer_min_x = min_x - half_thickness;
    let outer_max_x = max_x + half_thickness;
    let outer_min_z = min_z - half_thickness;
    let outer_max_z = max_z + half_thickness;
    let inner_min_x = min_x + half_thickness;
    let inner_max_x = max_x - half_thickness;
    let inner_min_z = min_z + half_thickness;
    let inner_max_z = max_z - half_thickness;

    builder.push_boundary(
        &[
            Vec2::new(outer_min_x, outer_min_z),
            Vec2::new(outer_max_x, outer_min_z),
            Vec2::new(outer_max_x, outer_max_z),
            Vec2::new(top_opening_x, outer_max_z),
            Vec2::new(top_opening_x, inner_max_z),
            Vec2::new(inner_max_x, inner_max_z),
            Vec2::new(inner_max_x, inner_min_z),
            Vec2::new(inner_min_x, inner_min_z),
            Vec2::new(inner_min_x, left_wall_end_z),
            Vec2::new(outer_min_x, left_wall_end_z),
        ],
        height,
        dirt_settings,
    );

    builder.push_top_rect(
        outer_min_x,
        outer_max_x,
        outer_min_z,
        inner_min_z,
        height,
        dirt_settings,
    );
    builder.push_top_rect(
        inner_max_x,
        outer_max_x,
        inner_min_z,
        outer_max_z,
        height,
        dirt_settings,
    );
    builder.push_top_rect(
        top_opening_x,
        inner_max_x,
        inner_max_z,
        outer_max_z,
        height,
        dirt_settings,
    );
    builder.push_top_rect(
        outer_min_x,
        inner_min_x,
        inner_min_z,
        left_wall_end_z,
        height,
        dirt_settings,
    );

    let mut mesh = Mesh::new(
        RenderPrimitiveTopology::TriangleList,
        bevy::asset::RenderAssetUsages::RENDER_WORLD | bevy::asset::RenderAssetUsages::MAIN_WORLD,
    );
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, builder.positions);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, builder.normals);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, builder.uvs);
    mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, builder.colors);
    mesh.insert_indices(Indices::U32(builder.indices));
    if let Err(err) = mesh.generate_tangents() {
        warn!("Failed to generate tangents for plaster wall debug mesh: {err:?}");
    }
    mesh
}

#[derive(Default)]
struct WallMeshBuilder {
    positions: Vec<[f32; 3]>,
    normals: Vec<[f32; 3]>,
    uvs: Vec<[f32; 2]>,
    colors: Vec<[f32; 4]>,
    indices: Vec<u32>,
}

impl WallMeshBuilder {
    fn push_boundary(
        &mut self,
        points: &[Vec2],
        height: f32,
        dirt_settings: &PlasterWallDirtSettings,
    ) {
        for i in 0..points.len() {
            let start = points[i];
            let end = points[(i + 1) % points.len()];
            let edge = end - start;
            let length = edge.length();
            if length <= f32::EPSILON {
                continue;
            }

            let outward = Vec3::new(edge.y, 0.0, -edge.x).normalize_or_zero();
            self.push_face(
                [
                    Vec3::new(start.x, 0.0, start.y),
                    Vec3::new(end.x, 0.0, end.y),
                    Vec3::new(end.x, height, end.y),
                    Vec3::new(start.x, height, start.y),
                ],
                outward,
                [length, height],
                true,
                dirt_settings,
            );
        }
    }

    fn push_top_rect(
        &mut self,
        min_x: f32,
        max_x: f32,
        min_z: f32,
        max_z: f32,
        height: f32,
        dirt_settings: &PlasterWallDirtSettings,
    ) {
        let width = max_x - min_x;
        let depth = max_z - min_z;
        if width <= f32::EPSILON || depth <= f32::EPSILON {
            return;
        }

        self.push_face(
            [
                Vec3::new(min_x, height, min_z),
                Vec3::new(max_x, height, min_z),
                Vec3::new(max_x, height, max_z),
                Vec3::new(min_x, height, max_z),
            ],
            Vec3::Y,
            [width, depth],
            false,
            dirt_settings,
        );
    }

    fn push_face(
        &mut self,
        corners: [Vec3; 4],
        normal: Vec3,
        uv_size: [f32; 2],
        dirt: bool,
        dirt_settings: &PlasterWallDirtSettings,
    ) {
        let Ok(base) = u32::try_from(self.positions.len()) else {
            unreachable!("wall debug mesh vertex count fits in u32");
        };
        let face_index = self.positions.len() / 4;
        let uv_offset = face_uv_offset(face_index);
        let uv_width = uv_size[0] * PLASTER_UV_TILES_PER_METER;
        let uv_height = uv_size[1] * PLASTER_UV_TILES_PER_METER;
        let columns = if dirt { WALL_FACE_COLUMNS } else { 1 };
        let rows = if dirt { WALL_FACE_ROWS } else { 1 };
        let normal = normal.to_array();

        for row in 0..=rows {
            let local_v = ratio(row, rows);
            for column in 0..=columns {
                let local_u = ratio(column, columns);
                let bottom = corners[0].lerp(corners[1], local_u);
                let top = corners[3].lerp(corners[2], local_u);
                let position = bottom.lerp(top, local_v);

                self.positions.push(position.to_array());
                self.normals.push(normal);
                self.uvs.push([
                    uv_width.mul_add(local_u, uv_offset[0]),
                    uv_height.mul_add(local_v, uv_offset[1]),
                ]);
                self.colors.push(dirt_vertex_color(
                    local_u,
                    height_ratio(position, corners),
                    dirt,
                    dirt_settings,
                ));
            }
        }

        let winding_normal = (corners[1] - corners[0])
            .cross(corners[2] - corners[0])
            .normalize_or_zero();
        let same_winding = winding_normal.dot(Vec3::from_array(normal)) >= 0.0;
        let stride = columns + 1;

        for row in 0..rows {
            for column in 0..columns {
                let i0 = base + u32::try_from(row * stride + column).unwrap_or(0);
                let i1 = i0 + 1;
                let i3 = base + u32::try_from((row + 1) * stride + column).unwrap_or(0);
                let i2 = i3 + 1;

                if same_winding {
                    self.indices.extend([i0, i1, i2, i0, i2, i3]);
                } else {
                    self.indices.extend([i0, i2, i1, i0, i3, i2]);
                }
            }
        }
    }
}

fn ratio(value: usize, max: usize) -> f32 {
    let value = u16::try_from(value).unwrap_or(0);
    let max = u16::try_from(max.max(1)).unwrap_or(1);
    f32::from(value) / f32::from(max)
}

fn height_ratio(position: Vec3, corners: [Vec3; 4]) -> f32 {
    let min_y = corners
        .iter()
        .map(|corner| corner.y)
        .fold(f32::INFINITY, f32::min);
    let max_y = corners
        .iter()
        .map(|corner| corner.y)
        .fold(f32::NEG_INFINITY, f32::max);
    ((position.y - min_y) / (max_y - min_y).max(0.001)).clamp(0.0, 1.0)
}

fn dirt_vertex_color(
    local_u: f32,
    height_ratio: f32,
    enabled: bool,
    settings: &PlasterWallDirtSettings,
) -> [f32; 4] {
    if !enabled {
        return [1.0, 1.0, 1.0, 1.0];
    }

    let floor_dirt = (1.0 - height_ratio).powf(1.55) * settings.floor_strength;
    let left_corner = corner_triangle_dirt(local_u, height_ratio);
    let right_corner = corner_triangle_dirt(1.0 - local_u, height_ratio);
    let dirt = left_corner
        .max(right_corner)
        .mul_add(settings.corner_strength, floor_dirt)
        .clamp(0.0, 0.86);
    [
        lerp(1.0, settings.color_r, dirt),
        lerp(1.0, settings.color_g, dirt),
        lerp(1.0, settings.color_b, dirt),
        1.0,
    ]
}

fn corner_triangle_dirt(distance_from_edge: f32, height_ratio: f32) -> f32 {
    let horizontal = distance_from_edge / 0.38;
    let vertical = height_ratio / 0.72;
    (1.0 - horizontal - vertical).clamp(0.0, 1.0).powf(1.4)
}

fn lerp(a: f32, b: f32, t: f32) -> f32 {
    (b - a).mul_add(t, a)
}

fn face_uv_offset(face_index: usize) -> [f32; 2] {
    let mut hash = u32::try_from(face_index).unwrap_or(0);
    hash ^= hash >> 16;
    hash = hash.wrapping_mul(0x7FEB_352D);
    hash ^= hash >> 15;
    hash = hash.wrapping_mul(0x846C_A68B);
    hash ^= hash >> 16;

    let u_bits = u16::try_from(hash & 0xFFFF).unwrap_or(0);
    let v_bits = u16::try_from(hash >> 16).unwrap_or(0);
    let u = (f32::from(u_bits) / f32::from(u16::MAX)) * 3.0;
    let v = (f32::from(v_bits) / f32::from(u16::MAX)) * 3.0;
    [u, v]
}
