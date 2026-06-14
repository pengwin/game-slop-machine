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
use crate::tile::{
    CardinalDir, CornerDir, TJunctionDir, TileGrid, TileType, WallAxis, WallKind, WallOpening,
    WallShape, WallTile,
};

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
        TileType::Wall(wall) => match wall.shape {
            WallShape::Straight(CardinalDir::Left | CardinalDir::Right) => (t, h, s),
            WallShape::Straight(CardinalDir::Bottom | CardinalDir::Top) => (s, h, t),
            WallShape::Corner(_) | WallShape::TJunction(_) | WallShape::Cross => (s, h, s),
        },
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
        TileType::Wall(wall) => match wall.opening {
            Some(WallOpening::Door { .. } | WallOpening::Doorway) => (0.4, 0.2, 0.0),
            Some(WallOpening::Window { .. }) => (0.5, 0.7, 1.0),
            None => (0.8, 0.8, 0.8),
        },
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
    pub foundation_mesh: MeshData,
    pub wall_mesh: MeshData,
    pub wall_top_mesh: MeshData,
    pub exterior_wall_mesh: MeshData,
    pub exterior_corner_mesh: MeshData,
    pub exterior_t_junction_mesh: MeshData,
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
    let wall_meshes = generate_wall_meshes(grid, config);
    BuildingMesh {
        foundation_mesh: generate_foundation_mesh(config),
        wall_mesh: wall_meshes.wall,
        wall_top_mesh: wall_meshes.top,
        exterior_wall_mesh: wall_meshes.exterior,
        exterior_corner_mesh: wall_meshes.exterior_corner,
        exterior_t_junction_mesh: wall_meshes.exterior_t_junction,
        floor_mesh: generate_floor_mesh(grid, config),
        roof_mesh: generate_roof_mesh(config.footprint, roof, config),
        door_mesh: generate_door_mesh(grid, config),
        window_mesh: generate_window_mesh(grid, config),
    }
}

// ---------------------------------------------------------------------------
// Wall mesh
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Default)]
struct WallMeshes {
    wall: MeshData,
    top: MeshData,
    exterior: MeshData,
    exterior_corner: MeshData,
    exterior_t_junction: MeshData,
}

fn generate_wall_meshes(grid: &TileGrid, config: &BuildingConfig) -> WallMeshes {
    let mut wall_mesh = MeshData::default();
    let mut wall_top_mesh = MeshData::default();
    let mut exterior_wall_mesh = MeshData::default();
    let mut exterior_corner_mesh = MeshData::default();
    let mut exterior_t_junction_mesh = MeshData::default();

    for y in 0..grid.height {
        for x in 0..grid.width {
            let TileType::Wall(wall) = grid.get(x, y) else {
                continue;
            };
            let exterior_faces = exterior_face_dirs(wall);
            for wall_box in wall_boxes(grid, x, y, wall, config) {
                append_wall_box(
                    &mut wall_mesh,
                    &mut wall_top_mesh,
                    &mut exterior_wall_mesh,
                    &mut exterior_corner_mesh,
                    &mut exterior_t_junction_mesh,
                    wall_box.bounds,
                    wall_box.axis,
                    wall_box.exterior_class,
                    &exterior_faces,
                    config,
                    wall_box.cutout,
                );
            }
        }
    }

    WallMeshes {
        wall: wall_mesh,
        top: wall_top_mesh,
        exterior: exterior_wall_mesh,
        exterior_corner: exterior_corner_mesh,
        exterior_t_junction: exterior_t_junction_mesh,
    }
}

#[derive(Debug, Clone, Copy)]
struct WallBox {
    bounds: ([f32; 3], [f32; 3]),
    axis: WallAxis,
    exterior_class: ExteriorFaceClass,
    cutout: Option<WallCutout>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ExteriorFaceClass {
    Straight,
    Corner,
    TJunction,
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

/// Type of cutout in a wall face.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum WallCutout {
    Door,
    Window,
}

fn wall_boxes(
    grid: &TileGrid,
    x: usize,
    y: usize,
    wall: WallTile,
    config: &BuildingConfig,
) -> Vec<WallBox> {
    if wall.kind == WallKind::Interior {
        if let Some(boxes) = interior_connector_boxes(grid, x, y, wall, config) {
            return boxes;
        }
    }

    let bounds = wall_bounds_for_tile(grid, x, y, wall, config);
    vec![WallBox {
        bounds,
        axis: wall.main_axis(),
        exterior_class: exterior_face_class(wall),
        cutout: opening_cutout(wall.opening),
    }]
}

fn interior_connector_boxes(
    grid: &TileGrid,
    x: usize,
    y: usize,
    wall: WallTile,
    config: &BuildingConfig,
) -> Option<Vec<WallBox>> {
    let dirs = occupied_dirs(wall.shape)?;
    let (tile_min_x, tile_max_x, tile_min_z, tile_max_z) = tile_xz_bounds(grid, x, y, config);
    let cx = (tile_min_x + tile_max_x) / 2.0;
    let cz = (tile_min_z + tile_max_z) / 2.0;
    let half = (config.interior_wall_thickness.min(config.tile_size) / 2.0).max(0.0);
    let min_x = cx - half;
    let max_x = cx + half;
    let min_z = cz - half;
    let max_z = cz + half;
    let mut boxes = Vec::new();
    let cutout = opening_cutout(wall.opening);

    boxes.push(WallBox {
        bounds: (
            [min_x, building_base_y(config), min_z],
            [max_x, building_top_y(config), max_z],
        ),
        axis: WallAxis::Both,
        exterior_class: exterior_face_class(wall),
        cutout: None,
    });

    if dirs.left && tile_min_x < min_x {
        boxes.push(connector_wall_box(
            [tile_min_x, building_base_y(config), min_z],
            [min_x, building_top_y(config), max_z],
            WallAxis::X,
            wall,
            cutout,
        ));
    }
    if dirs.right && max_x < tile_max_x {
        boxes.push(connector_wall_box(
            [max_x, building_base_y(config), min_z],
            [tile_max_x, building_top_y(config), max_z],
            WallAxis::X,
            wall,
            cutout,
        ));
    }
    if dirs.bottom && tile_min_z < min_z {
        boxes.push(connector_wall_box(
            [min_x, building_base_y(config), tile_min_z],
            [max_x, building_top_y(config), min_z],
            WallAxis::Z,
            wall,
            cutout,
        ));
    }
    if dirs.top && max_z < tile_max_z {
        boxes.push(connector_wall_box(
            [min_x, building_base_y(config), max_z],
            [max_x, building_top_y(config), tile_max_z],
            WallAxis::Z,
            wall,
            cutout,
        ));
    }

    Some(boxes)
}

fn connector_wall_box(
    min: [f32; 3],
    max: [f32; 3],
    axis: WallAxis,
    wall: WallTile,
    cutout: Option<WallCutout>,
) -> WallBox {
    WallBox {
        bounds: (min, max),
        axis,
        exterior_class: exterior_face_class(wall),
        cutout,
    }
}

#[derive(Debug, Clone, Copy)]
struct OccupiedDirs {
    left: bool,
    right: bool,
    bottom: bool,
    top: bool,
}

fn occupied_dirs(shape: WallShape) -> Option<OccupiedDirs> {
    let dirs = match shape {
        WallShape::Straight(_) => return None,
        WallShape::Corner(CornerDir::BottomLeft) => OccupiedDirs {
            left: true,
            right: false,
            bottom: true,
            top: false,
        },
        WallShape::Corner(CornerDir::BottomRight) => OccupiedDirs {
            left: false,
            right: true,
            bottom: true,
            top: false,
        },
        WallShape::Corner(CornerDir::TopLeft) => OccupiedDirs {
            left: true,
            right: false,
            bottom: false,
            top: true,
        },
        WallShape::Corner(CornerDir::TopRight) => OccupiedDirs {
            left: false,
            right: true,
            bottom: false,
            top: true,
        },
        WallShape::TJunction(TJunctionDir::Left) => OccupiedDirs {
            left: false,
            right: true,
            bottom: true,
            top: true,
        },
        WallShape::TJunction(TJunctionDir::Right) => OccupiedDirs {
            left: true,
            right: false,
            bottom: true,
            top: true,
        },
        WallShape::TJunction(TJunctionDir::Bottom) => OccupiedDirs {
            left: true,
            right: true,
            bottom: false,
            top: true,
        },
        WallShape::TJunction(TJunctionDir::Top) => OccupiedDirs {
            left: true,
            right: true,
            bottom: true,
            top: false,
        },
        WallShape::Cross => OccupiedDirs {
            left: true,
            right: true,
            bottom: true,
            top: true,
        },
    };
    Some(dirs)
}

fn opening_cutout(opening: Option<WallOpening>) -> Option<WallCutout> {
    match opening {
        Some(WallOpening::Door { .. } | WallOpening::Doorway) => Some(WallCutout::Door),
        Some(WallOpening::Window { .. }) => Some(WallCutout::Window),
        None => None,
    }
}

fn exterior_face_dirs(wall: WallTile) -> Vec<WallFaceDir> {
    if wall.kind != WallKind::Exterior {
        return Vec::new();
    }

    match wall.shape {
        WallShape::Straight(CardinalDir::Left) => vec![WallFaceDir::NegX],
        WallShape::Straight(CardinalDir::Right) => vec![WallFaceDir::PosX],
        WallShape::Straight(CardinalDir::Bottom) => vec![WallFaceDir::NegZ],
        WallShape::Straight(CardinalDir::Top) => vec![WallFaceDir::PosZ],
        WallShape::Corner(crate::tile::CornerDir::BottomLeft) => {
            vec![WallFaceDir::NegX, WallFaceDir::NegZ]
        }
        WallShape::Corner(crate::tile::CornerDir::BottomRight) => {
            vec![WallFaceDir::PosX, WallFaceDir::NegZ]
        }
        WallShape::Corner(crate::tile::CornerDir::TopLeft) => {
            vec![WallFaceDir::NegX, WallFaceDir::PosZ]
        }
        WallShape::Corner(crate::tile::CornerDir::TopRight) => {
            vec![WallFaceDir::PosX, WallFaceDir::PosZ]
        }
        WallShape::TJunction(crate::tile::TJunctionDir::Left) => vec![WallFaceDir::NegX],
        WallShape::TJunction(crate::tile::TJunctionDir::Right) => vec![WallFaceDir::PosX],
        WallShape::TJunction(crate::tile::TJunctionDir::Bottom) => vec![WallFaceDir::NegZ],
        WallShape::TJunction(crate::tile::TJunctionDir::Top) => vec![WallFaceDir::PosZ],
        WallShape::Cross => Vec::new(),
    }
}

fn exterior_face_class(wall: WallTile) -> ExteriorFaceClass {
    match wall.shape {
        WallShape::Corner(_) => ExteriorFaceClass::Corner,
        WallShape::TJunction(_) => ExteriorFaceClass::TJunction,
        WallShape::Straight(_) | WallShape::Cross => ExteriorFaceClass::Straight,
    }
}

fn wall_bounds_for_tile(
    grid: &TileGrid,
    x: usize,
    y: usize,
    wall: WallTile,
    config: &BuildingConfig,
) -> ([f32; 3], [f32; 3]) {
    let (tile_min_x, tile_max_x, tile_min_z, tile_max_z) = tile_xz_bounds(grid, x, y, config);
    let ext = config.wall_thickness.min(config.tile_size);
    let int = config.interior_wall_thickness.min(config.tile_size);

    let (min_x, max_x, min_z, max_z) = match wall.shape {
        WallShape::Straight(CardinalDir::Left) if wall.kind == WallKind::Exterior => {
            (tile_min_x, tile_min_x + ext, tile_min_z, tile_max_z)
        }
        WallShape::Straight(CardinalDir::Right) if wall.kind == WallKind::Exterior => {
            (tile_max_x - ext, tile_max_x, tile_min_z, tile_max_z)
        }
        WallShape::Straight(CardinalDir::Bottom) if wall.kind == WallKind::Exterior => {
            (tile_min_x, tile_max_x, tile_min_z, tile_min_z + ext)
        }
        WallShape::Straight(CardinalDir::Top) if wall.kind == WallKind::Exterior => {
            (tile_min_x, tile_max_x, tile_max_z - ext, tile_max_z)
        }
        WallShape::Straight(CardinalDir::Left | CardinalDir::Right) => {
            let cx = (tile_min_x + tile_max_x) / 2.0;
            (cx - int / 2.0, cx + int / 2.0, tile_min_z, tile_max_z)
        }
        WallShape::Straight(CardinalDir::Bottom | CardinalDir::Top) => {
            let cz = (tile_min_z + tile_max_z) / 2.0;
            (tile_min_x, tile_max_x, cz - int / 2.0, cz + int / 2.0)
        }
        WallShape::Corner(_) | WallShape::TJunction(_) | WallShape::Cross => {
            (tile_min_x, tile_max_x, tile_min_z, tile_max_z)
        }
    };

    (
        [min_x, building_base_y(config), min_z],
        [max_x, building_top_y(config), max_z],
    )
}

fn building_base_y(config: &BuildingConfig) -> f32 {
    config.foundation_height.max(0.0)
}

fn building_top_y(config: &BuildingConfig) -> f32 {
    building_base_y(config) + config.wall_height
}

fn tile_xz_bounds(
    grid: &TileGrid,
    x: usize,
    y: usize,
    config: &BuildingConfig,
) -> (f32, f32, f32, f32) {
    let ts = config.tile_size;
    let min_x = grid.origin.x + x as f32 * ts;
    let min_z = grid.origin.y + y as f32 * ts;
    (min_x, min_x + ts, min_z, min_z + ts)
}

fn append_wall_box(
    wall_mesh: &mut MeshData,
    wall_top_mesh: &mut MeshData,
    exterior_wall_mesh: &mut MeshData,
    exterior_corner_mesh: &mut MeshData,
    exterior_t_junction_mesh: &mut MeshData,
    bounds: ([f32; 3], [f32; 3]),
    axis: WallAxis,
    exterior_class: ExteriorFaceClass,
    exterior_faces: &[WallFaceDir],
    config: &BuildingConfig,
    cutout: Option<WallCutout>,
) {
    let cutout_dirs: &[WallFaceDir] = match axis {
        WallAxis::X => &[WallFaceDir::NegZ, WallFaceDir::PosZ],
        WallAxis::Z => &[WallFaceDir::NegX, WallFaceDir::PosX],
        WallAxis::Both => &[
            WallFaceDir::NegX,
            WallFaceDir::PosX,
            WallFaceDir::NegZ,
            WallFaceDir::PosZ,
        ],
    };

    for dir in [
        WallFaceDir::NegX,
        WallFaceDir::PosX,
        WallFaceDir::NegZ,
        WallFaceDir::PosZ,
    ] {
        let face_cutout = if cutout_dirs.contains(&dir) {
            cutout
        } else {
            None
        };
        let mesh = match exterior_class {
            ExteriorFaceClass::Corner if !exterior_faces.is_empty() => &mut *exterior_corner_mesh,
            ExteriorFaceClass::TJunction if exterior_faces.contains(&dir) => {
                &mut *exterior_t_junction_mesh
            }
            ExteriorFaceClass::Straight if exterior_faces.contains(&dir) => {
                &mut *exterior_wall_mesh
            }
            _ => &mut *wall_mesh,
        };
        append_wall_face(mesh, bounds, dir, config, face_cutout);
    }
    append_wall_face(wall_top_mesh, bounds, WallFaceDir::PosY, config, None);
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
            let door_width = config.door_width.min(u_len);
            let door_start = (u_len - door_width) / 2.0;
            let door_end = door_start + door_width;
            let door_h = config.door_height.min(max_y - min_y);
            append_wall_sub_faces(
                mesh, tl, tr, bl, br, normal, u_len, v_len, door_start, door_end, 0.0, door_h,
            );
        }
        Some(WallCutout::Window) => {
            let window_width = config.window_width.min(u_len);
            let win_start = (u_len - window_width) / 2.0;
            let win_end = win_start + window_width;
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
// Foundation and floor mesh
// ---------------------------------------------------------------------------

fn generate_foundation_mesh(config: &BuildingConfig) -> MeshData {
    let mut mesh = MeshData::default();
    let width = config.foundation_width.max(0.0);
    if width <= f32::EPSILON {
        return mesh;
    }

    let offset = config.foundation_wall_offset.max(0.0);
    let top_y = config.foundation_height.max(0.0);
    let bottom_y = 0.0;
    let inner_min_x = config.footprint.min.x - offset;
    let inner_max_x = config.footprint.max.x + offset;
    let inner_min_z = config.footprint.min.y - offset;
    let inner_max_z = config.footprint.max.y + offset;
    let outer_min_x = inner_min_x - width;
    let outer_max_x = inner_max_x + width;
    let outer_min_z = inner_min_z - width;
    let outer_max_z = inner_max_z + width;

    append_foundation_quad(
        &mut mesh,
        outer_min_x,
        outer_max_x,
        outer_min_z,
        inner_min_z,
        top_y,
    );
    append_foundation_quad(
        &mut mesh,
        outer_min_x,
        outer_max_x,
        inner_max_z,
        outer_max_z,
        top_y,
    );
    append_foundation_quad(
        &mut mesh,
        outer_min_x,
        inner_min_x,
        inner_min_z,
        inner_max_z,
        top_y,
    );
    append_foundation_quad(
        &mut mesh,
        inner_max_x,
        outer_max_x,
        inner_min_z,
        inner_max_z,
        top_y,
    );
    append_foundation_sides(
        &mut mesh,
        outer_min_x,
        outer_max_x,
        outer_min_z,
        outer_max_z,
        bottom_y,
        top_y,
    );

    mesh
}

fn append_foundation_quad(
    mesh: &mut MeshData,
    min_x: f32,
    max_x: f32,
    min_z: f32,
    max_z: f32,
    y: f32,
) {
    if max_x <= min_x || max_z <= min_z {
        return;
    }

    append_quad(
        mesh,
        [min_x, y, max_z],
        [max_x, y, max_z],
        [min_x, y, min_z],
        [max_x, y, min_z],
        [0.0, 1.0, 0.0],
        [min_x, min_z],
        [max_x, max_z],
    );
}

fn append_foundation_sides(
    mesh: &mut MeshData,
    min_x: f32,
    max_x: f32,
    min_z: f32,
    max_z: f32,
    bottom_y: f32,
    top_y: f32,
) {
    append_quad(
        mesh,
        [min_x, top_y, min_z],
        [max_x, top_y, min_z],
        [min_x, bottom_y, min_z],
        [max_x, bottom_y, min_z],
        [0.0, 0.0, -1.0],
        [min_x, bottom_y],
        [max_x, top_y],
    );
    append_quad(
        mesh,
        [max_x, top_y, max_z],
        [min_x, top_y, max_z],
        [max_x, bottom_y, max_z],
        [min_x, bottom_y, max_z],
        [0.0, 0.0, 1.0],
        [min_x, bottom_y],
        [max_x, top_y],
    );
    append_quad(
        mesh,
        [min_x, top_y, max_z],
        [min_x, top_y, min_z],
        [min_x, bottom_y, max_z],
        [min_x, bottom_y, min_z],
        [-1.0, 0.0, 0.0],
        [min_z, bottom_y],
        [max_z, top_y],
    );
    append_quad(
        mesh,
        [max_x, top_y, min_z],
        [max_x, top_y, max_z],
        [max_x, bottom_y, min_z],
        [max_x, bottom_y, max_z],
        [1.0, 0.0, 0.0],
        [min_z, bottom_y],
        [max_z, top_y],
    );
}

fn generate_floor_mesh(grid: &TileGrid, config: &BuildingConfig) -> MeshData {
    let mut mesh = MeshData::default();
    let ts = config.tile_size;
    let origin_x = grid.origin.x;
    let origin_z = grid.origin.y;
    let floor_y = building_base_y(config);

    for y in 0..grid.height {
        for x in 0..grid.width {
            if !needs_floor_surface(grid.get(x, y)) {
                continue;
            }
            let x0 = origin_x + x as f32 * ts;
            let z0 = origin_z + y as f32 * ts;
            let x1 = x0 + ts;
            let z1 = z0 + ts;

            append_quad(
                &mut mesh,
                [x0, floor_y, z1],
                [x1, floor_y, z1],
                [x0, floor_y, z0],
                [x1, floor_y, z0],
                [0.0, 1.0, 0.0],
                [x0, z0],
                [x1, z1],
            );
        }
    }

    mesh
}

fn needs_floor_surface(tile: TileType) -> bool {
    match tile {
        TileType::Floor => true,
        TileType::Wall(wall) => wall.kind == WallKind::Interior,
        TileType::Empty => false,
    }
}

// ---------------------------------------------------------------------------
// Door mesh (thin slabs filling door openings)
// ---------------------------------------------------------------------------

fn generate_door_mesh(grid: &TileGrid, config: &BuildingConfig) -> MeshData {
    let mut mesh = MeshData::default();

    for y in 0..grid.height {
        for x in 0..grid.width {
            let TileType::Wall(wall) = grid.get(x, y) else {
                continue;
            };
            if !matches!(wall.opening, Some(WallOpening::Door { render_panel: true })) {
                continue;
            }

            let bounds = wall_bounds_for_tile(grid, x, y, wall, config);
            let [min_x, min_y, min_z] = bounds.0;
            let [max_x, _max_y, max_z] = bounds.1;
            let h = min_y + config.door_height;

            match wall.main_axis() {
                WallAxis::Z => {
                    let width = max_z - min_z;
                    let door_width = config.door_width.min(width);
                    let door_start = (width - door_width) / 2.0;
                    let ds = min_z + door_start;
                    let de = ds + door_width;
                    let cx = (min_x + max_x) / 2.0;
                    let t = max_x - min_x;

                    // +X face
                    append_quad(
                        &mut mesh,
                        [cx + t / 2.0, h, ds],
                        [cx + t / 2.0, h, de],
                        [cx + t / 2.0, min_y, ds],
                        [cx + t / 2.0, min_y, de],
                        [1.0, 0.0, 0.0],
                        [0.0, 0.0],
                        [door_width, h],
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
                        [door_width, h],
                    );
                }
                WallAxis::X | WallAxis::Both => {
                    let width = max_x - min_x;
                    let door_width = config.door_width.min(width);
                    let door_start = (width - door_width) / 2.0;
                    let ds = min_x + door_start;
                    let de = ds + door_width;
                    let cz = (min_z + max_z) / 2.0;
                    let t = max_z - min_z;

                    // +Z face
                    append_quad(
                        &mut mesh,
                        [ds, h, cz + t / 2.0],
                        [de, h, cz + t / 2.0],
                        [ds, min_y, cz + t / 2.0],
                        [de, min_y, cz + t / 2.0],
                        [0.0, 0.0, 1.0],
                        [0.0, 0.0],
                        [door_width, h],
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
                        [door_width, h],
                    );
                }
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
            let TileType::Wall(wall) = grid.get(x, y) else {
                continue;
            };
            if !matches!(
                wall.opening,
                Some(WallOpening::Window { render_glass: true })
            ) {
                continue;
            }

            let bounds = wall_bounds_for_tile(grid, x, y, wall, config);
            let [min_x, _min_y, min_z] = bounds.0;
            let [max_x, _max_y, max_z] = bounds.1;
            let sill = building_base_y(config) + config.window_sill_height;
            let wh = config.window_height;

            match wall.main_axis() {
                WallAxis::Z => {
                    let width = max_z - min_z;
                    let window_width = config.window_width.min(width);
                    let win_start = (width - window_width) / 2.0;
                    let ws = min_z + win_start;
                    let we = ws + window_width;
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
                        [window_width, wh],
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
                        [window_width, wh],
                    );
                }
                WallAxis::X | WallAxis::Both => {
                    let width = max_x - min_x;
                    let window_width = config.window_width.min(width);
                    let win_start = (width - window_width) / 2.0;
                    let ws = min_x + win_start;
                    let we = ws + window_width;
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
                        [window_width, wh],
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
                        [window_width, wh],
                    );
                }
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
    let wall_h = building_top_y(config);

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

    fn exterior_wall(shape: WallShape) -> TileType {
        TileType::Wall(WallTile::exterior(shape))
    }

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
    fn test_foundation_width_controls_mesh() {
        let config = BuildingConfig {
            foundation_width: 0.75,
            foundation_wall_offset: 0.25,
            ..Default::default()
        };
        let mesh = generate_foundation_mesh(&config);

        assert_eq!(mesh.vertices.len(), 32);
        assert_eq!(mesh.indices.len(), 48);

        let disabled = BuildingConfig {
            foundation_width: 0.0,
            ..config
        };
        assert!(generate_foundation_mesh(&disabled).is_empty());
    }

    #[test]
    fn test_tile_scale_wall() {
        let config = BuildingConfig::default();
        let (x, y, z) = tile_scale(
            exterior_wall(WallShape::Straight(CardinalDir::Top)),
            &config,
        );
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
            !bmesh.foundation_mesh.is_empty(),
            "foundation mesh should not be empty"
        );
        assert!(
            !bmesh.wall_top_mesh.is_empty(),
            "wall top mesh should not be empty"
        );
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
            ("foundation", &bmesh.foundation_mesh),
            ("wall", &bmesh.wall_mesh),
            ("wall_top", &bmesh.wall_top_mesh),
            ("exterior_wall", &bmesh.exterior_wall_mesh),
            ("exterior_corner", &bmesh.exterior_corner_mesh),
            ("exterior_t_junction", &bmesh.exterior_t_junction_mesh),
            ("floor", &bmesh.floor_mesh),
            ("roof", &bmesh.roof_mesh),
            ("door", &bmesh.door_mesh),
            ("window", &bmesh.window_mesh),
        ] {
            if data.is_empty() {
                continue;
            }

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
            let shape = if floor_x == 1 {
                WallShape::Straight(CardinalDir::Bottom)
            } else {
                WallShape::Straight(CardinalDir::Left)
            };

            plain_grid.set(1, 1, exterior_wall(shape));
            plain_grid.set(floor_x, floor_y, TileType::Floor);
            window_grid.set(
                1,
                1,
                exterior_wall(shape).wall().map_or(TileType::Empty, |wall| {
                    TileType::Wall(wall.with_opening(WallOpening::Window { render_glass: true }))
                }),
            );
            window_grid.set(floor_x, floor_y, TileType::Floor);

            let plain_meshes = generate_wall_meshes(&plain_grid, &config);
            let window_meshes = generate_wall_meshes(&window_grid, &config);
            let plain_vertices = wall_mesh_vertices(&plain_meshes);
            let window_vertices = wall_mesh_vertices(&window_meshes);

            // A window cuts the exterior and room-side wall faces. Each cut face
            // replaces one full quad with four sub-quads: 16 - 4 vertices.
            assert_eq!(window_vertices - plain_vertices, 2 * 12);
        }
    }

    fn wall_mesh_vertices(meshes: &WallMeshes) -> usize {
        meshes.wall.vertices.len()
            + meshes.top.vertices.len()
            + meshes.exterior.vertices.len()
            + meshes.exterior_corner.vertices.len()
            + meshes.exterior_t_junction.vertices.len()
    }

    #[test]
    fn test_interior_wall_uses_interior_thickness() {
        use crate::config::BuildingConfig;
        use crate::geometry::{Rect, Vec2};

        let config = BuildingConfig {
            footprint: Rect::new(0.0, 0.0, 3.0, 3.0),
            tile_size: 1.0,
            wall_thickness: 0.5,
            interior_wall_thickness: 0.25,
            ..Default::default()
        };

        let mut grid = TileGrid::new(3, 3, config.tile_size, Vec2::ZERO);
        let wall = WallTile::interior(WallShape::Straight(CardinalDir::Left));
        grid.set(1, 1, TileType::Wall(wall));
        grid.set(0, 1, TileType::Floor);
        grid.set(2, 1, TileType::Floor);

        let (min, max) = wall_bounds_for_tile(&grid, 1, 1, wall, &config);
        assert_eq!(max[0] - min[0], config.interior_wall_thickness);

        let wall = WallTile::exterior(WallShape::Straight(CardinalDir::Left));
        grid.set(1, 1, TileType::Wall(wall));
        let (min, max) = wall_bounds_for_tile(&grid, 1, 1, wall, &config);
        assert_eq!(max[0] - min[0], config.wall_thickness);
    }

    #[test]
    fn test_wall_corner_uses_full_tile_bounds() {
        use crate::config::BuildingConfig;
        use crate::geometry::{Rect, Vec2};

        let config = BuildingConfig {
            footprint: Rect::new(0.0, 0.0, 1.0, 1.0),
            tile_size: 1.0,
            wall_thickness: 0.4,
            interior_wall_thickness: 0.2,
            ..Default::default()
        };
        let mut grid = TileGrid::new(1, 1, config.tile_size, Vec2::ZERO);
        let wall = WallTile::exterior(WallShape::Corner(crate::tile::CornerDir::BottomLeft));
        grid.set(0, 0, TileType::Wall(wall));

        let (min, max) = wall_bounds_for_tile(&grid, 0, 0, wall, &config);
        assert_eq!(max[0] - min[0], config.tile_size);
        assert_eq!(max[2] - min[2], config.tile_size);
    }

    #[test]
    fn test_door_mesh_width_is_clamped_to_wall_segment() {
        use crate::config::BuildingConfig;
        use crate::geometry::{Rect, Vec2};

        let config = BuildingConfig {
            footprint: Rect::new(0.0, 0.0, 1.0, 3.0),
            tile_size: 0.5,
            door_width: 0.9,
            ..Default::default()
        };
        let mut grid = TileGrid::new(1, 3, config.tile_size, Vec2::ZERO);
        grid.set(
            0,
            1,
            exterior_wall(WallShape::Straight(CardinalDir::Bottom))
                .wall()
                .map_or(TileType::Empty, |wall| {
                    TileType::Wall(wall.with_opening(WallOpening::Door { render_panel: true }))
                }),
        );
        grid.set(0, 2, TileType::Floor);

        let mesh = generate_door_mesh(&grid, &config);
        let min_x = mesh
            .vertices
            .iter()
            .map(|v| v[0])
            .fold(f32::INFINITY, f32::min);
        let max_x = mesh
            .vertices
            .iter()
            .map(|v| v[0])
            .fold(f32::NEG_INFINITY, f32::max);

        assert!(min_x >= 0.0);
        assert!(max_x <= config.tile_size);
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
