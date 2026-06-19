import re

with open('crates/building-gen/src/furniture/catalog.rs', 'r') as f:
    code = f.read()

box_mesh_new = """fn generate_box_mesh(w: f32, h: f32, d: f32, color: [f32; 3]) -> MeshData {
    let mut mesh = MeshData::default();
    let hw = w / 2.0;
    let hd = d / 2.0;
    let c = [color[0], color[1], color[2], 1.0];

    append_colored_quad(&mut mesh, Quad {
        tl: [-hw, h, hd], tr: [hw, h, hd], bl: [-hw, h, -hd], br: [hw, h, -hd],
        normal: [0.0, 1.0, 0.0], uv_min: [0.0, 0.0], uv_max: [1.0, 1.0],
    }, c);
    append_colored_quad(&mut mesh, Quad {
        tl: [-hw, h, -hd], tr: [hw, h, -hd], bl: [-hw, 0.0, -hd], br: [hw, 0.0, -hd],
        normal: [0.0, 0.0, -1.0], uv_min: [0.0, 0.0], uv_max: [1.0, 1.0],
    }, c);
    append_colored_quad(&mut mesh, Quad {
        tl: [hw, h, hd], tr: [-hw, h, hd], bl: [hw, 0.0, hd], br: [-hw, 0.0, hd],
        normal: [0.0, 0.0, 1.0], uv_min: [0.0, 0.0], uv_max: [1.0, 1.0],
    }, c);
    append_colored_quad(&mut mesh, Quad {
        tl: [-hw, h, hd], tr: [-hw, h, -hd], bl: [-hw, 0.0, hd], br: [-hw, 0.0, -hd],
        normal: [-1.0, 0.0, 0.0], uv_min: [0.0, 0.0], uv_max: [1.0, 1.0],
    }, c);
    append_colored_quad(&mut mesh, Quad {
        tl: [hw, h, -hd], tr: [hw, h, hd], bl: [hw, 0.0, -hd], br: [hw, 0.0, hd],
        normal: [1.0, 0.0, 0.0], uv_min: [0.0, 0.0], uv_max: [1.0, 1.0],
    }, c);
    mesh
}"""
code = re.sub(r'fn generate_box_mesh.*?mesh\n}', box_mesh_new, code, flags=re.DOTALL)

table_mesh_new = """fn generate_table_mesh(w: f32, top_y: f32, d: f32, color: [f32; 3]) -> MeshData {
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
        append_colored_quad(&mut mesh, Quad {
            tl: [lx - leg_t, leg_h, lz - leg_t], tr: [lx + leg_t, leg_h, lz - leg_t],
            bl: [lx - leg_t, 0.0, lz - leg_t], br: [lx + leg_t, 0.0, lz - leg_t],
            normal: [0.0, 0.0, -1.0], uv_min: [0.0, 0.0], uv_max: [1.0, 1.0],
        }, leg_color);
        append_colored_quad(&mut mesh, Quad {
            tl: [lx + leg_t, leg_h, lz + leg_t], tr: [lx - leg_t, leg_h, lz + leg_t],
            bl: [lx + leg_t, 0.0, lz + leg_t], br: [lx - leg_t, 0.0, lz + leg_t],
            normal: [0.0, 0.0, 1.0], uv_min: [0.0, 0.0], uv_max: [1.0, 1.0],
        }, leg_color);
        append_colored_quad(&mut mesh, Quad {
            tl: [lx - leg_t, leg_h, lz + leg_t], tr: [lx - leg_t, leg_h, lz - leg_t],
            bl: [lx - leg_t, 0.0, lz + leg_t], br: [lx - leg_t, 0.0, lz - leg_t],
            normal: [-1.0, 0.0, 0.0], uv_min: [0.0, 0.0], uv_max: [1.0, 1.0],
        }, leg_color);
        append_colored_quad(&mut mesh, Quad {
            tl: [lx + leg_t, leg_h, lz - leg_t], tr: [lx + leg_t, leg_h, lz + leg_t],
            bl: [lx + leg_t, 0.0, lz - leg_t], br: [lx + leg_t, 0.0, lz + leg_t],
            normal: [1.0, 0.0, 0.0], uv_min: [0.0, 0.0], uv_max: [1.0, 1.0],
        }, leg_color);
    }
    mesh
}"""
code = re.sub(r'fn generate_table_mesh.*?mesh\n}', table_mesh_new, code, flags=re.DOTALL)

chair_mesh_new = """fn generate_chair_mesh(w: f32, seat_h: f32, d: f32, color: [f32; 3]) -> MeshData {
    let mut mesh = MeshData::default();
    let hw = w / 2.0;
    let hd = d / 2.0;
    let leg_t = 0.03;
    let seat_t = 0.03;
    let back_h = 0.4;
    let seat_color = [color[0], color[1], color[2], 1.0];
    let leg_color = [color[0] * 0.6, color[1] * 0.6, color[2] * 0.6, 1.0];

    append_colored_quad(&mut mesh, Quad {
        tl: [-hw, seat_h, hd], tr: [hw, seat_h, hd], bl: [-hw, seat_h, -hd], br: [hw, seat_h, -hd],
        normal: [0.0, 1.0, 0.0], uv_min: [0.0, 0.0], uv_max: [1.0, 1.0],
    }, seat_color);
    append_colored_quad(&mut mesh, Quad {
        tl: [-hw, seat_h + back_h, hd], tr: [hw, seat_h + back_h, hd],
        bl: [-hw, seat_h, hd], br: [hw, seat_h, hd],
        normal: [0.0, 0.0, 1.0], uv_min: [0.0, 0.0], uv_max: [1.0, 1.0],
    }, seat_color);
    append_colored_quad(&mut mesh, Quad {
        tl: [hw, seat_h + back_h, hd - seat_t], tr: [-hw, seat_h + back_h, hd - seat_t],
        bl: [hw, seat_h, hd - seat_t], br: [-hw, seat_h, hd - seat_t],
        normal: [0.0, 0.0, -1.0], uv_min: [0.0, 0.0], uv_max: [1.0, 1.0],
    }, seat_color);

    let leg_positions = [
        (-hw + leg_t, -hd + leg_t),
        (hw - leg_t, -hd + leg_t),
        (-hw + leg_t, hd - leg_t),
        (hw - leg_t, hd - leg_t),
    ];
    for (lx, lz) in leg_positions {
        append_colored_quad(&mut mesh, Quad {
            tl: [lx - leg_t, seat_h, lz - leg_t], tr: [lx + leg_t, seat_h, lz - leg_t],
            bl: [lx - leg_t, 0.0, lz - leg_t], br: [lx + leg_t, 0.0, lz - leg_t],
            normal: [0.0, 0.0, -1.0], uv_min: [0.0, 0.0], uv_max: [1.0, 1.0],
        }, leg_color);
        append_colored_quad(&mut mesh, Quad {
            tl: [lx + leg_t, seat_h, lz + leg_t], tr: [lx - leg_t, seat_h, lz + leg_t],
            bl: [lx + leg_t, 0.0, lz + leg_t], br: [lx - leg_t, 0.0, lz + leg_t],
            normal: [0.0, 0.0, 1.0], uv_min: [0.0, 0.0], uv_max: [1.0, 1.0],
        }, leg_color);
        append_colored_quad(&mut mesh, Quad {
            tl: [lx - leg_t, seat_h, lz + leg_t], tr: [lx - leg_t, seat_h, lz - leg_t],
            bl: [lx - leg_t, 0.0, lz + leg_t], br: [lx - leg_t, 0.0, lz - leg_t],
            normal: [-1.0, 0.0, 0.0], uv_min: [0.0, 0.0], uv_max: [1.0, 1.0],
        }, leg_color);
        append_colored_quad(&mut mesh, Quad {
            tl: [lx + leg_t, seat_h, lz - leg_t], tr: [lx + leg_t, seat_h, lz + leg_t],
            bl: [lx + leg_t, 0.0, lz - leg_t], br: [lx + leg_t, 0.0, lz + leg_t],
            normal: [1.0, 0.0, 0.0], uv_min: [0.0, 0.0], uv_max: [1.0, 1.0],
        }, leg_color);
    }
    mesh
}"""
code = re.sub(r'fn generate_chair_mesh.*?mesh\n}', chair_mesh_new, code, flags=re.DOTALL)

desk_mesh_new = """fn generate_desk_mesh(w: f32, h: f32, d: f32, color: [f32; 3]) -> MeshData {
    let mut mesh = MeshData::default();
    let hw = w / 2.0;
    let hd = d / 2.0;
    let panel_t = 0.03;
    let shelf_h = h * 0.3;
    let top_color = [color[0], color[1], color[2], 1.0];
    let panel_color = [color[0] * 0.7, color[1] * 0.7, color[2] * 0.7, 1.0];

    append_colored_quad(&mut mesh, Quad {
        tl: [-hw, h, hd], tr: [hw, h, hd], bl: [-hw, h, -hd], br: [hw, h, -hd],
        normal: [0.0, 1.0, 0.0], uv_min: [0.0, 0.0], uv_max: [1.0, 1.0],
    }, top_color);

    append_colored_quad(&mut mesh, Quad {
        tl: [-hw, h, hd], tr: [-hw, h, -hd], bl: [-hw, 0.0, hd], br: [-hw, 0.0, -hd],
        normal: [-1.0, 0.0, 0.0], uv_min: [0.0, 0.0], uv_max: [1.0, 1.0],
    }, panel_color);
    append_colored_quad(&mut mesh, Quad {
        tl: [hw, h, -hd], tr: [hw, h, hd], bl: [hw, 0.0, -hd], br: [hw, 0.0, hd],
        normal: [1.0, 0.0, 0.0], uv_min: [0.0, 0.0], uv_max: [1.0, 1.0],
    }, panel_color);

    append_colored_quad(&mut mesh, Quad {
        tl: [-hw + panel_t, shelf_h, -hd + panel_t], tr: [hw - panel_t, shelf_h, -hd + panel_t],
        bl: [-hw + panel_t, shelf_h, hd - panel_t], br: [hw - panel_t, shelf_h, hd - panel_t],
        normal: [0.0, 1.0, 0.0], uv_min: [0.0, 0.0], uv_max: [1.0, 1.0],
    }, top_color);

    mesh
}"""
code = re.sub(r'fn generate_desk_mesh.*?mesh\n}', desk_mesh_new, code, flags=re.DOTALL)

crate_mesh_new = """fn generate_crate_mesh(w: f32, h: f32, d: f32, color: [f32; 3]) -> MeshData {
    let mut mesh = MeshData::default();
    let hw = w / 2.0;
    let hd = d / 2.0;
    let wood_color = [color[0], color[1], color[2], 1.0];
    let metal_color = [0.2, 0.2, 0.2, 1.0];

    append_colored_quad(&mut mesh, Quad {
        tl: [-hw, h, hd], tr: [hw, h, hd], bl: [-hw, h, -hd], br: [hw, h, -hd],
        normal: [0.0, 1.0, 0.0], uv_min: [0.0, 0.0], uv_max: [1.0, 1.0],
    }, wood_color);
    append_colored_quad(&mut mesh, Quad {
        tl: [-hw, h, -hd], tr: [hw, h, -hd], bl: [-hw, 0.0, -hd], br: [hw, 0.0, -hd],
        normal: [0.0, 0.0, -1.0], uv_min: [0.0, 0.0], uv_max: [1.0, 1.0],
    }, wood_color);
    append_colored_quad(&mut mesh, Quad {
        tl: [hw, h, hd], tr: [-hw, h, hd], bl: [hw, 0.0, hd], br: [-hw, 0.0, hd],
        normal: [0.0, 0.0, 1.0], uv_min: [0.0, 0.0], uv_max: [1.0, 1.0],
    }, wood_color);
    append_colored_quad(&mut mesh, Quad {
        tl: [-hw, h, hd], tr: [-hw, h, -hd], bl: [-hw, 0.0, hd], br: [-hw, 0.0, -hd],
        normal: [-1.0, 0.0, 0.0], uv_min: [0.0, 0.0], uv_max: [1.0, 1.0],
    }, wood_color);
    append_colored_quad(&mut mesh, Quad {
        tl: [hw, h, -hd], tr: [hw, h, hd], bl: [hw, 0.0, -hd], br: [hw, 0.0, hd],
        normal: [1.0, 0.0, 0.0], uv_min: [0.0, 0.0], uv_max: [1.0, 1.0],
    }, wood_color);

    // Cross brace details on front/back (-Z and +Z)
    let margin = 0.02;
    let t = 0.04;
    // Front cross
    append_colored_quad(&mut mesh, Quad {
        tl: [-hw + margin, h - margin, -hd - 0.01], tr: [hw - margin, margin, -hd - 0.01],
        bl: [-hw + margin + t, h - margin, -hd - 0.01], br: [hw - margin + t, margin, -hd - 0.01],
        normal: [0.0, 0.0, -1.0], uv_min: [0.0, 0.0], uv_max: [1.0, 1.0],
    }, metal_color);
    append_colored_quad(&mut mesh, Quad {
        tl: [hw - margin, h - margin, -hd - 0.01], tr: [-hw + margin, margin, -hd - 0.01],
        bl: [hw - margin - t, h - margin, -hd - 0.01], br: [-hw + margin - t, margin, -hd - 0.01],
        normal: [0.0, 0.0, -1.0], uv_min: [0.0, 0.0], uv_max: [1.0, 1.0],
    }, metal_color);
    
    // Back cross
    append_colored_quad(&mut mesh, Quad {
        tl: [hw - margin, h - margin, hd + 0.01], tr: [-hw + margin, margin, hd + 0.01],
        bl: [hw - margin - t, h - margin, hd + 0.01], br: [-hw + margin - t, margin, hd + 0.01],
        normal: [0.0, 0.0, 1.0], uv_min: [0.0, 0.0], uv_max: [1.0, 1.0],
    }, metal_color);
    append_colored_quad(&mut mesh, Quad {
        tl: [-hw + margin, h - margin, hd + 0.01], tr: [hw - margin, margin, hd + 0.01],
        bl: [-hw + margin + t, h - margin, hd + 0.01], br: [hw - margin + t, margin, hd + 0.01],
        normal: [0.0, 0.0, 1.0], uv_min: [0.0, 0.0], uv_max: [1.0, 1.0],
    }, metal_color);

    mesh
}"""
code = re.sub(r'fn generate_crate_mesh.*?mesh\n}', crate_mesh_new, code, flags=re.DOTALL)

bench_mesh_new = """fn generate_bench_mesh(w: f32, h: f32, d: f32, color: [f32; 3]) -> MeshData {
    let mut mesh = MeshData::default();
    let hw = w / 2.0;
    let hd = d / 2.0;
    let seat_t = 0.04;
    let leg_w = 0.05;
    let seat_color = [color[0], color[1], color[2], 1.0];
    let leg_color = [color[0] * 0.6, color[1] * 0.6, color[2] * 0.6, 1.0];

    append_colored_quad(&mut mesh, Quad {
        tl: [-hw, h, hd], tr: [hw, h, hd], bl: [-hw, h, -hd], br: [hw, h, -hd],
        normal: [0.0, 1.0, 0.0], uv_min: [0.0, 0.0], uv_max: [1.0, 1.0],
    }, seat_color);
    append_colored_quad(&mut mesh, Quad {
        tl: [-hw, h - seat_t, -hd], tr: [hw, h - seat_t, -hd],
        bl: [-hw, h - seat_t, hd], br: [hw, h - seat_t, hd],
        normal: [0.0, -1.0, 0.0], uv_min: [0.0, 0.0], uv_max: [1.0, 1.0],
    }, seat_color);

    let x_offset = hw * 0.8;
    append_colored_quad(&mut mesh, Quad {
        tl: [-x_offset - leg_w / 2.0, h - seat_t, hd], tr: [-x_offset - leg_w / 2.0, h - seat_t, -hd],
        bl: [-x_offset - leg_w / 2.0, 0.0, hd], br: [-x_offset - leg_w / 2.0, 0.0, -hd],
        normal: [-1.0, 0.0, 0.0], uv_min: [0.0, 0.0], uv_max: [1.0, 1.0],
    }, leg_color);
    append_colored_quad(&mut mesh, Quad {
        tl: [-x_offset + leg_w / 2.0, h - seat_t, -hd], tr: [-x_offset + leg_w / 2.0, h - seat_t, hd],
        bl: [-x_offset + leg_w / 2.0, 0.0, -hd], br: [-x_offset + leg_w / 2.0, 0.0, hd],
        normal: [1.0, 0.0, 0.0], uv_min: [0.0, 0.0], uv_max: [1.0, 1.0],
    }, leg_color);
    append_colored_quad(&mut mesh, Quad {
        tl: [x_offset - leg_w / 2.0, h - seat_t, hd], tr: [x_offset - leg_w / 2.0, h - seat_t, -hd],
        bl: [x_offset - leg_w / 2.0, 0.0, hd], br: [x_offset - leg_w / 2.0, 0.0, -hd],
        normal: [-1.0, 0.0, 0.0], uv_min: [0.0, 0.0], uv_max: [1.0, 1.0],
    }, leg_color);
    append_colored_quad(&mut mesh, Quad {
        tl: [x_offset + leg_w / 2.0, h - seat_t, -hd], tr: [x_offset + leg_w / 2.0, h - seat_t, hd],
        bl: [x_offset + leg_w / 2.0, 0.0, -hd], br: [x_offset + leg_w / 2.0, 0.0, hd],
        normal: [1.0, 0.0, 0.0], uv_min: [0.0, 0.0], uv_max: [1.0, 1.0],
    }, leg_color);

    mesh
}"""
code = re.sub(r'fn generate_bench_mesh.*?mesh\n}', bench_mesh_new, code, flags=re.DOTALL)

with open('crates/building-gen/src/furniture/catalog.rs', 'w') as f:
    f.write(code)
