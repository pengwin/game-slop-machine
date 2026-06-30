use std::hash::{Hash, Hasher};

/// Complete ancient concrete generation parameters.
#[derive(Clone, Debug)]
pub struct ConcreteParams {
    /// Deterministic texture seed.
    pub seed: u32,
    /// Version included in cache/hash keys by callers.
    pub version: u32,
    /// Main warm lime concrete color, linear-ish RGB.
    pub base_color: [f32; 3],
    /// How strong broad color variation is.
    pub tone_variation: f32,
    /// Cloudy lime/pozzolana variation strength.
    pub lime_cloud_strength: f32,
    /// Number of visible small aggregate flecks.
    pub aggregate_count: u32,
    /// Aggregate albedo contrast.
    pub aggregate_contrast: f32,
    /// Aggregate height contribution.
    pub aggregate_height: f32,
    /// Number of pores and small voids.
    pub void_count: u32,
    /// Void depth in the height field.
    pub void_depth: f32,
    /// Number of soft stain blobs.
    pub stain_count: u32,
    /// How much stains darken albedo.
    pub stain_darkening: f32,
    /// Number of rare hairline cracks.
    pub crack_count: u32,
    /// Crack depth in the height field.
    pub crack_depth: f32,
    /// Fine sandy grain strength in height.
    pub grain_height: f32,
    /// Normal strength multiplier.
    pub normal_strength: f32,
    /// Base ambient occlusion.
    pub ao_base: f32,
    /// Base roughness.
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
            stain_count: 16,
            stain_darkening: 0.13,
            crack_count: 3,
            crack_depth: 0.032,
            grain_height: 0.022,
            normal_strength: 5.5,
            ao_base: 0.93,
            rough_base: 0.88,
        }
    }
}
