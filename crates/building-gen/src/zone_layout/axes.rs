use crate::config::BuildingConfig;
use crate::geometry::{Rect, Vec2};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DepthAxis {
    X,
    Y,
}

#[derive(Debug, Clone, Copy)]
pub struct LayoutAxes {
    pub depth_axis: DepthAxis,
    pub depth_start: f32,
    pub depth_end: f32,
    pub width_start: f32,
    pub width_end: f32,
}

pub fn determine_depth_axis(entrance_dir: Vec2) -> DepthAxis {
    if entrance_dir.y.abs() >= entrance_dir.x.abs() {
        DepthAxis::Y
    } else {
        DepthAxis::X
    }
}

pub fn determine_layout_axes(config: &BuildingConfig) -> LayoutAxes {
    let fp = config.footprint;
    let depth_axis = determine_depth_axis(config.entrance_dir);
    let (depth_start, depth_end) = match depth_axis {
        DepthAxis::Y if config.entrance_dir.y < 0.0 => (fp.max.y, fp.min.y),
        DepthAxis::Y => (fp.min.y, fp.max.y),
        DepthAxis::X if config.entrance_dir.x < 0.0 => (fp.max.x, fp.min.x),
        DepthAxis::X => (fp.min.x, fp.max.x),
    };
    let (width_start, width_end) = match depth_axis {
        DepthAxis::Y => (fp.min.x, fp.max.x),
        DepthAxis::X => (fp.min.y, fp.max.y),
    };

    LayoutAxes {
        depth_axis,
        depth_start,
        depth_end,
        width_start,
        width_end,
    }
}

pub fn entrance_width_coord(config: &BuildingConfig, axes: LayoutAxes) -> f32 {
    match axes.depth_axis {
        DepthAxis::Y => config.entrance.x,
        DepthAxis::X => config.entrance.y,
    }
}

pub fn make_rect(
    depth_axis: DepthAxis,
    depth_min: f32,
    depth_max: f32,
    width_min: f32,
    width_max: f32,
) -> Rect {
    match depth_axis {
        DepthAxis::Y => Rect::new(width_min, depth_min, width_max, depth_max),
        DepthAxis::X => Rect::new(depth_min, width_min, depth_max, width_max),
    }
}
