use super::PbrTextureSet;
use crate::TextureSize;

/// Trait implemented by procedural texture material generators.
///
/// Each material (concrete, plaster, wood, etc.) implements this trait
/// to plug into the generic texture generation pipeline.
pub trait TextureMaterial: Send + Sync + 'static {
    /// Generation parameters (e.g. `ConcreteParams`).
    type Params: Clone + Default + std::hash::Hash + Send + Sync + 'static;
    /// Pipeline stage enum (e.g. `ConcreteGenerationStage`).
    type Stage: Copy + TextureStage + Send + Sync + 'static;

    /// Generates the full PBR texture set with progress reporting and cancellation.
    fn generate(
        params: &Self::Params,
        size: TextureSize,
        on_stage: impl FnMut(Self::Stage),
        cancel: impl Fn() -> bool,
    ) -> Option<PbrTextureSet>;

    /// Returns the default parameters used when entering the scene.
    fn default_scene_params() -> Self::Params;
}

/// Trait for pipeline stage enums.
pub trait TextureStage {
    /// Human-readable label for UI display.
    fn label(&self) -> &'static str;
    /// Progress fraction in `0.0..=1.0` range.
    fn fraction(&self) -> f32;
}
