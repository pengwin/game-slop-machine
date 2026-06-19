use crate::mesh::MeshData;
use crate::mesh::colored_shapes::append_colored_box;

pub fn generate_desk_mesh(w: f32, h: f32, d: f32, color: [f32; 3]) -> MeshData {
    let mut mesh = MeshData::default();
    let top_color = [color[0], color[1], color[2], 1.0];
    let panel_color = [color[0] * 0.7, color[1] * 0.7, color[2] * 0.7, 1.0];
    let pt = 0.03;
    append_colored_box(&mut mesh, [0.0, h - pt/2.0, 0.0], [w, pt, d], top_color);
    append_colored_box(&mut mesh, [-w/2.0 + pt/2.0, (h-pt)/2.0, 0.0], [pt, h-pt, d], panel_color);
    append_colored_box(&mut mesh, [w/2.0 - pt/2.0, (h-pt)/2.0, 0.0], [pt, h-pt, d], panel_color);
    append_colored_box(&mut mesh, [0.0, h*0.3, 0.0], [w - 2.0*pt, pt, d], top_color);
    mesh
}
