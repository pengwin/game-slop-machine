use super::placement::{self, OccupiedTiles};
use super::{FurnitureItem, FurnitureType};
use crate::config::BuildingConfig;
use crate::geometry::Vec3;
use crate::layout::Room;
use crate::mesh::math_util::{append_colored_quad, append_colored_triangle, Quad};
use crate::mesh::MeshData;
use crate::tile::TileGrid;

/// Generates furniture items for a room based on its label.
pub fn furniture_for_room(
    room: &Room,
    grid: &TileGrid,
    config: &BuildingConfig,
    floor_y: f32,
    occupied: &mut OccupiedTiles,
) -> Vec<FurnitureItem> {
    match room.label.as_str() {
        "kitchen" => kitchen_items(room, grid, config, floor_y, occupied),
        "bedroom" => bedroom_items(room, grid, config, floor_y, occupied),
        "hall" | "foyer" | "entry" => hall_items(room, grid, config, floor_y, occupied),
        "storage" | "closet" | "pantry" => storage_items(room, grid, config, floor_y, occupied),
        _ => generic_items(room, grid, config, floor_y, occupied),
    }
}

fn kitchen_items(
    room: &Room,
    grid: &TileGrid,
    _config: &BuildingConfig,
    floor_y: f32,
    occupied: &mut OccupiedTiles,
) -> Vec<FurnitureItem> {
    let mut items = Vec::new();
    let wall_positions = placement::find_wall_positions(room.bounds, grid, occupied);

    // Stove (wall-attached)
    if !wall_positions.is_empty() {
        let (tx, ty, rot) = wall_positions[0];
        let (wx, wz) = placement::tile_to_world(tx, ty, grid);
        let w = 0.6;
        let h = 0.85;
        let d = 0.6;
        items.push(FurnitureItem {
            position: Vec3::new(wx, floor_y, wz),
            rotation: rot,
            item_type: FurnitureType::Stove,
            width: w,
            height: h,
            depth: d,
            color: [0.25, 0.25, 0.25],
            mesh: generate_box_mesh(w, h, d, [0.25, 0.25, 0.25]),
        });
        occupied.mark(tx, ty);
    }

    // Counter (wall-attached)
    if wall_positions.len() >= 2 {
        let (tx, ty, rot) = wall_positions[1];
        let (wx, wz) = placement::tile_to_world(tx, ty, grid);
        let w = 0.9;
        let h = 0.9;
        let d = 0.5;
        items.push(FurnitureItem {
            position: Vec3::new(wx, floor_y, wz),
            rotation: rot,
            item_type: FurnitureType::Counter,
            width: w,
            height: h,
            depth: d,
            color: [0.55, 0.4, 0.25],
            mesh: generate_counter_mesh(w, h, d, [0.55, 0.4, 0.25]),
        });
        occupied.mark(tx, ty);
    }

    // Table (center)
    let center_positions = placement::find_center_positions(room.bounds, grid, occupied, 1);
    if !center_positions.is_empty() {
        let (tx, ty) = center_positions[0];
        let (wx, wz) = placement::tile_to_world(tx, ty, grid);
        let w = 0.7;
        let h = 0.75;
        let d = 0.5;
        items.push(FurnitureItem {
            position: Vec3::new(wx, floor_y, wz),
            rotation: 0.0,
            item_type: FurnitureType::Table,
            width: w,
            height: h,
            depth: d,
            color: [0.6, 0.45, 0.25],
            mesh: generate_table_mesh(w, h, d, [0.6, 0.45, 0.25]),
        });
        occupied.mark(tx, ty);

        // Chairs around table
        let chair_offsets = [(0, -1), (0, 1), (-1, 0), (1, 0)];
        let mut chairs_placed = 0;
        for (dx, dy) in chair_offsets {
            if chairs_placed >= 2 {
                break;
            }
            let cx = tx.wrapping_add_signed(dx);
            let cy = ty.wrapping_add_signed(dy);
            if cx < grid.width
                && cy < grid.height
                && grid.get(cx, cy) == crate::tile::TileType::Floor
                && !occupied.is_occupied(cx, cy)
            {
                let (cwx, cwz) = placement::tile_to_world(cx, cy, grid);
                let rot = if dx == 0 && dy < 0 {
                    std::f32::consts::PI
                } else if dx == 0 && dy > 0 {
                    0.0
                } else if dx < 0 {
                    std::f32::consts::FRAC_PI_2
                } else {
                    -std::f32::consts::FRAC_PI_2
                };
                items.push(FurnitureItem {
                    position: Vec3::new(cwx, floor_y, cwz),
                    rotation: rot,
                    item_type: FurnitureType::Chair,
                    width: 0.35,
                    height: 0.45,
                    depth: 0.35,
                    color: [0.5, 0.35, 0.2],
                    mesh: generate_chair_mesh(0.35, 0.45, 0.35, [0.5, 0.35, 0.2]),
                });
                occupied.mark(cx, cy);
                chairs_placed += 1;
            }
        }
    }

    items
}

fn bedroom_items(
    room: &Room,
    grid: &TileGrid,
    _config: &BuildingConfig,
    floor_y: f32,
    occupied: &mut OccupiedTiles,
) -> Vec<FurnitureItem> {
    let mut items = Vec::new();
    let wall_positions = placement::find_wall_positions(room.bounds, grid, occupied);

    // Bed (wall-attached, takes 2 tiles)
    if !wall_positions.is_empty() {
        let (tx, ty, rot) = wall_positions[0];
        let (wx, wz) = placement::tile_to_world(tx, ty, grid);
        let w = 1.0;
        let h = 0.45;
        let d = 0.9;
        items.push(FurnitureItem {
            position: Vec3::new(wx, floor_y, wz),
            rotation: rot,
            item_type: FurnitureType::Bed,
            width: w,
            height: h,
            depth: d,
            color: [0.9, 0.9, 0.85],
            mesh: generate_bed_mesh(w, h, d, &BedConfig::default()),
        });
        occupied.mark(tx, ty);
    }

    // Desk (wall-attached)
    if wall_positions.len() >= 2 {
        let (tx, ty, rot) = wall_positions[1];
        let (wx, wz) = placement::tile_to_world(tx, ty, grid);
        let w = 0.7;
        let h = 0.75;
        let d = 0.45;
        items.push(FurnitureItem {
            position: Vec3::new(wx, floor_y, wz),
            rotation: rot,
            item_type: FurnitureType::Desk,
            width: w,
            height: h,
            depth: d,
            color: [0.5, 0.35, 0.2],
            mesh: generate_desk_mesh(w, h, d, [0.5, 0.35, 0.2]),
        });
        occupied.mark(tx, ty);
    }

    // Chair (center, near desk)
    let center_positions = placement::find_center_positions(room.bounds, grid, occupied, 1);
    if !center_positions.is_empty() {
        let (tx, ty) = center_positions[0];
        let (wx, wz) = placement::tile_to_world(tx, ty, grid);
        items.push(FurnitureItem {
            position: Vec3::new(wx, floor_y, wz),
            rotation: 0.0,
            item_type: FurnitureType::Chair,
            width: 0.35,
            height: 0.45,
            depth: 0.35,
            color: [0.5, 0.35, 0.2],
            mesh: generate_chair_mesh(0.35, 0.45, 0.35, [0.5, 0.35, 0.2]),
        });
        occupied.mark(tx, ty);
    }

    items
}

fn hall_items(
    room: &Room,
    grid: &TileGrid,
    _config: &BuildingConfig,
    floor_y: f32,
    occupied: &mut OccupiedTiles,
) -> Vec<FurnitureItem> {
    let mut items = Vec::new();
    let wall_positions = placement::find_wall_positions(room.bounds, grid, occupied);

    // Bench (wall-attached)
    if let Some(&(tx, ty, rot)) = wall_positions.first() {
        let (wx, wz) = placement::tile_to_world(tx, ty, grid);
        let w = 0.8;
        let h = 0.45;
        let d = 0.35;
        items.push(FurnitureItem {
            position: Vec3::new(wx, floor_y, wz),
            rotation: rot,
            item_type: FurnitureType::Bench,
            width: w,
            height: h,
            depth: d,
            color: [0.45, 0.32, 0.18],
            mesh: generate_bench_mesh(w, h, d, [0.45, 0.32, 0.18]),
        });
        occupied.mark(tx, ty);
    }

    items
}

fn storage_items(
    room: &Room,
    grid: &TileGrid,
    _config: &BuildingConfig,
    floor_y: f32,
    occupied: &mut OccupiedTiles,
) -> Vec<FurnitureItem> {
    let mut items = Vec::new();
    let center_positions = placement::find_center_positions(room.bounds, grid, occupied, 0);

    // Barrels and crates
    for (i, &(tx, ty)) in center_positions.iter().enumerate().take(4) {
        let (wx, wz) = placement::tile_to_world(tx, ty, grid);
        if i % 2 == 0 {
            items.push(FurnitureItem {
                position: Vec3::new(wx, floor_y, wz),
                rotation: 0.0,
                item_type: FurnitureType::Barrel,
                width: 0.4,
                height: 0.6,
                depth: 0.4,
                color: [0.4, 0.28, 0.15],
                mesh: generate_barrel_mesh(0.4, 0.6, &BarrelConfig::default()),
            });
        } else {
            items.push(FurnitureItem {
                position: Vec3::new(wx, floor_y, wz),
                rotation: 0.0,
                item_type: FurnitureType::Crate,
                width: 0.5,
                height: 0.5,
                depth: 0.5,
                color: [0.65, 0.55, 0.35],
                mesh: generate_crate_mesh(0.5, 0.5, 0.5, [0.65, 0.55, 0.35]),
            });
        }
        occupied.mark(tx, ty);
    }

    items
}

fn generic_items(
    room: &Room,
    grid: &TileGrid,
    _config: &BuildingConfig,
    floor_y: f32,
    occupied: &mut OccupiedTiles,
) -> Vec<FurnitureItem> {
    let mut items = Vec::new();
    let center_positions = placement::find_center_positions(room.bounds, grid, occupied, 1);

    // Table
    if !center_positions.is_empty() {
        let (tx, ty) = center_positions[0];
        let (wx, wz) = placement::tile_to_world(tx, ty, grid);
        items.push(FurnitureItem {
            position: Vec3::new(wx, floor_y, wz),
            rotation: 0.0,
            item_type: FurnitureType::Table,
            width: 0.7,
            height: 0.75,
            depth: 0.5,
            color: [0.6, 0.45, 0.25],
            mesh: generate_table_mesh(0.7, 0.75, 0.5, [0.6, 0.45, 0.25]),
        });
        occupied.mark(tx, ty);
    }

    items
}

// ── Mesh Generators ──────────────────────────────────────────────────────


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

fn append_colored_beveled_box(mesh: &mut MeshData, center: [f32; 3], size: [f32; 3], bevel: f32, color: [f32; 4]) {
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
fn generate_box_mesh(w: f32, h: f32, d: f32, color: [f32; 3]) -> MeshData {
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

/// Table: 4 legs + top surface.
fn generate_table_mesh(w: f32, top_y: f32, d: f32, color: [f32; 3]) -> MeshData {
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

/// Chair: seat + back + 4 legs.
fn generate_chair_mesh(w: f32, seat_h: f32, d: f32, color: [f32; 3]) -> MeshData {
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

/// Bed: frame + mattress + headboard.
#[derive(Debug, Clone)]
pub struct BedConfig {
    pub num_pillows: u32,
    pub pillow_size: [f32; 3], // width, height, depth
    pub headboard_height: f32, // absolute height or factor. Let's use relative to 'h'
    pub footboard_height: f32,
    pub frame_height: f32, // height of the side rails
    pub wood_color: [f32; 4],
    pub sheet_color: [f32; 4],
    pub blanket_color: [f32; 4],
}

impl Default for BedConfig {
    fn default() -> Self {
        Self {
            num_pillows: 1,
            pillow_size: [0.4, 0.08, 0.25], // w, h, d
            headboard_height: 1.0, // relative to h
            footboard_height: 0.7, // relative to h
            frame_height: 0.15, // absolute height
            wood_color: [0.5, 0.3, 0.15, 1.0], // More brown
            sheet_color: [0.95, 0.95, 0.95, 1.0],
            blanket_color: [0.65, 0.35, 0.25, 1.0], // Terracotta
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


/// Counter: open-top box.
fn generate_counter_mesh(w: f32, h: f32, d: f32, color: [f32; 3]) -> MeshData {
    generate_box_mesh(w, h, d, color)
}

/// Desk: top + 2 side panels + bottom shelf.
fn generate_desk_mesh(w: f32, h: f32, d: f32, color: [f32; 3]) -> MeshData {
    let mut mesh = MeshData::default();
    let top_color = [color[0], color[1], color[2], 1.0];
    let panel_color = [color[0] * 0.7, color[1] * 0.7, color[2] * 0.7, 1.0];
    let pt = 0.03;
    append_colored_box(&mut mesh, [0.0, h - pt/2.0, 0.0], [w, pt, d], top_color);
    append_colored_box(&mut mesh, [-w/2.0 + pt/2.0, (h-pt)/2.0, 0.0], [pt, h-pt, d], panel_color);
    append_colored_box(&mut mesh, [w/2.0 - pt/2.0, (h-pt)/2.0, 0.0], [pt, h-pt, d], panel_color);
    append_colored_box(&mut mesh, [0.0, h*0.3, 0.0], [w - 2.0*pt, pt, d], top_color);
    mesh
}

#[derive(Debug, Clone)]
pub struct BarrelConfig {
    pub max_radius_factor: f32,
    pub cap_radius_factor: f32,
    pub wood_color: [f32; 4],
    pub metal_color: [f32; 4],
    pub cap_color: [f32; 4],
}

impl Default for BarrelConfig {
    fn default() -> Self {
        Self {
            max_radius_factor: 1.25,
            cap_radius_factor: 1.0,
            wood_color: [0.4, 0.28, 0.15, 1.0],
            metal_color: [0.2, 0.2, 0.2, 1.0],
            cap_color: [0.25, 0.15, 0.05, 1.0],
        }
    }
}

/// Barrel: segmented profile to allow for metal rings and smooth bulges.
fn generate_barrel_mesh(diameter: f32, h: f32, config: &BarrelConfig) -> MeshData {
    let mut mesh = MeshData::default();
    let r = diameter / 2.0;
    let sides = 16;
    let mid_r = r * config.max_radius_factor;
    let cap_r = r * config.cap_radius_factor;
    let rim_h = h * 0.08;

    let ring_h = 0.04;
    let ring_extrusion = 0.015;
    let recess_depth = 0.04;
    let recess_r = cap_r - 0.03;

    let wood_color = config.wood_color;
    let metal_color = config.metal_color;
    let cap_color = config.cap_color;

    let get_r = |y: f32| -> f32 {
        let half_body = (h / 2.0) - rim_h;
        let cy = y - h / 2.0;
        let t = cy / half_body;
        r + (mid_r - r) * (1.0 - t * t)
    };

    let mut profile = Vec::new();
    
    // Bottom recess and outer rim
    profile.push((recess_depth, recess_r, wood_color));
    profile.push((0.0, recess_r, wood_color));
    profile.push((0.0, cap_r, wood_color));
    profile.push((rim_h, r, wood_color));
    
    // Ring 1
    let y_r1 = h * 0.25;
    profile.push((y_r1 - ring_h / 2.0, get_r(y_r1 - ring_h / 2.0), wood_color));
    profile.push((y_r1 - ring_h / 2.0, get_r(y_r1) + ring_extrusion, metal_color));
    profile.push((y_r1 + ring_h / 2.0, get_r(y_r1) + ring_extrusion, metal_color));
    profile.push((y_r1 + ring_h / 2.0, get_r(y_r1 + ring_h / 2.0), wood_color));
    
    // Ring 2
    let y_r2 = h * 0.5;
    profile.push((y_r2 - ring_h / 2.0, get_r(y_r2 - ring_h / 2.0), wood_color));
    profile.push((y_r2 - ring_h / 2.0, get_r(y_r2) + ring_extrusion, metal_color));
    profile.push((y_r2 + ring_h / 2.0, get_r(y_r2) + ring_extrusion, metal_color));
    profile.push((y_r2 + ring_h / 2.0, get_r(y_r2 + ring_h / 2.0), wood_color));

    // Ring 3
    let y_r3 = h * 0.75;
    profile.push((y_r3 - ring_h / 2.0, get_r(y_r3 - ring_h / 2.0), wood_color));
    profile.push((y_r3 - ring_h / 2.0, get_r(y_r3) + ring_extrusion, metal_color));
    profile.push((y_r3 + ring_h / 2.0, get_r(y_r3) + ring_extrusion, metal_color));
    profile.push((y_r3 + ring_h / 2.0, get_r(y_r3 + ring_h / 2.0), wood_color));

    // Top rim and recess
    profile.push((h - rim_h, r, wood_color));
    profile.push((h, cap_r, wood_color));
    profile.push((h, recess_r, wood_color));
    profile.push((h - recess_depth, recess_r, wood_color));

    for i in 0..sides {
        let angle0 = std::f32::consts::TAU * i as f32 / sides as f32;
        let angle1 = std::f32::consts::TAU * (i + 1) as f32 / sides as f32;

        let nx = ((angle0 + angle1) / 2.0).cos();
        let nz = ((angle0 + angle1) / 2.0).sin();

        for p in profile.windows(2) {
            let (y0, r0, _) = p[0];
            let (y1, r1, color1) = p[1];
            
            if (y1 - y0).abs() < 1e-5 && (r1 - r0).abs() < 1e-5 {
                continue; // Skip zero-length segments
            }
            
            let dy = y1 - y0;
            let dr = r1 - r0;
            
            let mut n_r = dy;
            let mut n_y = -dr;
            let len = (n_r * n_r + n_y * n_y).sqrt();
            if len > 0.0 {
                n_r /= len;
                n_y /= len;
            } else {
                n_r = 1.0;
                n_y = 0.0;
            }
            
            let norm = [nx * n_r, n_y, nz * n_r];

            let x0_bottom = angle0.cos() * r0;
            let z0_bottom = angle0.sin() * r0;
            let x1_bottom = angle1.cos() * r0;
            let z1_bottom = angle1.sin() * r0;

            let x0_top = angle0.cos() * r1;
            let z0_top = angle0.sin() * r1;
            let x1_top = angle1.cos() * r1;
            let z1_top = angle1.sin() * r1;

            append_colored_quad(&mut mesh, Quad {
                tl: [x0_top, y1, z0_top], tr: [x1_top, y1, z1_top],
                bl: [x0_bottom, y0, z0_bottom], br: [x1_bottom, y0, z1_bottom],
                normal: norm, uv_min: [0.0, 0.0], uv_max: [1.0, 1.0],
            }, color1);
        }

        let tx0 = angle0.cos() * recess_r;
        let tz0 = angle0.sin() * recess_r;
        let tx1 = angle1.cos() * recess_r;
        let tz1 = angle1.sin() * recess_r;
        
        append_colored_triangle(
            &mut mesh,
            [0.0, h - recess_depth, 0.0],
            [tx1, h - recess_depth, tz1],
            [tx0, h - recess_depth, tz0],
            [0.0, 1.0, 0.0],
            cap_color,
        );
        append_colored_triangle(
            &mut mesh,
            [0.0, recess_depth, 0.0],
            [tx0, recess_depth, tz0],
            [tx1, recess_depth, tz1],
            [0.0, -1.0, 0.0],
            cap_color,
        );
    }

    mesh
}



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


/// Crate: box with no bottom.
fn generate_crate_mesh(w: f32, h: f32, d: f32, color: [f32; 3]) -> MeshData {
    let mut mesh = MeshData::default();
    let wood_color = [color[0], color[1], color[2], 1.0];
    let metal_color = [0.2, 0.2, 0.2, 1.0];
    
    append_colored_box(&mut mesh, [0.0, h/2.0, 0.0], [w, h, d], wood_color);
    // Simple metal straps
    let t = 0.02;
    append_colored_box(&mut mesh, [0.0, h/2.0, d/2.0], [w, t, t], metal_color);
    append_colored_box(&mut mesh, [0.0, h/2.0, -d/2.0], [w, t, t], metal_color);
    mesh
}

/// Bench: flat seat + 2 side supports.
fn generate_bench_mesh(w: f32, h: f32, d: f32, color: [f32; 3]) -> MeshData {
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
}

/// Generates a single furniture item by type with default dimensions.
/// Useful for previewing individual items.
pub fn single_item(item_type: FurnitureType) -> FurnitureItem {
    use crate::geometry::Vec3;

    let (w, h, d, color, mesh) = match item_type {
        FurnitureType::Table => {
            let (w, h, d) = (0.8, 0.75, 0.5);
            (w, h, d, [0.6, 0.45, 0.25], generate_table_mesh(w, h, d, [0.6, 0.45, 0.25]))
        }
        FurnitureType::Chair => {
            let (w, h, d) = (0.4, 0.45, 0.4);
            (w, h, d, [0.5, 0.35, 0.2], generate_chair_mesh(w, h, d, [0.5, 0.35, 0.2]))
        }
        FurnitureType::Bed => {
            let (w, h, d) = (1.0, 0.45, 0.9);
            (w, h, d, [0.9, 0.9, 0.85], generate_bed_mesh(w, h, d, &BedConfig::default()))
        }
        FurnitureType::Stove => {
            let (w, h, d) = (0.6, 0.85, 0.6);
            (w, h, d, [0.25, 0.25, 0.25], generate_box_mesh(w, h, d, [0.25, 0.25, 0.25]))
        }
        FurnitureType::Counter => {
            let (w, h, d) = (0.9, 0.9, 0.5);
            (w, h, d, [0.55, 0.4, 0.25], generate_counter_mesh(w, h, d, [0.55, 0.4, 0.25]))
        }
        FurnitureType::Desk => {
            let (w, h, d) = (0.7, 0.75, 0.45);
            (w, h, d, [0.5, 0.35, 0.2], generate_desk_mesh(w, h, d, [0.5, 0.35, 0.2]))
        }
        FurnitureType::Barrel => {
            let (d, h) = (0.4, 0.6);
            (d, h, d, [0.4, 0.28, 0.15], generate_barrel_mesh(d, h, &BarrelConfig::default()))
        }
        FurnitureType::Crate => {
            let (w, h, d) = (0.5, 0.5, 0.5);
            (w, h, d, [0.65, 0.55, 0.35], generate_crate_mesh(w, h, d, [0.65, 0.55, 0.35]))
        }
        FurnitureType::Bench => {
            let (w, h, d) = (0.8, 0.45, 0.35);
            (w, h, d, [0.45, 0.32, 0.18], generate_bench_mesh(w, h, d, [0.45, 0.32, 0.18]))
        }
        FurnitureType::Shelf => {
            let (w, h, d) = (0.6, 1.2, 0.3);
            (w, h, d, [0.5, 0.35, 0.2], generate_shelf_mesh(w, h, d, &ShelfConfig::default()))
        }
    };

    FurnitureItem {
        position: Vec3::ZERO,
        rotation: 0.0,
        item_type,
        width: w,
        height: h,
        depth: d,
        color,
        mesh,
    }
}
