import re

with open('crates/building-gen/src/furniture/catalog.rs', 'r') as f:
    code = f.read()

# 1. Replace append_quad with append_colored_quad in imports
code = code.replace("use crate::mesh::math_util::{append_colored_quad, append_colored_triangle, append_quad, Quad};",
                    "use crate::mesh::math_util::{append_colored_quad, append_colored_triangle, Quad};")

# 2. Add append_colored_box, append_colored_beveled_box, append_colored_rotated_box
helpers = """
fn append_colored_box(mesh: &mut MeshData, center: [f32; 3], size: [f32; 3], color: [f32; 4]) {
    let hw = size[0] / 2.0;
    let hh = size[1] / 2.0;
    let hd = size[2] / 2.0;
    let cx = center[0];
    let cy = center[1];
    let cz = center[2];

    append_colored_quad(mesh, Quad {
        tl: [cx - hw, cy + hh, cz + hd], tr: [cx + hw, cy + hh, cz + hd],
        bl: [cx - hw, cy + hh, cz - hd], br: [cx + hw, cy + hh, cz - hd],
        normal: [0.0, 1.0, 0.0], uv_min: [0.0, 0.0], uv_max: [1.0, 1.0],
    }, color);
    append_colored_quad(mesh, Quad {
        tl: [cx - hw, cy - hh, cz - hd], tr: [cx + hw, cy - hh, cz - hd],
        bl: [cx - hw, cy - hh, cz + hd], br: [cx + hw, cy - hh, cz + hd],
        normal: [0.0, -1.0, 0.0], uv_min: [0.0, 0.0], uv_max: [1.0, 1.0],
    }, color);
    append_colored_quad(mesh, Quad {
        tl: [cx - hw, cy + hh, cz - hd], tr: [cx + hw, cy + hh, cz - hd],
        bl: [cx - hw, cy - hh, cz - hd], br: [cx + hw, cy - hh, cz - hd],
        normal: [0.0, 0.0, -1.0], uv_min: [0.0, 0.0], uv_max: [1.0, 1.0],
    }, color);
    append_colored_quad(mesh, Quad {
        tl: [cx + hw, cy + hh, cz + hd], tr: [cx - hw, cy + hh, cz + hd],
        bl: [cx + hw, cy - hh, cz + hd], br: [cx - hw, cy - hh, cz + hd],
        normal: [0.0, 0.0, 1.0], uv_min: [0.0, 0.0], uv_max: [1.0, 1.0],
    }, color);
    append_colored_quad(mesh, Quad {
        tl: [cx - hw, cy + hh, cz + hd], tr: [cx - hw, cy + hh, cz - hd],
        bl: [cx - hw, cy - hh, cz + hd], br: [cx - hw, cy - hh, cz - hd],
        normal: [-1.0, 0.0, 0.0], uv_min: [0.0, 0.0], uv_max: [1.0, 1.0],
    }, color);
    append_colored_quad(mesh, Quad {
        tl: [cx + hw, cy + hh, cz - hd], tr: [cx + hw, cy + hh, cz + hd],
        bl: [cx + hw, cy - hh, cz - hd], br: [cx + hw, cy - hh, cz + hd],
        normal: [1.0, 0.0, 0.0], uv_min: [0.0, 0.0], uv_max: [1.0, 1.0],
    }, color);
}

fn append_colored_beveled_box(mesh: &mut MeshData, center: [f32; 3], size: [f32; 3], bevel: f32, color: [f32; 4]) {
    let hw = size[0] / 2.0;
    let hh = size[1] / 2.0;
    let hd = size[2] / 2.0;
    let cx = center[0];
    let cy = center[1];
    let cz = center[2];

    let thw = (hw - bevel).max(0.001);
    let thd = (hd - bevel).max(0.001);

    append_colored_quad(mesh, Quad {
        tl: [cx - thw, cy + hh, cz + thd], tr: [cx + thw, cy + hh, cz + thd],
        bl: [cx - thw, cy + hh, cz - thd], br: [cx + thw, cy + hh, cz - thd],
        normal: [0.0, 1.0, 0.0], uv_min: [0.0, 0.0], uv_max: [1.0, 1.0],
    }, color);
    append_colored_quad(mesh, Quad {
        tl: [cx - hw, cy - hh, cz - hd], tr: [cx + hw, cy - hh, cz - hd],
        bl: [cx - hw, cy - hh, cz + hd], br: [cx + hw, cy - hh, cz + hd],
        normal: [0.0, -1.0, 0.0], uv_min: [0.0, 0.0], uv_max: [1.0, 1.0],
    }, color);

    let ny_z = bevel;
    let nz_z = -2.0 * hh;
    let len_z = (ny_z * ny_z + nz_z * nz_z).sqrt();
    let norm_front = [0.0, ny_z / len_z, nz_z / len_z];
    append_colored_quad(mesh, Quad {
        tl: [cx - thw, cy + hh, cz - thd], tr: [cx + thw, cy + hh, cz - thd],
        bl: [cx - hw, cy - hh, cz - hd], br: [cx + hw, cy - hh, cz - hd],
        normal: norm_front, uv_min: [0.0, 0.0], uv_max: [1.0, 1.0],
    }, color);

    let norm_back = [0.0, ny_z / len_z, -nz_z / len_z];
    append_colored_quad(mesh, Quad {
        tl: [cx + thw, cy + hh, cz + thd], tr: [cx - thw, cy + hh, cz + thd],
        bl: [cx + hw, cy - hh, cz + hd], br: [cx - hw, cy - hh, cz + hd],
        normal: norm_back, uv_min: [0.0, 0.0], uv_max: [1.0, 1.0],
    }, color);

    let nx_x = -2.0 * hh;
    let ny_x = bevel;
    let len_x = (nx_x * nx_x + ny_x * ny_x).sqrt();
    let norm_left = [nx_x / len_x, ny_x / len_x, 0.0];
    append_colored_quad(mesh, Quad {
        tl: [cx - thw, cy + hh, cz + thd], tr: [cx - thw, cy + hh, cz - thd],
        bl: [cx - hw, cy - hh, cz + hd], br: [cx - hw, cy - hh, cz - hd],
        normal: norm_left, uv_min: [0.0, 0.0], uv_max: [1.0, 1.0],
    }, color);

    let norm_right = [-nx_x / len_x, ny_x / len_x, 0.0];
    append_colored_quad(mesh, Quad {
        tl: [cx + thw, cy + hh, cz - thd], tr: [cx + thw, cy + hh, cz + thd],
        bl: [cx + hw, cy - hh, cz - hd], br: [cx + hw, cy - hh, cz + hd],
        normal: norm_right, uv_min: [0.0, 0.0], uv_max: [1.0, 1.0],
    }, color);
}

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
code = code.replace("/// Generates a box with top and 4 sides (no bottom).", helpers + "\n/// Generates a box with top and 4 sides (no bottom).", 1)

# 3. Box
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

# 4. Table
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
        append_colored_box(&mut mesh, [lx, leg_h/2.0, lz], [leg_t*2.0, leg_h, leg_t*2.0], leg_color);
    }
    mesh
}"""
code = re.sub(r'fn generate_table_mesh.*?mesh\n}', table_mesh_new, code, flags=re.DOTALL)

# 5. Chair
chair_mesh_new = """fn generate_chair_mesh(w: f32, seat_h: f32, d: f32, color: [f32; 3]) -> MeshData {
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
}"""
code = re.sub(r'fn generate_chair_mesh.*?mesh\n}', chair_mesh_new, code, flags=re.DOTALL)

# 6. Bed
bed_code = """
#[derive(Debug, Clone)]
pub struct BedConfig {
    pub num_pillows: u32,
    pub pillow_size: [f32; 3],
    pub headboard_height: f32,
    pub footboard_height: f32,
    pub frame_height: f32,
    pub wood_color: [f32; 4],
    pub sheet_color: [f32; 4],
    pub blanket_color: [f32; 4],
}
impl Default for BedConfig {
    fn default() -> Self {
        Self {
            num_pillows: 1,
            pillow_size: [0.4, 0.08, 0.25],
            headboard_height: 1.0,
            footboard_height: 0.7,
            frame_height: 0.15,
            wood_color: [0.5, 0.3, 0.15, 1.0], // More brown
            sheet_color: [0.95, 0.95, 0.95, 1.0],
            blanket_color: [0.65, 0.35, 0.25, 1.0],
        }
    }
}

fn generate_bed_mesh(w: f32, h: f32, d: f32, config: &BedConfig) -> MeshData {
    let mut mesh = MeshData::default();
    
    let frame_color = config.wood_color;
    let sheet_color = config.sheet_color;
    let blanket_color = config.blanket_color;

    let pt = 0.08; 
    let front_h = h * config.footboard_height;
    let back_h = h * config.headboard_height;
    
    let px = w / 2.0 - pt / 2.0;
    let pz = d / 2.0 - pt / 2.0;
    
    append_colored_box(&mut mesh, [-px, front_h/2.0, -pz], [pt, front_h, pt], frame_color);
    append_colored_box(&mut mesh, [px, front_h/2.0, -pz], [pt, front_h, pt], frame_color);
    append_colored_box(&mut mesh, [-px, back_h/2.0, pz], [pt, back_h, pt], frame_color);
    append_colored_box(&mut mesh, [px, back_h/2.0, pz], [pt, back_h, pt], frame_color);

    let hb_h = back_h - config.frame_height;
    append_colored_box(&mut mesh, [0.0, config.frame_height + hb_h/2.0, pz], [w - pt*2.0, hb_h, pt/2.0], frame_color);

    let fb_h = front_h - config.frame_height;
    append_colored_box(&mut mesh, [0.0, config.frame_height + fb_h/2.0, -pz], [w - pt*2.0, fb_h, pt/2.0], frame_color);

    let rail_h = config.frame_height;
    let rail_y = 0.12 + rail_h/2.0; 
    let rail_len = d - pt*2.0;
    append_colored_box(&mut mesh, [-px, rail_y, 0.0], [pt/2.0, rail_h, rail_len], frame_color);
    append_colored_box(&mut mesh, [px, rail_y, 0.0], [pt/2.0, rail_h, rail_len], frame_color);

    let mattress_w = w - pt*1.5;
    let mattress_d = d - pt*1.5;
    let mattress_y = rail_y;
    let mattress_h = rail_h + 0.02;
    append_colored_box(&mut mesh, [0.0, mattress_y, 0.0], [mattress_w, mattress_h, mattress_d], sheet_color);

    let blanket_z_min = -pz + pt/2.0;
    let blanket_z_max = pz - pt/2.0 - 0.3; 
    if blanket_z_max > blanket_z_min {
        let blanket_len = blanket_z_max - blanket_z_min;
        let blanket_z = blanket_z_min + blanket_len/2.0;
        let blanket_w = mattress_w + 0.02;
        let blanket_h = mattress_h + 0.02;
        append_colored_box(&mut mesh, [0.0, mattress_y + 0.01, blanket_z], [blanket_w, blanket_h, blanket_len], blanket_color);
    }

    if config.num_pillows > 0 {
        let pillow_y = mattress_y + mattress_h/2.0 + config.pillow_size[1]/2.0;
        let pillow_z = pz - pt/2.0 - 0.15;
        
        let total_pillow_w = config.pillow_size[0] * config.num_pillows as f32;
        let spacing = if config.num_pillows > 1 {
            (mattress_w - 0.1 - total_pillow_w) / (config.num_pillows as f32 - 1.0).max(1.0)
        } else {
            0.0
        };
        
        let start_x = if config.num_pillows == 1 {
            0.0
        } else {
            -(total_pillow_w + spacing * (config.num_pillows as f32 - 1.0)) / 2.0 + config.pillow_size[0] / 2.0
        };

        for i in 0..config.num_pillows {
            let px = start_x + (config.pillow_size[0] + spacing) * i as f32;
            let bevel_amount = config.pillow_size[1] * 0.4;
            append_colored_beveled_box(&mut mesh, [px, pillow_y, pillow_z], config.pillow_size, bevel_amount, sheet_color);
        }
    }

    mesh
}
"""
code = re.sub(r'fn generate_bed_mesh.*?mesh\n}', bed_code, code, flags=re.DOTALL)
code = code.replace("generate_bed_mesh(w, h, d, color)", "generate_bed_mesh(w, h, d, &BedConfig::default())")

# 7. Desk, Crate, Bench
desk_mesh_new = """fn generate_desk_mesh(w: f32, h: f32, d: f32, color: [f32; 3]) -> MeshData {
    let mut mesh = MeshData::default();
    let top_color = [color[0], color[1], color[2], 1.0];
    let panel_color = [color[0] * 0.7, color[1] * 0.7, color[2] * 0.7, 1.0];
    let pt = 0.03;
    append_colored_box(&mut mesh, [0.0, h - pt/2.0, 0.0], [w, pt, d], top_color);
    append_colored_box(&mut mesh, [-w/2.0 + pt/2.0, (h-pt)/2.0, 0.0], [pt, h-pt, d], panel_color);
    append_colored_box(&mut mesh, [w/2.0 - pt/2.0, (h-pt)/2.0, 0.0], [pt, h-pt, d], panel_color);
    append_colored_box(&mut mesh, [0.0, h*0.3, 0.0], [w - 2.0*pt, pt, d], top_color);
    mesh
}"""
code = re.sub(r'fn generate_desk_mesh.*?mesh\n}', desk_mesh_new, code, flags=re.DOTALL)

crate_mesh_new = """fn generate_crate_mesh(w: f32, h: f32, d: f32, color: [f32; 3]) -> MeshData {
    let mut mesh = MeshData::default();
    let wood_color = [color[0], color[1], color[2], 1.0];
    let metal_color = [0.2, 0.2, 0.2, 1.0];
    
    append_colored_box(&mut mesh, [0.0, h/2.0, 0.0], [w, h, d], wood_color);
    // Simple metal straps
    let t = 0.02;
    append_colored_box(&mut mesh, [0.0, h/2.0, d/2.0], [w, t, t], metal_color);
    append_colored_box(&mut mesh, [0.0, h/2.0, -d/2.0], [w, t, t], metal_color);
    mesh
}"""
code = re.sub(r'fn generate_crate_mesh.*?mesh\n}', crate_mesh_new, code, flags=re.DOTALL)

bench_mesh_new = """fn generate_bench_mesh(w: f32, h: f32, d: f32, color: [f32; 3]) -> MeshData {
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
}"""
code = re.sub(r'fn generate_bench_mesh.*?mesh\n}', bench_mesh_new, code, flags=re.DOTALL)

# 8. Shelf
shelf_code = """
#[derive(Debug, Clone)]
pub struct ShelfConfig {
    pub height: f32,
    pub rows: u32,
    pub wood_color: [f32; 4],
    pub cabinet_color: [f32; 4],
    pub books: Vec<[f32; 4]>,
}
impl Default for ShelfConfig {
    fn default() -> Self {
        Self {
            height: 1.2,
            rows: 2,
            wood_color: [0.55, 0.40, 0.25, 1.0],
            cabinet_color: [0.5, 0.35, 0.2, 1.0],
            books: vec![
                [0.2, 0.5, 0.8, 1.0], // Blue
                [0.2, 0.5, 0.8, 1.0], // Blue
                [0.8, 0.3, 0.2, 1.0], // Red
            ],
        }
    }
}

fn generate_shelf_mesh(w: f32, h: f32, d: f32, config: &ShelfConfig) -> MeshData {
    let mut mesh = MeshData::default();
    let hw = w / 2.0;
    let hd = d / 2.0;
    let pt = 0.04;
    
    let mut actual_h = config.height;
    if actual_h <= 0.0 { actual_h = h; }
    let rows = config.rows.max(1);

    let shelf_d = d - pt;
    let shelf_z = hd - shelf_d / 2.0;
    
    append_colored_box(&mut mesh, [-hw + pt/2.0, actual_h/2.0, 0.0], [pt, actual_h, d], config.wood_color);
    append_colored_box(&mut mesh, [hw - pt/2.0, actual_h/2.0, 0.0], [pt, actual_h, d], config.wood_color);
    append_colored_box(&mut mesh, [0.0, actual_h - pt/2.0, 0.0], [w - 2.0*pt, pt, d], config.wood_color);
    append_colored_box(&mut mesh, [0.0, actual_h/2.0, -hd + pt/2.0], [w - 2.0*pt, actual_h, pt], config.wood_color);

    let base_h = actual_h * 0.35;
    append_colored_box(&mut mesh, [0.0, base_h/2.0, shelf_z], [w - 2.0*pt, base_h, shelf_d], config.cabinet_color);

    let door_w = (w - 2.0*pt) / 2.0 - 0.02;
    let door_h = base_h - 0.04;
    let door_z = hd - 0.01;
    append_colored_box(&mut mesh, [-door_w/2.0 - 0.01, base_h/2.0, door_z], [door_w, door_h, 0.02], config.wood_color);
    append_colored_box(&mut mesh, [door_w/2.0 + 0.01, base_h/2.0, door_z], [door_w, door_h, 0.02], config.wood_color);
    
    let knob_color = [0.2, 0.2, 0.2, 1.0];
    append_colored_box(&mut mesh, [-0.05, base_h/2.0 + 0.05, door_z + 0.015], [0.02, 0.06, 0.02], knob_color);
    append_colored_box(&mut mesh, [0.05, base_h/2.0 + 0.05, door_z + 0.015], [0.02, 0.06, 0.02], knob_color);

    let usable_h = actual_h - base_h - pt;
    let spacing = usable_h / rows as f32;
    
    for i in 1..rows {
        let sy = base_h + i as f32 * spacing;
        append_colored_box(&mut mesh, [0.0, sy + pt/2.0, shelf_z], [w - 2.0*pt, pt, shelf_d], config.wood_color);
    }

    let mut book_idx = 0;
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
}
"""
code = re.sub(r'fn generate_shelf_mesh.*?mesh\n}', shelf_code, code, flags=re.DOTALL)
code = code.replace("generate_box_mesh(w, h, d, [0.5, 0.35, 0.2])", "generate_shelf_mesh(w, h, d, &ShelfConfig::default())")

with open('crates/building-gen/src/furniture/catalog.rs', 'w') as f:
    f.write(code)

