use bevy::asset::RenderAssetUsages;
use bevy::mesh::Indices;
use bevy::prelude::*;
use bevy::render::render_resource::PrimitiveTopology;
use building_gen::geometry::Vec2 as BVec2;
use building_gen::mesh::MeshData;

/// Converts a `MeshData` into a Bevy `Mesh`.
pub fn convert_mesh(data: &MeshData) -> Mesh {
    let mut mesh = Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD,
    );

    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, data.vertices.clone());
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, data.normals.clone());
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, data.uvs.clone());
    mesh.insert_indices(Indices::U32(data.indices.clone()));

    mesh
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
        vec![[0.0, 0.0], [1.0, 0.0], [1.0, 1.0], [0.0, 1.0]],
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
