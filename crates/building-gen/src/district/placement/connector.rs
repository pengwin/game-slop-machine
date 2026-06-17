use super::super::layout::{Lot, RoadSegment};
use crate::geometry::Vec2;

pub fn connector_road_for_lot(
    lot: &Lot,
    all_roads: &[RoadSegment],
    road_width: f32,
) -> Option<RoadSegment> {
    super::super::road::connector_road_from_entrance(
        lot.entrance,
        Vec2::new(-lot.entrance_dir.x, -lot.entrance_dir.y),
        all_roads,
        road_width,
    )
}
