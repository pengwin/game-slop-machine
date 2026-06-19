use crate::mesh::MeshData;
use super::mesh_utils::append_colored_box;

pub fn generate_chair_mesh(w: f32, seat_h: f32, d: f32, color: [f32; 3]) -> MeshData {
    let mut mesh = MeshData::default();
    let hw = w / 2.0;
    let hd = d / 2.0;
    let leg_t = 0.03;
    let seat_t = 0.03;
    let back_h = 0.4;
    let seat_color = [color[0], color[1], color[2], 1.0];
    let leg_color = [color[0] * 0.6, color[1] * 0.6, color[2] * 0.6, 1.0];

    append_colored_box(&mut mesh, [0.0, seat_h - seat_t/2.0, 0.0], [w, seat_t, d], seat_color);
    append_colored_box(&mut mesh, [0.0, seat_h + back_h/2.0, hd - seat_t/2.0], [w, back_h, seat_t], seat_color);

    let leg_positions = [
        (-hw + leg_t, -hd + leg_t),
        (hw - leg_t, -hd + leg_t),
        (-hw + leg_t, hd - leg_t),
        (hw - leg_t, hd - leg_t),
    ];
    let leg_h = seat_h - seat_t;
    for (lx, lz) in leg_positions {
        append_colored_box(&mut mesh, [lx, leg_h/2.0, lz], [leg_t*2.0, leg_h, leg_t*2.0], leg_color);
    }
    mesh
}
