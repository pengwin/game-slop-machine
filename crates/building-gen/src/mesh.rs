//! 3D mesh generation for building tiles.
//!
//! Uses simple box primitives scaled to the appropriate dimensions.
//! This is much simpler than generating custom meshes with cutouts.
//!
//! Each tile type maps to:
//! - A base mesh (unit cube or floor quad)
//! - A scale transform (width, height, depth)
//! - A color/material hint

use crate::config::BuildingConfig;
use crate::geometry::Rect;
use crate::layout::RoofGeometry;
use crate::tile::{TileGrid, TileType};

/// A unit cube mesh (1x1x1) centered at origin.
///
/// This is the base for all wall tiles. It gets scaled by the
/// tile's dimensions (tile_size, wall_height, wall_thickness).
pub fn unit_cube() -> Vec<[f32; 3]> {
    vec![
        // Front face
        [-0.5, 0.0, -0.5],
        [0.5, 0.0, -0.5],
        [0.5, 1.0, -0.5],
        [-0.5, 1.0, -0.5],
        // Back face
        [-0.5, 0.0, 0.5],
        [0.5, 0.0, 0.5],
        [0.5, 1.0, 0.5],
        [-0.5, 1.0, 0.5],
        // Top face
        [-0.5, 1.0, -0.5],
        [0.5, 1.0, -0.5],
        [0.5, 1.0, 0.5],
        [-0.5, 1.0, 0.5],
        // Bottom face
        [-0.5, 0.0, -0.5],
        [0.5, 0.0, -0.5],
        [0.5, 0.0, 0.5],
        [-0.5, 0.0, 0.5],
        // Left face
        [-0.5, 0.0, -0.5],
        [-0.5, 1.0, -0.5],
        [-0.5, 1.0, 0.5],
        [-0.5, 0.0, 0.5],
        // Right face
        [0.5, 0.0, -0.5],
        [0.5, 1.0, -0.5],
        [0.5, 1.0, 0.5],
        [0.5, 0.0, 0.5],
    ]
}

/// Normals for the unit cube (4 per face, 6 faces).
pub fn unit_cube_normals() -> Vec<[f32; 3]> {
    vec![
        [0.0, 0.0, -1.0],
        [0.0, 0.0, -1.0],
        [0.0, 0.0, -1.0],
        [0.0, 0.0, -1.0],
        [0.0, 0.0, 1.0],
        [0.0, 0.0, 1.0],
        [0.0, 0.0, 1.0],
        [0.0, 0.0, 1.0],
        [0.0, 1.0, 0.0],
        [0.0, 1.0, 0.0],
        [0.0, 1.0, 0.0],
        [0.0, 1.0, 0.0],
        [0.0, -1.0, 0.0],
        [0.0, -1.0, 0.0],
        [0.0, -1.0, 0.0],
        [0.0, -1.0, 0.0],
        [-1.0, 0.0, 0.0],
        [-1.0, 0.0, 0.0],
        [-1.0, 0.0, 0.0],
        [-1.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
    ]
}

/// UV coordinates for the unit cube.
pub fn unit_cube_uvs() -> Vec<[f32; 2]> {
    vec![
        [0.0, 0.0],
        [1.0, 0.0],
        [1.0, 1.0],
        [0.0, 1.0],
        [0.0, 0.0],
        [1.0, 0.0],
        [1.0, 1.0],
        [0.0, 1.0],
        [0.0, 0.0],
        [1.0, 0.0],
        [1.0, 1.0],
        [0.0, 1.0],
        [0.0, 0.0],
        [1.0, 0.0],
        [1.0, 1.0],
        [0.0, 1.0],
        [0.0, 0.0],
        [1.0, 0.0],
        [1.0, 1.0],
        [0.0, 1.0],
        [0.0, 0.0],
        [1.0, 0.0],
        [1.0, 1.0],
        [0.0, 1.0],
    ]
}

/// Indices for the unit cube (2 triangles per face, 6 faces).
pub fn unit_cube_indices() -> Vec<u32> {
    vec![
        0, 2, 1, 0, 3, 2, 4, 5, 6, 4, 6, 7, 8, 9, 10, 8, 10, 11, 12, 14, 13, 12, 15, 14, 16, 17,
        18, 16, 18, 19, 20, 22, 21, 20, 23, 22,
    ]
}

/// A floor quad (1x1 at y=0).
pub fn floor_quad() -> Vec<[f32; 3]> {
    vec![
        [-0.5, 0.0, -0.5],
        [0.5, 0.0, -0.5],
        [0.5, 0.0, 0.5],
        [-0.5, 0.0, 0.5],
    ]
}

/// Normals for the floor quad (all pointing up).
pub fn floor_quad_normals() -> Vec<[f32; 3]> {
    vec![
        [0.0, 1.0, 0.0],
        [0.0, 1.0, 0.0],
        [0.0, 1.0, 0.0],
        [0.0, 1.0, 0.0],
    ]
}

/// UVs for the floor quad.
pub fn floor_quad_uvs() -> Vec<[f32; 2]> {
    vec![[0.0, 0.0], [1.0, 0.0], [1.0, 1.0], [0.0, 1.0]]
}

/// Indices for the floor quad.
pub fn floor_quad_indices() -> Vec<u32> {
    vec![0, 1, 2, 0, 2, 3]
}

/// Gets the scale transform for a tile type.
///
/// Returns (x, y, z) scale factors to apply to the unit cube.
/// The unit cube is 1x1x1, so scaling by these values gives the
/// correct dimensions for each tile type.
pub fn tile_scale(tile_type: TileType, config: &BuildingConfig) -> (f32, f32, f32) {
    let s = config.tile_size;
    let h = config.wall_height;
    let t = config.wall_thickness;

    match tile_type {
        // Walls: full tile width, wall height, wall thickness
        TileType::Wall => (s, h, t),
        // Corner: full tile in all dimensions
        TileType::WallCorner => (s, h, s),
        // Doorway tiles are logical openings and should not be rendered as walls.
        TileType::Doorway => (0.0, 0.0, 0.0),
        TileType::Door => (s, config.door_height, t),
        TileType::Window => (s, config.window_height, t),
        // Floor: full tile width, small height for visibility, full tile depth
        TileType::Floor => (s, 0.1, s),
        // Empty: no scale (shouldn't be rendered)
        TileType::Empty => (0.0, 0.0, 0.0),
    }
}

/// Gets the color hint for a tile type.
///
/// Returns (r, g, b) color values for visual distinction.
/// The actual rendering should use materials, but this helps
/// with debugging and ASCII visualization.
pub fn tile_color(tile_type: TileType) -> (f32, f32, f32) {
    match tile_type {
        TileType::Floor => (0.6, 0.6, 0.6),
        TileType::Wall => (0.8, 0.8, 0.8),
        TileType::WallCorner => (0.7, 0.7, 0.7),
        TileType::Doorway => (0.4, 0.2, 0.0),
        TileType::Door => (0.4, 0.2, 0.0),
        TileType::Window => (0.5, 0.7, 1.0),
        TileType::Empty => (0.0, 0.0, 0.0),
    }
}

// ---------------------------------------------------------------------------
// Merged building mesh generation
// ---------------------------------------------------------------------------

/// Raw mesh data (vertices, normals, UVs, indices).
#[derive(Debug, Clone, Default)]
pub struct MeshData {
    pub vertices: Vec<[f32; 3]>,
    pub normals: Vec<[f32; 3]>,
    pub uvs: Vec<[f32; 2]>,
    pub indices: Vec<u32>,
}

impl MeshData {
    pub fn is_empty(&self) -> bool {
        self.indices.is_empty()
    }
}

/// Complete building mesh split by material.
#[derive(Debug, Clone, Default)]
pub struct BuildingMesh {
    pub wall_mesh: MeshData,
    pub floor_mesh: MeshData,
    pub roof_mesh: MeshData,
    pub door_mesh: MeshData,
    pub window_mesh: MeshData,
}

/// Generates a complete merged building mesh from the tile grid.
///
/// Instead of spawning one entity per tile, this produces a single `MeshData`
/// per material category. Exposed faces between wall and empty space are emitted,
/// internal faces between adjacent walls are skipped.
pub fn generate_building_mesh(
    grid: &TileGrid,
    config: &BuildingConfig,
    roof: &RoofGeometry,
) -> BuildingMesh {
    BuildingMesh {
        wall_mesh: generate_wall_mesh(grid, config),
        floor_mesh: generate_floor_mesh(grid, config),
        roof_mesh: generate_roof_mesh(config.footprint, roof, config),
        door_mesh: generate_door_mesh(grid, config),
        window_mesh: generate_window_mesh(grid, config),
    }
}

// ---------------------------------------------------------------------------
// Wall mesh
// ---------------------------------------------------------------------------

fn generate_wall_mesh(grid: &TileGrid, config: &BuildingConfig) -> MeshData {
    let mut mesh = MeshData::default();

    for y in 0..grid.height {
        for x in 0..grid.width {
            let tile = grid.get(x, y);
            if !matches!(
                tile,
                TileType::Wall | TileType::WallCorner | TileType::Door | TileType::Window
            ) {
                continue;
            }

            let (bounds, _faces_dir) = wall_bounds(grid, x, y, config);
            let along_z = grid.wall_runs_along_z(x, y);

            // Check each cardinal direction.
            for dir in [
                WallFaceDir::NegX,
                WallFaceDir::PosX,
                WallFaceDir::NegZ,
                WallFaceDir::PosZ,
            ] {
                let (dx, dy) = dir.to_offset();
                let neighbor = grid.get_neighbor(x, y, dx, dy);
                let face_cutout = wall_tile_cutout(tile, along_z, dir);

                match neighbor {
                    Some(TileType::Floor | TileType::Doorway) => {
                        append_wall_face(&mut mesh, bounds, dir, config, face_cutout);
                    }
                    Some(TileType::Door) => {
                        let cutout =
                            face_cutout.or_else(|| wall_tile_cutout(TileType::Door, along_z, dir));
                        append_wall_face(&mut mesh, bounds, dir, config, cutout);
                    }
                    Some(TileType::Window) => {
                        let cutout = face_cutout
                            .or_else(|| wall_tile_cutout(TileType::Window, along_z, dir));
                        append_wall_face(&mut mesh, bounds, dir, config, cutout);
                    }
                    // Empty or OOB: exterior face — cutout if current tile is Door/Window.
                    None | Some(TileType::Empty) => {
                        append_wall_face(&mut mesh, bounds, dir, config, face_cutout);
                    }
                    Some(TileType::Wall | TileType::WallCorner) => {}
                }
            }

            // Top face (always visible).
            append_wall_face(&mut mesh, bounds, WallFaceDir::PosY, config, None);
        }
    }

    mesh
}

fn wall_tile_cutout(tile: TileType, along_z: bool, dir: WallFaceDir) -> Option<WallCutout> {
    let is_wall_face = if along_z {
        matches!(dir, WallFaceDir::NegX | WallFaceDir::PosX)
    } else {
        matches!(dir, WallFaceDir::NegZ | WallFaceDir::PosZ)
    };

    match (tile, is_wall_face) {
        (TileType::Door, true) => Some(WallCutout::Door),
        (TileType::Window, true) => Some(WallCutout::Window),
        _ => None,
    }
}

/// Wall face direction.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum WallFaceDir {
    NegX,
    PosX,
    NegZ,
    PosZ,
    PosY,
}

impl WallFaceDir {
    fn to_offset(self) -> (i32, i32) {
        match self {
            Self::NegX => (-1, 0),
            Self::PosX => (1, 0),
            Self::NegZ => (0, -1),
            Self::PosZ => (0, 1),
            Self::PosY => (0, 0),
        }
    }
}

/// Type of cutout in a wall face.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum WallCutout {
    Door,
    Window,
}

/// Computes the 3D bounding box of a wall tile.
///
/// For interior walls (rooms on both sides), the box is centered in the tile.
/// For exterior walls, the box is aligned to the room-side tile boundary.
fn wall_bounds(
    grid: &TileGrid,
    x: usize,
    y: usize,
    config: &BuildingConfig,
) -> (([f32; 3], [f32; 3]), WallFaceDir) {
    let t = config.wall_thickness;
    let ts = config.tile_size;
    let origin_x = grid.origin.x;
    let origin_z = grid.origin.y;

    let tile_min_x = origin_x + x as f32 * ts;
    let tile_min_z = origin_z + y as f32 * ts;

    if grid.wall_runs_along_z(x, y) {
        // Wall runs along Z, thin in X.
        let room_left = grid.is_room_neighbor(x, y, -1, 0);
        let room_right = grid.is_room_neighbor(x, y, 1, 0);

        let (wall_min_x, wall_max_x, face_dir) = if room_left && room_right {
            // Interior wall: center in tile.
            let center = tile_min_x + ts / 2.0;
            (center - t / 2.0, center + t / 2.0, WallFaceDir::NegX)
        } else if room_left {
            (tile_min_x, tile_min_x + t, WallFaceDir::NegX)
        } else {
            (tile_min_x + ts - t, tile_min_x + ts, WallFaceDir::PosX)
        };
        let min = [wall_min_x, 0.0, tile_min_z];
        let max = [wall_max_x, config.wall_height, tile_min_z + ts];
        ((min, max), face_dir)
    } else {
        // Wall runs along X, thin in Z.
        let room_below = grid.is_room_neighbor(x, y, 0, -1);
        let room_above = grid.is_room_neighbor(x, y, 0, 1);

        let (wall_min_z, wall_max_z, face_dir) = if room_below && room_above {
            // Interior wall: center in tile.
            let center = tile_min_z + ts / 2.0;
            (center - t / 2.0, center + t / 2.0, WallFaceDir::NegZ)
        } else if room_below {
            (tile_min_z, tile_min_z + t, WallFaceDir::NegZ)
        } else {
            (tile_min_z + ts - t, tile_min_z + ts, WallFaceDir::PosZ)
        };
        let min = [tile_min_x, 0.0, wall_min_z];
        let max = [tile_min_x + ts, config.wall_height, wall_max_z];
        ((min, max), face_dir)
    }
}

/// Appends a wall face quad (or sub-quads with cutout) to the mesh.
fn append_wall_face(
    mesh: &mut MeshData,
    bounds: ([f32; 3], [f32; 3]),
    dir: WallFaceDir,
    config: &BuildingConfig,
    cutout: Option<WallCutout>,
) {
    let [min_x, min_y, min_z] = bounds.0;
    let [max_x, max_y, max_z] = bounds.1;

    let (normal, tl, tr, bl, br) = match dir {
        WallFaceDir::PosX => {
            let n = [1.0, 0.0, 0.0];
            (
                n,
                [max_x, max_y, min_z],
                [max_x, max_y, max_z],
                [max_x, min_y, min_z],
                [max_x, min_y, max_z],
            )
        }
        WallFaceDir::NegX => {
            let n = [-1.0, 0.0, 0.0];
            (
                n,
                [min_x, max_y, max_z],
                [min_x, max_y, min_z],
                [min_x, min_y, max_z],
                [min_x, min_y, min_z],
            )
        }
        WallFaceDir::PosZ => {
            let n = [0.0, 0.0, 1.0];
            (
                n,
                [min_x, max_y, max_z],
                [max_x, max_y, max_z],
                [min_x, min_y, max_z],
                [max_x, min_y, max_z],
            )
        }
        WallFaceDir::NegZ => {
            let n = [0.0, 0.0, -1.0];
            (
                n,
                [max_x, max_y, min_z],
                [min_x, max_y, min_z],
                [max_x, min_y, min_z],
                [min_x, min_y, min_z],
            )
        }
        WallFaceDir::PosY => {
            let n = [0.0, 1.0, 0.0];
            (
                n,
                [min_x, max_y, max_z],
                [max_x, max_y, max_z],
                [min_x, max_y, min_z],
                [max_x, max_y, min_z],
            )
        }
    };

    // Compute the face-local coordinate system for cutout positioning.
    // For vertical faces: u = horizontal along wall, v = vertical (height).
    // We derive u_axis and v_axis from the four corner positions.
    let u_axis = sub3(tr, tl); // horizontal direction
    let v_axis = sub3(bl, tl); // vertical direction (bottom - top, so pointing down)
                               // Actually: tl is top-left, bl is bottom-left. v = bl - tl points downward.
                               // For cutout positioning, we want v pointing upward, so use tl..bl range for v.

    let u_len = vec3_length(u_axis);
    let v_len = vec3_length(v_axis);

    match cutout {
        Some(WallCutout::Door) => {
            let door_start = (u_len - config.door_width) / 2.0;
            let door_end = door_start + config.door_width;
            let door_h = config.door_height.min(max_y - min_y);
            append_wall_sub_faces(
                mesh, tl, tr, bl, br, normal, u_len, v_len, door_start, door_end, 0.0, door_h,
            );
        }
        Some(WallCutout::Window) => {
            let win_start = (u_len - config.window_width) / 2.0;
            let win_end = win_start + config.window_width;
            let sill = config.window_sill_height;
            let win_top = (sill + config.window_height).min(max_y - min_y);
            append_wall_sub_faces(
                mesh, tl, tr, bl, br, normal, u_len, v_len, win_start, win_end, sill, win_top,
            );
        }
        None => {
            append_quad(mesh, tl, tr, bl, br, normal, [0.0, 0.0], [u_len, v_len]);
        }
    }
}

/// Splits a wall face into sub-quads around a rectangular cutout.
///
/// `tl, tr, bl, br` are the four corners of the face.
/// `u_len, v_len` are the face dimensions in the horizontal and vertical directions.
/// `cutout_start, cutout_end` are horizontal cutout bounds in face-local coords.
/// `cutout_bottom, cutout_top` are vertical cutout bounds.
fn append_wall_sub_faces(
    mesh: &mut MeshData,
    tl: [f32; 3],
    tr: [f32; 3],
    bl: [f32; 3],
    br: [f32; 3],
    normal: [f32; 3],
    u_len: f32,
    v_len: f32,
    cutout_start: f32,
    cutout_end: f32,
    cutout_bottom: f32,
    cutout_top: f32,
) {
    let cs = cutout_start.max(0.0).min(u_len);
    let ce = cutout_end.max(0.0).min(u_len);
    let cb = cutout_bottom.max(0.0).min(v_len);
    let ct = cutout_top.max(0.0).min(v_len);

    // Face-local interpolation helpers.
    // v goes from 0 at bottom (bl/br) to v_len at top (tl/tr).
    let lerp_face = |u: f32, v: f32| -> [f32; 3] {
        let ut = if u_len > 0.0 { u / u_len } else { 0.0 };
        let vt = if v_len > 0.0 { v / v_len } else { 0.0 };
        let top = lerp3(tl, tr, ut);
        let bot = lerp3(bl, br, ut);
        lerp3(bot, top, vt)
    };

    // Left of cutout
    if cs > 0.0 {
        let a = lerp_face(0.0, 0.0);
        let b = lerp_face(cs, 0.0);
        let c = lerp_face(cs, v_len);
        let d = lerp_face(0.0, v_len);
        append_quad(mesh, d, c, a, b, normal, [0.0, 0.0], [cs, v_len]);
    }

    // Right of cutout
    if ce < u_len {
        let a = lerp_face(ce, 0.0);
        let b = lerp_face(u_len, 0.0);
        let c = lerp_face(u_len, v_len);
        let d = lerp_face(ce, v_len);
        append_quad(mesh, d, c, a, b, normal, [ce, 0.0], [u_len, v_len]);
    }

    // Below cutout
    if cb > 0.0 {
        let a = lerp_face(cs, 0.0);
        let b = lerp_face(ce, 0.0);
        let c = lerp_face(ce, cb);
        let d = lerp_face(cs, cb);
        append_quad(mesh, d, c, a, b, normal, [cs, 0.0], [ce, cb]);
    }

    // Above cutout
    if ct < v_len {
        let a = lerp_face(cs, ct);
        let b = lerp_face(ce, ct);
        let c = lerp_face(ce, v_len);
        let d = lerp_face(cs, v_len);
        append_quad(mesh, d, c, a, b, normal, [cs, ct], [ce, v_len]);
    }
}

// ---------------------------------------------------------------------------
// Floor mesh
// ---------------------------------------------------------------------------

fn generate_floor_mesh(grid: &TileGrid, config: &BuildingConfig) -> MeshData {
    let mut mesh = MeshData::default();
    let ts = config.tile_size;
    let origin_x = grid.origin.x;
    let origin_z = grid.origin.y;

    for y in 0..grid.height {
        for x in 0..grid.width {
            if grid.get(x, y) != TileType::Floor {
                continue;
            }
            let x0 = origin_x + x as f32 * ts;
            let z0 = origin_z + y as f32 * ts;
            let x1 = x0 + ts;
            let z1 = z0 + ts;

            append_quad(
                &mut mesh,
                [x0, 0.0, z1],
                [x1, 0.0, z1],
                [x0, 0.0, z0],
                [x1, 0.0, z0],
                [0.0, 1.0, 0.0],
                [x0, z0],
                [x1, z1],
            );
        }
    }

    mesh
}

// ---------------------------------------------------------------------------
// Door mesh (thin slabs filling door openings)
// ---------------------------------------------------------------------------

fn generate_door_mesh(grid: &TileGrid, config: &BuildingConfig) -> MeshData {
    let mut mesh = MeshData::default();

    for y in 0..grid.height {
        for x in 0..grid.width {
            if grid.get(x, y) != TileType::Door {
                continue;
            }

            let (bounds, _face_dir) = wall_bounds(grid, x, y, config);
            let [min_x, min_y, min_z] = bounds.0;
            let [max_x, _max_y, max_z] = bounds.1;
            let h = config.door_height;
            let t = config.wall_thickness * 0.5;

            // Determine wall axis and compute door position.
            let along_z = grid.wall_runs_along_z(x, y);
            if along_z {
                // Wall runs along Z, door face is in XY plane.
                let width = max_z - min_z;
                let door_start = (width - config.door_width) / 2.0;
                let ds = min_z + door_start;
                let de = ds + config.door_width;
                let cx = (min_x + max_x) / 2.0;

                // +X face
                append_quad(
                    &mut mesh,
                    [cx + t / 2.0, h, ds],
                    [cx + t / 2.0, h, de],
                    [cx + t / 2.0, min_y, ds],
                    [cx + t / 2.0, min_y, de],
                    [1.0, 0.0, 0.0],
                    [0.0, 0.0],
                    [config.door_width, h],
                );
                // -X face
                append_quad(
                    &mut mesh,
                    [cx - t / 2.0, h, de],
                    [cx - t / 2.0, h, ds],
                    [cx - t / 2.0, min_y, de],
                    [cx - t / 2.0, min_y, ds],
                    [-1.0, 0.0, 0.0],
                    [0.0, 0.0],
                    [config.door_width, h],
                );
            } else {
                // Wall runs along X, door face is in YZ plane.
                let width = max_x - min_x;
                let door_start = (width - config.door_width) / 2.0;
                let ds = min_x + door_start;
                let de = ds + config.door_width;
                let cz = (min_z + max_z) / 2.0;

                // +Z face
                append_quad(
                    &mut mesh,
                    [ds, h, cz + t / 2.0],
                    [de, h, cz + t / 2.0],
                    [ds, min_y, cz + t / 2.0],
                    [de, min_y, cz + t / 2.0],
                    [0.0, 0.0, 1.0],
                    [0.0, 0.0],
                    [config.door_width, h],
                );
                // -Z face
                append_quad(
                    &mut mesh,
                    [de, h, cz - t / 2.0],
                    [ds, h, cz - t / 2.0],
                    [de, min_y, cz - t / 2.0],
                    [ds, min_y, cz - t / 2.0],
                    [0.0, 0.0, -1.0],
                    [0.0, 0.0],
                    [config.door_width, h],
                );
            }
        }
    }

    mesh
}

// ---------------------------------------------------------------------------
// Window mesh (thin glass panes filling window openings)
// ---------------------------------------------------------------------------

fn generate_window_mesh(grid: &TileGrid, config: &BuildingConfig) -> MeshData {
    let mut mesh = MeshData::default();

    for y in 0..grid.height {
        for x in 0..grid.width {
            if grid.get(x, y) != TileType::Window {
                continue;
            }

            let (bounds, _face_dir) = wall_bounds(grid, x, y, config);
            let [min_x, _min_y, min_z] = bounds.0;
            let [max_x, _max_y, max_z] = bounds.1;
            let sill = config.window_sill_height;
            let wh = config.window_height;

            let along_z = grid.wall_runs_along_z(x, y);
            if along_z {
                let width = max_z - min_z;
                let win_start = (width - config.window_width) / 2.0;
                let ws = min_z + win_start;
                let we = ws + config.window_width;
                let offset = 0.02;

                // Glass at exterior face (+X side), facing outward
                append_quad(
                    &mut mesh,
                    [max_x - offset, sill + wh, ws],
                    [max_x - offset, sill + wh, we],
                    [max_x - offset, sill, ws],
                    [max_x - offset, sill, we],
                    [1.0, 0.0, 0.0],
                    [0.0, 0.0],
                    [config.window_width, wh],
                );
                // Glass at interior face (-X side), facing inward
                append_quad(
                    &mut mesh,
                    [min_x + offset, sill + wh, we],
                    [min_x + offset, sill + wh, ws],
                    [min_x + offset, sill, we],
                    [min_x + offset, sill, ws],
                    [-1.0, 0.0, 0.0],
                    [0.0, 0.0],
                    [config.window_width, wh],
                );
            } else {
                let width = max_x - min_x;
                let win_start = (width - config.window_width) / 2.0;
                let ws = min_x + win_start;
                let we = ws + config.window_width;
                let offset = 0.02;

                // Glass at exterior face (+Z side), facing outward
                append_quad(
                    &mut mesh,
                    [ws, sill + wh, max_z - offset],
                    [we, sill + wh, max_z - offset],
                    [ws, sill, max_z - offset],
                    [we, sill, max_z - offset],
                    [0.0, 0.0, 1.0],
                    [0.0, 0.0],
                    [config.window_width, wh],
                );
                // Glass at interior face (-Z side), facing inward
                append_quad(
                    &mut mesh,
                    [we, sill + wh, min_z + offset],
                    [ws, sill + wh, min_z + offset],
                    [we, sill, min_z + offset],
                    [ws, sill, min_z + offset],
                    [0.0, 0.0, -1.0],
                    [0.0, 0.0],
                    [config.window_width, wh],
                );
            }
        }
    }

    mesh
}

// ---------------------------------------------------------------------------
// Roof mesh (with proper normals and gable ends)
// ---------------------------------------------------------------------------

fn generate_roof_mesh(bounds: Rect, _roof: &RoofGeometry, config: &BuildingConfig) -> MeshData {
    let mut mesh = MeshData::default();
    let overhang = config.roof_overhang;
    let wall_h = config.wall_height;

    let min_x = bounds.min.x - overhang;
    let max_x = bounds.max.x + overhang;
    let min_z = bounds.min.y - overhang;
    let max_z = bounds.max.y + overhang;
    let center_z = (bounds.min.y + bounds.max.y) / 2.0;
    let ridge_y = wall_h + config.roof_height;

    // South slope (facing -Z). Vertices: bottom-left, bottom-right, top-right, top-left.
    let n_south = {
        let a = [max_x - min_x, 0.0, 0.0];
        let b = [0.0, ridge_y - wall_h, center_z - min_z];
        normalize3(cross3(a, b))
    };
    append_quad(
        &mut mesh,
        [min_x, wall_h, min_z],
        [max_x, wall_h, min_z],
        [min_x, ridge_y, center_z],
        [max_x, ridge_y, center_z],
        n_south,
        [min_x, min_z],
        [max_x, center_z],
    );

    // North slope (facing +Z).
    let n_north = {
        let a = [min_x - max_x, 0.0, 0.0];
        let b = [0.0, ridge_y - wall_h, center_z - max_z];
        normalize3(cross3(a, b))
    };
    append_quad(
        &mut mesh,
        [max_x, wall_h, max_z],
        [min_x, wall_h, max_z],
        [max_x, ridge_y, center_z],
        [min_x, ridge_y, center_z],
        n_north,
        [min_x, max_z],
        [max_x, center_z],
    );

    // Gable ends (triangles).
    let ridge = [(min_x + max_x) / 2.0, ridge_y];

    // -Z gable
    let n_gable_neg_z = [0.0, 0.0, -1.0];
    let base0 = mesh.vertices.len() as u32;
    mesh.vertices.push([min_x, wall_h, min_z]);
    mesh.vertices.push([max_x, wall_h, min_z]);
    mesh.vertices.push([ridge[0], ridge[1], min_z]);
    mesh.normals.push(n_gable_neg_z);
    mesh.normals.push(n_gable_neg_z);
    mesh.normals.push(n_gable_neg_z);
    mesh.uvs.push([min_x, wall_h]);
    mesh.uvs.push([max_x, wall_h]);
    mesh.uvs.push([ridge[0], ridge[1]]);
    mesh.indices
        .extend_from_slice(&[base0, base0 + 1, base0 + 2]);

    // +Z gable
    let n_gable_pos_z = [0.0, 0.0, 1.0];
    let base1 = mesh.vertices.len() as u32;
    mesh.vertices.push([max_x, wall_h, max_z]);
    mesh.vertices.push([min_x, wall_h, max_z]);
    mesh.vertices.push([ridge[0], ridge[1], max_z]);
    mesh.normals.push(n_gable_pos_z);
    mesh.normals.push(n_gable_pos_z);
    mesh.normals.push(n_gable_pos_z);
    mesh.uvs.push([max_x, max_z]);
    mesh.uvs.push([min_x, max_z]);
    mesh.uvs.push([ridge[0], ridge[1]]);
    mesh.indices
        .extend_from_slice(&[base1, base1 + 1, base1 + 2]);

    mesh
}

// ---------------------------------------------------------------------------
// Low-level helpers
// ---------------------------------------------------------------------------

/// Appends a quad (two triangles) defined by four corners.
///
/// Vertices are expected in winding order such that the cross product of
/// (tr - tl) x (bl - tl) points in the same direction as `normal`.
fn append_quad(
    mesh: &mut MeshData,
    tl: [f32; 3],
    tr: [f32; 3],
    bl: [f32; 3],
    br: [f32; 3],
    normal: [f32; 3],
    uv_min: [f32; 2],
    uv_max: [f32; 2],
) {
    let base = mesh.vertices.len() as u32;

    mesh.vertices.push(tl);
    mesh.vertices.push(tr);
    mesh.vertices.push(bl);
    mesh.vertices.push(br);

    mesh.normals.push(normal);
    mesh.normals.push(normal);
    mesh.normals.push(normal);
    mesh.normals.push(normal);

    mesh.uvs.push([uv_min[0], uv_max[1]]);
    mesh.uvs.push([uv_max[0], uv_max[1]]);
    mesh.uvs.push([uv_min[0], uv_min[1]]);
    mesh.uvs.push([uv_max[0], uv_min[1]]);

    // Triangle 1: tl, tr, br  |  Triangle 2: tl, br, bl
    mesh.indices.push(base);
    mesh.indices.push(base + 1);
    mesh.indices.push(base + 3);
    mesh.indices.push(base);
    mesh.indices.push(base + 3);
    mesh.indices.push(base + 2);
}

fn sub3(a: [f32; 3], b: [f32; 3]) -> [f32; 3] {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
}

fn lerp3(a: [f32; 3], b: [f32; 3], t: f32) -> [f32; 3] {
    [
        a[0] + (b[0] - a[0]) * t,
        a[1] + (b[1] - a[1]) * t,
        a[2] + (b[2] - a[2]) * t,
    ]
}

fn cross3(a: [f32; 3], b: [f32; 3]) -> [f32; 3] {
    [
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
    ]
}

fn vec3_length(v: [f32; 3]) -> f32 {
    (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt()
}

fn normalize3(v: [f32; 3]) -> [f32; 3] {
    let len = vec3_length(v);
    if len < f32::EPSILON {
        return [0.0, 1.0, 0.0];
    }
    [v[0] / len, v[1] / len, v[2] / len]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unit_cube_vertices() {
        let vertices = unit_cube();
        assert_eq!(vertices.len(), 24);
    }

    #[test]
    fn test_unit_cube_indices() {
        let indices = unit_cube_indices();
        assert_eq!(indices.len(), 36);
    }

    #[test]
    fn test_floor_quad_vertices() {
        let vertices = floor_quad();
        assert_eq!(vertices.len(), 4);
    }

    #[test]
    fn test_tile_scale_wall() {
        let config = BuildingConfig::default();
        let (x, y, z) = tile_scale(TileType::Wall, &config);
        assert_eq!(x, config.tile_size);
        assert_eq!(y, config.wall_height);
        assert_eq!(z, config.wall_thickness);
    }

    #[test]
    fn test_tile_scale_floor() {
        let config = BuildingConfig::default();
        let (x, y, z) = tile_scale(TileType::Floor, &config);
        assert_eq!(x, config.tile_size);
        assert!(y > 0.0);
        assert_eq!(z, config.tile_size);
    }

    #[test]
    fn test_generate_building_mesh_not_empty() {
        use crate::config::BuildingConfig;
        use crate::geometry::Rect;
        use crate::layout::RoofGeometry;

        let config = BuildingConfig {
            footprint: Rect::new(0.0, 0.0, 10.0, 8.0),
            ..Default::default()
        };
        let layout = crate::generate_layout(&config, 42);
        let roof = RoofGeometry {
            ridge_start: crate::geometry::Vec3::new(0.0, 5.0, 4.0),
            ridge_end: crate::geometry::Vec3::new(10.0, 5.0, 4.0),
            slope_height: 2.0,
            overhang: 0.5,
        };
        let bmesh = generate_building_mesh(&layout.tile_grid, &config, &roof);

        assert!(!bmesh.wall_mesh.is_empty(), "wall mesh should not be empty");
        assert!(
            !bmesh.floor_mesh.is_empty(),
            "floor mesh should not be empty"
        );
        assert!(!bmesh.roof_mesh.is_empty(), "roof mesh should not be empty");
    }

    #[test]
    fn test_generate_building_mesh_index_integrity() {
        use crate::config::BuildingConfig;
        use crate::geometry::Rect;
        use crate::layout::RoofGeometry;

        let config = BuildingConfig {
            footprint: Rect::new(0.0, 0.0, 10.0, 8.0),
            ..Default::default()
        };
        let layout = crate::generate_layout(&config, 42);
        let roof = RoofGeometry {
            ridge_start: crate::geometry::Vec3::new(0.0, 5.0, 4.0),
            ridge_end: crate::geometry::Vec3::new(10.0, 5.0, 4.0),
            slope_height: 2.0,
            overhang: 0.5,
        };
        let bmesh = generate_building_mesh(&layout.tile_grid, &config, &roof);

        for (name, data) in [
            ("wall", &bmesh.wall_mesh),
            ("floor", &bmesh.floor_mesh),
            ("roof", &bmesh.roof_mesh),
            ("door", &bmesh.door_mesh),
            ("window", &bmesh.window_mesh),
        ] {
            let max_idx = data.indices.iter().copied().max().unwrap_or(0) as usize;
            assert!(
                max_idx < data.vertices.len(),
                "{name}: index {max_idx} out of bounds ({} verts)",
                data.vertices.len()
            );
            assert_eq!(
                data.vertices.len(),
                data.normals.len(),
                "{name}: vertex/normal count mismatch"
            );
            assert_eq!(
                data.vertices.len(),
                data.uvs.len(),
                "{name}: vertex/uv count mismatch"
            );
        }
    }

    #[test]
    fn test_window_tiles_cut_both_wall_faces_for_each_orientation() {
        use crate::config::BuildingConfig;
        use crate::geometry::{Rect, Vec2};

        let config = BuildingConfig {
            footprint: Rect::new(0.0, 0.0, 3.0, 3.0),
            tile_size: 1.0,
            wall_thickness: 0.25,
            window_width: 0.5,
            ..Default::default()
        };

        for (floor_x, floor_y) in [(1, 2), (2, 1)] {
            let mut plain_grid = TileGrid::new(3, 3, config.tile_size, Vec2::ZERO);
            let mut window_grid = TileGrid::new(3, 3, config.tile_size, Vec2::ZERO);

            plain_grid.set(1, 1, TileType::Wall);
            plain_grid.set(floor_x, floor_y, TileType::Floor);
            window_grid.set(1, 1, TileType::Window);
            window_grid.set(floor_x, floor_y, TileType::Floor);

            let plain_mesh = generate_wall_mesh(&plain_grid, &config);
            let window_mesh = generate_wall_mesh(&window_grid, &config);

            // A window cuts the exterior and room-side wall faces. Each cut face
            // replaces one full quad with four sub-quads: 16 - 4 vertices.
            assert_eq!(
                window_mesh.vertices.len() - plain_mesh.vertices.len(),
                2 * 12
            );
        }
    }

    #[test]
    fn test_append_quad_winding() {
        let mut mesh = MeshData::default();
        // Quad in XZ plane at y=0, normal pointing up.
        append_quad(
            &mut mesh,
            [0.0, 0.0, 1.0], // tl
            [1.0, 0.0, 1.0], // tr
            [0.0, 0.0, 0.0], // bl
            [1.0, 0.0, 0.0], // br
            [0.0, 1.0, 0.0],
            [0.0, 0.0],
            [1.0, 1.0],
        );
        assert_eq!(mesh.vertices.len(), 4);
        assert_eq!(mesh.indices.len(), 6);
        // Indices should be: 0,1,3, 0,3,2
        assert_eq!(mesh.indices, vec![0, 1, 3, 0, 3, 2]);
    }
}
