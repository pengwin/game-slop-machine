import re

with open('crates/building-gen/src/furniture/catalog.rs', 'r') as f:
    code = f.read()

# Add append_colored_rotated_box helper
rotated_box_code = """
fn append_colored_rotated_box(mesh: &mut MeshData, center: [f32; 3], size: [f32; 3], rot_y: f32, rot_z: f32, color: [f32; 4]) {
    let mut bmesh = MeshData::default();
    append_colored_box(&mut bmesh, [0.0, 0.0, 0.0], size, color);
    
    let cy = rot_y.cos();
    let sy = rot_y.sin();
    let cz = rot_z.cos();
    let sz = rot_z.sin();
    
    // Z rotation shifts bottom, so let's set pivot to bottom-center
    // Before rotation, box is from -h/2 to h/2. We want to pivot around bottom.
    let pivot_y = -size[1] / 2.0;

    for v in &mut bmesh.vertices {
        // Shift to pivot
        let mut x = v[0];
        let mut y = v[1] - pivot_y;
        let mut z = v[2];

        // Z rotation (lean)
        let x1 = x * cz - y * sz;
        let y1 = x * sz + y * cz;
        let z1 = z;

        // Y rotation (twist on shelf)
        let x2 = x1 * cy + z1 * sy;
        let y2 = y1;
        let z2 = -x1 * sy + z1 * cy;

        // Shift back from pivot and to center
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

# Now fix the books placement
old_books_code_start = "let mut book_idx = 0;"
old_books_code_end = "    mesh\n}"

new_books_code = """let mut book_idx = 0;
    let total_books = config.books.len();
    
    if total_books > 0 {
        let books_per_row = (total_books as f32 / rows as f32).ceil() as usize;
        
        for r in 0..rows {
            // Fix floating books: cabinet base is at base_h, upper shelves are at base_h + r * spacing + pt
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
                    
                    // Lean the first two books (Z rotation)
                    let rot_z = if i == 0 {
                        -0.5_f32 // approx 30 degrees lean
                    } else if i == 1 {
                        -0.25_f32 // slight lean to rest on the first book
                    } else {
                        0.0
                    };
                    
                    // Slightly rotate all books relative to the shelf (Y rotation)
                    let rot_y = (i as f32 * 0.15) - 0.1; // roughly 10-20 degrees
                    
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

idx_start = code.find(old_books_code_start)
idx_end = code.find(old_books_code_end) + len(old_books_code_end)

code = code[:idx_start] + new_books_code + code[idx_end:]

with open('crates/building-gen/src/furniture/catalog.rs', 'w') as f:
    f.write(code)
