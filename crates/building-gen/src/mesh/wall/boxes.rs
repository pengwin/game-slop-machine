use super::bounds::{building_base_y, building_top_y, tile_xz_bounds, wall_bounds_for_tile};
use super::classify::{ExteriorFaceClass, WallCutout, exterior_face_class, opening_cutout};
use crate::config::BuildingConfig;
use crate::tile::{CornerDir, TJunctionDir, TileGrid, WallAxis, WallKind, WallShape, WallTile};

#[derive(Debug, Clone, Copy)]
pub struct WallBox {
    pub bounds: ([f32; 3], [f32; 3]),
    pub axis: WallAxis,
    pub exterior_class: ExteriorFaceClass,
    pub cutout: Option<WallCutout>,
    pub cull_neighbor_faces: bool,
    pub visible_side_faces: Option<[bool; 4]>,
}

pub fn wall_boxes(
    grid: &TileGrid,
    x: usize,
    y: usize,
    wall: WallTile,
    config: &BuildingConfig,
) -> Vec<WallBox> {
    if wall.kind == WallKind::Interior
        && let Some(boxes) = interior_connector_boxes(grid, x, y, wall, config)
    {
        return boxes;
    }

    let bounds = wall_bounds_for_tile(grid, x, y, wall, config);
    vec![WallBox {
        bounds,
        axis: wall.main_axis(),
        exterior_class: exterior_face_class(wall),
        cutout: opening_cutout(wall.opening),
        cull_neighbor_faces: true,
        visible_side_faces: None,
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
        cull_neighbor_faces: false,
        visible_side_faces: Some([!dirs.left, !dirs.right, !dirs.bottom, !dirs.top]),
    });

    if dirs.left && tile_min_x < min_x {
        boxes.push(connector_wall_box(
            [tile_min_x, building_base_y(config), min_z],
            [min_x, building_top_y(config), max_z],
            WallAxis::X,
            wall,
            cutout,
            Some([false, false, true, true]),
        ));
    }
    if dirs.right && max_x < tile_max_x {
        boxes.push(connector_wall_box(
            [max_x, building_base_y(config), min_z],
            [tile_max_x, building_top_y(config), max_z],
            WallAxis::X,
            wall,
            cutout,
            Some([false, false, true, true]),
        ));
    }
    if dirs.bottom && tile_min_z < min_z {
        boxes.push(connector_wall_box(
            [min_x, building_base_y(config), tile_min_z],
            [max_x, building_top_y(config), min_z],
            WallAxis::Z,
            wall,
            cutout,
            Some([true, true, false, false]),
        ));
    }
    if dirs.top && max_z < tile_max_z {
        boxes.push(connector_wall_box(
            [min_x, building_base_y(config), max_z],
            [max_x, building_top_y(config), tile_max_z],
            WallAxis::Z,
            wall,
            cutout,
            Some([true, true, false, false]),
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
    visible_side_faces: Option<[bool; 4]>,
) -> WallBox {
    WallBox {
        bounds: (min, max),
        axis,
        exterior_class: exterior_face_class(wall),
        cutout,
        cull_neighbor_faces: false,
        visible_side_faces,
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
