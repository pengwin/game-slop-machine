use crate::mesh::MeshData;
use crate::mesh::colored_shapes::append_colored_box;

pub fn generate_bench_mesh(w: f32, h: f32, d: f32, color: [f32; 3]) -> MeshData {
    let mut mesh = MeshData::default();
    let seat_color = [color[0], color[1], color[2], 1.0];
    let leg_color = [color[0] * 0.6, color[1] * 0.6, color[2] * 0.6, 1.0];
    let seat_t = 0.04;
    let leg_w = 0.05;
    append_colored_box(&mut mesh, [0.0, h - seat_t/2.0, 0.0], [w, seat_t, d], seat_color);
    let x_offset = w/2.0 * 0.8;
    append_colored_box(&mut mesh, [-x_offset, (h-seat_t)/2.0, 0.0], [leg_w, h-seat_t, d], leg_color);
    append_colored_box(&mut mesh, [x_offset, (h-seat_t)/2.0, 0.0], [leg_w, h-seat_t, d], leg_color);
    mesh
}
