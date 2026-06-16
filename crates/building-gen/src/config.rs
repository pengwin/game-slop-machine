use crate::geometry::{Rect, Vec2};

/// Specification for a single room in the building.
#[derive(Debug, Clone)]
pub struct RoomSpec {
    /// Room label (e.g. "kitchen", "bedroom", "hall").
    pub name: String,
    /// Target number of exterior windows for this room.
    pub windows: usize,
    /// Minimum acceptable area in world units. Used for validation/scoring hooks.
    pub min_area: f32,
    /// Preferred area in world units. Layout uses this as a relative sizing weight.
    pub preferred_area: f32,
    /// Whether this room should have an exterior wall for windows or access.
    pub exterior_required: bool,
    /// Broad placement preference from public/front rooms to private/deep rooms.
    pub placement: RoomPlacement,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum RoomPlacement {
    NearEntrance,
    Flexible,
    Private,
}

impl RoomSpec {
    pub fn new(name: &str, windows: usize) -> Self {
        let defaults = RoomDefaults::for_name(name);
        Self {
            name: name.to_string(),
            windows,
            min_area: defaults.min_area,
            preferred_area: defaults.preferred_area,
            exterior_required: defaults.exterior_required,
            placement: defaults.placement,
        }
    }

    pub fn with_area(mut self, min_area: f32, preferred_area: f32) -> Self {
        self.min_area = min_area;
        self.preferred_area = preferred_area.max(min_area);
        self
    }

    pub fn with_placement(mut self, placement: RoomPlacement) -> Self {
        self.placement = placement;
        self
    }

    pub fn require_exterior(mut self, exterior_required: bool) -> Self {
        self.exterior_required = exterior_required;
        self
    }
}

struct RoomDefaults {
    min_area: f32,
    preferred_area: f32,
    exterior_required: bool,
    placement: RoomPlacement,
}

impl RoomDefaults {
    fn for_name(name: &str) -> Self {
        match name.trim().to_ascii_lowercase().as_str() {
            "hall" | "foyer" | "entry" | "entrance" => Self {
                min_area: 4.0,
                preferred_area: 7.0,
                exterior_required: false,
                placement: RoomPlacement::NearEntrance,
            },
            "kitchen" => Self {
                min_area: 8.0,
                preferred_area: 14.0,
                exterior_required: true,
                placement: RoomPlacement::NearEntrance,
            },
            "bathroom" | "wc" | "toilet" => Self {
                min_area: 3.0,
                preferred_area: 5.0,
                exterior_required: false,
                placement: RoomPlacement::Flexible,
            },
            "bedroom" => Self {
                min_area: 8.0,
                preferred_area: 12.0,
                exterior_required: true,
                placement: RoomPlacement::Private,
            },
            "storage" | "closet" | "pantry" => Self {
                min_area: 2.0,
                preferred_area: 4.0,
                exterior_required: false,
                placement: RoomPlacement::Private,
            },
            _ => Self {
                min_area: 6.0,
                preferred_area: 10.0,
                exterior_required: true,
                placement: RoomPlacement::Flexible,
            },
        }
    }
}

#[derive(Debug, Clone)]
pub struct BuildingVisualStyle {
    pub wall_color: [f32; 3],
    pub exterior_wall_color: [f32; 3],
    pub corner_color: [f32; 3],
    pub t_junction_color: [f32; 3],
    pub roof_color: [f32; 3],
    pub door_color: [f32; 3],
    pub trim_color: [f32; 3],
    pub foundation_color: [f32; 3],
    pub floor_color: [f32; 3],
}

impl Default for BuildingVisualStyle {
    fn default() -> Self {
        Self {
            wall_color: [0.8, 0.8, 0.8],
            exterior_wall_color: [0.92, 0.88, 0.68],
            corner_color: [0.96, 0.9, 0.62],
            t_junction_color: [0.86, 0.78, 0.48],
            roof_color: [0.55, 0.35, 0.2],
            door_color: [0.4, 0.2, 0.0],
            trim_color: [0.18, 0.16, 0.13],
            foundation_color: [0.42, 0.42, 0.38],
            floor_color: [0.6, 0.6, 0.6],
        }
    }
}

#[derive(Debug, Clone)]
pub struct BuildingConfig {
    pub footprint: Rect,
    /// World-space point on the exterior wall where the main entrance should be placed.
    pub entrance: Vec2,
    /// Direction INTO the building from the entrance (unit vector).
    /// The entrance door is placed on the wall opposite to this direction.
    /// Default: (0, 1) — entering from south wall going north.
    pub entrance_dir: Vec2,
    /// Ordered list of room specifications. First rooms are near the entrance,
    /// last rooms are deepest in the building. Room count = specs length.
    pub room_specs: Vec<RoomSpec>,
    /// If true, a center corridor runs from entrance to back, with rooms on both sides.
    /// If false, rooms connect directly via doorways.
    pub has_corridor: bool,
    /// If true, layout scoring may choose a corridor for larger room programs.
    pub auto_corridor: bool,
    /// Width of the center corridor in grid tiles (only used when has_corridor=true).
    /// Preferred over `corridor_width` because building layout is tile-based.
    pub corridor_width_tiles: usize,
    /// Width of the center corridor in world units (only used when has_corridor=true).
    /// Used only when `corridor_width_tiles` is 0.
    pub corridor_width: f32,
    pub tile_size: f32,
    pub wall_height: f32,
    pub wall_thickness: f32,
    pub interior_wall_thickness: f32,
    /// Minimum room dimension in world units.
    pub min_room_size: f32,
    pub door_width: f32,
    pub door_height: f32,
    pub window_width: f32,
    pub window_height: f32,
    pub window_sill_height: f32,
    pub window_spacing: f32,
    pub roof_height: f32,
    pub roof_overhang: f32,
    pub foundation_width: f32,
    pub foundation_wall_offset: f32,
    pub foundation_height: f32,
    pub opening_trim_thickness: f32,
    pub opening_trim_depth: f32,
    pub interior_door_render_panel: bool,
    pub exterior_window_render_glass: bool,
    pub interior_window_render_glass: bool,
    pub render_roof: bool,
    pub visual_style: BuildingVisualStyle,
}

impl Default for BuildingConfig {
    fn default() -> Self {
        Self {
            footprint: Rect::new(0.0, 0.0, 10.0, 8.0),
            entrance: Vec2::new(5.0, 0.0),
            entrance_dir: Vec2::new(0.0, 1.0),
            room_specs: vec![RoomSpec::new("room", 0)],
            has_corridor: false,
            auto_corridor: false,
            corridor_width_tiles: 2,
            corridor_width: 1.0,
            tile_size: 0.5,
            wall_height: 3.0,
            wall_thickness: 0.5, // Same as tile_size so exterior walls fill their tile
            interior_wall_thickness: 0.2, // Thinner dividers between rooms
            min_room_size: 2.5,
            door_width: 0.9,
            door_height: 2.1,
            window_width: 1.0,
            window_height: 1.2,
            window_sill_height: 0.9,
            window_spacing: 1.5,
            roof_height: 2.0,
            roof_overhang: 0.5,
            foundation_width: 0.25,
            foundation_wall_offset: 0.0,
            foundation_height: 0.12,
            opening_trim_thickness: 0.08,
            opening_trim_depth: 0.05,
            interior_door_render_panel: false,
            exterior_window_render_glass: true,
            interior_window_render_glass: false,
            render_roof: false,
            visual_style: BuildingVisualStyle::default(),
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

    pub fn corridor_width_world(&self) -> f32 {
        if self.corridor_width_tiles > 0 {
            self.corridor_width_tiles as f32 * self.tile_size
        } else {
            self.corridor_width
        }
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
        assert_eq!(config.entrance, Vec2::new(5.0, 0.0));
        assert_eq!(config.corridor_width_tiles, 2);
        assert_eq!(config.corridor_width_world(), 1.0);
        assert_eq!(config.room_specs.len(), 1);
        assert_eq!(config.room_specs[0].name, "room");
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
