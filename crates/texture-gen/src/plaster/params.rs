use std::hash::{Hash, Hasher};
use ui_derive::Controls;

/// Complete plaster generation parameters.
#[derive(Clone, Debug, Controls)]
pub struct PlasterParams {
    /// Deterministic texture seed.
    #[slider(min = 0.0, max = 9999.0, step = 1.0, precision = 0)]
    pub seed: u32,
    /// Version included in cache/hash keys by callers.
    pub version: u32,
    /// Main plaster color, linear-ish RGB.
    pub base_color: [f32; 3],
    /// How strong broad color variation is.
    #[slider(min = 0.0, max = 0.3)]
    pub tone_variation: f32,
    /// Fine sandy grain strength in height.
    #[slider(min = 0.0, max = 0.08, step = 0.001, precision = 3)]
    pub grain_height: f32,
    /// Pit depth in the height field.
    #[slider(min = 0.0, max = 0.12, step = 0.001, precision = 3)]
    pub pit_depth: f32,
    /// Number of tiny pits.
    #[slider(min = 0.0, max = 400.0, step = 1.0, precision = 0, label = "Pores")]
    pub pit_count: u32,
    /// Crack depth in the height field.
    #[slider(min = 0.0, max = 0.14, step = 0.001, precision = 3)]
    pub crack_depth: f32,
    /// Number of generated hairline cracks.
    #[slider(min = 0.0, max = 40.0, step = 1.0, precision = 0)]
    pub crack_count: u32,
    /// How much stains darken albedo.
    #[slider(min = 0.0, max = 0.4)]
    pub stain_darkening: f32,
    /// Number of soft dirt/stain blobs.
    #[slider(min = 0.0, max = 80.0, step = 1.0, precision = 0)]
    pub stain_count: u32,
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

impl Hash for PlasterParams {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.seed.hash(state);
        self.version.hash(state);
        self.base_color[0].to_bits().hash(state);
        self.base_color[1].to_bits().hash(state);
        self.base_color[2].to_bits().hash(state);
        self.tone_variation.to_bits().hash(state);
        self.grain_height.to_bits().hash(state);
        self.pit_depth.to_bits().hash(state);
        self.pit_count.hash(state);
        self.crack_depth.to_bits().hash(state);
        self.crack_count.hash(state);
        self.stain_darkening.to_bits().hash(state);
        self.stain_count.hash(state);
        self.normal_strength.to_bits().hash(state);
        self.ao_base.to_bits().hash(state);
        self.rough_base.to_bits().hash(state);
    }
}

impl Default for PlasterParams {
    fn default() -> Self {
        Self {
            seed: 0,
            version: 2,
            base_color: [0.78, 0.71, 0.60],
            tone_variation: 0.09,
            grain_height: 0.018,
            pit_depth: 0.035,
            pit_count: 120,
            crack_depth: 0.045,
            crack_count: 5,
            stain_darkening: 0.14,
            stain_count: 18,
            normal_strength: 6.0,
            ao_base: 0.94,
            rough_base: 0.9,
        }
    }
}
