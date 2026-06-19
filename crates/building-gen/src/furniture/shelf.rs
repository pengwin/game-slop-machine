use crate::mesh::MeshData;
use crate::mesh::colored_shapes::{append_colored_box, append_colored_beveled_box, append_colored_rotated_box};

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

pub fn generate_shelf_mesh(w: f32, h: f32, d: f32, config: &ShelfConfig) -> MeshData {
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
                    
                    let rot_y = if rot_z != 0.0 {
                        0.0
                    } else {
                        (i as f32 * 0.15) - 0.1
                    };
                    
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
