use crate::config::BuildingConfig;
use crate::furniture;
use crate::geometry::{Rect, Vec2, Vec3};
use crate::layout::{Doorway, Room, Window};
use crate::mesh::building_base_y;
use crate::random::{SeededRng, deterministic_lot_unit};
use crate::scene::{SceneObject, SceneObjectKind};
use crate::tile::{TileGrid, TileType};
use std::collections::HashSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SceneletKind {
    Sleeping,
    Dining,
    StorageWall,
    KitchenWork,
    EntryDrop,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RoomProfile {
    Bedroom,
    Kitchen,
    HallEntry,
    Storage,
    StudioHome,
    Generic,
}

#[derive(Debug, Clone)]
pub struct SceneletCandidate {
    pub kind: SceneletKind,
    pub anchor: (usize, usize),
    pub rotation: f32,
    pub footprint: Vec<(usize, usize)>,
    pub clearance: Vec<(usize, usize)>,
    pub items: Vec<SceneObject>,
    pub score: f32,
}

#[derive(Debug, Clone)]
pub struct SceneletPlan {
    pub profile: RoomProfile,
    pub scenelets: Vec<SceneletKind>,
}

pub struct SceneletContext<'a> {
    pub room: &'a Room,
    pub grid: &'a TileGrid,
    pub doorways: &'a [Doorway],
    pub windows: &'a [Window],
    pub floor_y: f32,
    pub seed: u64,
}

#[derive(Debug, Clone)]
pub struct SceneOccupancy {
    hard: HashSet<(usize, usize)>,
    clearance: HashSet<(usize, usize)>,
    width: usize,
    height: usize,
}

impl SceneOccupancy {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            hard: HashSet::new(),
            clearance: HashSet::new(),
            width,
            height,
        }
    }

    pub fn reserve_doorway(&mut self, x: usize, y: usize) {
        for dy in -1isize..=1 {
            for dx in -1isize..=1 {
                self.reserve_hard_signed(x, y, dx, dy);
            }
        }
    }

    pub fn reserve_candidate(&mut self, candidate: &SceneletCandidate) {
        for &tile in &candidate.footprint {
            self.hard.insert(tile);
        }
        for &tile in &candidate.clearance {
            self.clearance.insert(tile);
        }
    }

    fn reserve_hard_signed(&mut self, x: usize, y: usize, dx: isize, dy: isize) {
        let nx = x as isize + dx;
        let ny = y as isize + dy;
        if nx >= 0 && ny >= 0 && (nx as usize) < self.width && (ny as usize) < self.height {
            self.hard.insert((nx as usize, ny as usize));
        }
    }

    fn blocks_hard(&self, tile: (usize, usize)) -> bool {
        self.hard.contains(&tile) || self.clearance.contains(&tile)
    }

    fn blocks_clearance(&self, tile: (usize, usize)) -> bool {
        self.hard.contains(&tile)
    }
}

pub fn generate_scene_objects(
    rooms: &[Room],
    grid: &TileGrid,
    config: &BuildingConfig,
    doorways: &[Doorway],
    windows: &[Window],
) -> Vec<SceneObject> {
    if !config.furniture {
        return Vec::new();
    }

    let floor_y = building_base_y(config);
    let mut objects = Vec::new();
    let mut occupancy = SceneOccupancy::new(grid.width, grid.height);

    for doorway in doorways {
        if let Some((x, y)) = grid.tile_coord(doorway.position) {
            occupancy.reserve_doorway(x, y);
        }
    }

    let single_room_home = rooms.len() == 1
        && rooms[0].bounds.area() >= 25.0
        && rooms[0].label.eq_ignore_ascii_case("bedroom");

    for room in rooms {
        let profile = derive_room_profile(room, single_room_home);
        let plan = SceneletPlan {
            profile,
            scenelets: scenelets_for_profile(profile, config),
        };
        let context = SceneletContext {
            room,
            grid,
            doorways,
            windows,
            floor_y,
            seed: config.seed,
        };
        for kind in plan.scenelets {
            if let Some(candidate) = choose_candidate(kind, &context, &occupancy) {
                occupancy.reserve_candidate(&candidate);
                objects.extend(candidate.items);
            }
        }
    }

    objects
}

fn derive_room_profile(room: &Room, single_room_home: bool) -> RoomProfile {
    if single_room_home {
        return RoomProfile::StudioHome;
    }
    match room.label.trim().to_ascii_lowercase().as_str() {
        "bedroom" => RoomProfile::Bedroom,
        "kitchen" => RoomProfile::Kitchen,
        "hall" | "foyer" | "entry" | "entrance" => RoomProfile::HallEntry,
        "storage" | "closet" | "pantry" => RoomProfile::Storage,
        _ => RoomProfile::Generic,
    }
}

fn scenelets_for_profile(profile: RoomProfile, config: &BuildingConfig) -> Vec<SceneletKind> {
    match profile {
        RoomProfile::StudioHome => vec![
            SceneletKind::Sleeping,
            SceneletKind::Dining,
            SceneletKind::StorageWall,
            SceneletKind::EntryDrop,
        ],
        RoomProfile::Bedroom => vec![
            SceneletKind::Sleeping,
            SceneletKind::StorageWall,
            SceneletKind::Dining,
        ],
        RoomProfile::Kitchen => {
            let mut plan = vec![SceneletKind::Dining];
            if config.has_stove {
                plan.insert(0, SceneletKind::KitchenWork);
            }
            plan
        }
        RoomProfile::HallEntry => vec![SceneletKind::EntryDrop],
        RoomProfile::Storage => vec![SceneletKind::StorageWall],
        RoomProfile::Generic => vec![SceneletKind::Dining],
    }
}

fn choose_candidate(
    kind: SceneletKind,
    context: &SceneletContext<'_>,
    occupancy: &SceneOccupancy,
) -> Option<SceneletCandidate> {
    let mut candidates = match kind {
        SceneletKind::Sleeping => sleeping_candidates(context, occupancy),
        SceneletKind::Dining => dining_candidates(context, occupancy),
        SceneletKind::StorageWall => storage_candidates(context, occupancy),
        SceneletKind::KitchenWork => kitchen_candidates(context, occupancy),
        SceneletKind::EntryDrop => entry_candidates(context, occupancy),
    };
    candidates.retain(|candidate| candidate.score >= minimum_score(kind));
    candidates.sort_by(|a, b| {
        b.score
            .partial_cmp(&a.score)
            .unwrap_or(std::cmp::Ordering::Equal)
            .then_with(|| {
                tie_break(context.seed, kind, a)
                    .partial_cmp(&tie_break(context.seed, kind, b))
                    .unwrap()
            })
    });

    let top_score = candidates.first().map(|c| c.score)?;
    let mut top: Vec<_> = candidates
        .into_iter()
        .filter(|candidate| (candidate.score - top_score).abs() < 0.75)
        .collect();
    let mut rng = SeededRng::new(context.seed ^ scenelet_seed(kind) ^ context.room.id.0 as u64);
    rng.shuffle(&mut top);
    top.into_iter().next()
}

fn minimum_score(kind: SceneletKind) -> f32 {
    match kind {
        SceneletKind::Dining => 8.0,
        SceneletKind::Sleeping => 10.0,
        SceneletKind::StorageWall => 7.0,
        SceneletKind::KitchenWork => 7.0,
        SceneletKind::EntryDrop => 6.0,
    }
}

fn sleeping_candidates(
    context: &SceneletContext<'_>,
    occupancy: &SceneOccupancy,
) -> Vec<SceneletCandidate> {
    wall_positions(context, occupancy)
        .into_iter()
        .filter_map(|anchor| {
            let mut item = furniture::single_item(SceneObjectKind::Bed);
            item.color = [0.72, 0.38, 0.28];
            make_wall_candidate(
                SceneletKind::Sleeping,
                context,
                occupancy,
                anchor,
                item,
                16.0,
                2,
            )
        })
        .collect()
}

fn dining_candidates(
    context: &SceneletContext<'_>,
    occupancy: &SceneOccupancy,
) -> Vec<SceneletCandidate> {
    center_positions(context, occupancy, 2)
        .into_iter()
        .filter_map(|(x, y)| {
            let mut footprint = vec![(x, y)];
            let mut clearance = neighbors4(x, y, context.grid);
            let mut items = Vec::new();
            let mut table = furniture::single_item(SceneObjectKind::Table);
            set_item(&mut table, x, y, 0.0, context);
            items.push(table);

            let chair_specs = [
                (0isize, -1isize, std::f32::consts::PI),
                (0, 1, 0.0),
                (-1, 0, -std::f32::consts::FRAC_PI_2),
                (1, 0, std::f32::consts::FRAC_PI_2),
            ];
            let mut chair_count = 0;
            for (dx, dy, rotation) in chair_specs {
                let Some(tile) = offset_tile(x, y, dx, dy, context.grid) else {
                    continue;
                };
                if !valid_floor(tile, context.grid) || occupancy.blocks_hard(tile) {
                    continue;
                }
                let mut chair = furniture::single_item(SceneObjectKind::Chair);
                set_item(&mut chair, tile.0, tile.1, rotation, context);
                footprint.push(tile);
                clearance.extend(neighbors4(tile.0, tile.1, context.grid));
                items.push(chair);
                chair_count += 1;
            }
            if chair_count < 2 {
                return None;
            }
            let score = score_center(context, x, y) + chair_count as f32 * 2.5
                - doorway_penalty(context, &footprint);
            candidate_if_valid(
                SceneletKind::Dining,
                (x, y),
                0.0,
                footprint,
                clearance,
                items,
                score,
                occupancy,
            )
        })
        .collect()
}

fn storage_candidates(
    context: &SceneletContext<'_>,
    occupancy: &SceneOccupancy,
) -> Vec<SceneletCandidate> {
    wall_positions(context, occupancy)
        .into_iter()
        .filter_map(|anchor| {
            let item = furniture::single_item(SceneObjectKind::Shelf);
            let window_penalty = window_penalty(context, anchor.0, anchor.1, 5.0);
            let anchor = (anchor.0, anchor.1, shelf_wall_rotation(anchor.2));
            make_wall_candidate(
                SceneletKind::StorageWall,
                context,
                occupancy,
                anchor,
                item,
                12.0 - window_penalty,
                1,
            )
        })
        .collect()
}

fn shelf_wall_rotation(rotation: f32) -> f32 {
    if rotation.abs() < 0.01 {
        std::f32::consts::PI
    } else if (rotation - std::f32::consts::PI).abs() < 0.01 {
        0.0
    } else {
        rotation
    }
}

fn kitchen_candidates(
    context: &SceneletContext<'_>,
    occupancy: &SceneOccupancy,
) -> Vec<SceneletCandidate> {
    wall_positions(context, occupancy)
        .into_iter()
        .filter_map(|anchor| {
            let mut items = Vec::new();
            let mut footprint = Vec::new();
            let clearance = neighbors4(anchor.0, anchor.1, context.grid);

            let mut stove = furniture::single_item(SceneObjectKind::Stove);
            set_item(&mut stove, anchor.0, anchor.1, anchor.2, context);
            items.push(stove);
            footprint.push((anchor.0, anchor.1));

            for (dx, dy) in side_offsets_for_rotation(anchor.2) {
                if let Some(tile) = offset_tile(anchor.0, anchor.1, dx, dy, context.grid)
                    && valid_floor(tile, context.grid)
                    && !occupancy.blocks_hard(tile)
                {
                    let mut counter = furniture::single_item(SceneObjectKind::Counter);
                    set_item(&mut counter, tile.0, tile.1, anchor.2, context);
                    items.push(counter);
                    footprint.push(tile);
                    break;
                }
            }

            let score = 13.0
                - doorway_penalty(context, &footprint)
                - window_penalty(context, anchor.0, anchor.1, 1.0);
            candidate_if_valid(
                SceneletKind::KitchenWork,
                (anchor.0, anchor.1),
                anchor.2,
                footprint,
                clearance,
                items,
                score,
                occupancy,
            )
        })
        .collect()
}

fn entry_candidates(
    context: &SceneletContext<'_>,
    occupancy: &SceneOccupancy,
) -> Vec<SceneletCandidate> {
    let mut candidates = Vec::new();
    for doorway in context.doorways {
        if !Rect::new(
            context.room.bounds.min.x - context.grid.tile_size,
            context.room.bounds.min.y - context.grid.tile_size,
            context.room.bounds.max.x + context.grid.tile_size,
            context.room.bounds.max.y + context.grid.tile_size,
        )
        .contains(doorway.position)
        {
            continue;
        }
        if let Some((door_x, door_y)) = context.grid.tile_coord(doorway.position) {
            for tile in neighbors4(door_x, door_y, context.grid) {
                if !valid_floor(tile, context.grid) || occupancy.blocks_hard(tile) {
                    continue;
                }
                let mut bench = furniture::single_item(SceneObjectKind::Bench);
                set_item(&mut bench, tile.0, tile.1, 0.0, context);
                let footprint = vec![tile];
                let clearance = neighbors4(tile.0, tile.1, context.grid);
                if let Some(candidate) = candidate_if_valid(
                    SceneletKind::EntryDrop,
                    tile,
                    0.0,
                    footprint,
                    clearance,
                    vec![bench],
                    11.0,
                    occupancy,
                ) {
                    candidates.push(candidate);
                }
            }
        }
    }
    if candidates.is_empty() {
        storage_candidates(context, occupancy)
            .into_iter()
            .map(|mut candidate| {
                candidate.kind = SceneletKind::EntryDrop;
                candidate
            })
            .collect()
    } else {
        candidates
    }
}

fn make_wall_candidate(
    kind: SceneletKind,
    context: &SceneletContext<'_>,
    occupancy: &SceneOccupancy,
    anchor: (usize, usize, f32),
    mut item: SceneObject,
    base_score: f32,
    access_radius: usize,
) -> Option<SceneletCandidate> {
    set_item(&mut item, anchor.0, anchor.1, anchor.2, context);
    offset_item_away_from_wall(&mut item, anchor.0, anchor.1, context.grid);
    let footprint = vec![(anchor.0, anchor.1)];
    let mut clearance = neighbors4(anchor.0, anchor.1, context.grid);
    if access_radius > 1 {
        clearance.extend(neighbors8(anchor.0, anchor.1, context.grid));
    }
    let score = base_score + wall_depth_score(context, anchor.0, anchor.1)
        - doorway_penalty(context, &footprint);
    candidate_if_valid(
        kind,
        (anchor.0, anchor.1),
        anchor.2,
        footprint,
        clearance,
        vec![item],
        score,
        occupancy,
    )
}

fn candidate_if_valid(
    kind: SceneletKind,
    anchor: (usize, usize),
    rotation: f32,
    footprint: Vec<(usize, usize)>,
    clearance: Vec<(usize, usize)>,
    items: Vec<SceneObject>,
    score: f32,
    occupancy: &SceneOccupancy,
) -> Option<SceneletCandidate> {
    if footprint.iter().any(|&tile| occupancy.blocks_hard(tile)) {
        return None;
    }
    if clearance
        .iter()
        .any(|&tile| occupancy.blocks_clearance(tile))
    {
        return None;
    }
    Some(SceneletCandidate {
        kind,
        anchor,
        rotation,
        footprint,
        clearance,
        items,
        score,
    })
}

fn wall_positions(
    context: &SceneletContext<'_>,
    occupancy: &SceneOccupancy,
) -> Vec<(usize, usize, f32)> {
    let (min_x, min_y, max_x, max_y) = room_tile_bounds(context.room.bounds, context.grid);
    let mut positions = Vec::new();
    for y in min_y..=max_y {
        for x in min_x..=max_x {
            let tile = (x, y);
            if !valid_floor(tile, context.grid) || occupancy.blocks_hard(tile) {
                continue;
            }
            if y > 0 && context.grid.get(x, y - 1).is_wall() {
                positions.push((x, y, std::f32::consts::PI));
            }
            if y + 1 < context.grid.height && context.grid.get(x, y + 1).is_wall() {
                positions.push((x, y, 0.0));
            }
            if x > 0 && context.grid.get(x - 1, y).is_wall() {
                positions.push((x, y, std::f32::consts::FRAC_PI_2));
            }
            if x + 1 < context.grid.width && context.grid.get(x + 1, y).is_wall() {
                positions.push((x, y, -std::f32::consts::FRAC_PI_2));
            }
        }
    }
    positions
}

fn center_positions(
    context: &SceneletContext<'_>,
    occupancy: &SceneOccupancy,
    inset: usize,
) -> Vec<(usize, usize)> {
    let (min_x, min_y, max_x, max_y) = room_tile_bounds(context.room.bounds, context.grid);
    if max_x <= min_x + inset * 2 || max_y <= min_y + inset * 2 {
        return Vec::new();
    }
    let mut positions = Vec::new();
    for y in (min_y + inset)..=(max_y - inset) {
        for x in (min_x + inset)..=(max_x - inset) {
            let tile = (x, y);
            if valid_floor(tile, context.grid) && !occupancy.blocks_hard(tile) {
                positions.push(tile);
            }
        }
    }
    positions
}

fn room_tile_bounds(bounds: Rect, grid: &TileGrid) -> (usize, usize, usize, usize) {
    let min_x = ((bounds.min.x - grid.origin.x) / grid.tile_size)
        .floor()
        .max(0.0) as usize;
    let min_y = ((bounds.min.y - grid.origin.y) / grid.tile_size)
        .floor()
        .max(0.0) as usize;
    let max_x = ((bounds.max.x - grid.origin.x) / grid.tile_size)
        .ceil()
        .max(0.0) as usize;
    let max_y = ((bounds.max.y - grid.origin.y) / grid.tile_size)
        .ceil()
        .max(0.0) as usize;
    (
        min_x.min(grid.width.saturating_sub(1)),
        min_y.min(grid.height.saturating_sub(1)),
        max_x.min(grid.width.saturating_sub(1)),
        max_y.min(grid.height.saturating_sub(1)),
    )
}

fn valid_floor(tile: (usize, usize), grid: &TileGrid) -> bool {
    grid.get(tile.0, tile.1) == TileType::Floor
}

fn set_item(
    item: &mut SceneObject,
    x: usize,
    y: usize,
    rotation: f32,
    context: &SceneletContext<'_>,
) {
    let (wx, wz) = tile_to_world(x, y, context.grid);
    item.position = Vec3::new(wx, context.floor_y, wz);
    item.rotation = rotation;
}

fn offset_item_away_from_wall(item: &mut SceneObject, x: usize, y: usize, grid: &TileGrid) {
    let Some(away) = wall_away_direction(x, y, grid) else {
        return;
    };
    let half_extent = half_extent_along_direction(item, away);
    let clearance = if half_extent < grid.tile_size / 2.0 {
        -0.06
    } else {
        0.04
    };
    let offset = half_extent - grid.tile_size / 2.0 + clearance;
    item.position.x += away.x * offset;
    item.position.z += away.y * offset;
}

fn half_extent_along_direction(item: &SceneObject, direction: Vec2) -> f32 {
    let cos = item.rotation.cos().abs();
    let sin = item.rotation.sin().abs();
    let half_x = cos * item.width / 2.0 + sin * item.depth / 2.0;
    let half_z = sin * item.width / 2.0 + cos * item.depth / 2.0;
    if direction.x.abs() > direction.y.abs() {
        half_x
    } else {
        half_z
    }
}

fn wall_away_direction(x: usize, y: usize, grid: &TileGrid) -> Option<Vec2> {
    if y > 0 && grid.get(x, y - 1).is_wall() {
        Some(Vec2::new(0.0, 1.0))
    } else if y + 1 < grid.height && grid.get(x, y + 1).is_wall() {
        Some(Vec2::new(0.0, -1.0))
    } else if x > 0 && grid.get(x - 1, y).is_wall() {
        Some(Vec2::new(1.0, 0.0))
    } else if x + 1 < grid.width && grid.get(x + 1, y).is_wall() {
        Some(Vec2::new(-1.0, 0.0))
    } else {
        None
    }
}

fn tile_to_world(x: usize, y: usize, grid: &TileGrid) -> (f32, f32) {
    (
        grid.origin.x + (x as f32 + 0.5) * grid.tile_size,
        grid.origin.y + (y as f32 + 0.5) * grid.tile_size,
    )
}

fn neighbors4(x: usize, y: usize, grid: &TileGrid) -> Vec<(usize, usize)> {
    [(-1, 0), (1, 0), (0, -1), (0, 1)]
        .into_iter()
        .filter_map(|(dx, dy)| offset_tile(x, y, dx, dy, grid))
        .collect()
}

fn neighbors8(x: usize, y: usize, grid: &TileGrid) -> Vec<(usize, usize)> {
    let mut tiles = Vec::new();
    for dy in -1..=1 {
        for dx in -1..=1 {
            if dx == 0 && dy == 0 {
                continue;
            }
            if let Some(tile) = offset_tile(x, y, dx, dy, grid) {
                tiles.push(tile);
            }
        }
    }
    tiles
}

fn offset_tile(
    x: usize,
    y: usize,
    dx: isize,
    dy: isize,
    grid: &TileGrid,
) -> Option<(usize, usize)> {
    let nx = x as isize + dx;
    let ny = y as isize + dy;
    if nx >= 0 && ny >= 0 && (nx as usize) < grid.width && (ny as usize) < grid.height {
        Some((nx as usize, ny as usize))
    } else {
        None
    }
}

fn side_offsets_for_rotation(rotation: f32) -> [(isize, isize); 2] {
    if (rotation - std::f32::consts::PI).abs() < 0.01 || rotation.abs() < 0.01 {
        [(-1, 0), (1, 0)]
    } else {
        [(0, -1), (0, 1)]
    }
}

fn score_center(context: &SceneletContext<'_>, x: usize, y: usize) -> f32 {
    let (wx, wz) = tile_to_world(x, y, context.grid);
    let center = context.room.bounds.center();
    let distance = Vec2::new(wx, wz).distance_to(center);
    16.0 - distance
}

fn wall_depth_score(context: &SceneletContext<'_>, x: usize, y: usize) -> f32 {
    let (wx, wz) = tile_to_world(x, y, context.grid);
    let center = context.room.bounds.center();
    Vec2::new(wx, wz).distance_to(center).min(4.0) * 0.5
}

fn doorway_penalty(context: &SceneletContext<'_>, footprint: &[(usize, usize)]) -> f32 {
    let mut penalty = 0.0;
    for doorway in context.doorways {
        for &(x, y) in footprint {
            let (wx, wz) = tile_to_world(x, y, context.grid);
            let distance = Vec2::new(wx, wz).distance_to(doorway.position);
            if distance < context.grid.tile_size * 2.0 {
                penalty += 6.0;
            }
        }
    }
    penalty
}

fn window_penalty(context: &SceneletContext<'_>, x: usize, y: usize, amount: f32) -> f32 {
    let (wx, wz) = tile_to_world(x, y, context.grid);
    context
        .windows
        .iter()
        .filter(|window| {
            Vec2::new(wx, wz).distance_to(window.position) < context.grid.tile_size * 1.5
        })
        .count() as f32
        * amount
}

fn tie_break(seed: u64, kind: SceneletKind, candidate: &SceneletCandidate) -> f32 {
    deterministic_lot_unit(
        candidate.anchor.0 as f32,
        candidate.anchor.1 as f32,
        candidate.score,
        candidate.rotation,
        seed ^ scenelet_seed(kind),
    )
}

fn scenelet_seed(kind: SceneletKind) -> u64 {
    match kind {
        SceneletKind::Sleeping => 0x51EE_9171,
        SceneletKind::Dining => 0xD171_1716,
        SceneletKind::StorageWall => 0x570A_9E00,
        SceneletKind::KitchenWork => 0xC17C_4E11,
        SceneletKind::EntryDrop => 0xE477_D209,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::{BuildingConfig, RoomSpec};
    use crate::geometry::Rect;

    fn studio_config(seed: u64) -> BuildingConfig {
        BuildingConfig {
            seed,
            footprint: Rect::new(0.0, 0.0, 8.0, 6.0),
            room_specs: vec![RoomSpec::new("bedroom", 4)],
            render_roof: false,
            ..Default::default()
        }
    }

    #[test]
    fn studio_home_places_core_scenelets() {
        let config = studio_config(42);
        let layout = crate::generate_layout(&config);
        let objects = generate_scene_objects(
            &layout.rooms,
            &layout.tile_grid,
            &config,
            &layout.doorways,
            &layout.windows,
        );
        assert!(objects.iter().any(|o| o.item_type == SceneObjectKind::Bed));
        assert!(
            objects
                .iter()
                .any(|o| o.item_type == SceneObjectKind::Table)
        );
        assert!(
            objects
                .iter()
                .any(|o| o.item_type == SceneObjectKind::Chair)
        );
        assert!(
            objects
                .iter()
                .any(|o| o.item_type == SceneObjectKind::Shelf)
        );
    }

    #[test]
    fn dining_scenelet_places_table_with_multiple_chairs() {
        let config = studio_config(42);
        let layout = crate::generate_layout(&config);
        let objects = generate_scene_objects(
            &layout.rooms,
            &layout.tile_grid,
            &config,
            &layout.doorways,
            &layout.windows,
        );
        assert_eq!(
            objects
                .iter()
                .filter(|o| o.item_type == SceneObjectKind::Table)
                .count(),
            1
        );
        assert!(
            objects
                .iter()
                .filter(|o| o.item_type == SceneObjectKind::Chair)
                .count()
                >= 2
        );
    }

    #[test]
    fn dining_chairs_face_the_table() {
        let config = studio_config(42);
        let layout = crate::generate_layout(&config);
        let objects = generate_scene_objects(
            &layout.rooms,
            &layout.tile_grid,
            &config,
            &layout.doorways,
            &layout.windows,
        );
        let table = objects
            .iter()
            .find(|object| object.item_type == SceneObjectKind::Table)
            .expect("table should be placed");
        for chair in objects
            .iter()
            .filter(|object| object.item_type == SceneObjectKind::Chair)
        {
            let to_table = Vec2::new(
                table.position.x - chair.position.x,
                table.position.z - chair.position.z,
            );
            let facing = chair_facing(chair.rotation);
            let dot = to_table.x * facing.x + to_table.y * facing.y;
            assert!(dot > 0.0, "chair should face table, dot={dot}");
        }
    }

    #[test]
    fn sleeping_scenelet_places_bed_against_wall_with_access() {
        let config = studio_config(42);
        let layout = crate::generate_layout(&config);
        let objects = generate_scene_objects(
            &layout.rooms,
            &layout.tile_grid,
            &config,
            &layout.doorways,
            &layout.windows,
        );
        let bed = objects
            .iter()
            .find(|o| o.item_type == SceneObjectKind::Bed)
            .expect("bed should be placed");
        let room = &layout.rooms[0];
        let wall_distance =
            distance_to_room_edge(Vec2::new(bed.position.x, bed.position.z), room.bounds);
        let accessible_side = layout
            .tile_grid
            .tile_coord(Vec2::new(bed.position.x, bed.position.z))
            .map(|(x, y)| {
                neighbors4(x, y, &layout.tile_grid)
                    .iter()
                    .any(|&(nx, ny)| layout.tile_grid.get(nx, ny) == TileType::Floor)
            })
            .unwrap_or(false);
        assert!(wall_distance < 1.25);
        assert!(accessible_side);
    }

    #[test]
    fn wall_anchored_bed_is_offset_away_from_wall() {
        let config = studio_config(42);
        let layout = crate::generate_layout(&config);
        let objects = generate_scene_objects(
            &layout.rooms,
            &layout.tile_grid,
            &config,
            &layout.doorways,
            &layout.windows,
        );
        let bed = objects
            .iter()
            .find(|o| o.item_type == SceneObjectKind::Bed)
            .expect("bed should be placed");
        let wall_distance = distance_to_room_edge(
            Vec2::new(bed.position.x, bed.position.z),
            layout.rooms[0].bounds,
        );
        let half_extent = bed.width.max(bed.depth) / 2.0;
        assert!(
            wall_distance > half_extent,
            "bed should clear wall, wall_distance={wall_distance}, half_extent={half_extent}"
        );
    }

    #[test]
    fn wall_storage_sits_close_to_wall() {
        let config = studio_config(42);
        let layout = crate::generate_layout(&config);
        let objects = generate_scene_objects(
            &layout.rooms,
            &layout.tile_grid,
            &config,
            &layout.doorways,
            &layout.windows,
        );
        let shelf = objects
            .iter()
            .find(|o| o.item_type == SceneObjectKind::Shelf)
            .expect("shelf should be placed");
        let wall_distance = distance_to_room_edge(
            Vec2::new(shelf.position.x, shelf.position.z),
            layout.rooms[0].bounds,
        ) - layout.tile_grid.tile_size;
        let half_extent = half_extent_along_direction(
            shelf,
            nearest_edge_direction(
                Vec2::new(shelf.position.x, shelf.position.z),
                layout.rooms[0].bounds,
            ),
        );
        assert!(
            (wall_distance - half_extent).abs() <= 0.12,
            "shelf should sit close to wall, wall_distance={wall_distance}, half_extent={half_extent}"
        );
    }

    fn nearest_edge_direction(point: Vec2, room: Rect) -> Vec2 {
        [
            (point.x - room.min.x, Vec2::new(1.0, 0.0)),
            (room.max.x - point.x, Vec2::new(-1.0, 0.0)),
            (point.y - room.min.y, Vec2::new(0.0, 1.0)),
            (room.max.y - point.y, Vec2::new(0.0, -1.0)),
        ]
        .into_iter()
        .min_by(|a, b| a.0.partial_cmp(&b.0).unwrap())
        .map(|(_, direction)| direction)
        .unwrap()
    }

    #[test]
    fn generation_is_deterministic_for_same_seed() {
        let config = studio_config(7);
        let layout = crate::generate_layout(&config);
        let a = generate_scene_objects(
            &layout.rooms,
            &layout.tile_grid,
            &config,
            &layout.doorways,
            &layout.windows,
        );
        let b = generate_scene_objects(
            &layout.rooms,
            &layout.tile_grid,
            &config,
            &layout.doorways,
            &layout.windows,
        );
        assert_eq!(object_signature(&a), object_signature(&b));
    }

    #[test]
    fn different_seeds_can_choose_different_valid_arrangements() {
        let config_a = studio_config(7);
        let config_b = studio_config(8);
        let layout_a = crate::generate_layout(&config_a);
        let layout_b = crate::generate_layout(&config_b);
        let a = generate_scene_objects(
            &layout_a.rooms,
            &layout_a.tile_grid,
            &config_a,
            &layout_a.doorways,
            &layout_a.windows,
        );
        let b = generate_scene_objects(
            &layout_b.rooms,
            &layout_b.tile_grid,
            &config_b,
            &layout_b.doorways,
            &layout_b.windows,
        );
        assert_ne!(object_signature(&a), object_signature(&b));
    }

    #[test]
    fn generated_objects_do_not_overlap_tiles() {
        let config = studio_config(11);
        let layout = crate::generate_layout(&config);
        let objects = generate_scene_objects(
            &layout.rooms,
            &layout.tile_grid,
            &config,
            &layout.doorways,
            &layout.windows,
        );
        let mut seen = HashSet::new();
        for object in objects {
            let tile = layout
                .tile_grid
                .tile_coord(Vec2::new(object.position.x, object.position.z))
                .expect("object should be on grid");
            assert!(seen.insert(tile), "overlapping object tile: {:?}", tile);
            assert_eq!(layout.tile_grid.get(tile.0, tile.1), TileType::Floor);
        }
    }

    fn object_signature(objects: &[SceneObject]) -> Vec<(SceneObjectKind, i32, i32)> {
        objects
            .iter()
            .map(|object| {
                (
                    object.item_type,
                    (object.position.x * 10.0).round() as i32,
                    (object.position.z * 10.0).round() as i32,
                )
            })
            .collect()
    }

    fn chair_facing(rotation: f32) -> Vec2 {
        Vec2::new(-rotation.sin(), -rotation.cos())
    }

    fn distance_to_room_edge(point: Vec2, room: Rect) -> f32 {
        (point.x - room.min.x)
            .min(room.max.x - point.x)
            .min(point.y - room.min.y)
            .min(room.max.y - point.y)
    }
}
