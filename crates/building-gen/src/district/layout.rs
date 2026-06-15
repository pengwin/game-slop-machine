use crate::geometry::{Rect, Vec2};

/// Output of trade district generation: lots, roads, and metadata.
#[derive(Debug, Clone)]
pub struct TradeDistrictLayout {
    pub lots: Vec<Lot>,
    pub roads: Vec<RoadSegment>,
    pub town_square_center: Vec2,
    pub bounds: Rect,
}

/// A lot placed in the trade district.
#[derive(Debug, Clone)]
pub struct Lot {
    /// Center of the lot in world space.
    pub position: Vec2,
    /// Width of the lot (along local X axis).
    pub width: f32,
    /// Depth of the lot (along local Z axis, toward center).
    pub depth: f32,
    /// Y-axis rotation in radians.
    pub rotation: f32,
    /// Entrance point (center of the center-facing side).
    pub entrance: Vec2,
    /// Unit vector pointing INTO the lot from the entrance.
    pub entrance_dir: Vec2,
}

/// A road segment connecting two points.
#[derive(Debug, Clone, Copy)]
pub struct RoadSegment {
    pub start: Vec2,
    pub end: Vec2,
    pub width: f32,
}
