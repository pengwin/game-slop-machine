use crate::config::BuildingConfig;
use crate::geometry::{Rect, Vec2};
use crate::layout::BuildingLayout;
use crate::tile::{TileType, WallOpening, WallTile};

/// Output of trade district generation: lots, roads, and metadata.
#[derive(Debug, Clone)]
pub struct TradeDistrictLayout {
    pub lots: Vec<Lot>,
    pub buildings: Vec<DistrictBuilding>,
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

#[derive(Debug, Clone)]
pub struct DistrictBuilding {
    pub lot_index: usize,
    pub description_name: String,
    pub config: BuildingConfig,
    pub layout: BuildingLayout,
    pub world_position: Vec2,
    pub rotation: f32,
}

impl DistrictBuilding {
    pub fn exterior_door_position(&self) -> Option<Vec2> {
        let grid = &self.layout.tile_grid;
        for y in 0..grid.height {
            for x in 0..grid.width {
                if matches!(
                    grid.get(x, y),
                    TileType::Wall(WallTile {
                        opening: Some(WallOpening::Door { render_panel: true }),
                        ..
                    })
                ) {
                    return Some(
                        grid.world_pos(x, y) - self.config.entrance_dir * (grid.tile_size / 2.0),
                    );
                }
            }
        }
        None
    }

    pub fn exterior_door_world_position(&self) -> Option<Vec2> {
        self.exterior_door_position()
            .map(|position| self.local_to_world(position))
    }

    fn local_to_world(&self, local: Vec2) -> Vec2 {
        let sin = self.rotation.sin();
        let cos = self.rotation.cos();
        Vec2::new(
            self.world_position.x + local.x * cos + local.y * sin,
            self.world_position.y - local.x * sin + local.y * cos,
        )
    }
}

/// A road segment connecting two points.
#[derive(Debug, Clone, Copy)]
pub struct RoadSegment {
    pub start: Vec2,
    pub end: Vec2,
    pub width: f32,
}
