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
        }
    }
}
