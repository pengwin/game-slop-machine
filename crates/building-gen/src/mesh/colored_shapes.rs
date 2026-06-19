use super::math_util::{append_colored_quad, Quad};
use super::MeshData;

pub fn append_colored_box(mesh: &mut MeshData, center: [f32; 3], size: [f32; 3], color: [f32; 4]) {
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

pub fn append_colored_rotated_box(mesh: &mut MeshData, center: [f32; 3], size: [f32; 3], rot_y: f32, rot_z: f32, color: [f32; 4]) {
    let mut bmesh = MeshData::default();
    append_colored_box(&mut bmesh, [0.0, 0.0, 0.0], size, color);
    
    let cy = rot_y.cos();
    let sy = rot_y.sin();
    let cz = rot_z.cos();
    let sz = rot_z.sin();
    let pivot_y = -size[1] / 2.0;

    // First pass: find minimum Y after rotation
    let mut min_y = f32::MAX;
    for v in &bmesh.vertices {
        let x = v[0];
        let y = v[1] - pivot_y;

        let _x1 = x * cz - y * sz;
        let y1 = x * sz + y * cz;
        
        let y2 = y1;
        if y2 < min_y {
            min_y = y2;
        }
    }

    // Second pass: apply rotation and shift up so min_y == 0
    for v in &mut bmesh.vertices {
        let x = v[0];
        let y = v[1] - pivot_y;
        let z = v[2];

        let x1 = x * cz - y * sz;
        let y1 = x * sz + y * cz;
        let z1 = z;

        let x2 = x1 * cy + z1 * sy;
        let y2 = y1 - min_y; // Shift up to sit on shelf
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
    
    mesh.merge_from(&bmesh);
}

pub fn append_colored_beveled_box(mesh: &mut MeshData, center: [f32; 3], size: [f32; 3], bevel: f32, color: [f32; 4]) {
    let hw = size[0] / 2.0;
    let hh = size[1] / 2.0;
    let hd = size[2] / 2.0;
    let cx = center[0];
    let cy = center[1];
    let cz = center[2];

    let thw = (hw - bevel).max(0.001);
    let thd = (hd - bevel).max(0.001);

    // Top
    append_colored_quad(mesh, Quad {
        tl: [cx - thw, cy + hh, cz + thd], tr: [cx + thw, cy + hh, cz + thd],
        bl: [cx - thw, cy + hh, cz - thd], br: [cx + thw, cy + hh, cz - thd],
        normal: [0.0, 1.0, 0.0], uv_min: [0.0, 0.0], uv_max: [1.0, 1.0],
    }, color);
    // Bottom
    append_colored_quad(mesh, Quad {
        tl: [cx - hw, cy - hh, cz - hd], tr: [cx + hw, cy - hh, cz - hd],
        bl: [cx - hw, cy - hh, cz + hd], br: [cx + hw, cy - hh, cz + hd],
        normal: [0.0, -1.0, 0.0], uv_min: [0.0, 0.0], uv_max: [1.0, 1.0],
    }, color);

    // Front (-Z)
    let ny_z = bevel;
    let nz_z = -2.0 * hh;
    let len_z = (ny_z * ny_z + nz_z * nz_z).sqrt();
    let norm_front = [0.0, ny_z / len_z, nz_z / len_z];
    append_colored_quad(mesh, Quad {
        tl: [cx - thw, cy + hh, cz - thd], tr: [cx + thw, cy + hh, cz - thd],
        bl: [cx - hw, cy - hh, cz - hd], br: [cx + hw, cy - hh, cz - hd],
        normal: norm_front, uv_min: [0.0, 0.0], uv_max: [1.0, 1.0],
    }, color);

    // Back (+Z)
    let norm_back = [0.0, ny_z / len_z, -nz_z / len_z];
    append_colored_quad(mesh, Quad {
        tl: [cx + thw, cy + hh, cz + thd], tr: [cx - thw, cy + hh, cz + thd],
        bl: [cx + hw, cy - hh, cz + hd], br: [cx - hw, cy - hh, cz + hd],
        normal: norm_back, uv_min: [0.0, 0.0], uv_max: [1.0, 1.0],
    }, color);

    // Left (-X)
    let nx_x = -2.0 * hh;
    let ny_x = bevel;
    let len_x = (nx_x * nx_x + ny_x * ny_x).sqrt();
    let norm_left = [nx_x / len_x, ny_x / len_x, 0.0];
    append_colored_quad(mesh, Quad {
        tl: [cx - thw, cy + hh, cz + thd], tr: [cx - thw, cy + hh, cz - thd],
        bl: [cx - hw, cy - hh, cz + hd], br: [cx - hw, cy - hh, cz - hd],
        normal: norm_left, uv_min: [0.0, 0.0], uv_max: [1.0, 1.0],
    }, color);

    // Right (+X)
    let norm_right = [-nx_x / len_x, ny_x / len_x, 0.0];
    append_colored_quad(mesh, Quad {
        tl: [cx + thw, cy + hh, cz - thd], tr: [cx + thw, cy + hh, cz + thd],
        bl: [cx + hw, cy - hh, cz - hd], br: [cx + hw, cy - hh, cz + hd],
        normal: norm_right, uv_min: [0.0, 0.0], uv_max: [1.0, 1.0],
    }, color);
}

/// Generates a box with top and 4 sides (no bottom).
pub fn generate_box_mesh(w: f32, h: f32, d: f32, color: [f32; 3]) -> MeshData {
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
}
