use crate::geometry::{LineSegment2D, Rect, Vec2, Vec3};
use crate::tile::TileGrid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RoomId(pub u32);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct WallId(pub u32);

#[derive(Debug, Clone)]
pub struct Room {
    pub id: RoomId,
    pub bounds: Rect,
}

impl Room {
    pub fn new(id: u32, bounds: Rect) -> Self {
        Self {
            id: RoomId(id),
            bounds,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WallType {
    Exterior,
    Interior,
}

#[derive(Debug, Clone)]
pub struct Wall {
    pub id: WallId,
    pub segment: LineSegment2D,
    pub wall_type: WallType,
}

impl Wall {
    pub fn new(id: u32, segment: LineSegment2D, wall_type: WallType) -> Self {
        Self {
            id: WallId(id),
            segment,
            wall_type,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Doorway {
    pub wall_id: WallId,
    pub position: Vec2,
    pub width: f32,
    pub height: f32,
}

#[derive(Debug, Clone)]
pub struct Window {
    pub wall_id: WallId,
    pub position: Vec2,
    pub width: f32,
    pub height: f32,
    pub sill_height: f32,
}

#[derive(Debug, Clone)]
pub struct RoofGeometry {
    pub ridge_start: Vec3,
    pub ridge_end: Vec3,
    pub slope_height: f32,
    pub overhang: f32,
}

#[derive(Debug, Clone)]
pub struct BuildingLayout {
    pub rooms: Vec<Room>,
    pub walls: Vec<Wall>,
    pub doorways: Vec<Doorway>,
    pub windows: Vec<Window>,
    pub tile_grid: TileGrid,
    pub roof: RoofGeometry,
    pub bounds: Rect,
}

impl BuildingLayout {
    pub fn is_connected(&self) -> bool {
        if self.rooms.is_empty() {
            return true;
        }

        if self.rooms.len() == 1 {
            return true;
        }

        let mut room_reachable = vec![false; self.rooms.len()];
        room_reachable[0] = true;
        let mut changed = true;

        while changed {
            changed = false;
            for doorway in &self.doorways {
                let nearby_rooms: Vec<usize> = self
                    .rooms
                    .iter()
                    .enumerate()
                    .filter(|(_, room)| {
                        let expanded = Rect::new(
                            room.bounds.min.x - 1.0,
                            room.bounds.min.y - 1.0,
                            room.bounds.max.x + 1.0,
                            room.bounds.max.y + 1.0,
                        );
                        expanded.contains(doorway.position)
                    })
                    .map(|(i, _)| i)
                    .collect();

                let any_reachable = nearby_rooms.iter().any(|&i| room_reachable[i]);
                if any_reachable {
                    for &i in &nearby_rooms {
                        if !room_reachable[i] {
                            room_reachable[i] = true;
                            changed = true;
                        }
                    }
                }
            }
        }

        room_reachable.iter().all(|&v| v)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_room_creation() {
        let room = Room::new(0, Rect::new(0.0, 0.0, 5.0, 4.0));
        assert_eq!(room.id, RoomId(0));
        assert_eq!(room.bounds.width(), 5.0);
    }

    #[test]
    fn test_wall_types() {
        let ext = Wall::new(
            0,
            LineSegment2D::new(Vec2::ZERO, Vec2::new(10.0, 0.0)),
            WallType::Exterior,
        );
        assert_eq!(ext.wall_type, WallType::Exterior);
    }
}
