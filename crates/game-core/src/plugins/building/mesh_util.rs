use bevy::asset::RenderAssetUsages;
use bevy::mesh::Indices;
use bevy::prelude::*;
use bevy::render::render_resource::PrimitiveTopology;
use building_gen::geometry::Vec2 as BVec2;
use building_gen::mesh::MeshData;

pub fn subdivide_mesh_data(data: &MeshData, max_len: f32) -> MeshData {
    let mut out = data.clone();
    let mut changed = true;
    while changed {
        changed = false;
        let mut new_vertices = out.vertices.clone();
        let mut new_normals = out.normals.clone();
        let mut new_uvs = out.uvs.clone();
        let mut new_colors = out.colors.clone();
        let mut new_indices = Vec::new();
        let mut new_surface_materials = Vec::new();

        let has_colors = !out.colors.is_empty();

        let mut i = 0;
        while i < out.indices.len() {
            let i0 = out.indices[i] as usize;
            let i1 = out.indices[i + 1] as usize;
            let i2 = out.indices[i + 2] as usize;
            let material = out
                .surface_materials
                .get(i / 3)
                .copied()
                .unwrap_or_default();
            i += 3;

            let v0 = out.vertices[i0];
            let v1 = out.vertices[i1];
            let v2 = out.vertices[i2];

            let l01 = building_gen::mesh::math_util::vec3_length(
                building_gen::mesh::math_util::sub3(v0, v1),
            );
            let l12 = building_gen::mesh::math_util::vec3_length(
                building_gen::mesh::math_util::sub3(v1, v2),
            );
            let l20 = building_gen::mesh::math_util::vec3_length(
                building_gen::mesh::math_util::sub3(v2, v0),
            );

            if l01 > max_len || l12 > max_len || l20 > max_len {
                changed = true;
                if l01 >= l12 && l01 >= l20 {
                    let n = new_vertices.len();
                    new_vertices.push(building_gen::mesh::math_util::lerp3(v0, v1, 0.5));
                    new_normals.push(building_gen::mesh::math_util::normalize3(
                        building_gen::mesh::math_util::lerp3(out.normals[i0], out.normals[i1], 0.5),
                    ));
                    new_uvs.push([
                        (out.uvs[i0][0] + out.uvs[i1][0]) * 0.5,
                        (out.uvs[i0][1] + out.uvs[i1][1]) * 0.5,
                    ]);
                    if has_colors {
                        new_colors.push([
                            (out.colors[i0][0] + out.colors[i1][0]) * 0.5,
                            (out.colors[i0][1] + out.colors[i1][1]) * 0.5,
                            (out.colors[i0][2] + out.colors[i1][2]) * 0.5,
                            (out.colors[i0][3] + out.colors[i1][3]) * 0.5,
                        ]);
                    }
                    new_indices.extend_from_slice(&[i0 as u32, n as u32, i2 as u32]);
                    new_indices.extend_from_slice(&[n as u32, i1 as u32, i2 as u32]);
                    new_surface_materials.extend([material; 2]);
                } else if l12 >= l01 && l12 >= l20 {
                    let n = new_vertices.len();
                    new_vertices.push(building_gen::mesh::math_util::lerp3(v1, v2, 0.5));
                    new_normals.push(building_gen::mesh::math_util::normalize3(
                        building_gen::mesh::math_util::lerp3(out.normals[i1], out.normals[i2], 0.5),
                    ));
                    new_uvs.push([
                        (out.uvs[i1][0] + out.uvs[i2][0]) * 0.5,
                        (out.uvs[i1][1] + out.uvs[i2][1]) * 0.5,
                    ]);
                    if has_colors {
                        new_colors.push([
                            (out.colors[i1][0] + out.colors[i2][0]) * 0.5,
                            (out.colors[i1][1] + out.colors[i2][1]) * 0.5,
                            (out.colors[i1][2] + out.colors[i2][2]) * 0.5,
                            (out.colors[i1][3] + out.colors[i2][3]) * 0.5,
                        ]);
                    }
                    new_indices.extend_from_slice(&[i1 as u32, n as u32, i0 as u32]);
                    new_indices.extend_from_slice(&[n as u32, i2 as u32, i0 as u32]);
                    new_surface_materials.extend([material; 2]);
                } else {
                    let n = new_vertices.len();
                    new_vertices.push(building_gen::mesh::math_util::lerp3(v2, v0, 0.5));
                    new_normals.push(building_gen::mesh::math_util::normalize3(
                        building_gen::mesh::math_util::lerp3(out.normals[i2], out.normals[i0], 0.5),
                    ));
                    new_uvs.push([
                        (out.uvs[i2][0] + out.uvs[i0][0]) * 0.5,
                        (out.uvs[i2][1] + out.uvs[i0][1]) * 0.5,
                    ]);
                    if has_colors {
                        new_colors.push([
                            (out.colors[i2][0] + out.colors[i0][0]) * 0.5,
                            (out.colors[i2][1] + out.colors[i0][1]) * 0.5,
                            (out.colors[i2][2] + out.colors[i0][2]) * 0.5,
                            (out.colors[i2][3] + out.colors[i0][3]) * 0.5,
                        ]);
                    }
                    new_indices.extend_from_slice(&[i2 as u32, n as u32, i1 as u32]);
                    new_indices.extend_from_slice(&[n as u32, i0 as u32, i1 as u32]);
                    new_surface_materials.extend([material; 2]);
                }
            } else {
                new_indices.extend_from_slice(&[i0 as u32, i1 as u32, i2 as u32]);
                new_surface_materials.push(material);
            }
        }
        out.vertices = new_vertices;
        out.normals = new_normals;
        out.uvs = new_uvs;
        out.colors = new_colors;
        out.indices = new_indices;
        out.surface_materials = new_surface_materials;
    }
    out
}

/// Converts a `MeshData` into a Bevy `Mesh`.
pub fn convert_mesh(data: &MeshData) -> Mesh {
    let mut mesh = Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD,
    );

    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, data.vertices.clone());
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, data.normals.clone());
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, data.uvs.clone());
    if !data.colors.is_empty() {
        mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, data.colors.clone());
    }
    mesh.insert_indices(Indices::U32(data.indices.clone()));

    if let Err(err) = mesh.generate_tangents() {
        warn!("Failed to generate tangents for procedural mesh: {err:?}");
    }

    mesh
}

/// Applies procedural dirt colors to vertices of a converted wall mesh
pub fn apply_dirt_vertex_colors(mesh: &mut Mesh, seed: u32, intensity: f32) {
    if let Some(bevy::render::mesh::VertexAttributeValues::Float32x3(positions)) =
        mesh.attribute(Mesh::ATTRIBUTE_POSITION)
    {
        if let Some(bevy::render::mesh::VertexAttributeValues::Float32x3(normals)) =
            mesh.attribute(Mesh::ATTRIBUTE_NORMAL)
        {
            let existing_colors = match mesh.attribute(Mesh::ATTRIBUTE_COLOR) {
                Some(bevy::render::mesh::VertexAttributeValues::Float32x4(c)) => Some(c),
                _ => None,
            };

            let mut colors = Vec::with_capacity(positions.len());
            for (i, (p, n)) in positions.iter().zip(normals.iter()).enumerate() {
                let dirt = super::procedural_texture::global_dirt_color(seed, *p, *n, intensity);
                let base = existing_colors.map_or([1.0, 1.0, 1.0, 1.0], |c| c[i]);
                colors.push([
                    base[0] * dirt[0],
                    base[1] * dirt[1],
                    base[2] * dirt[2],
                    base[3] * dirt[3],
                ]);
            }
            mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, colors);
        }
    }
}

/// Creates a flat quad mesh at the given position.
pub fn make_ground_quad(center: Vec3, width: f32, depth: f32) -> Mesh {
    let hw = width / 2.0;
    let hd = depth / 2.0;
    let mut mesh = Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD,
    );

    let cx = center.x;
    let cy = center.y;
    let cz = center.z;

    mesh.insert_attribute(
        Mesh::ATTRIBUTE_POSITION,
        vec![
            [cx - hw, cy, cz - hd],
            [cx + hw, cy, cz - hd],
            [cx + hw, cy, cz + hd],
            [cx - hw, cy, cz + hd],
        ],
    );
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, vec![[0.0, 1.0, 0.0]; 4]);
    mesh.insert_attribute(
        Mesh::ATTRIBUTE_UV_0,
        vec![[0.0, 0.0], [width, 0.0], [width, depth], [0.0, depth]],
    );
    mesh.insert_indices(Indices::U32(vec![0, 2, 1, 0, 3, 2]));

    mesh
}

/// Rotates a local point by `rotation` around origin, then translates.
pub fn local_to_world(origin: BVec2, rotation: f32, local: BVec2) -> BVec2 {
    let sin = rotation.sin();
    let cos = rotation.cos();
    BVec2::new(
        origin.x + local.x * cos + local.y * sin,
        origin.y - local.x * sin + local.y * cos,
    )
}
