//! Binary Space Partitioning (BSP) algorithm for generating room layouts.
//!
//! BSP recursively subdivides a rectangular region into smaller rectangles (rooms).
//! The algorithm:
//! 1. Start with the building footprint
//! 2. Choose a split axis (horizontal or vertical)
//! 3. Choose a random split position along that axis
//! 4. Recursively subdivide each half until rooms are small enough
//!
//! Design decisions:
//! - Split position is snapped to tile grid for clean wall alignment
//! - Axis choice prefers splitting the longer dimension (1.25:1 ratio threshold)
//! - Random split position ensures varied layouts with same seed
//! - Minimum room size prevents rooms that are too small to be useful

use crate::config::BuildingConfig;
use crate::geometry::{Axis, Rect};
use crate::layout::Room;
use crate::random::SeededRng;

/// A node in the BSP tree representing either a room (leaf) or a split (internal).
#[derive(Debug, Clone)]
pub enum BspNode {
    /// A leaf node containing a single room
    Leaf { room: Room },
    /// An internal node representing a split of the region
    Internal {
        axis: Axis,
        position: f32,
        left: Box<BspNode>,
        right: Box<BspNode>,
    },
}

/// Entry point for BSP subdivision. Returns the root of the BSP tree.
///
/// The tree is then flattened into a list of rooms using `collect_rooms()`.
pub fn bsp_subdivide(config: &BuildingConfig, rng: &mut SeededRng) -> BspNode {
    let mut room_id = 0;
    subdivide_recursive(config.footprint, config, rng, &mut room_id)
}

/// Recursively subdivides a region into rooms.
///
/// Base case: Region is too small to split further → create a Leaf room
/// Recursive case: Split region and recurse on each half
fn subdivide_recursive(
    region: Rect,
    config: &BuildingConfig,
    rng: &mut SeededRng,
    next_id: &mut u32,
) -> BspNode {
    // Check if region can be split along each axis
    let can_split_x = region.width() >= config.min_room_size * 2.0;
    let can_split_y = region.height() >= config.min_room_size * 2.0;

    // Base case: region too small to split → create a room
    if !can_split_x && !can_split_y {
        let id = *next_id;
        *next_id += 1;
        return BspNode::Leaf {
            room: Room::new(id, region),
        };
    }

    // Choose which axis to split along
    let axis = choose_split_axis(region, can_split_x, can_split_y, rng);

    match axis {
        Axis::Horizontal => {
            // Can't split horizontally → create a room
            if !can_split_y {
                let id = *next_id;
                *next_id += 1;
                return BspNode::Leaf {
                    room: Room::new(id, region),
                };
            }

            // Calculate valid split range (ensures min_room_size on both sides)
            let min_split = region.min.y + config.min_room_size;
            let max_split = region.max.y - config.min_room_size;

            if min_split >= max_split {
                let id = *next_id;
                *next_id += 1;
                return BspNode::Leaf {
                    room: Room::new(id, region),
                };
            }

            // Choose random split position, snapped to tile grid
            let split = rng.gen_range(min_split, max_split);
            let snap = snap_to_grid(split, config.tile_size);
            let split_pos = snap.clamp(min_split, max_split);

            // Create two sub-regions
            let top = Rect::new(region.min.x, split_pos, region.max.x, region.max.y);
            let bottom = Rect::new(region.min.x, region.min.y, region.max.x, split_pos);

            BspNode::Internal {
                axis: Axis::Horizontal,
                position: split_pos,
                left: Box::new(subdivide_recursive(top, config, rng, next_id)),
                right: Box::new(subdivide_recursive(bottom, config, rng, next_id)),
            }
        }
        Axis::Vertical => {
            // Can't split vertically → create a room
            if !can_split_x {
                let id = *next_id;
                *next_id += 1;
                return BspNode::Leaf {
                    room: Room::new(id, region),
                };
            }

            // Calculate valid split range
            let min_split = region.min.x + config.min_room_size;
            let max_split = region.max.x - config.min_room_size;

            if min_split >= max_split {
                let id = *next_id;
                *next_id += 1;
                return BspNode::Leaf {
                    room: Room::new(id, region),
                };
            }

            // Choose random split position, snapped to tile grid
            let split = rng.gen_range(min_split, max_split);
            let snap = snap_to_grid(split, config.tile_size);
            let split_pos = snap.clamp(min_split, max_split);

            // Create two sub-regions
            let left = Rect::new(region.min.x, region.min.y, split_pos, region.max.y);
            let right = Rect::new(split_pos, region.min.y, region.max.x, region.max.y);

            BspNode::Internal {
                axis: Axis::Vertical,
                position: split_pos,
                left: Box::new(subdivide_recursive(left, config, rng, next_id)),
                right: Box::new(subdivide_recursive(right, config, rng, next_id)),
            }
        }
    }
}

/// Chooses which axis to split along.
///
/// Strategy:
/// - If only one axis can be split, use that one
/// - If the region is significantly wider than tall (1.25:1), split vertically
/// - If significantly taller than wide, split horizontally
/// - Otherwise, choose randomly (50/50)
fn choose_split_axis(
    region: Rect,
    can_split_x: bool,
    can_split_y: bool,
    rng: &mut SeededRng,
) -> Axis {
    match (can_split_x, can_split_y) {
        (true, false) => Axis::Vertical,
        (false, true) => Axis::Horizontal,
        (true, true) => {
            // Prefer splitting the longer dimension
            if region.width() > region.height() * 1.25 {
                Axis::Vertical
            } else if region.height() > region.width() * 1.25 {
                Axis::Horizontal
            } else if rng.gen_bool(0.5) {
                Axis::Vertical
            } else {
                Axis::Horizontal
            }
        }
        (false, false) => unreachable!(),
    }
}

/// Snaps a value to the nearest tile grid position.
///
/// This ensures split positions align with the tile grid,
/// which produces cleaner wall placement.
fn snap_to_grid(value: f32, tile_size: f32) -> f32 {
    (value / tile_size).round() * tile_size
}

/// Flattens the BSP tree into a list of rooms.
///
/// Traverses the tree in-order, collecting all Leaf nodes.
pub fn collect_rooms(node: &BspNode) -> Vec<Room> {
    let mut rooms = Vec::new();
    collect_rooms_recursive(node, &mut rooms);
    rooms
}

fn collect_rooms_recursive(node: &BspNode, rooms: &mut Vec<Room>) {
    match node {
        BspNode::Leaf { room } => rooms.push(room.clone()),
        BspNode::Internal { left, right, .. } => {
            collect_rooms_recursive(left, rooms);
            collect_rooms_recursive(right, rooms);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_config(target_rooms: usize) -> BuildingConfig {
        BuildingConfig {
            footprint: Rect::new(0.0, 0.0, 10.0, 8.0),
            min_room_size: 2.5,
            target_rooms,
            tile_size: 0.5,
            ..Default::default()
        }
    }

    #[test]
    fn test_bsp_produces_rooms() {
        let config = test_config(4);
        let mut rng = SeededRng::new(42);
        let tree = bsp_subdivide(&config, &mut rng);
        let rooms = collect_rooms(&tree);
        assert!(!rooms.is_empty());
    }

    #[test]
    fn test_bsp_deterministic() {
        let config = test_config(4);
        let mut rng1 = SeededRng::new(42);
        let mut rng2 = SeededRng::new(42);

        let tree1 = bsp_subdivide(&config, &mut rng1);
        let tree2 = bsp_subdivide(&config, &mut rng2);

        let rooms1 = collect_rooms(&tree1);
        let rooms2 = collect_rooms(&tree2);

        assert_eq!(rooms1.len(), rooms2.len());
        for (r1, r2) in rooms1.iter().zip(rooms2.iter()) {
            assert_eq!(r1.bounds, r2.bounds);
        }
    }

    #[test]
    fn test_bsp_rooms_fill_footprint() {
        let config = test_config(4);
        let mut rng = SeededRng::new(42);
        let tree = bsp_subdivide(&config, &mut rng);
        let rooms = collect_rooms(&tree);

        for room in &rooms {
            assert!(room.bounds.width() >= config.min_room_size - 0.01);
            assert!(room.bounds.height() >= config.min_room_size - 0.01);
        }
    }

    #[test]
    fn test_bsp_rooms_no_overlap() {
        let config = test_config(6);
        let mut rng = SeededRng::new(42);
        let tree = bsp_subdivide(&config, &mut rng);
        let rooms = collect_rooms(&tree);

        for i in 0..rooms.len() {
            for j in (i + 1)..rooms.len() {
                assert!(
                    !rooms[i].bounds.intersects(rooms[j].bounds),
                    "Rooms {} and {} overlap",
                    i,
                    j
                );
            }
        }
    }

    #[test]
    fn test_snap_to_grid() {
        assert_eq!(snap_to_grid(1.23, 0.5), 1.0);
        assert_eq!(snap_to_grid(1.26, 0.5), 1.5);
        assert_eq!(snap_to_grid(2.0, 1.0), 2.0);
    }
}
