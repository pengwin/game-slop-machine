use std::hash::{Hash, Hasher};
use crate::Sliders;

/// Complete ancient concrete generation parameters.
#[derive(Clone, Debug, Sliders)]
pub struct ConcreteParams {
    /// Deterministic texture seed.
    #[slider(min = 0.0, max = 9999.0, step = 1.0, precision = 0)]
    pub seed: u32,
    /// Version included in cache/hash keys by callers.
    pub version: u32,
    /// Main warm lime concrete color, linear-ish RGB.
    pub base_color: [f32; 3],
    /// How strong broad color variation is.
    #[slider(min = 0.0, max = 0.3)]
    pub tone_variation: f32,
    /// Cloudy lime/pozzolana variation strength.
    #[slider(min = 0.0, max = 0.3, label = "Lime clouds")]
    pub lime_cloud_strength: f32,
    /// Number of visible small aggregate flecks.
    #[slider(min = 0.0, max = 800.0, step = 1.0, precision = 0)]
    pub aggregate_count: u32,
    /// Aggregate albedo contrast.
    #[slider(min = 0.0, max = 0.5)]
    pub aggregate_contrast: f32,
    /// Aggregate height contribution.
    #[slider(min = 0.0, max = 0.08, step = 0.001, precision = 3)]
    pub aggregate_height: f32,
    /// Number of pores and small voids.
    #[slider(min = 0.0, max = 260.0, step = 1.0, precision = 0)]
    pub void_count: u32,
    /// Void depth in the height field.
    #[slider(min = 0.0, max = 0.14, step = 0.001, precision = 3)]
    pub void_depth: f32,
    /// Number of soft stain blobs.
    #[slider(min = 0.0, max = 80.0, step = 1.0, precision = 0)]
    pub stain_count: u32,
    /// How much stains darken albedo.
    #[slider(min = 0.0, max = 0.4)]
    pub stain_darkening: f32,
    /// Number of rare hairline cracks.
    #[slider(min = 0.0, max = 30.0, step = 1.0, precision = 0)]
    pub crack_count: u32,
    /// Crack depth in the height field.
    #[slider(min = 0.0, max = 0.14, step = 0.001, precision = 3)]
    pub crack_depth: f32,
    /// Fine sandy grain strength in height.
    #[slider(min = 0.0, max = 0.08, step = 0.001, precision = 3)]
    pub grain_height: f32,
    /// Strength of horizontal formwork board marks.
    #[slider(min = 0.0, max = 0.5)]
    pub formwork_strength: f32,
    /// Number of large exposed aggregate stones.
    #[slider(min = 0.0, max = 40.0, step = 1.0, precision = 0, label = "Exposed agg")]
    pub exposed_aggregate_count: u32,
    /// Height of exposed aggregate stones in the height field.
    #[slider(min = 0.0, max = 0.06, step = 0.001, precision = 3, label = "Exp height")]
    pub exposed_aggregate_height: f32,
    /// Strength of white efflorescence mineral deposits.
    #[slider(min = 0.0, max = 0.4)]
    pub efflorescence_strength: f32,
    /// Normal strength multiplier.
    #[slider(min = 0.0, max = 12.0, step = 0.1, precision = 1)]
    pub normal_strength: f32,
    /// Base ambient occlusion.
    #[slider(min = 0.0, max = 1.0)]
    pub ao_base: f32,
    /// Base roughness.
    #[slider(min = 0.0, max = 1.0)]
    pub rough_base: f32,
}

impl Hash for ConcreteParams {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.seed.hash(state);
        self.version.hash(state);
        self.base_color[0].to_bits().hash(state);
        self.base_color[1].to_bits().hash(state);
        self.base_color[2].to_bits().hash(state);
        self.tone_variation.to_bits().hash(state);
        self.lime_cloud_strength.to_bits().hash(state);
        self.aggregate_count.hash(state);
        self.aggregate_contrast.to_bits().hash(state);
        self.aggregate_height.to_bits().hash(state);
        self.void_count.hash(state);
        self.void_depth.to_bits().hash(state);
        self.stain_count.hash(state);
        self.stain_darkening.to_bits().hash(state);
        self.crack_count.hash(state);
        self.crack_depth.to_bits().hash(state);
        self.grain_height.to_bits().hash(state);
        self.formwork_strength.to_bits().hash(state);
        self.exposed_aggregate_count.hash(state);
        self.exposed_aggregate_height.to_bits().hash(state);
        self.efflorescence_strength.to_bits().hash(state);
        self.normal_strength.to_bits().hash(state);
        self.ao_base.to_bits().hash(state);
        self.rough_base.to_bits().hash(state);
    }
}

impl Default for ConcreteParams {
    fn default() -> Self {
        Self {
            seed: 77,
            version: 1,
            base_color: [0.58, 0.55, 0.48],
            tone_variation: 0.1,
            lime_cloud_strength: 0.12,
            aggregate_count: 220,
            aggregate_contrast: 0.18,
            aggregate_height: 0.018,
            void_count: 85,
            void_depth: 0.05,
            stain_count: 5,
            stain_darkening: 0.13,
            crack_count: 3,
            crack_depth: 0.032,
            grain_height: 0.022,
            formwork_strength: 0.15,
            exposed_aggregate_count: 8,
            exposed_aggregate_height: 0.025,
            efflorescence_strength: 0.12,
            normal_strength: 5.5,
            ao_base: 0.93,
            rough_base: 0.88,
        }
    }
}
