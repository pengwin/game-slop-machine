use super::axes::{LayoutAxes, make_rect};
use super::splits::{distribute_weighted_splits, sorted_pair};
use crate::config::{BuildingConfig, RoomSpec};
use crate::layout::Room;

pub fn generate_row_rooms(
    config: &BuildingConfig,
    axes: LayoutAxes,
    specs: &[&RoomSpec],
) -> (Vec<Room>, Option<super::corridor::CorridorInfo>) {
    let rooms_per_row = super::scoring::balanced_rooms_per_row(config, axes, specs.len());
    generate_row_rooms_with_columns(config, axes, specs, rooms_per_row)
}

pub fn generate_row_rooms_with_columns(
    config: &BuildingConfig,
    axes: LayoutAxes,
    specs: &[&RoomSpec],
    rooms_per_row: usize,
) -> (Vec<Room>, Option<super::corridor::CorridorInfo>) {
    let ts = config.tile_size;
    let rooms_per_row = rooms_per_row.clamp(1, super::scoring::max_rooms_per_row(config, axes, specs.len()));

    let row_ranges = room_row_ranges(specs.len(), rooms_per_row);
    let row_weights: Vec<f32> = row_ranges
        .iter()
        .map(|range| preferred_area_sum(&specs[range.clone()]))
        .collect();

    let depth_splits =
        distribute_weighted_splits(axes.depth_start, axes.depth_end, &row_weights, ts);

    let mut rooms = Vec::new();
    let mut room_id = 0u32;

    for (row, range) in row_ranges.iter().enumerate() {
        let (row_min, row_max) = sorted_pair(depth_splits[row], depth_splits[row + 1]);
        let row_specs = &specs[range.clone()];

        let width_weights: Vec<f32> = row_specs
            .iter()
            .map(|spec| room_area_weight(spec))
            .collect();
        let width_splits =
            distribute_weighted_splits(axes.width_start, axes.width_end, &width_weights, ts);

        for (col, spec) in row_specs.iter().enumerate() {
            let col_min = width_splits[col];
            let col_max = width_splits[col + 1];

            let bounds = make_rect(axes.depth_axis, row_min, row_max, col_min, col_max);
            let label = &spec.name;
            rooms.push(Room::new(room_id, bounds, label));
            room_id += 1;
        }
    }

    (rooms, None)
}

pub fn room_row_ranges(room_count: usize, rooms_per_row: usize) -> Vec<std::ops::Range<usize>> {
    let mut ranges = Vec::new();
    let mut start = 0;
    while start < room_count {
        let end = (start + rooms_per_row).min(room_count);
        ranges.push(start..end);
        start = end;
    }
    ranges
}

pub fn preferred_area_sum(specs: &[&RoomSpec]) -> f32 {
    specs.iter().map(|spec| room_area_weight(spec)).sum()
}

pub fn room_area_weight(spec: &RoomSpec) -> f32 {
    spec.preferred_area.max(spec.min_area).max(1.0)
}
