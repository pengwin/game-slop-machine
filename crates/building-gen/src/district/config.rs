use crate::config::{BuildingConfig, BuildingVisualStyle, RoomSpec};

/// Configuration for procedural trade district generation.
#[derive(Debug, Clone)]
pub struct TradeDistrictConfig {
    /// Seed for deterministic generation.
    pub seed: u64,
    /// Number of lots to place.
    pub lot_count: usize,
    /// Radius of the town square.
    pub town_square_radius: f32,
    /// Number of radial (spoke) roads from the square outward.
    pub radial_count: usize,
    /// Number of concentric ring roads.
    pub ring_count: usize,
    /// Distance between concentric rings.
    pub ring_spacing: f32,
    /// Width of roads.
    pub road_width: f32,
    /// Minimum gap between lots.
    pub lot_gap: f32,
    /// Width fill across the road segment (1.0 = maximum safe width between radial roads).
    pub lot_width: f32,
    /// Height fill between parallel roads (1.0 = maximum safe radial length).
    pub lot_height: f32,
    /// Entrance setback from the center-facing road (0.0 = minimum safe setback).
    pub lot_depth: f32,
    /// Per-lot random positive width fill offset.
    pub lot_width_randomness: f32,
    /// Per-lot random positive height fill offset.
    pub lot_height_randomness: f32,
    /// Per-lot random positive entrance setback offset.
    pub lot_depth_randomness: f32,
    /// Catalog of building descriptions available for lot conversion.
    pub building_descriptions: Vec<BuildingDescription>,
    /// Inset from lot edges before generating the building footprint.
    pub building_lot_inset: f32,
    /// Maximum number of buildings that can be placed on one lot.
    pub max_buildings_per_lot: usize,
    /// Gap between multiple buildings placed on the same lot.
    pub building_gap: f32,
    /// Parcels at or above this area stay whole so large landmark buildings can occupy them.
    pub preserve_large_lot_area: f32,
    /// For large parcels that choose one building, chance to use the full parcel as a landmark.
    pub landmark_lot_chance: f32,
    /// Width scale for a standalone non-landmark building on a larger parcel.
    pub standalone_lot_width_scale: f32,
    /// Depth scale for a standalone non-landmark building on a larger parcel.
    pub standalone_lot_depth_scale: f32,
    /// Chance that an otherwise splittable medium parcel is split into multiple building lots.
    pub building_lot_split_chance: f32,
    /// Relative chance that a splittable parcel keeps one building.
    pub one_building_lot_weight: f32,
    /// Relative chance that a splittable parcel becomes two medium building lots.
    pub two_building_lot_weight: f32,
    /// Relative chance that a splittable parcel becomes three small building lots.
    pub three_building_lot_weight: f32,
    /// Maximum asymmetry for split parcel frontage. 0.0 means equal halves.
    pub building_lot_split_jitter: f32,
    /// Whether district generation should create a building for each lot.
    pub generate_buildings: bool,
}

/// A reusable building program that can be matched to a district lot.
#[derive(Debug, Clone)]
pub struct BuildingDescription {
    pub name: String,
    pub min_lot_area: f32,
    pub max_lot_area: f32,
    pub rooms: Vec<RoomSpec>,
    pub auto_corridor: bool,
    pub corridor_width_tiles: usize,
    pub render_roof: bool,
    pub config_overrides: BuildingConfigOverrides,
}

/// Optional generation settings carried by a district building preset.
#[derive(Debug, Clone, Default)]
pub struct BuildingConfigOverrides {
    pub footprint_area_scale: Option<f32>,
    pub footprint_aspect: Option<f32>,
    pub wall_height: Option<f32>,
    pub roof_height: Option<f32>,
    pub roof_overhang: Option<f32>,
    pub foundation_width: Option<f32>,
    pub opening_trim_thickness: Option<f32>,
    pub opening_trim_depth: Option<f32>,
    pub window_width: Option<f32>,
    pub window_height: Option<f32>,
    pub window_spacing: Option<f32>,
    pub visual_style: Option<BuildingVisualStyle>,
}

impl BuildingDescription {
    pub fn new(name: &str, min_lot_area: f32, max_lot_area: f32, rooms: Vec<RoomSpec>) -> Self {
        Self {
            name: name.to_string(),
            min_lot_area,
            max_lot_area,
            rooms,
            auto_corridor: true,
            corridor_width_tiles: BuildingConfig::default().corridor_width_tiles,
            render_roof: true,
            config_overrides: BuildingConfigOverrides::default(),
        }
    }

    pub fn with_overrides(mut self, overrides: BuildingConfigOverrides) -> Self {
        self.config_overrides = overrides;
        self
    }

    pub fn with_corridor(mut self, auto_corridor: bool, corridor_width_tiles: usize) -> Self {
        self.auto_corridor = auto_corridor;
        self.corridor_width_tiles = corridor_width_tiles;
        self
    }
}

impl Default for TradeDistrictConfig {
    fn default() -> Self {
        Self {
            seed: 42,
            lot_count: 12,
            town_square_radius: 5.0,
            radial_count: 6,
            ring_count: 2,
            ring_spacing: 14.0,
            road_width: 1.5,
            lot_gap: 1.5,
            lot_width: 0.85,
            lot_height: 0.45,
            lot_depth: 0.0,
            lot_width_randomness: 0.5,
            lot_height_randomness: 0.5,
            lot_depth_randomness: 0.5,
            building_descriptions: default_building_descriptions(),
            building_lot_inset: 0.75,
            max_buildings_per_lot: 3,
            building_gap: 1.0,
            preserve_large_lot_area: 260.0,
            landmark_lot_chance: 0.45,
            standalone_lot_width_scale: 0.52,
            standalone_lot_depth_scale: 0.75,
            building_lot_split_chance: 0.75,
            one_building_lot_weight: 0.35,
            two_building_lot_weight: 0.45,
            three_building_lot_weight: 0.2,
            building_lot_split_jitter: 0.28,
            generate_buildings: true,
        }
    }
}

pub fn default_building_descriptions() -> Vec<BuildingDescription> {
    vec![
        BuildingDescription::new(
            "tiny cottage",
            0.0,
            55.0,
            vec![
                RoomSpec::new("hall", 1),
                RoomSpec::new("kitchen", 1),
                RoomSpec::new("bedroom", 1),
            ],
        )
        .with_corridor(false, 1)
        .with_overrides(BuildingConfigOverrides {
            footprint_area_scale: Some(0.85),
            footprint_aspect: Some(1.05),
            wall_height: Some(2.7),
            roof_height: Some(1.7),
            roof_overhang: Some(0.35),
            foundation_width: Some(0.18),
            opening_trim_thickness: Some(0.06),
            window_width: Some(0.75),
            window_spacing: Some(1.2),
            visual_style: Some(visual_style(
                [0.78, 0.84, 0.68],
                [0.62, 0.22, 0.16],
                [0.28, 0.20, 0.14],
            )),
            ..Default::default()
        }),
        BuildingDescription::new(
            "small shop/home",
            35.0,
            85.0,
            vec![
                RoomSpec::new("hall", 1),
                RoomSpec::new("kitchen", 2),
                RoomSpec::new("bathroom", 0),
                RoomSpec::new("bedroom", 1),
            ],
        )
        .with_overrides(BuildingConfigOverrides {
            footprint_area_scale: Some(1.0),
            footprint_aspect: Some(1.25),
            wall_height: Some(3.0),
            roof_height: Some(1.9),
            window_width: Some(0.9),
            window_spacing: Some(1.4),
            visual_style: Some(visual_style(
                [0.88, 0.78, 0.58],
                [0.56, 0.32, 0.18],
                [0.22, 0.16, 0.12],
            )),
            ..Default::default()
        }),
        BuildingDescription::new(
            "workshop",
            45.0,
            140.0,
            vec![
                RoomSpec::new("entry", 1).with_area(3.0, 5.0),
                RoomSpec::new("room", 2).with_area(10.0, 22.0),
                RoomSpec::new("storage", 0).with_area(4.0, 8.0),
            ],
        )
        .with_corridor(false, 1)
        .with_overrides(BuildingConfigOverrides {
            footprint_area_scale: Some(0.78),
            footprint_aspect: Some(1.8),
            wall_height: Some(3.0),
            roof_height: Some(1.6),
            roof_overhang: Some(0.32),
            foundation_width: Some(0.2),
            window_width: Some(0.7),
            window_spacing: Some(2.0),
            visual_style: Some(visual_style(
                [0.66, 0.64, 0.56],
                [0.35, 0.31, 0.25],
                [0.13, 0.11, 0.09],
            )),
            ..Default::default()
        }),
        BuildingDescription::new(
            "narrow townhouse",
            55.0,
            125.0,
            vec![
                RoomSpec::new("entry", 1),
                RoomSpec::new("kitchen", 1),
                RoomSpec::new("bathroom", 0),
                RoomSpec::new("bedroom", 1),
                RoomSpec::new("storage", 0),
            ],
        )
        .with_corridor(false, 1)
        .with_overrides(BuildingConfigOverrides {
            footprint_area_scale: Some(0.9),
            footprint_aspect: Some(0.75),
            wall_height: Some(3.2),
            roof_height: Some(2.1),
            roof_overhang: Some(0.4),
            window_width: Some(0.8),
            window_spacing: Some(1.1),
            visual_style: Some(visual_style(
                [0.72, 0.76, 0.78],
                [0.34, 0.39, 0.42],
                [0.14, 0.16, 0.17],
            )),
            ..Default::default()
        }),
        BuildingDescription::new(
            "merchant house",
            80.0,
            185.0,
            vec![
                RoomSpec::new("entry", 1),
                RoomSpec::new("hall", 1),
                RoomSpec::new("kitchen", 1),
                RoomSpec::new("room", 2).with_area(8.0, 16.0),
                RoomSpec::new("bedroom", 1),
                RoomSpec::new("storage", 0),
                RoomSpec::new("bathroom", 0),
            ],
        )
        .with_corridor(true, 2)
        .with_overrides(BuildingConfigOverrides {
            footprint_area_scale: Some(0.98),
            footprint_aspect: Some(1.15),
            wall_height: Some(3.35),
            roof_height: Some(2.4),
            roof_overhang: Some(0.55),
            foundation_width: Some(0.26),
            opening_trim_thickness: Some(0.085),
            window_width: Some(0.9),
            window_spacing: Some(1.35),
            visual_style: Some(visual_style(
                [0.76, 0.66, 0.70],
                [0.36, 0.20, 0.24],
                [0.16, 0.09, 0.10],
            )),
            ..Default::default()
        }),
        BuildingDescription::new(
            "medium hall house",
            55.0,
            150.0,
            vec![
                RoomSpec::new("hall", 1),
                RoomSpec::new("kitchen", 2),
                RoomSpec::new("bathroom", 0),
                RoomSpec::new("bedroom", 1),
                RoomSpec::new("bedroom", 1),
                RoomSpec::new("storage", 0),
            ],
        )
        .with_overrides(BuildingConfigOverrides {
            footprint_area_scale: Some(1.05),
            footprint_aspect: Some(1.45),
            wall_height: Some(3.1),
            roof_height: Some(2.0),
            roof_overhang: Some(0.5),
            foundation_width: Some(0.24),
            opening_trim_depth: Some(0.055),
            visual_style: Some(visual_style(
                [0.82, 0.73, 0.62],
                [0.47, 0.25, 0.16],
                [0.20, 0.14, 0.10],
            )),
            ..Default::default()
        }),
        BuildingDescription::new(
            "great warehouse",
            160.0,
            f32::MAX,
            vec![
                RoomSpec::new("entry", 1).with_area(4.0, 8.0),
                RoomSpec::new("room", 2).with_area(30.0, 70.0),
                RoomSpec::new("storage", 0).with_area(15.0, 35.0),
                RoomSpec::new("storage", 0).with_area(15.0, 35.0),
                RoomSpec::new("bathroom", 0),
            ],
        )
        .with_corridor(false, 1)
        .with_overrides(BuildingConfigOverrides {
            footprint_area_scale: Some(1.2),
            footprint_aspect: Some(2.8),
            wall_height: Some(3.7),
            roof_height: Some(2.0),
            roof_overhang: Some(0.5),
            foundation_width: Some(0.3),
            opening_trim_thickness: Some(0.07),
            window_width: Some(0.75),
            window_spacing: Some(2.4),
            visual_style: Some(visual_style(
                [0.63, 0.61, 0.54],
                [0.24, 0.23, 0.21],
                [0.10, 0.09, 0.08],
            )),
            ..Default::default()
        }),
        BuildingDescription::new(
            "large inn",
            110.0,
            220.0,
            vec![
                RoomSpec::new("hall", 1),
                RoomSpec::new("kitchen", 2),
                RoomSpec::new("bathroom", 0),
                RoomSpec::new("bathroom", 0),
                RoomSpec::new("bedroom", 1),
                RoomSpec::new("bedroom", 1),
                RoomSpec::new("storage", 0),
                RoomSpec::new("room", 1),
            ],
        )
        .with_overrides(BuildingConfigOverrides {
            footprint_area_scale: Some(1.15),
            footprint_aspect: Some(1.9),
            wall_height: Some(3.4),
            roof_height: Some(2.3),
            roof_overhang: Some(0.6),
            foundation_width: Some(0.28),
            opening_trim_thickness: Some(0.09),
            opening_trim_depth: Some(0.065),
            window_width: Some(1.0),
            window_height: Some(1.1),
            window_spacing: Some(1.7),
            visual_style: Some(visual_style(
                [0.76, 0.70, 0.58],
                [0.42, 0.24, 0.14],
                [0.16, 0.11, 0.08],
            )),
        }),
        BuildingDescription::new(
            "grand hall",
            180.0,
            f32::MAX,
            vec![
                RoomSpec::new("foyer", 2),
                RoomSpec::new("hall", 3),
                RoomSpec::new("kitchen", 2),
                RoomSpec::new("bathroom", 0),
                RoomSpec::new("bathroom", 0),
                RoomSpec::new("storage", 0),
                RoomSpec::new("room", 2),
                RoomSpec::new("room", 2),
                RoomSpec::new("bedroom", 1),
                RoomSpec::new("bedroom", 1),
                RoomSpec::new("bedroom", 1),
                RoomSpec::new("bedroom", 1),
            ],
        )
        .with_corridor(true, 3)
        .with_overrides(BuildingConfigOverrides {
            footprint_area_scale: Some(1.45),
            footprint_aspect: Some(2.35),
            wall_height: Some(4.1),
            roof_height: Some(2.8),
            roof_overhang: Some(0.75),
            foundation_width: Some(0.34),
            opening_trim_thickness: Some(0.11),
            opening_trim_depth: Some(0.08),
            window_width: Some(1.1),
            window_height: Some(1.25),
            window_spacing: Some(1.9),
            visual_style: Some(visual_style(
                [0.70, 0.68, 0.61],
                [0.30, 0.28, 0.25],
                [0.12, 0.10, 0.09],
            )),
        }),
    ]
}

fn visual_style(wall: [f32; 3], roof: [f32; 3], trim: [f32; 3]) -> BuildingVisualStyle {
    BuildingVisualStyle {
        wall_color: lighten(wall, 0.06),
        wall_top_color: darken(lighten(wall, 0.06), 0.34),
        exterior_wall_color: wall,
        corner_color: lighten(wall, 0.08),
        t_junction_color: darken(wall, 0.08),
        roof_color: roof,
        door_color: darken(trim, 0.02),
        trim_color: trim,
        foundation_color: [0.38, 0.38, 0.34],
        floor_color: [0.55, 0.54, 0.50],
        dirt_intensity: 1.2,
    }
}

fn lighten(color: [f32; 3], amount: f32) -> [f32; 3] {
    [
        (color[0] + amount).min(1.0),
        (color[1] + amount).min(1.0),
        (color[2] + amount).min(1.0),
    ]
}

fn darken(color: [f32; 3], amount: f32) -> [f32; 3] {
    [
        (color[0] - amount).max(0.0),
        (color[1] - amount).max(0.0),
        (color[2] - amount).max(0.0),
    ]
}
