use crate::mesh::MeshData;
use super::mesh_utils::generate_box_mesh;

pub fn generate_counter_mesh(w: f32, h: f32, d: f32, color: [f32; 3]) -> MeshData {
    generate_box_mesh(w, h, d, color)
}
