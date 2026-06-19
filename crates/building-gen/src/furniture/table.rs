use crate::mesh::MeshData;
use super::mesh_utils::append_colored_box;
use crate::mesh::math_util::{append_colored_quad, Quad};

pub fn generate_table_mesh(w: f32, top_y: f32, d: f32, color: [f32; 3]) -> MeshData {
    let mut mesh = MeshData::default();
    let hw = w / 2.0;
    let hd = d / 2.0;
    let leg_t = 0.04;
    let leg_h = top_y - 0.02;
    let top_color = [color[0], color[1], color[2], 1.0];
    let leg_color = [color[0] * 0.6, color[1] * 0.6, color[2] * 0.6, 1.0];

    append_colored_quad(&mut mesh, Quad {
        tl: [-hw, top_y, hd], tr: [hw, top_y, hd], bl: [-hw, top_y, -hd], br: [hw, top_y, -hd],
        normal: [0.0, 1.0, 0.0], uv_min: [0.0, 0.0], uv_max: [1.0, 1.0],
    }, top_color);
    append_colored_quad(&mut mesh, Quad {
        tl: [-hw, top_y - 0.02, -hd], tr: [hw, top_y - 0.02, -hd],
        bl: [-hw, top_y - 0.02, hd], br: [hw, top_y - 0.02, hd],
        normal: [0.0, -1.0, 0.0], uv_min: [0.0, 0.0], uv_max: [1.0, 1.0],
    }, top_color);

    let leg_positions = [
        (-hw + leg_t, -hd + leg_t),
        (hw - leg_t, -hd + leg_t),
        (-hw + leg_t, hd - leg_t),
        (hw - leg_t, hd - leg_t),
    ];
    for (lx, lz) in leg_positions {
        append_colored_box(&mut mesh, [lx, leg_h/2.0, lz], [leg_t*2.0, leg_h, leg_t*2.0], leg_color);
    }
    mesh
}
