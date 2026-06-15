//! Zone-row layout algorithm for generating room layouts.
//!
//! Instead of random BSP subdivision, this algorithm:
//! 1. Determines depth/width axes from `entrance_dir`
//! 2. Splits the building into rows along the depth axis (entrance → back)
//! 3. Subdivides each row into rooms along the width axis
//! 4. Optionally adds a center corridor with rooms on both sides
//!
//! Room order matters: first rooms in `room_specs` are near the entrance,
//! last rooms are deepest in the building.

use crate::config::{BuildingConfig, RoomSpec};
use crate::geometry::{Rect, Vec2};
use crate::layout::Room;

/// Information about the center corridor (when `has_corridor = true`).
#[derive(Debug, Clone)]
pub struct CorridorInfo {
    /// The corridor strip bounds (floor tiles).
    pub bounds: Rect,
}

/// Which axis runs from entrance to back of building.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DepthAxis {
    X,
    Y,
}

#[derive(Debug, Clone, Copy)]
struct LayoutAxes {
    depth_axis: DepthAxis,
    depth_start: f32,
    depth_end: f32,
    width_start: f32,
    width_end: f32,
}

#[derive(Debug, Clone)]
struct LayoutCandidate {
    rooms: Vec<Room>,
    corridor: Option<CorridorInfo>,
    score: f32,
}

/// Generates rooms using the zone-row algorithm.
///
/// Returns `(rooms, optional_corridor_info)`.
pub fn generate_rooms(config: &BuildingConfig) -> (Vec<Room>, Option<CorridorInfo>) {
    let specs = ordered_room_specs(config);

    if specs.is_empty() {
        return (vec![Room::new(0, config.footprint, "room")], None);
    }

    if specs.len() == 1 {
        return (vec![Room::new(0, config.footprint, &specs[0].name)], None);
    }

    let axes = determine_layout_axes(config);

    choose_best_layout(config, axes, &specs)
}

/// Computes the world position of the entrance door.
pub fn entrance_door_position(config: &BuildingConfig) -> Vec2 {
    let fp = config.footprint;
    let entrance = config.entrance;

    if config.entrance_dir.y.abs() >= config.entrance_dir.x.abs() {
        let y = if config.entrance_dir.y >= 0.0 {
            fp.min.y
        } else {
            fp.max.y
        };
        Vec2::new(entrance.x.clamp(fp.min.x, fp.max.x), y)
    } else {
        let x = if config.entrance_dir.x >= 0.0 {
            fp.min.x
        } else {
            fp.max.x
        };
        Vec2::new(x, entrance.y.clamp(fp.min.y, fp.max.y))
    }
}

fn determine_depth_axis(entrance_dir: Vec2) -> DepthAxis {
    if entrance_dir.y.abs() >= entrance_dir.x.abs() {
        DepthAxis::Y
    } else {
        DepthAxis::X
    }
}

fn determine_layout_axes(config: &BuildingConfig) -> LayoutAxes {
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

// ── Non-corridor mode ────────────────────────────────────────────────────────

fn generate_row_rooms(
    config: &BuildingConfig,
    axes: LayoutAxes,
    specs: &[&RoomSpec],
) -> (Vec<Room>, Option<CorridorInfo>) {
    let rooms_per_row = balanced_rooms_per_row(config, axes, specs.len());
    generate_row_rooms_with_columns(config, axes, specs, rooms_per_row)
}

fn generate_row_rooms_with_columns(
    config: &BuildingConfig,
    axes: LayoutAxes,
    specs: &[&RoomSpec],
    rooms_per_row: usize,
) -> (Vec<Room>, Option<CorridorInfo>) {
    let ts = config.tile_size;
    let rooms_per_row = rooms_per_row.clamp(1, max_rooms_per_row(config, axes, specs.len()));

    let row_ranges = room_row_ranges(specs.len(), rooms_per_row);
    let row_weights: Vec<f32> = row_ranges
        .iter()
        .map(|range| preferred_area_sum(&specs[range.clone()]))
        .collect();

    // Depth splits (row boundaries)
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

// ── Corridor mode ────────────────────────────────────────────────────────────

/// Interior corridor running from entrance to back, with rooms on both sides.
///
/// ```text
/// ┌───────┬──────┬───────┐
/// │ room3 │ corr │ room4 │
/// ├───────┤      ├───────┤
/// │ room1 │      │ room2 │
/// └───────┴──────┴───────┘
/// ```
fn generate_corridor_rooms(
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
        .map(|range| preferred_area_sum(&specs[range.clone()]))
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

    // Corridor bounds — full depth
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

// ── Helpers ──────────────────────────────────────────────────────────────────

fn choose_best_layout(
    config: &BuildingConfig,
    axes: LayoutAxes,
    specs: &[&RoomSpec],
) -> (Vec<Room>, Option<CorridorInfo>) {
    let mut candidates = Vec::new();

    if !config.has_corridor {
        let max_columns = max_rooms_per_row(config, axes, specs.len());
        let balanced = balanced_rooms_per_row(config, axes, specs.len());
        for columns in candidate_column_counts(max_columns, balanced) {
            let (rooms, corridor) = generate_row_rooms_with_columns(config, axes, specs, columns);
            candidates.push(scored_candidate(config, specs, rooms, corridor));
        }
    }

    if config.has_corridor || config.auto_corridor {
        let (rooms, corridor) = generate_corridor_rooms(config, axes, specs);
        candidates.push(scored_candidate(config, specs, rooms, corridor));
    }

    candidates
        .into_iter()
        .max_by(|a, b| a.score.total_cmp(&b.score))
        .map(|candidate| (candidate.rooms, candidate.corridor))
        .unwrap_or_else(|| generate_row_rooms(config, axes, specs))
}

fn scored_candidate(
    config: &BuildingConfig,
    specs: &[&RoomSpec],
    rooms: Vec<Room>,
    corridor: Option<CorridorInfo>,
) -> LayoutCandidate {
    let score = score_layout(config, specs, &rooms, corridor.as_ref());
    LayoutCandidate {
        rooms,
        corridor,
        score,
    }
}

fn candidate_column_counts(max_columns: usize, balanced: usize) -> Vec<usize> {
    let mut counts = vec![balanced];
    if balanced > 1 {
        counts.push(balanced - 1);
    }
    if balanced < max_columns {
        counts.push(balanced + 1);
    }
    counts.push(1);
    counts.push(max_columns);
    counts.sort_unstable();
    counts.dedup();
    counts
}

fn max_rooms_per_row(config: &BuildingConfig, axes: LayoutAxes, room_count: usize) -> usize {
    let width = axes.width_end - axes.width_start;
    ((width / config.min_room_size).floor() as usize)
        .max(1)
        .min(room_count)
}

fn balanced_rooms_per_row(config: &BuildingConfig, axes: LayoutAxes, room_count: usize) -> usize {
    let balanced_columns = (room_count as f32).sqrt().ceil() as usize;
    balanced_columns.clamp(1, max_rooms_per_row(config, axes, room_count))
}

fn score_layout(
    config: &BuildingConfig,
    specs: &[&RoomSpec],
    rooms: &[Room],
    corridor: Option<&CorridorInfo>,
) -> f32 {
    let mut score = 100.0;

    for (room, spec) in rooms.iter().zip(specs.iter()) {
        score -= area_penalty(room, spec);
        score -= aspect_ratio_penalty(room, spec);

        if spec.exterior_required && !touches_exterior(room.bounds, config.footprint) {
            score -= 40.0;
        }
    }

    if let Some(corridor) = corridor {
        let corridor_area_fraction = corridor.bounds.area() / config.footprint.area().max(1.0);
        score -= corridor_area_fraction * 25.0;

        if specs.len() <= 3 {
            score -= 20.0;
        } else if specs.len() >= 5 || config.has_corridor {
            score += 12.0;
        }
    } else if config.auto_corridor && specs.len() >= 6 {
        score -= 12.0;
    }

    score
}

fn area_penalty(room: &Room, spec: &RoomSpec) -> f32 {
    let preferred = spec.preferred_area.max(1.0);
    let ratio = (room.bounds.area() - preferred).abs() / preferred;
    ratio.min(2.0) * 12.0
}

fn aspect_ratio_penalty(room: &Room, spec: &RoomSpec) -> f32 {
    let width = room.bounds.width().max(0.01);
    let height = room.bounds.height().max(0.01);
    let ratio = (width / height).max(height / width);
    let target = if spec.preferred_area <= 5.0 { 3.0 } else { 2.2 };

    (ratio - target).max(0.0) * 10.0
}

fn touches_exterior(bounds: Rect, footprint: Rect) -> bool {
    let eps = 0.01;
    (bounds.min.x - footprint.min.x).abs() < eps
        || (bounds.max.x - footprint.max.x).abs() < eps
        || (bounds.min.y - footprint.min.y).abs() < eps
        || (bounds.max.y - footprint.max.y).abs() < eps
}

fn ordered_room_specs(config: &BuildingConfig) -> Vec<&RoomSpec> {
    let mut specs: Vec<_> = config.room_specs.iter().collect();
    specs.sort_by_key(|spec| spec.placement);
    specs
}

fn room_row_ranges(room_count: usize, rooms_per_row: usize) -> Vec<std::ops::Range<usize>> {
    let mut ranges = Vec::new();
    let mut start = 0;
    while start < room_count {
        let end = (start + rooms_per_row).min(room_count);
        ranges.push(start..end);
        start = end;
    }
    ranges
}

fn preferred_area_sum(specs: &[&RoomSpec]) -> f32 {
    specs.iter().map(|spec| room_area_weight(spec)).sum()
}

fn room_area_weight(spec: &RoomSpec) -> f32 {
    spec.preferred_area.max(spec.min_area).max(1.0)
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

fn entrance_width_coord(config: &BuildingConfig, axes: LayoutAxes) -> f32 {
    match axes.depth_axis {
        DepthAxis::Y => config.entrance.x,
        DepthAxis::X => config.entrance.y,
    }
}

/// Constructs a Rect from axis-aligned values.
fn make_rect(
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

/// Distributes `count` intervals evenly between `start` and `end`,
/// snapping internal split positions to the tile grid.
///
/// Returns `count + 1` positions: [start, split1, ..., splitN, end].
fn distribute_splits(start: f32, end: f32, count: usize, tile_size: f32) -> Vec<f32> {
    if count <= 1 {
        return vec![start, end];
    }

    let total = end - start;
    let step = total / count as f32;
    let dir = if total >= 0.0 { 1.0 } else { -1.0 };

    let mut splits = Vec::with_capacity(count + 1);
    splits.push(start);

    for i in 1..count {
        let raw = start + i as f32 * step;
        splits.push(snap_to_grid(raw, tile_size));
    }

    splits.push(end);

    // Ensure monotonicity (no degenerate rooms)
    for i in 1..splits.len() - 1 {
        if dir > 0.0 {
            let min_pos = splits[i - 1] + tile_size;
            let max_pos = splits[i + 1] - tile_size;
            if splits[i] < min_pos {
                splits[i] = min_pos;
            }
            if splits[i] > max_pos {
                splits[i] = max_pos;
            }
        } else {
            let max_pos = splits[i - 1] - tile_size;
            let min_pos = splits[i + 1] + tile_size;
            if splits[i] > max_pos {
                splits[i] = max_pos;
            }
            if splits[i] < min_pos {
                splits[i] = min_pos;
            }
        }
    }

    splits
}

fn distribute_weighted_splits(start: f32, end: f32, weights: &[f32], tile_size: f32) -> Vec<f32> {
    if weights.len() <= 1 {
        return vec![start, end];
    }

    let total_weight: f32 = weights.iter().sum();
    if total_weight <= f32::EPSILON {
        return distribute_splits(start, end, weights.len(), tile_size);
    }

    let total = end - start;
    let mut splits = Vec::with_capacity(weights.len() + 1);
    let mut accumulated = 0.0;
    splits.push(start);

    for weight in weights.iter().take(weights.len() - 1) {
        accumulated += *weight;
        let raw = start + total * (accumulated / total_weight);
        splits.push(snap_to_grid(raw, tile_size));
    }

    splits.push(end);
    enforce_minimum_split_spacing(&mut splits, tile_size);
    splits
}

fn enforce_minimum_split_spacing(splits: &mut [f32], tile_size: f32) {
    if splits.len() <= 2 {
        return;
    }

    let dir = if splits[splits.len() - 1] >= splits[0] {
        1.0
    } else {
        -1.0
    };

    for i in 1..splits.len() - 1 {
        if dir > 0.0 {
            let min_pos = splits[i - 1] + tile_size;
            let max_pos = splits[i + 1] - tile_size;
            if splits[i] < min_pos {
                splits[i] = min_pos;
            }
            if splits[i] > max_pos {
                splits[i] = max_pos;
            }
        } else {
            let max_pos = splits[i - 1] - tile_size;
            let min_pos = splits[i + 1] + tile_size;
            if splits[i] > max_pos {
                splits[i] = max_pos;
            }
            if splits[i] < min_pos {
                splits[i] = min_pos;
            }
        }
    }
}

fn snap_to_grid(value: f32, tile_size: f32) -> f32 {
    (value / tile_size).round() * tile_size
}

fn sorted_pair(a: f32, b: f32) -> (f32, f32) {
    if a <= b {
        (a, b)
    } else {
        (b, a)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::RoomSpec;

    fn default_config() -> BuildingConfig {
        BuildingConfig {
            footprint: Rect::new(0.0, 0.0, 10.0, 8.0),
            tile_size: 0.5,
            min_room_size: 2.5,
            room_specs: vec![
                RoomSpec::new("hall", 1),
                RoomSpec::new("kitchen", 2),
                RoomSpec::new("bedroom", 1),
                RoomSpec::new("bathroom", 0),
            ],
            ..Default::default()
        }
    }

    #[test]
    fn test_single_room() {
        let config = BuildingConfig::default();
        let (rooms, corridor) = generate_rooms(&config);
        assert_eq!(rooms.len(), 1);
        assert_eq!(rooms[0].label, "room");
        assert!(corridor.is_none());
    }

    #[test]
    fn test_room_count_matches_specs() {
        let config = default_config();
        let (rooms, _) = generate_rooms(&config);
        assert_eq!(rooms.len(), config.room_specs.len());
    }

    #[test]
    fn test_labels_assigned_in_order() {
        let config = default_config();
        let (rooms, _) = generate_rooms(&config);
        assert_eq!(rooms[0].label, "hall");
        assert_eq!(rooms[1].label, "kitchen");
        assert_eq!(rooms[2].label, "bathroom");
        assert_eq!(rooms[3].label, "bedroom");
    }

    #[test]
    fn test_preferred_area_controls_room_size() {
        let config = BuildingConfig {
            footprint: Rect::new(0.0, 0.0, 12.0, 8.0),
            tile_size: 0.5,
            room_specs: vec![
                RoomSpec::new("bathroom", 0).with_area(3.0, 4.0),
                RoomSpec::new("kitchen", 1).with_area(8.0, 18.0),
            ],
            ..Default::default()
        };
        let (rooms, _) = generate_rooms(&config);
        let bathroom = rooms.iter().find(|room| room.label == "bathroom").unwrap();
        let kitchen = rooms.iter().find(|room| room.label == "kitchen").unwrap();

        assert!(kitchen.bounds.area() > bathroom.bounds.area());
    }

    #[test]
    fn test_candidate_scoring_avoids_skinny_room_strip() {
        let config = BuildingConfig {
            footprint: Rect::new(0.0, 0.0, 12.0, 8.0),
            tile_size: 0.5,
            room_specs: vec![
                RoomSpec::new("hall", 0),
                RoomSpec::new("kitchen", 1),
                RoomSpec::new("bathroom", 0),
                RoomSpec::new("bedroom", 1),
            ],
            ..Default::default()
        };
        let (rooms, corridor) = generate_rooms(&config);

        assert!(corridor.is_none());
        assert!(rooms.iter().all(|room| {
            let ratio = (room.bounds.width() / room.bounds.height())
                .max(room.bounds.height() / room.bounds.width());
            ratio <= 2.5
        }));
    }

    #[test]
    fn test_auto_corridor_chooses_corridor_for_large_program() {
        let config = BuildingConfig {
            footprint: Rect::new(0.0, 0.0, 14.0, 10.0),
            tile_size: 0.5,
            auto_corridor: true,
            room_specs: vec![
                RoomSpec::new("hall", 0),
                RoomSpec::new("kitchen", 1),
                RoomSpec::new("bathroom", 0),
                RoomSpec::new("bedroom", 1),
                RoomSpec::new("bedroom", 1),
                RoomSpec::new("storage", 0),
            ],
            ..Default::default()
        };
        let (_, corridor) = generate_rooms(&config);

        assert!(corridor.is_some());
    }

    #[test]
    fn test_rooms_fill_footprint() {
        let config = default_config();
        let (rooms, _) = generate_rooms(&config);

        // All rooms should be within the footprint
        for room in &rooms {
            assert!(room.bounds.min.x >= config.footprint.min.x - 0.01);
            assert!(room.bounds.min.y >= config.footprint.min.y - 0.01);
            assert!(room.bounds.max.x <= config.footprint.max.x + 0.01);
            assert!(room.bounds.max.y <= config.footprint.max.y + 0.01);
        }
    }

    #[test]
    fn test_rooms_no_overlap() {
        let config = default_config();
        let (rooms, _) = generate_rooms(&config);

        for i in 0..rooms.len() {
            for j in (i + 1)..rooms.len() {
                assert!(
                    !rooms[i].bounds.intersects(rooms[j].bounds),
                    "Rooms {} ({}) and {} ({}) overlap",
                    i,
                    rooms[i].label,
                    j,
                    rooms[j].label,
                );
            }
        }
    }

    #[test]
    fn test_corridor_mode_odd_rooms() {
        let config = BuildingConfig {
            footprint: Rect::new(0.0, 0.0, 10.0, 8.0),
            tile_size: 0.5,
            min_room_size: 2.5,
            room_specs: vec![
                RoomSpec::new("a", 0),
                RoomSpec::new("b", 0),
                RoomSpec::new("c", 0),
                RoomSpec::new("d", 0),
                RoomSpec::new("e", 0),
            ],
            has_corridor: true,
            corridor_width: 1.0,
            ..Default::default()
        };
        let (rooms, corridor) = generate_rooms(&config);

        assert_eq!(rooms.len(), 5);
        assert!(corridor.is_some());

        assert_eq!(rooms[0].label, "a");
        assert_eq!(rooms[1].label, "b");
        assert_eq!(rooms[2].label, "c");
        assert_eq!(rooms[3].label, "d");
        assert_eq!(rooms[4].label, "e");

        let cb = corridor.unwrap().bounds;
        assert!(cb.min.x > config.footprint.min.x);
        assert!(cb.max.x < config.footprint.max.x);
        assert!((cb.height() - 8.0).abs() < 0.1);

        assert!(rooms.iter().any(|room| room.bounds.max.x <= cb.min.x));
        assert!(rooms.iter().any(|room| room.bounds.min.x >= cb.max.x));

        let room_width = rooms[0].bounds.width();
        for room in &rooms {
            assert!((room.bounds.width() - room_width).abs() < 0.1);
        }
    }

    #[test]
    fn test_corridor_mode_even_rooms() {
        let config = BuildingConfig {
            footprint: Rect::new(0.0, 0.0, 10.0, 8.0),
            tile_size: 0.5,
            min_room_size: 2.5,
            room_specs: vec![
                RoomSpec::new("a", 0),
                RoomSpec::new("b", 0),
                RoomSpec::new("c", 0),
                RoomSpec::new("d", 0),
            ],
            has_corridor: true,
            corridor_width: 1.0,
            ..Default::default()
        };
        let (rooms, corridor) = generate_rooms(&config);

        assert_eq!(rooms.len(), 4);
        assert!(corridor.is_some());

        assert_eq!(rooms[0].label, "a");
        assert_eq!(rooms[1].label, "b");
        assert_eq!(rooms[2].label, "c");
        assert_eq!(rooms[3].label, "d");

        let cb = corridor.unwrap().bounds;
        assert!((cb.min.x - 4.5).abs() < 0.1);
        assert!((cb.max.x - 5.5).abs() < 0.1);
        assert_eq!(rooms[0].bounds.max.x, cb.min.x);
        assert_eq!(rooms[1].bounds.min.x, cb.max.x);
        assert_eq!(rooms[2].bounds.max.x, cb.min.x);
        assert_eq!(rooms[3].bounds.min.x, cb.max.x);
    }

    #[test]
    fn test_corridor_width_tiles_controls_corridor_size() {
        let config = BuildingConfig {
            footprint: Rect::new(0.0, 0.0, 12.0, 8.0),
            tile_size: 0.5,
            corridor_width_tiles: 4,
            room_specs: vec![
                RoomSpec::new("hall", 0),
                RoomSpec::new("kitchen", 0),
                RoomSpec::new("bedroom", 0),
                RoomSpec::new("bathroom", 0),
            ],
            has_corridor: true,
            ..Default::default()
        };
        let (_, corridor) = generate_rooms(&config);
        let cb = corridor.unwrap().bounds;

        assert!((cb.width() - 2.0).abs() < 0.1);
    }

    #[test]
    fn test_corridor_follows_offset_entrance_without_touching_edge() {
        let config = BuildingConfig {
            footprint: Rect::new(0.0, 0.0, 12.0, 8.0),
            entrance: Vec2::new(8.0, 0.0),
            tile_size: 0.5,
            min_room_size: 2.5,
            room_specs: vec![
                RoomSpec::new("hall", 0),
                RoomSpec::new("kitchen", 0),
                RoomSpec::new("bedroom", 0),
                RoomSpec::new("bathroom", 0),
            ],
            has_corridor: true,
            corridor_width: 1.0,
            ..Default::default()
        };
        let (_, corridor) = generate_rooms(&config);
        let cb = corridor.unwrap().bounds;

        assert!((cb.center().x - 8.0).abs() < 0.1);
        assert!(cb.min.x > config.footprint.min.x);
        assert!(cb.max.x < config.footprint.max.x);
    }

    #[test]
    fn test_entrance_door_position_south() {
        let config = BuildingConfig {
            footprint: Rect::new(0.0, 0.0, 10.0, 8.0),
            entrance: Vec2::new(2.5, 0.0),
            entrance_dir: Vec2::new(0.0, 1.0),
            ..Default::default()
        };
        let pos = entrance_door_position(&config);
        assert!((pos.x - 2.5).abs() < 0.01);
        assert!((pos.y - 0.0).abs() < 0.01);
    }

    #[test]
    fn test_entrance_door_position_west() {
        let config = BuildingConfig {
            footprint: Rect::new(0.0, 0.0, 10.0, 8.0),
            entrance: Vec2::new(0.0, 6.0),
            entrance_dir: Vec2::new(1.0, 0.0),
            ..Default::default()
        };
        let pos = entrance_door_position(&config);
        assert!((pos.x - 0.0).abs() < 0.01);
        assert!((pos.y - 6.0).abs() < 0.01);
    }

    #[test]
    fn test_room_order_starts_at_north_entrance() {
        let config = BuildingConfig {
            entrance: Vec2::new(5.0, 8.0),
            entrance_dir: Vec2::new(0.0, -1.0),
            room_specs: vec![
                RoomSpec::new("hall", 0),
                RoomSpec::new("kitchen", 0),
                RoomSpec::new("bathroom", 0),
                RoomSpec::new("bedroom", 0),
                RoomSpec::new("storage", 0),
            ],
            ..default_config()
        };
        let (rooms, _) = generate_rooms(&config);

        assert_eq!(rooms[0].label, "hall");
        assert!(rooms[0].bounds.min.y > rooms[4].bounds.min.y);
    }

    #[test]
    fn test_snap_to_grid() {
        assert_eq!(snap_to_grid(1.23, 0.5), 1.0);
        assert_eq!(snap_to_grid(1.26, 0.5), 1.5);
        assert_eq!(snap_to_grid(2.0, 1.0), 2.0);
    }

    #[test]
    fn test_distribute_splits_even() {
        let splits = distribute_splits(0.0, 10.0, 2, 0.5);
        assert_eq!(splits.len(), 3);
        assert_eq!(splits[0], 0.0);
        assert_eq!(splits[2], 10.0);
        // Middle split should be snapped to grid
        assert_eq!(splits[1] % 0.5, 0.0);
    }
}
