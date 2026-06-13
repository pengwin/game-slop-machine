use crate::geometry::Rect;

#[derive(Debug, Clone)]
pub struct BuildingConfig {
    pub footprint: Rect,
    pub tile_size: f32,
    pub wall_height: f32,
    pub wall_thickness: f32,
    pub min_room_size: f32,
    pub target_rooms: usize,
    pub door_width: f32,
    pub door_height: f32,
    pub window_width: f32,
    pub window_height: f32,
    pub window_sill_height: f32,
    pub window_spacing: f32,
    pub roof_height: f32,
    pub roof_overhang: f32,
}

impl Default for BuildingConfig {
    fn default() -> Self {
        Self {
            footprint: Rect::new(0.0, 0.0, 10.0, 8.0),
            tile_size: 0.5,
            wall_height: 3.0,
            wall_thickness: 0.5,  // Same as tile_size so walls fill their tile
            min_room_size: 2.5,
            target_rooms: 4,
            door_width: 0.9,
            door_height: 2.1,
            window_width: 1.0,
            window_height: 1.2,
            window_sill_height: 0.9,
            window_spacing: 1.5,
            roof_height: 2.0,
            roof_overhang: 0.5,
        }
    }
}

impl BuildingConfig {
    pub fn tiles_x(&self) -> usize {
        (self.footprint.width() / self.tile_size).ceil() as usize
    }

    pub fn tiles_y(&self) -> usize {
        (self.footprint.height() / self.tile_size).ceil() as usize
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = BuildingConfig::default();
        assert_eq!(config.tiles_x(), 20);
        assert_eq!(config.tiles_y(), 16);
    }

    #[test]
    fn test_custom_tile_size() {
        let config = BuildingConfig {
            tile_size: 1.0,
            ..Default::default()
        };
        assert_eq!(config.tiles_x(), 10);
        assert_eq!(config.tiles_y(), 8);
    }
}
