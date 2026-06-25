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

/// Color palette for building mesh materials. Each field is an `[r, g, b]` array with values 0.0–1.0.
#[derive(Debug, Clone)]
pub struct BuildingVisualStyle {
    /// Color for interior wall faces.
    pub wall_color: [f32; 3],
    /// Vertex shading settings for wall faces.
    pub wall_shading: WallVertexShadingSettings,
    /// Color for the top face of walls (slightly darker for depth).
    pub wall_top_color: [f32; 3],
    /// Color for exterior wall faces.
    pub exterior_wall_color: [f32; 3],
    /// Color for exterior corner wall faces.
    pub corner_color: [f32; 3],
    /// Color for T-junction wall faces.
    pub t_junction_color: [f32; 3],
    /// Color for roof surfaces.
    pub roof_color: [f32; 3],
    /// Color for door panels.
    pub door_color: [f32; 3],
    /// Color for opening trim frames.
    pub trim_color: [f32; 3],
    /// Color for the foundation ledge.
    pub foundation_color: [f32; 3],
    /// Color for floor tiles.
    pub floor_color: [f32; 3],
    /// Vertex ambient-occlusion settings for floor shading near walls.
    pub floor_ao: FloorAmbientOcclusionSettings,
    /// Overlay grout-line settings for floor tile seams.
    pub floor_grout: FloorGroutSettings,
    /// Multiplier for procedural dirt/wear mapping (default 1.0).
    pub dirt_intensity: f32,
}

#[derive(Debug, Clone, Copy)]
pub struct WallVertexShadingSettings {
    /// Strength of the vertical bottom darkening on wall faces.
    pub bottom_strength: f32,
    /// World-space falloff rate for bottom darkening.
    pub bottom_falloff: f32,
    /// Brightness multiplier for upward-facing wall-top geometry.
    pub top_face_multiplier: f32,
    /// Brightness multiplier for downward-facing wall geometry.
    pub bottom_face_multiplier: f32,
    /// Directional darkening applied to X-facing side walls.
    pub side_x_strength: f32,
    /// Directional darkening applied to Z-facing side walls.
    pub side_z_strength: f32,
    /// Darkening for a face that points directly into an adjacent wall tile.
    pub adjacent_wall_strength: f32,
    /// Extra bottom-heavy darkening for interior room corners.
    pub interior_corner_strength: f32,
    /// Vertical fade exponent for interior corner darkening.
    pub interior_corner_vertical_falloff: f32,
}

impl Default for WallVertexShadingSettings {
    fn default() -> Self {
        Self {
            bottom_strength: 0.15,
            bottom_falloff: 0.78,
            top_face_multiplier: 1.15,
            bottom_face_multiplier: 0.7,
            side_x_strength: 0.04,
            side_z_strength: 0.12,
            adjacent_wall_strength: 0.2,
            interior_corner_strength: 0.075,
            interior_corner_vertical_falloff: 1.2,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct FloorAmbientOcclusionSettings {
    /// Darkening strength along wall edges, 0.0 disables edge AO.
    pub edge_strength: f32,
    /// Additional darkening where floor vertices are near two wall directions.
    pub corner_strength: f32,
    /// Width of the floor AO falloff, expressed in tile sizes.
    pub width_tiles: f32,
    /// Curve exponent for the fade. Higher values keep the darkest part closer to the wall.
    pub falloff: f32,
    /// Number of subdivisions per floor tile used for vertex-color interpolation.
    pub subdivisions: usize,
}

impl Default for FloorAmbientOcclusionSettings {
    fn default() -> Self {
        Self {
            edge_strength: 0.18,
            corner_strength: 0.12,
            width_tiles: 1.0,
            falloff: 1.8,
            subdivisions: 8,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct FloorGroutSettings {
    /// Grout line width as a multiplier of tile size.
    pub line_width_factor: f32,
    /// Minimum grout line width in world units.
    pub min_line_width: f32,
    /// Maximum grout line width in world units.
    pub max_line_width: f32,
    /// Width multiplier for center seam lines inside each tile.
    pub center_line_scale: f32,
    /// Height offset above the floor mesh to avoid z-fighting.
    pub height_offset: f32,
    /// Number of segments per grout strip for alpha/noise variation.
    pub strip_subdivisions: usize,
    /// Base grout RGB color before warmth variation.
    pub color: [f32; 3],
    /// Base warmth multiplier for grout color.
    pub warmth_base: f32,
    /// Additional warmth variation multiplied by strip noise.
    pub warmth_noise: f32,
    /// Base alpha for grout strips.
    pub alpha_base: f32,
    /// Additional alpha variation multiplied by strip noise.
    pub alpha_noise: f32,
}

impl Default for FloorGroutSettings {
    fn default() -> Self {
        Self {
            line_width_factor: 0.0025,
            min_line_width: 0.0010,
            max_line_width: 0.0022,
            center_line_scale: 1.0,
            height_offset: 0.004,
            strip_subdivisions: 4,
            color: [0.40, 0.36, 0.29],
            warmth_base: 0.88,
            warmth_noise: 0.18,
            alpha_base: 0.004,
            alpha_noise: 0.012,
        }
    }
}

impl Default for BuildingVisualStyle {
    fn default() -> Self {
        Self {
            wall_color: [0.8, 0.8, 0.8],
            wall_shading: WallVertexShadingSettings::default(),
            wall_top_color: [0.34, 0.34, 0.34],
            exterior_wall_color: [0.92, 0.88, 0.68],
            corner_color: [0.96, 0.9, 0.62],
            t_junction_color: [0.86, 0.78, 0.48],
            roof_color: [0.55, 0.35, 0.2],
            door_color: [0.4, 0.2, 0.0],
            trim_color: [0.18, 0.16, 0.13],
            foundation_color: [0.42, 0.42, 0.38],
            floor_color: [0.6, 0.6, 0.6],
            floor_ao: FloorAmbientOcclusionSettings::default(),
            floor_grout: FloorGroutSettings::default(),
            dirt_intensity: 1.2,
        }
    }
}

#[derive(Debug, Clone)]
pub struct BuildingConfig {
    /// Seed for deterministic procedural generation.
    pub seed: u64,
    /// Whether the procedural generation should guarantee a stove for heating (northern houses).
    pub has_stove: bool,
    /// Building footprint rectangle in world space (min corner, max corner).
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
    /// Size of each grid tile in world units. Controls resolution of wall/door placement.
    pub tile_size: f32,
    /// Height of exterior walls in world units.
    pub wall_height: f32,
    /// Thickness of exterior walls in world units. Should be >= tile_size for full tile coverage.
    pub wall_thickness: f32,
    /// Thickness of interior walls between rooms. Typically thinner than exterior walls.
    pub interior_wall_thickness: f32,
    /// Minimum room dimension in world units. Rooms smaller than this are merged or expanded.
    pub min_room_size: f32,
    /// Width of door openings in world units.
    pub door_width: f32,
    /// Height of door openings in world units.
    pub door_height: f32,
    /// Width of window openings in world units.
    pub window_width: f32,
    /// Height of window openings in world units.
    pub window_height: f32,
    /// Distance from floor to bottom of window opening in world units.
    pub window_sill_height: f32,
    /// Minimum spacing between adjacent windows in world units.
    pub window_spacing: f32,
    /// Height of the roof ridge above the wall top in world units.
    pub roof_height: f32,
    /// Horizontal overhang of the roof beyond the wall face in world units.
    pub roof_overhang: f32,
    /// Width of the foundation ledge extending beyond the wall face in world units.
    pub foundation_width: f32,
    /// Inward offset of the foundation wall from the exterior wall face in world units.
    pub foundation_wall_offset: f32,
    /// Height of the foundation above ground in world units.
    pub foundation_height: f32,
    /// Thickness of the trim frame around doors and windows in world units.
    pub opening_trim_thickness: f32,
    /// Depth the trim protrudes from the wall face in world units.
    pub opening_trim_depth: f32,
    /// Whether to render a solid panel for interior doors.
    pub interior_door_render_panel: bool,
    /// Whether to render glass for exterior windows.
    pub exterior_window_render_glass: bool,
    /// Whether to render glass for interior windows.
    pub interior_window_render_glass: bool,
    /// Whether to generate and render roof geometry.
    pub render_roof: bool,
    /// Whether to generate furniture for rooms (kitchen, bedroom, etc.).
    pub furniture: bool,
    /// Color palette for walls, roof, trim, floor, etc.
    pub visual_style: BuildingVisualStyle,
}

impl Default for BuildingConfig {
    fn default() -> Self {
        Self {
            seed: 42,
            has_stove: false,
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
            furniture: true,
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
        assert_eq!(config.visual_style.wall_shading.bottom_strength, 0.15);
        assert_eq!(config.visual_style.wall_shading.bottom_falloff, 0.78);
        assert_eq!(config.visual_style.wall_shading.adjacent_wall_strength, 0.2);
        assert_eq!(
            config.visual_style.wall_shading.interior_corner_strength,
            0.075
        );
        assert_eq!(config.visual_style.floor_ao.edge_strength, 0.18);
        assert_eq!(config.visual_style.floor_ao.corner_strength, 0.12);
        assert_eq!(config.visual_style.floor_ao.width_tiles, 1.0);
        assert_eq!(config.visual_style.floor_ao.subdivisions, 8);
        assert_eq!(config.visual_style.floor_grout.line_width_factor, 0.0025);
        assert_eq!(config.visual_style.floor_grout.height_offset, 0.004);
        assert_eq!(config.visual_style.floor_grout.color, [0.40, 0.36, 0.29]);
        assert_eq!(config.visual_style.floor_grout.alpha_base, 0.004);
        assert_eq!(config.visual_style.floor_grout.alpha_noise, 0.012);
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
