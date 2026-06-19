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
