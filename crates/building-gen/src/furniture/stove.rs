use crate::mesh::MeshData;
use crate::mesh::colored_shapes::generate_box_mesh;

pub fn generate_stove_mesh(w: f32, h: f32, d: f32, color: [f32; 3]) -> MeshData {
    generate_box_mesh(w, h, d, color)
}
