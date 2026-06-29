use bevy::{
    mesh::Indices, prelude::*,
    render::render_resource::PrimitiveTopology as RenderPrimitiveTopology,
};

use super::{
    super::InspectorSceneState, material, root::PlasterWallMaterialSceneRoot,
    scene_sets::PlasterWallMaterialSceneSet,
};

const PLASTER_UV_TILES_PER_METER: f32 = 0.32;

pub fn plugin(app: &mut App) {
    app.add_systems(
        OnEnter(InspectorSceneState::PlasterWallMaterial),
        spawn_plaster_wall_geometry.in_set(PlasterWallMaterialSceneSet::Content),
    );
}

fn spawn_plaster_wall_geometry(
    mut commands: Commands<'_, '_>,
    root: Query<'_, '_, Entity, With<PlasterWallMaterialSceneRoot>>,
    mut meshes: ResMut<'_, Assets<Mesh>>,
    mut materials: ResMut<'_, Assets<StandardMaterial>>,
) {
    let Ok(root) = root.single() else {
        return;
    };

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
            Mesh3d(meshes.add(wall_mesh())),
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

fn wall_mesh() -> Mesh {
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
    );

    builder.push_top_rect(outer_min_x, outer_max_x, outer_min_z, inner_min_z, height);
    builder.push_top_rect(inner_max_x, outer_max_x, inner_min_z, outer_max_z, height);
    builder.push_top_rect(top_opening_x, inner_max_x, inner_max_z, outer_max_z, height);
    builder.push_top_rect(
        outer_min_x,
        inner_min_x,
        inner_min_z,
        left_wall_end_z,
        height,
    );

    let mut mesh = Mesh::new(
        RenderPrimitiveTopology::TriangleList,
        bevy::asset::RenderAssetUsages::RENDER_WORLD | bevy::asset::RenderAssetUsages::MAIN_WORLD,
    );
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, builder.positions);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, builder.normals);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, builder.uvs);
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
    indices: Vec<u32>,
}

impl WallMeshBuilder {
    fn push_boundary(&mut self, points: &[Vec2], height: f32) {
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
            );
        }
    }

    fn push_top_rect(&mut self, min_x: f32, max_x: f32, min_z: f32, max_z: f32, height: f32) {
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
        );
    }

    fn push_face(&mut self, corners: [Vec3; 4], normal: Vec3, uv_size: [f32; 2]) {
        let Ok(base) = u32::try_from(self.positions.len()) else {
            unreachable!("wall debug mesh vertex count fits in u32");
        };
        let face_index = self.positions.len() / 4;
        let uv_offset = face_uv_offset(face_index);
        let uv_width = uv_size[0] * PLASTER_UV_TILES_PER_METER;
        let uv_height = uv_size[1] * PLASTER_UV_TILES_PER_METER;

        self.positions
            .extend(corners.map(|corner| corner.to_array()));
        self.normals.extend([normal.to_array(); 4]);
        self.uvs.extend([
            [uv_offset[0], uv_offset[1]],
            [uv_offset[0] + uv_width, uv_offset[1]],
            [uv_offset[0] + uv_width, uv_offset[1] + uv_height],
            [uv_offset[0], uv_offset[1] + uv_height],
        ]);
        let winding_normal = (corners[1] - corners[0])
            .cross(corners[2] - corners[0])
            .normalize_or_zero();
        if winding_normal.dot(normal) >= 0.0 {
            self.indices
                .extend([base, base + 1, base + 2, base, base + 2, base + 3]);
        } else {
            self.indices
                .extend([base, base + 2, base + 1, base, base + 3, base + 2]);
        }
    }
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
