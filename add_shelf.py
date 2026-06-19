import re

with open('crates/building-gen/src/furniture/catalog.rs', 'r') as f:
    code = f.read()

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
    let pt = 0.04; // panel thickness
    
    // Config values
    let mut actual_h = config.height;
    if actual_h <= 0.0 { actual_h = h; } // fallback
    let rows = config.rows.max(1);

    // Frame
    let back_d = pt;
    let shelf_d = d - back_d;
    let shelf_z = -hd + shelf_d / 2.0;
    
    // Left panel
    append_colored_box(&mut mesh, [-hw + pt/2.0, actual_h/2.0, 0.0], [pt, actual_h, d], config.wood_color);
    // Right panel
    append_colored_box(&mut mesh, [hw - pt/2.0, actual_h/2.0, 0.0], [pt, actual_h, d], config.wood_color);
    // Top panel
    append_colored_box(&mut mesh, [0.0, actual_h - pt/2.0, 0.0], [w - 2.0*pt, pt, d], config.wood_color);
    // Back panel
    append_colored_box(&mut mesh, [0.0, actual_h/2.0, hd - pt/2.0], [w - 2.0*pt, actual_h, pt], config.wood_color);

    // Cabinet base
    let base_h = actual_h * 0.35;
    append_colored_box(&mut mesh, [0.0, base_h/2.0, shelf_z], [w - 2.0*pt, base_h, shelf_d], config.cabinet_color);

    // Cabinet doors detailing
    let door_w = (w - 2.0*pt) / 2.0 - 0.02;
    let door_h = base_h - 0.04;
    let door_z = -hd + 0.01;
    // Left door
    append_colored_box(&mut mesh, [-door_w/2.0 - 0.01, base_h/2.0, door_z], [door_w, door_h, 0.02], config.wood_color);
    // Right door
    append_colored_box(&mut mesh, [door_w/2.0 + 0.01, base_h/2.0, door_z], [door_w, door_h, 0.02], config.wood_color);
    
    // Knobs
    let knob_color = [0.2, 0.2, 0.2, 1.0];
    append_colored_box(&mut mesh, [-0.05, base_h/2.0 + 0.05, door_z - 0.015], [0.02, 0.06, 0.02], knob_color);
    append_colored_box(&mut mesh, [0.05, base_h/2.0 + 0.05, door_z - 0.015], [0.02, 0.06, 0.02], knob_color);

    // Shelves
    let usable_h = actual_h - base_h - pt;
    let spacing = usable_h / rows as f32;
    
    for i in 1..rows {
        let sy = base_h + i as f32 * spacing;
        append_colored_box(&mut mesh, [0.0, sy + pt/2.0, shelf_z], [w - 2.0*pt, pt, shelf_d], config.wood_color);
    }

    // Books
    let mut book_idx = 0;
    let total_books = config.books.len();
    
    if total_books > 0 {
        let books_per_row = (total_books as f32 / rows as f32).ceil() as usize;
        
        for r in 0..rows {
            let sy = base_h + r as f32 * spacing + pt;
            let start_x = -hw + pt + 0.05;
            let book_w = 0.04;
            let book_h = spacing * 0.6;
            let book_d = shelf_d * 0.7;
            
            for i in 0..books_per_row {
                if book_idx < total_books {
                    let bx = start_x + (i as f32) * (book_w + 0.01) + book_w/2.0;
                    // Slightly tilt the first book of the row to look natural
                    if i == 0 {
                        // Creating a leaning book by drawing a rotated box manually
                        let mut bmesh = MeshData::default();
                        append_colored_box(&mut bmesh, [0.0, 0.0, 0.0], [book_w, book_h, book_d], config.books[book_idx]);
                        let angle = -0.2_f32; // lean left
                        let cos_a = angle.cos();
                        let sin_a = angle.sin();
                        for v in &mut bmesh.vertices {
                            let nx = v[0] * cos_a - v[1] * sin_a;
                            let ny = v[0] * sin_a + v[1] * cos_a;
                            v[0] = nx + bx + 0.02; // shift right a bit
                            v[1] = ny + sy + book_h/2.0;
                            v[2] += shelf_z;
                        }
                        for n in &mut bmesh.normals {
                            let nx = n[0] * cos_a - n[1] * sin_a;
                            let ny = n[0] * sin_a + n[1] * cos_a;
                            n[0] = nx;
                            n[1] = ny;
                        }
                        let base_idx = mesh.vertices.len() as u32;
                        mesh.vertices.extend(bmesh.vertices);
                        mesh.normals.extend(bmesh.normals);
                        mesh.uvs.extend(bmesh.uvs);
                        mesh.colors.extend(bmesh.colors);
                        mesh.indices.extend(bmesh.indices.iter().map(|&idx| idx + base_idx));
                    } else {
                        // Standing book
                        append_colored_box(&mut mesh, [bx, sy + book_h/2.0, shelf_z], [book_w, book_h, book_d], config.books[book_idx]);
                    }
                    book_idx += 1;
                }
            }
            
            // Add a vase on the second row if there are books on the first row
            if r == 1 {
                let vase_x = hw - pt - 0.15;
                let vase_size = 0.1;
                let vase_color = [0.8, 0.8, 0.8, 1.0];
                append_colored_beveled_box(&mut mesh, [vase_x, sy + vase_size/2.0, shelf_z], [vase_size, vase_size, vase_size], 0.03, vase_color);
            }
        }
    }

    mesh
}
"""

# Replace `FurnitureType::Shelf` generation block
code = code.replace(
"""        FurnitureType::Shelf => {
            let (w, h, d) = (0.6, 1.2, 0.3);
            (w, h, d, [0.5, 0.35, 0.2], generate_box_mesh(w, h, d, [0.5, 0.35, 0.2]))
        }""",
"""        FurnitureType::Shelf => {
            let (w, h, d) = (0.6, 1.2, 0.3);
            (w, h, d, [0.5, 0.35, 0.2], generate_shelf_mesh(w, h, d, &ShelfConfig::default()))
        }""")

# Find a good place to insert shelf_code (e.g. before generate_crate_mesh)
idx = code.find("/// Crate: box with no bottom.")
code = code[:idx] + shelf_code + "\n" + code[idx:]

with open('crates/building-gen/src/furniture/catalog.rs', 'w') as f:
    f.write(code)
