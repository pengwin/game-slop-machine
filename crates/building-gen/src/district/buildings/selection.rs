use crate::district::config::BuildingDescription;
use crate::district::layout::Lot;
use crate::random::deterministic_lot_unit;

pub fn select_building_description_for_lot<'a>(
    lot: &Lot,
    descriptions: &'a [BuildingDescription],
    seed: u64,
) -> &'a BuildingDescription {
    let area = lot.width * lot.depth;
    let mut scored: Vec<(f32, &'a BuildingDescription)> = descriptions
        .iter()
        .filter(|description| area >= description.min_lot_area && area <= description.max_lot_area)
        .map(|description| (description_score(lot, description), description))
        .collect();

    if scored.is_empty() {
        scored = descriptions
            .iter()
            .map(|description| (fallback_description_score(lot, description), description))
            .collect();
    }

    scored.sort_by(|a, b| {
        a.0.total_cmp(&b.0)
            .then_with(|| a.1.rooms.len().cmp(&b.1.rooms.len()))
    });

    let best_score = scored[0].0;
    let candidate_count = scored
        .iter()
        .take_while(|(score, _)| *score <= best_score + 0.45)
        .count()
        .clamp(1, 4);
    let candidates = &scored[..candidate_count];
    let total_weight: f32 = candidates
        .iter()
        .map(|(score, _)| 1.0 / (0.15 + score - best_score))
        .sum();
    let mut pick = deterministic_lot_unit(lot.position.x, lot.position.y, lot.width, lot.depth, seed) * total_weight;

    for (score, description) in candidates {
        pick -= 1.0 / (0.15 + score - best_score);
        if pick <= 0.0 {
            return description;
        }
    }

    candidates[0].1
}

pub fn select_building_description<'a>(
    lot: &Lot,
    descriptions: &'a [BuildingDescription],
) -> &'a BuildingDescription {
    let area = lot.width * lot.depth;

    descriptions
        .iter()
        .filter(|description| area >= description.min_lot_area && area <= description.max_lot_area)
        .min_by(|a, b| {
            description_score(lot, a)
                .total_cmp(&description_score(lot, b))
                .then_with(|| a.rooms.len().cmp(&b.rooms.len()))
        })
        .or_else(|| {
            descriptions.iter().min_by(|a, b| {
                fallback_description_score(lot, a)
                    .total_cmp(&fallback_description_score(lot, b))
                    .then_with(|| a.rooms.len().cmp(&b.rooms.len()))
            })
        })
        .expect("building description list is non-empty")
}

fn description_score(lot: &Lot, description: &BuildingDescription) -> f32 {
    let area = lot.width * lot.depth;
    let area_mid = if description.max_lot_area.is_finite() {
        (description.min_lot_area + description.max_lot_area) / 2.0
    } else {
        description.min_lot_area
    };
    let area_score = (area - area_mid).abs() / area.max(1.0);
    let room_score = (description.rooms.len() as f32 - desired_room_count(area) as f32).abs();
    let program_score = program_fit_score(lot, description);
    area_score * 0.35 + room_score * 0.2 + program_score
}

fn fallback_description_score(lot: &Lot, description: &BuildingDescription) -> f32 {
    let area = lot.width * lot.depth;
    let range_distance = if area < description.min_lot_area {
        description.min_lot_area - area
    } else if area > description.max_lot_area {
        area - description.max_lot_area
    } else {
        0.0
    };
    range_distance / area.max(1.0)
        + (description.rooms.len() as f32 - desired_room_count(area) as f32).abs() * 0.25
        + program_fit_score(lot, description)
}

pub(super) fn program_fit_score(lot: &Lot, description: &BuildingDescription) -> f32 {
    let lot_area = (lot.width * lot.depth).max(1.0);
    let program_area = preferred_program_area(description);
    let area_score = if program_area > lot_area {
        (program_area - lot_area) / lot_area
    } else {
        (lot_area - program_area) / lot_area * 0.25
    };
    let lot_aspect = lot.width / lot.depth.max(0.1);
    let aspect_score =
        (lot_aspect - preferred_program_aspect(description)).abs() / lot_aspect.max(0.1) * 0.2;

    area_score + aspect_score
}

fn desired_room_count(area: f32) -> usize {
    if area < 55.0 {
        4
    } else if area < 110.0 {
        6
    } else {
        8
    }
}

pub(super) fn preferred_program_area(description: &BuildingDescription) -> f32 {
    let room_area: f32 = description
        .rooms
        .iter()
        .map(|room| room.preferred_area.max(room.min_area))
        .sum();
    let wall_and_circulation = 1.18;
    let corridor_area = if description.auto_corridor && description.rooms.len() >= 6 {
        description.rooms.len() as f32 * 1.5
    } else {
        0.0
    };

    (room_area * wall_and_circulation + corridor_area).max(12.0)
}

pub(super) fn preferred_program_aspect(description: &BuildingDescription) -> f32 {
    match description.rooms.len() {
        0..=3 => 1.0,
        4..=5 => 1.25,
        6..=7 => 1.55,
        _ => 1.85,
    }
}
