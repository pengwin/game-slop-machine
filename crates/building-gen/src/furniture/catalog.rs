use super::placement::{self, OccupiedTiles};
use super::{FurnitureItem, FurnitureType};
use crate::config::BuildingConfig;
use crate::geometry::Vec3;
use crate::layout::Room;
use crate::mesh::math_util::{append_quad, Quad};
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
            mesh: generate_bed_mesh(w, h, d, [0.9, 0.9, 0.85]),
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
                mesh: generate_barrel_mesh(0.4, 0.6, [0.4, 0.28, 0.15]),
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

/// Generates a box with top and 4 sides (no bottom).
fn generate_box_mesh(w: f32, h: f32, d: f32, color: [f32; 3]) -> MeshData {
    let mut mesh = MeshData::default();
    let hw = w / 2.0;
    let hd = d / 2.0;
    let c = color;

    // Top
    append_quad(&mut mesh, Quad {
        tl: [-hw, h, hd], tr: [hw, h, hd], bl: [-hw, h, -hd], br: [hw, h, -hd],
        normal: [0.0, 1.0, 0.0], uv_min: [0.0, 0.0], uv_max: [c[0], c[1]],
    });
    // Front (-Z)
    append_quad(&mut mesh, Quad {
        tl: [-hw, h, -hd], tr: [hw, h, -hd], bl: [-hw, 0.0, -hd], br: [hw, 0.0, -hd],
        normal: [0.0, 0.0, -1.0], uv_min: [0.0, 0.0], uv_max: [c[0], c[2]],
    });
    // Back (+Z)
    append_quad(&mut mesh, Quad {
        tl: [hw, h, hd], tr: [-hw, h, hd], bl: [hw, 0.0, hd], br: [-hw, 0.0, hd],
        normal: [0.0, 0.0, 1.0], uv_min: [0.0, 0.0], uv_max: [c[0], c[2]],
    });
    // Left (-X)
    append_quad(&mut mesh, Quad {
        tl: [-hw, h, hd], tr: [-hw, h, -hd], bl: [-hw, 0.0, hd], br: [-hw, 0.0, -hd],
        normal: [-1.0, 0.0, 0.0], uv_min: [0.0, 0.0], uv_max: [c[1], c[2]],
    });
    // Right (+X)
    append_quad(&mut mesh, Quad {
        tl: [hw, h, -hd], tr: [hw, h, hd], bl: [hw, 0.0, -hd], br: [hw, 0.0, hd],
        normal: [1.0, 0.0, 0.0], uv_min: [0.0, 0.0], uv_max: [c[1], c[2]],
    });
    mesh
}

/// Table: 4 legs + top surface.
fn generate_table_mesh(w: f32, top_y: f32, d: f32, _color: [f32; 3]) -> MeshData {
    let mut mesh = MeshData::default();
    let hw = w / 2.0;
    let hd = d / 2.0;
    let leg_t = 0.04;
    let leg_h = top_y - 0.02;

    // Top surface
    append_quad(&mut mesh, Quad {
        tl: [-hw, top_y, hd], tr: [hw, top_y, hd], bl: [-hw, top_y, -hd], br: [hw, top_y, -hd],
        normal: [0.0, 1.0, 0.0], uv_min: [0.0, 0.0], uv_max: [1.0, 1.0],
    });
    // Top underside
    append_quad(&mut mesh, Quad {
        tl: [-hw, top_y - 0.02, -hd], tr: [hw, top_y - 0.02, -hd],
        bl: [-hw, top_y - 0.02, hd], br: [hw, top_y - 0.02, hd],
        normal: [0.0, -1.0, 0.0], uv_min: [0.0, 0.0], uv_max: [1.0, 1.0],
    });

    // 4 legs
    let leg_positions = [
        (-hw + leg_t, -hd + leg_t),
        (hw - leg_t, -hd + leg_t),
        (-hw + leg_t, hd - leg_t),
        (hw - leg_t, hd - leg_t),
    ];
    for (lx, lz) in leg_positions {
        append_quad(&mut mesh, Quad {
            tl: [lx - leg_t, leg_h, lz - leg_t], tr: [lx + leg_t, leg_h, lz - leg_t],
            bl: [lx - leg_t, 0.0, lz - leg_t], br: [lx + leg_t, 0.0, lz - leg_t],
            normal: [0.0, 0.0, -1.0], uv_min: [0.0, 0.0], uv_max: [1.0, 1.0],
        });
        append_quad(&mut mesh, Quad {
            tl: [lx + leg_t, leg_h, lz + leg_t], tr: [lx - leg_t, leg_h, lz + leg_t],
            bl: [lx + leg_t, 0.0, lz + leg_t], br: [lx - leg_t, 0.0, lz + leg_t],
            normal: [0.0, 0.0, 1.0], uv_min: [0.0, 0.0], uv_max: [1.0, 1.0],
        });
        append_quad(&mut mesh, Quad {
            tl: [lx - leg_t, leg_h, lz + leg_t], tr: [lx - leg_t, leg_h, lz - leg_t],
            bl: [lx - leg_t, 0.0, lz + leg_t], br: [lx - leg_t, 0.0, lz - leg_t],
            normal: [-1.0, 0.0, 0.0], uv_min: [0.0, 0.0], uv_max: [1.0, 1.0],
        });
        append_quad(&mut mesh, Quad {
            tl: [lx + leg_t, leg_h, lz - leg_t], tr: [lx + leg_t, leg_h, lz + leg_t],
            bl: [lx + leg_t, 0.0, lz - leg_t], br: [lx + leg_t, 0.0, lz + leg_t],
            normal: [1.0, 0.0, 0.0], uv_min: [0.0, 0.0], uv_max: [1.0, 1.0],
        });
    }

    mesh
}

/// Chair: seat + back + 4 legs.
fn generate_chair_mesh(w: f32, seat_h: f32, d: f32, _color: [f32; 3]) -> MeshData {
    let mut mesh = MeshData::default();
    let hw = w / 2.0;
    let hd = d / 2.0;
    let leg_t = 0.03;
    let seat_t = 0.03;
    let back_h = 0.4;

    // Seat surface
    append_quad(&mut mesh, Quad {
        tl: [-hw, seat_h, hd], tr: [hw, seat_h, hd], bl: [-hw, seat_h, -hd], br: [hw, seat_h, -hd],
        normal: [0.0, 1.0, 0.0], uv_min: [0.0, 0.0], uv_max: [1.0, 1.0],
    });

    // Back rest (vertical panel at back edge)
    append_quad(&mut mesh, Quad {
        tl: [-hw, seat_h + back_h, hd], tr: [hw, seat_h + back_h, hd],
        bl: [-hw, seat_h, hd], br: [hw, seat_h, hd],
        normal: [0.0, 0.0, 1.0], uv_min: [0.0, 0.0], uv_max: [1.0, 1.0],
    });
    // Back rest front face
    append_quad(&mut mesh, Quad {
        tl: [hw, seat_h + back_h, hd - seat_t], tr: [-hw, seat_h + back_h, hd - seat_t],
        bl: [hw, seat_h, hd - seat_t], br: [-hw, seat_h, hd - seat_t],
        normal: [0.0, 0.0, -1.0], uv_min: [0.0, 0.0], uv_max: [1.0, 1.0],
    });

    // 4 legs
    let leg_positions = [
        (-hw + leg_t, -hd + leg_t),
        (hw - leg_t, -hd + leg_t),
        (-hw + leg_t, hd - leg_t),
        (hw - leg_t, hd - leg_t),
    ];
    for (lx, lz) in leg_positions {
        append_quad(&mut mesh, Quad {
            tl: [lx - leg_t, seat_h, lz - leg_t], tr: [lx + leg_t, seat_h, lz - leg_t],
            bl: [lx - leg_t, 0.0, lz - leg_t], br: [lx + leg_t, 0.0, lz - leg_t],
            normal: [0.0, 0.0, -1.0], uv_min: [0.0, 0.0], uv_max: [1.0, 1.0],
        });
        append_quad(&mut mesh, Quad {
            tl: [lx + leg_t, seat_h, lz + leg_t], tr: [lx - leg_t, seat_h, lz + leg_t],
            bl: [lx + leg_t, 0.0, lz + leg_t], br: [lx - leg_t, 0.0, lz + leg_t],
            normal: [0.0, 0.0, 1.0], uv_min: [0.0, 0.0], uv_max: [1.0, 1.0],
        });
        append_quad(&mut mesh, Quad {
            tl: [lx - leg_t, seat_h, lz + leg_t], tr: [lx - leg_t, seat_h, lz - leg_t],
            bl: [lx - leg_t, 0.0, lz + leg_t], br: [lx - leg_t, 0.0, lz - leg_t],
            normal: [-1.0, 0.0, 0.0], uv_min: [0.0, 0.0], uv_max: [1.0, 1.0],
        });
        append_quad(&mut mesh, Quad {
            tl: [lx + leg_t, seat_h, lz - leg_t], tr: [lx + leg_t, seat_h, lz + leg_t],
            bl: [lx + leg_t, 0.0, lz - leg_t], br: [lx + leg_t, 0.0, lz + leg_t],
            normal: [1.0, 0.0, 0.0], uv_min: [0.0, 0.0], uv_max: [1.0, 1.0],
        });
    }

    mesh
}

/// Bed: frame + mattress + headboard.
fn generate_bed_mesh(w: f32, h: f32, d: f32, _color: [f32; 3]) -> MeshData {
    let mut mesh = MeshData::default();
    let hw = w / 2.0;
    let hd = d / 2.0;
    let frame_h = 0.15;
    let mattress_h = h - frame_h;
    let headboard_h = 0.5;

    // Frame top
    append_quad(&mut mesh, Quad {
        tl: [-hw, frame_h, hd], tr: [hw, frame_h, hd], bl: [-hw, frame_h, -hd], br: [hw, frame_h, -hd],
        normal: [0.0, 1.0, 0.0], uv_min: [0.0, 0.0], uv_max: [1.0, 1.0],
    });
    // Frame front
    append_quad(&mut mesh, Quad {
        tl: [-hw, frame_h, -hd], tr: [hw, frame_h, -hd], bl: [-hw, 0.0, -hd], br: [hw, 0.0, -hd],
        normal: [0.0, 0.0, -1.0], uv_min: [0.0, 0.0], uv_max: [1.0, 1.0],
    });
    // Frame sides
    append_quad(&mut mesh, Quad {
        tl: [-hw, frame_h, hd], tr: [-hw, frame_h, -hd], bl: [-hw, 0.0, hd], br: [-hw, 0.0, -hd],
        normal: [-1.0, 0.0, 0.0], uv_min: [0.0, 0.0], uv_max: [1.0, 1.0],
    });
    append_quad(&mut mesh, Quad {
        tl: [hw, frame_h, -hd], tr: [hw, frame_h, hd], bl: [hw, 0.0, -hd], br: [hw, 0.0, hd],
        normal: [1.0, 0.0, 0.0], uv_min: [0.0, 0.0], uv_max: [1.0, 1.0],
    });

    // Mattress top
    let mattress_top = frame_h + mattress_h;
    append_quad(&mut mesh, Quad {
        tl: [-hw, mattress_top, hd], tr: [hw, mattress_top, hd],
        bl: [-hw, mattress_top, -hd], br: [hw, mattress_top, -hd],
        normal: [0.0, 1.0, 0.0], uv_min: [0.0, 0.0], uv_max: [1.0, 1.0],
    });
    // Mattress front
    append_quad(&mut mesh, Quad {
        tl: [-hw, mattress_top, -hd], tr: [hw, mattress_top, -hd],
        bl: [-hw, frame_h, -hd], br: [hw, frame_h, -hd],
        normal: [0.0, 0.0, -1.0], uv_min: [0.0, 0.0], uv_max: [1.0, 1.0],
    });

    // Headboard (vertical panel at back)
    let hb_top = frame_h + headboard_h;
    append_quad(&mut mesh, Quad {
        tl: [-hw, hb_top, hd], tr: [hw, hb_top, hd], bl: [-hw, 0.0, hd], br: [hw, 0.0, hd],
        normal: [0.0, 0.0, 1.0], uv_min: [0.0, 0.0], uv_max: [1.0, 1.0],
    });

    mesh
}

/// Counter: open-top box.
fn generate_counter_mesh(w: f32, h: f32, d: f32, color: [f32; 3]) -> MeshData {
    generate_box_mesh(w, h, d, color)
}

/// Desk: top + 2 side panels + bottom shelf.
fn generate_desk_mesh(w: f32, h: f32, d: f32, _color: [f32; 3]) -> MeshData {
    let mut mesh = MeshData::default();
    let hw = w / 2.0;
    let hd = d / 2.0;
    let panel_t = 0.03;
    let shelf_h = h * 0.3;

    // Top surface
    append_quad(&mut mesh, Quad {
        tl: [-hw, h, hd], tr: [hw, h, hd], bl: [-hw, h, -hd], br: [hw, h, -hd],
        normal: [0.0, 1.0, 0.0], uv_min: [0.0, 0.0], uv_max: [1.0, 1.0],
    });

    // Left side panel
    append_quad(&mut mesh, Quad {
        tl: [-hw, h, hd], tr: [-hw, h, -hd], bl: [-hw, 0.0, hd], br: [-hw, 0.0, -hd],
        normal: [-1.0, 0.0, 0.0], uv_min: [0.0, 0.0], uv_max: [1.0, 1.0],
    });
    // Right side panel
    append_quad(&mut mesh, Quad {
        tl: [hw, h, -hd], tr: [hw, h, hd], bl: [hw, 0.0, -hd], br: [hw, 0.0, hd],
        normal: [1.0, 0.0, 0.0], uv_min: [0.0, 0.0], uv_max: [1.0, 1.0],
    });

    // Bottom shelf
    append_quad(&mut mesh, Quad {
        tl: [-hw + panel_t, shelf_h, -hd + panel_t], tr: [hw - panel_t, shelf_h, -hd + panel_t],
        bl: [-hw + panel_t, shelf_h, hd - panel_t], br: [hw - panel_t, shelf_h, hd - panel_t],
        normal: [0.0, 1.0, 0.0], uv_min: [0.0, 0.0], uv_max: [1.0, 1.0],
    });

    mesh
}

/// Barrel: octagonal approximation (8 vertical faces).
fn generate_barrel_mesh(diameter: f32, h: f32, _color: [f32; 3]) -> MeshData {
    let mut mesh = MeshData::default();
    let r = diameter / 2.0;
    let sides = 8;
    let mid_r = r * 1.08; // slight bulge in the middle

    for i in 0..sides {
        let angle0 = std::f32::consts::TAU * i as f32 / sides as f32;
        let angle1 = std::f32::consts::TAU * (i + 1) as f32 / sides as f32;

        let x0 = angle0.cos() * r;
        let z0 = angle0.sin() * r;
        let x1 = angle1.cos() * r;
        let z1 = angle1.sin() * r;

        let mx0 = angle0.cos() * mid_r;
        let mz0 = angle0.sin() * mid_r;
        let mx1 = angle1.cos() * mid_r;
        let mz1 = angle1.sin() * mid_r;

        let nx = ((angle0 + angle1) / 2.0).cos();
        let nz = ((angle0 + angle1) / 2.0).sin();

        // Lower half
        append_quad(&mut mesh, Quad {
            tl: [mx0, h * 0.6, mz0], tr: [mx1, h * 0.6, mz1],
            bl: [x0, 0.0, z0], br: [x1, 0.0, z1],
            normal: [nx, 0.0, nz], uv_min: [0.0, 0.0], uv_max: [1.0, 1.0],
        });
        // Upper half
        append_quad(&mut mesh, Quad {
            tl: [x0, h, z0], tr: [x1, h, z1],
            bl: [mx0, h * 0.6, mz0], br: [mx1, h * 0.6, mz1],
            normal: [nx, 0.0, nz], uv_min: [0.0, 0.0], uv_max: [1.0, 1.0],
        });
    }

    mesh
}

/// Crate: box with no bottom.
fn generate_crate_mesh(w: f32, h: f32, d: f32, _color: [f32; 3]) -> MeshData {
    let mut mesh = MeshData::default();
    let hw = w / 2.0;
    let hd = d / 2.0;

    // Top
    append_quad(&mut mesh, Quad {
        tl: [-hw, h, hd], tr: [hw, h, hd], bl: [-hw, h, -hd], br: [hw, h, -hd],
        normal: [0.0, 1.0, 0.0], uv_min: [0.0, 0.0], uv_max: [1.0, 1.0],
    });
    // Front
    append_quad(&mut mesh, Quad {
        tl: [-hw, h, -hd], tr: [hw, h, -hd], bl: [-hw, 0.0, -hd], br: [hw, 0.0, -hd],
        normal: [0.0, 0.0, -1.0], uv_min: [0.0, 0.0], uv_max: [1.0, 1.0],
    });
    // Back
    append_quad(&mut mesh, Quad {
        tl: [hw, h, hd], tr: [-hw, h, hd], bl: [hw, 0.0, hd], br: [-hw, 0.0, hd],
        normal: [0.0, 0.0, 1.0], uv_min: [0.0, 0.0], uv_max: [1.0, 1.0],
    });
    // Left
    append_quad(&mut mesh, Quad {
        tl: [-hw, h, hd], tr: [-hw, h, -hd], bl: [-hw, 0.0, hd], br: [-hw, 0.0, -hd],
        normal: [-1.0, 0.0, 0.0], uv_min: [0.0, 0.0], uv_max: [1.0, 1.0],
    });
    // Right
    append_quad(&mut mesh, Quad {
        tl: [hw, h, -hd], tr: [hw, h, hd], bl: [hw, 0.0, -hd], br: [hw, 0.0, hd],
        normal: [1.0, 0.0, 0.0], uv_min: [0.0, 0.0], uv_max: [1.0, 1.0],
    });

    // Cross detail on front face
    let inset = 0.02;
    append_quad(&mut mesh, Quad {
        tl: [-hw + inset, h - inset, -hd - 0.001], tr: [hw - inset, inset, -hd - 0.001],
        bl: [-hw + inset, inset, -hd - 0.001], br: [hw - inset, h - inset, -hd - 0.001],
        normal: [0.0, 0.0, -1.0], uv_min: [0.0, 0.0], uv_max: [1.0, 1.0],
    });

    mesh
}

/// Bench: flat seat + 2 side supports.
fn generate_bench_mesh(w: f32, h: f32, d: f32, _color: [f32; 3]) -> MeshData {
    let mut mesh = MeshData::default();
    let hw = w / 2.0;
    let hd = d / 2.0;
    let support_t = 0.05;

    // Seat surface
    append_quad(&mut mesh, Quad {
        tl: [-hw, h, hd], tr: [hw, h, hd], bl: [-hw, h, -hd], br: [hw, h, -hd],
        normal: [0.0, 1.0, 0.0], uv_min: [0.0, 0.0], uv_max: [1.0, 1.0],
    });

    // Left support
    append_quad(&mut mesh, Quad {
        tl: [-hw, h, hd], tr: [-hw + support_t, h, hd],
        bl: [-hw, 0.0, hd], br: [-hw + support_t, 0.0, hd],
        normal: [0.0, 0.0, 1.0], uv_min: [0.0, 0.0], uv_max: [1.0, 1.0],
    });
    append_quad(&mut mesh, Quad {
        tl: [-hw + support_t, h, -hd], tr: [-hw, h, -hd],
        bl: [-hw + support_t, 0.0, -hd], br: [-hw, 0.0, -hd],
        normal: [0.0, 0.0, -1.0], uv_min: [0.0, 0.0], uv_max: [1.0, 1.0],
    });

    // Right support
    append_quad(&mut mesh, Quad {
        tl: [hw - support_t, h, hd], tr: [hw, h, hd],
        bl: [hw - support_t, 0.0, hd], br: [hw, 0.0, hd],
        normal: [0.0, 0.0, 1.0], uv_min: [0.0, 0.0], uv_max: [1.0, 1.0],
    });
    append_quad(&mut mesh, Quad {
        tl: [hw, h, -hd], tr: [hw - support_t, h, -hd],
        bl: [hw, 0.0, -hd], br: [hw - support_t, 0.0, -hd],
        normal: [0.0, 0.0, -1.0], uv_min: [0.0, 0.0], uv_max: [1.0, 1.0],
    });

    mesh
}
