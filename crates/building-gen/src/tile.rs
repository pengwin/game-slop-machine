use crate::geometry::Vec2;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TileType {
    Empty,
    Floor,
    Wall(WallTile),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct WallTile {
    pub kind: WallKind,
    pub shape: WallShape,
    pub opening: Option<WallOpening>,
}

impl WallTile {
    pub fn new(kind: WallKind, shape: WallShape) -> Self {
        Self {
            kind,
            shape,
            opening: None,
        }
    }

    pub fn exterior(shape: WallShape) -> Self {
        Self::new(WallKind::Exterior, shape)
    }

    pub fn interior(shape: WallShape) -> Self {
        Self::new(WallKind::Interior, shape)
    }

    pub fn with_opening(mut self, opening: WallOpening) -> Self {
        self.opening = Some(opening);
        self
    }

    pub fn is_solid(self) -> bool {
        !matches!(
            self.opening,
            Some(WallOpening::Doorway)
                | Some(WallOpening::Door {
                    render_panel: false
                })
                | Some(WallOpening::Window {
                    render_glass: false
                })
        )
    }

    pub fn is_passable(self) -> bool {
        matches!(
            self.opening,
            Some(WallOpening::Doorway)
                | Some(WallOpening::Door {
                    render_panel: false
                })
        )
    }

    pub fn main_axis(self) -> WallAxis {
        match self.shape {
            WallShape::Straight(CardinalDir::Left | CardinalDir::Right) => WallAxis::Z,
            WallShape::Straight(CardinalDir::Bottom | CardinalDir::Top) => WallAxis::X,
            WallShape::Corner(_) | WallShape::TJunction(_) | WallShape::Cross => WallAxis::Both,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum WallKind {
    Exterior,
    Interior,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CardinalDir {
    Left,
    Right,
    Bottom,
    Top,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CornerDir {
    BottomLeft,
    BottomRight,
    TopLeft,
    TopRight,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TJunctionDir {
    Left,
    Right,
    Bottom,
    Top,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum WallShape {
    Straight(CardinalDir),
    Corner(CornerDir),
    TJunction(TJunctionDir),
    Cross,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum WallOpening {
    Door { render_panel: bool },
    Window { render_glass: bool },
    Doorway,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WallAxis {
    X,
    Z,
    Both,
}

impl TileType {
    pub fn is_wall(self) -> bool {
        matches!(self, Self::Wall(_))
    }

    pub fn wall(self) -> Option<WallTile> {
        match self {
            Self::Wall(wall) => Some(wall),
            _ => None,
        }
    }

    pub fn is_solid(self) -> bool {
        matches!(self, Self::Wall(wall) if wall.is_solid())
    }

    pub fn is_opening(self) -> bool {
        matches!(
            self,
            Self::Wall(WallTile {
                opening: Some(_),
                ..
            })
        )
    }

    pub fn is_passable(self) -> bool {
        matches!(self, Self::Floor) || matches!(self, Self::Wall(wall) if wall.is_passable())
    }

    /// Returns true if this tile counts as a room side for wall classification.
    pub fn is_room_adjacent(self) -> bool {
        matches!(self, Self::Floor) || self.is_passable()
    }

    pub fn ascii_char(self) -> char {
        match self {
            Self::Empty => '.',
            Self::Floor => ' ',
            Self::Wall(wall) => match wall.opening {
                Some(WallOpening::Door { render_panel: true }) => 'd',
                Some(
                    WallOpening::Door {
                        render_panel: false,
                    }
                    | WallOpening::Doorway,
                ) => 'D',
                Some(WallOpening::Window { .. }) => 'w',
                None => match wall.shape {
                    WallShape::Corner(_) => '+',
                    WallShape::TJunction(_) => 'T',
                    WallShape::Cross => 'X',
                    WallShape::Straight(CardinalDir::Left | CardinalDir::Right) => '|',
                    WallShape::Straight(CardinalDir::Bottom | CardinalDir::Top) => '-',
                },
            },
        }
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

    pub fn set_wall_opening(&mut self, x: usize, y: usize, opening: WallOpening) -> bool {
        match self.get(x, y) {
            TileType::Wall(wall) => {
                self.set(x, y, TileType::Wall(wall.with_opening(opening)));
                true
            }
            _ => false,
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

    pub fn count_matching_tiles(&self, matches: impl Fn(TileType) -> bool) -> usize {
        self.tiles.iter().filter(|&&t| matches(t)).count()
    }

    pub fn is_room_tile(&self, x: usize, y: usize) -> bool {
        self.get(x, y).is_room_adjacent()
    }

    pub fn is_room_neighbor(&self, x: usize, y: usize, dx: i32, dy: i32) -> bool {
        let nx = x as i32 + dx;
        let ny = y as i32 + dy;
        if nx < 0 || ny < 0 || nx >= self.width as i32 || ny >= self.height as i32 {
            return false;
        }
        self.is_room_tile(nx as usize, ny as usize)
    }

    pub fn is_empty_neighbor(&self, x: usize, y: usize, dx: i32, dy: i32) -> bool {
        let nx = x as i32 + dx;
        let ny = y as i32 + dy;
        if nx < 0 || ny < 0 || nx >= self.width as i32 || ny >= self.height as i32 {
            return true;
        }
        self.get(nx as usize, ny as usize) == TileType::Empty
    }

    pub fn get_neighbor(&self, x: usize, y: usize, dx: i32, dy: i32) -> Option<TileType> {
        let nx = x as i32 + dx;
        let ny = y as i32 + dy;
        if nx < 0 || ny < 0 || nx >= self.width as i32 || ny >= self.height as i32 {
            return None;
        }
        Some(self.get(nx as usize, ny as usize))
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
        let wall = TileType::Wall(WallTile::exterior(WallShape::Straight(CardinalDir::Top)));
        grid.set(3, 4, wall);
        assert_eq!(grid.get(3, 4), wall);
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
        let wall = TileType::Wall(WallTile::exterior(WallShape::Straight(CardinalDir::Top)));
        let open_door = TileType::Wall(
            WallTile::interior(WallShape::Straight(CardinalDir::Top)).with_opening(
                WallOpening::Door {
                    render_panel: false,
                },
            ),
        );
        assert!(wall.is_solid());
        assert!(!TileType::Floor.is_solid());
        assert!(open_door.is_opening());
        assert!(open_door.is_passable());
        assert!(!wall.is_passable());
    }
}
