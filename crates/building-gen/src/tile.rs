use crate::geometry::Vec2;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TileType {
    Empty,
    Floor,
    Wall,
    Doorway,
    Door,
    Window,
    WallCorner,
}

impl TileType {
    pub fn is_solid(self) -> bool {
        matches!(self, Self::Wall | Self::WallCorner)
    }

    pub fn is_opening(self) -> bool {
        matches!(self, Self::Doorway | Self::Door | Self::Window)
    }

    pub fn is_passable(self) -> bool {
        matches!(self, Self::Floor | Self::Doorway | Self::Door)
    }
}

#[derive(Debug, Clone)]
pub struct TileGrid {
    pub width: usize,
    pub height: usize,
    pub tile_size: f32,
    pub origin: Vec2,
    tiles: Vec<TileType>,
}

impl TileGrid {
    pub fn new(width: usize, height: usize, tile_size: f32, origin: Vec2) -> Self {
        Self {
            width,
            height,
            tile_size,
            origin,
            tiles: vec![TileType::Empty; width * height],
        }
    }

    pub fn get(&self, x: usize, y: usize) -> TileType {
        if x < self.width && y < self.height {
            self.tiles[y * self.width + x]
        } else {
            TileType::Empty
        }
    }

    pub fn set(&mut self, x: usize, y: usize, tile: TileType) {
        if x < self.width && y < self.height {
            self.tiles[y * self.width + x] = tile;
        }
    }

    pub fn world_pos(&self, x: usize, y: usize) -> Vec2 {
        Vec2::new(
            self.origin.x + (x as f32 + 0.5) * self.tile_size,
            self.origin.y + (y as f32 + 0.5) * self.tile_size,
        )
    }

    pub fn tile_coord(&self, world_pos: Vec2) -> Option<(usize, usize)> {
        let local = world_pos - self.origin;
        let x = (local.x / self.tile_size).floor() as i32;
        let y = (local.y / self.tile_size).floor() as i32;

        if x >= 0 && y >= 0 && (x as usize) < self.width && (y as usize) < self.height {
            Some((x as usize, y as usize))
        } else {
            None
        }
    }

    pub fn neighbors(&self, x: usize, y: usize) -> Vec<(usize, usize, TileType)> {
        let mut result = Vec::new();
        let offsets: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

        for (dx, dy) in offsets {
            let nx = x as i32 + dx;
            let ny = y as i32 + dy;
            if nx >= 0 && ny >= 0 && (nx as usize) < self.width && (ny as usize) < self.height {
                let ux = nx as usize;
                let uy = ny as usize;
                result.push((ux, uy, self.get(ux, uy)));
            }
        }

        result
    }

    pub fn count_tiles(&self, tile_type: TileType) -> usize {
        self.tiles.iter().filter(|&&t| t == tile_type).count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grid_creation() {
        let grid = TileGrid::new(10, 8, 0.5, Vec2::ZERO);
        assert_eq!(grid.width, 10);
        assert_eq!(grid.height, 8);
        assert_eq!(grid.get(0, 0), TileType::Empty);
    }

    #[test]
    fn test_grid_set_get() {
        let mut grid = TileGrid::new(10, 10, 1.0, Vec2::ZERO);
        grid.set(3, 4, TileType::Wall);
        assert_eq!(grid.get(3, 4), TileType::Wall);
        assert_eq!(grid.get(3, 5), TileType::Empty);
    }

    #[test]
    fn test_world_pos() {
        let grid = TileGrid::new(10, 10, 0.5, Vec2::ZERO);
        let pos = grid.world_pos(2, 3);
        assert_eq!(pos, Vec2::new(1.25, 1.75));
    }

    #[test]
    fn test_tile_coord() {
        let grid = TileGrid::new(10, 10, 0.5, Vec2::ZERO);
        assert_eq!(grid.tile_coord(Vec2::new(1.25, 1.75)), Some((2, 3)));
        assert_eq!(grid.tile_coord(Vec2::new(-1.0, 0.0)), None);
    }

    #[test]
    fn test_neighbors() {
        let grid = TileGrid::new(10, 10, 1.0, Vec2::ZERO);
        let n = grid.neighbors(5, 5);
        assert_eq!(n.len(), 4);
    }

    #[test]
    fn test_tile_properties() {
        assert!(TileType::Wall.is_solid());
        assert!(!TileType::Floor.is_solid());
        assert!(TileType::Door.is_opening());
        assert!(TileType::Doorway.is_passable());
        assert!(!TileType::Wall.is_passable());
    }
}
