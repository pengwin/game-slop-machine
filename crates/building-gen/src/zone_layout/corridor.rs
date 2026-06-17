use super::axes::{LayoutAxes, entrance_width_coord, make_rect};
use super::rows::room_row_ranges;
use super::splits::{distribute_weighted_splits, snap_to_grid, sorted_pair};
use crate::config::{BuildingConfig, RoomSpec};
use crate::geometry::Rect;
use crate::layout::Room;

#[derive(Debug, Clone)]
pub struct CorridorInfo {
    pub bounds: Rect,
}

pub fn generate_corridor_rooms(
    config: &BuildingConfig,
    axes: LayoutAxes,
    specs: &[&RoomSpec],
) -> (Vec<Room>, Option<CorridorInfo>) {
    let ts = config.tile_size;

    let (corridor_min, corridor_max) = corridor_width_range(config, axes);
    let side_ranges = [
        (axes.width_start, corridor_min),
        (corridor_max, axes.width_end),
    ];

    let row_ranges = room_row_ranges(specs.len(), 2);
    let row_weights: Vec<f32> = row_ranges
        .iter()
        .map(|range| super::rows::preferred_area_sum(&specs[range.clone()]))
        .collect();
    let depth_splits =
        distribute_weighted_splits(axes.depth_start, axes.depth_end, &row_weights, ts);

    let mut rooms = Vec::new();
    let mut spec_idx = 0;

    for row in 0..row_ranges.len() {
        let (row_min, row_max) = sorted_pair(depth_splits[row], depth_splits[row + 1]);

        for (side_min, side_max) in side_ranges {
            if spec_idx >= specs.len() {
                break;
            }
            if side_max - side_min < ts {
                continue;
            }

            let spec = &specs[spec_idx];
            let bounds = make_rect(axes.depth_axis, row_min, row_max, side_min, side_max);
            rooms.push(Room::new(spec_idx as u32, bounds, &spec.name));
            spec_idx += 1;
        }
    }

    let (corridor_depth_min, corridor_depth_max) = sorted_pair(axes.depth_start, axes.depth_end);
    let corridor_bounds = make_rect(
        axes.depth_axis,
        corridor_depth_min,
        corridor_depth_max,
        corridor_min,
        corridor_max,
    );

    (
        rooms,
        Some(CorridorInfo {
            bounds: corridor_bounds,
        }),
    )
}

fn corridor_width_range(config: &BuildingConfig, axes: LayoutAxes) -> (f32, f32) {
    let total_width = axes.width_end - axes.width_start;
    let desired =
        snap_to_grid(config.corridor_width_world(), config.tile_size).max(config.tile_size);
    let max_with_rooms = total_width - config.min_room_size * 2.0;
    let corridor_width = if max_with_rooms >= config.tile_size {
        desired.min(max_with_rooms)
    } else {
        desired
            .min(total_width)
            .max(config.tile_size.min(total_width))
    };

    let requested_center =
        entrance_width_coord(config, axes).clamp(axes.width_start, axes.width_end);
    let half = corridor_width / 2.0;
    let centered_min = requested_center - half;
    let centered_max = requested_center + half;

    let min_with_room_space = axes.width_start + config.min_room_size;
    let max_with_room_space = axes.width_end - config.min_room_size;
    let (corridor_min, corridor_max) =
        if min_with_room_space + corridor_width <= max_with_room_space {
            let min = centered_min.clamp(min_with_room_space, max_with_room_space - corridor_width);
            (min, min + corridor_width)
        } else {
            let min = centered_min.clamp(axes.width_start, axes.width_end - corridor_width);
            let max = centered_max.clamp(axes.width_start + corridor_width, axes.width_end);
            (min, max)
        };

    (
        snap_to_grid(corridor_min, config.tile_size),
        snap_to_grid(corridor_max, config.tile_size),
    )
}
