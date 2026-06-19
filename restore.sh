#!/bin/bash

# Restore fix_furniture.py
cat << 'PYEOF' > fix_furniture.py
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
PYEOF

cat << 'PYEOF' > fix_bed.py
import re

with open('crates/building-gen/src/furniture/catalog.rs', 'r') as f:
    code = f.read()

box_helper = """
fn append_colored_box(mesh: &mut MeshData, center: [f32; 3], size: [f32; 3], color: [f32; 4]) {
    let hw = size[0] / 2.0;
    let hh = size[1] / 2.0;
    let hd = size[2] / 2.0;
    let cx = center[0];
    let cy = center[1];
    let cz = center[2];

    // Top
    append_colored_quad(mesh, Quad {
        tl: [cx - hw, cy + hh, cz + hd], tr: [cx + hw, cy + hh, cz + hd],
        bl: [cx - hw, cy + hh, cz - hd], br: [cx + hw, cy + hh, cz - hd],
        normal: [0.0, 1.0, 0.0], uv_min: [0.0, 0.0], uv_max: [1.0, 1.0],
    }, color);
    // Bottom
    append_colored_quad(mesh, Quad {
        tl: [cx - hw, cy - hh, cz - hd], tr: [cx + hw, cy - hh, cz - hd],
        bl: [cx - hw, cy - hh, cz + hd], br: [cx + hw, cy - hh, cz + hd],
        normal: [0.0, -1.0, 0.0], uv_min: [0.0, 0.0], uv_max: [1.0, 1.0],
    }, color);
    // Front (-Z)
    append_colored_quad(mesh, Quad {
        tl: [cx - hw, cy + hh, cz - hd], tr: [cx + hw, cy + hh, cz - hd],
        bl: [cx - hw, cy - hh, cz - hd], br: [cx + hw, cy - hh, cz - hd],
        normal: [0.0, 0.0, -1.0], uv_min: [0.0, 0.0], uv_max: [1.0, 1.0],
    }, color);
    // Back (+Z)
    append_colored_quad(mesh, Quad {
        tl: [cx + hw, cy + hh, cz + hd], tr: [cx - hw, cy + hh, cz + hd],
        bl: [cx + hw, cy - hh, cz + hd], br: [cx - hw, cy - hh, cz + hd],
        normal: [0.0, 0.0, 1.0], uv_min: [0.0, 0.0], uv_max: [1.0, 1.0],
    }, color);
    // Left (-X)
    append_colored_quad(mesh, Quad {
        tl: [cx - hw, cy + hh, cz + hd], tr: [cx - hw, cy + hh, cz - hd],
        bl: [cx - hw, cy - hh, cz + hd], br: [cx - hw, cy - hh, cz - hd],
        normal: [-1.0, 0.0, 0.0], uv_min: [0.0, 0.0], uv_max: [1.0, 1.0],
    }, color);
    // Right (+X)
    append_colored_quad(mesh, Quad {
        tl: [cx + hw, cy + hh, cz - hd], tr: [cx + hw, cy + hh, cz + hd],
        bl: [cx + hw, cy - hh, cz - hd], br: [cx + hw, cy - hh, cz + hd],
        normal: [1.0, 0.0, 0.0], uv_min: [0.0, 0.0], uv_max: [1.0, 1.0],
    }, color);
}

/// Generates a box with top and 4 sides (no bottom).
"""
code = code.replace("/// Generates a box with top and 4 sides (no bottom).\n", box_helper)

with open('crates/building-gen/src/furniture/catalog.rs', 'w') as f:
    f.write(code)
PYEOF

python3 fix_furniture.py
sed -i 's/use crate::mesh::math_util::{append_colored_quad, append_colored_triangle, append_quad, Quad};/use crate::mesh::math_util::{append_colored_quad, append_colored_triangle, Quad};/' crates/building-gen/src/furniture/catalog.rs
python3 fix_bed.py
python3 update_bed.py
python3 bevel_pillow.py
python3 add_shelf.py
python3 fix_shelf_z.py
python3 fix_knob_z.py

cat << 'PYEOF' > fix_books2.py
import re

with open('crates/building-gen/src/furniture/catalog.rs', 'r') as f:
    code = f.read()

rotated_box_code = """
fn append_colored_rotated_box(mesh: &mut MeshData, center: [f32; 3], size: [f32; 3], rot_y: f32, rot_z: f32, color: [f32; 4]) {
    let mut bmesh = MeshData::default();
    append_colored_box(&mut bmesh, [0.0, 0.0, 0.0], size, color);
    
    let cy = rot_y.cos();
    let sy = rot_y.sin();
    let cz = rot_z.cos();
    let sz = rot_z.sin();
    
    let pivot_y = -size[1] / 2.0;

    for v in &mut bmesh.vertices {
        let x = v[0];
        let y = v[1] - pivot_y;
        let z = v[2];

        let x1 = x * cz - y * sz;
        let y1 = x * sz + y * cz;
        let z1 = z;

        let x2 = x1 * cy + z1 * sy;
        let y2 = y1;
        let z2 = -x1 * sy + z1 * cy;

        v[0] = x2 + center[0];
        v[1] = y2 + pivot_y + center[1];
        v[2] = z2 + center[2];
    }
    
    for n in &mut bmesh.normals {
        let x = n[0];
        let y = n[1];
        let z = n[2];
        
        let x1 = x * cz - y * sz;
        let y1 = x * sz + y * cz;
        let z1 = z;
        
        let x2 = x1 * cy + z1 * sy;
        let y2 = y1;
        let z2 = -x1 * sy + z1 * cy;
        
        n[0] = x2;
        n[1] = y2;
        n[2] = z2;
    }
    
    let base_idx = mesh.vertices.len() as u32;
    mesh.vertices.extend(bmesh.vertices);
    mesh.normals.extend(bmesh.normals);
    mesh.uvs.extend(bmesh.uvs);
    mesh.colors.extend(bmesh.colors);
    mesh.indices.extend(bmesh.indices.iter().map(|&idx| idx + base_idx));
}
"""

code = code.replace("fn append_colored_beveled_box", rotated_box_code + "\nfn append_colored_beveled_box")

old_books_code_start = "let mut book_idx = 0;"
old_books_code_end = "    mesh\n}"

new_books_code = """let mut book_idx = 0;
    let total_books = config.books.len();
    
    if total_books > 0 {
        let books_per_row = (total_books as f32 / rows as f32).ceil() as usize;
        
        for r in 0..rows {
            let shelf_top_y = if r == 0 {
                base_h
            } else {
                base_h + r as f32 * spacing + pt
            };
            
            let start_x = -hw + pt + 0.05;
            let book_w = 0.04;
            let book_h = spacing * 0.6;
            let book_d = shelf_d * 0.7;
            
            for i in 0..books_per_row {
                if book_idx < total_books {
                    let bx = start_x + (i as f32) * (book_w + 0.02) + book_w/2.0;
                    
                    let rot_z = if i == 0 {
                        -0.5_f32
                    } else if i == 1 {
                        -0.25_f32
                    } else {
                        0.0
                    };
                    
                    let rot_y = (i as f32 * 0.15) - 0.1;
                    
                    append_colored_rotated_box(
                        &mut mesh, 
                        [bx, shelf_top_y + book_h/2.0, shelf_z], 
                        [book_w, book_h, book_d], 
                        rot_y, 
                        rot_z, 
                        config.books[book_idx]
                    );
                    
                    book_idx += 1;
                }
            }
            
            if r == 1 {
                let vase_x = hw - pt - 0.15;
                let vase_size = 0.1;
                let vase_color = [0.8, 0.8, 0.8, 1.0];
                append_colored_beveled_box(&mut mesh, [vase_x, shelf_top_y + vase_size/2.0, shelf_z], [vase_size, vase_size, vase_size], 0.03, vase_color);
            }
        }
    }

    mesh
}"""

idx_start = code.rfind(old_books_code_start)
idx_end = code.find(old_books_code_end, idx_start) + len(old_books_code_end)

code = code[:idx_start] + new_books_code + code[idx_end:]

with open('crates/building-gen/src/furniture/catalog.rs', 'w') as f:
    f.write(code)
PYEOF

python3 fix_books2.py
cargo check -p building-gen
