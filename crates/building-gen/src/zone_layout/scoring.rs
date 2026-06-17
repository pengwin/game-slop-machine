use super::axes::LayoutAxes;
use super::corridor::CorridorInfo;
use crate::config::{BuildingConfig, RoomSpec};
use crate::geometry::Rect;
use crate::layout::Room;

#[derive(Debug, Clone)]
pub struct LayoutCandidate {
    pub rooms: Vec<Room>,
    pub corridor: Option<CorridorInfo>,
    pub score: f32,
}

pub fn choose_best_layout(
    config: &BuildingConfig,
    axes: LayoutAxes,
    specs: &[&RoomSpec],
) -> (Vec<Room>, Option<CorridorInfo>) {
    let mut candidates = Vec::new();

    if !config.has_corridor {
        let max_columns = max_rooms_per_row(config, axes, specs.len());
        let balanced = balanced_rooms_per_row(config, axes, specs.len());
        for columns in candidate_column_counts(max_columns, balanced) {
            let (rooms, corridor) =
                super::rows::generate_row_rooms_with_columns(config, axes, specs, columns);
            candidates.push(scored_candidate(config, specs, rooms, corridor));
        }
    }

    if config.has_corridor || config.auto_corridor {
        let (rooms, corridor) = super::corridor::generate_corridor_rooms(config, axes, specs);
        candidates.push(scored_candidate(config, specs, rooms, corridor));
    }

    candidates
        .into_iter()
        .max_by(|a, b| a.score.total_cmp(&b.score))
        .map(|candidate| (candidate.rooms, candidate.corridor))
        .unwrap_or_else(|| super::rows::generate_row_rooms(config, axes, specs))
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

pub fn max_rooms_per_row(config: &BuildingConfig, axes: LayoutAxes, room_count: usize) -> usize {
    let width = axes.width_end - axes.width_start;
    ((width / config.min_room_size).floor() as usize)
        .max(1)
        .min(room_count)
}

pub fn balanced_rooms_per_row(
    config: &BuildingConfig,
    axes: LayoutAxes,
    room_count: usize,
) -> usize {
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
