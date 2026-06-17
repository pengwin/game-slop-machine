use super::math_util::append_quad;
use super::MeshData;
use crate::config::BuildingConfig;
use crate::tile::{
    CardinalDir, CornerDir, TJunctionDir, TileGrid, TileType, WallAxis, WallKind, WallOpening,
    WallShape, WallTile,
};

#[derive(Debug, Clone, Default)]
pub struct WallMeshes {
    pub wall: MeshData,
    pub top: MeshData,
    pub exterior: MeshData,
    pub exterior_corner: MeshData,
    pub exterior_t_junction: MeshData,
}

pub fn generate_wall_meshes(grid: &TileGrid, config: &BuildingConfig) -> WallMeshes {
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum WallFaceDir {
    NegX,
    PosX,
    NegZ,
    PosZ,
    PosY,
}

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

pub fn wall_bounds_for_tile(
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

pub fn building_base_y(config: &BuildingConfig) -> f32 {
    config.foundation_height.max(0.0)
}

pub fn building_top_y(config: &BuildingConfig) -> f32 {
    building_base_y(config) + config.wall_height
}

pub fn tile_xz_bounds(
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
                [max_x, max_y, max_z],
                [min_x, max_y, max_z],
                [max_x, min_y, max_z],
                [min_x, min_y, max_z],
            )
        }
        WallFaceDir::NegZ => {
            let n = [0.0, 0.0, -1.0];
            (
                n,
                [min_x, max_y, min_z],
                [max_x, max_y, min_z],
                [min_x, min_y, min_z],
                [max_x, min_y, min_z],
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

    let u_axis = super::math_util::sub3(tr, tl);
    let v_axis = super::math_util::sub3(bl, tl);

    let u_len = super::math_util::vec3_length(u_axis);
    let v_len = super::math_util::vec3_length(v_axis);

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

    let lerp_face = |u: f32, v: f32| -> [f32; 3] {
        let ut = if u_len > 0.0 { u / u_len } else { 0.0 };
        let vt = if v_len > 0.0 { v / v_len } else { 0.0 };
        let top = super::math_util::lerp3(tl, tr, ut);
        let bot = super::math_util::lerp3(bl, br, ut);
        super::math_util::lerp3(bot, top, vt)
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::geometry::{Rect, Vec2};

    fn exterior_wall(shape: WallShape) -> TileType {
        TileType::Wall(WallTile::exterior(shape))
    }

    #[test]
    fn test_window_tiles_cut_both_wall_faces_for_each_orientation() {
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
    fn test_wall_triangle_winding_matches_normals() {
        let config = BuildingConfig {
            footprint: Rect::new(0.0, 0.0, 3.0, 3.0),
            tile_size: 1.0,
            wall_thickness: 0.25,
            ..Default::default()
        };

        for shape in [
            WallShape::Straight(CardinalDir::Left),
            WallShape::Straight(CardinalDir::Right),
            WallShape::Straight(CardinalDir::Bottom),
            WallShape::Straight(CardinalDir::Top),
        ] {
            let mut grid = TileGrid::new(3, 3, config.tile_size, Vec2::ZERO);
            grid.set(1, 1, exterior_wall(shape));

            let meshes = generate_wall_meshes(&grid, &config);
            for (name, mesh) in [
                ("wall", &meshes.wall),
                ("top", &meshes.top),
                ("exterior", &meshes.exterior),
                ("corner", &meshes.exterior_corner),
                ("t_junction", &meshes.exterior_t_junction),
            ] {
                assert_mesh_winding_matches_normals(name, mesh);
            }
        }
    }

    fn assert_mesh_winding_matches_normals(name: &str, mesh: &MeshData) {
        for triangle in mesh.indices.chunks_exact(3) {
            let a_index = triangle[0] as usize;
            let b_index = triangle[1] as usize;
            let c_index = triangle[2] as usize;
            let a = mesh.vertices[a_index];
            let b = mesh.vertices[b_index];
            let c = mesh.vertices[c_index];
            let normal = mesh.normals[a_index];
            let edge_ab = super::super::math_util::sub3(b, a);
            let edge_ac = super::super::math_util::sub3(c, a);
            let winding_normal = super::super::math_util::normalize3(
                super::super::math_util::cross3(edge_ab, edge_ac),
            );
            let dot = winding_normal[0] * normal[0]
                + winding_normal[1] * normal[1]
                + winding_normal[2] * normal[2];

            assert!(
                dot > 0.99,
                "{name}: triangle winding opposes normal: dot={dot}, normal={normal:?}, winding_normal={winding_normal:?}"
            );
        }
    }
}
