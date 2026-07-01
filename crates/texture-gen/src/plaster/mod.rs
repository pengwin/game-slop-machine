//! Plaster texture generation.

mod generate;
mod params;
#[cfg(test)]
mod tests;

pub use generate::{
    PlasterGenerationStage, PlasterTextureSet, generate_plaster_set,
    generate_plaster_set_with_progress, generate_plaster_set_with_progress_and_cancellation,
};
pub use params::PlasterParams;

use crate::TextureSize;
use crate::material::{PbrTextureSet, TextureMaterial};

/// Plaster wall material implementation of `TextureMaterial`.
pub struct PlasterMaterial;

impl TextureMaterial for PlasterMaterial {
    type Params = PlasterParams;
    type Stage = PlasterGenerationStage;

    fn generate(
        params: &Self::Params,
        size: TextureSize,
        on_stage: impl FnMut(Self::Stage),
        cancel: impl Fn() -> bool,
    ) -> Option<PbrTextureSet> {
        generate_plaster_set_with_progress_and_cancellation(params, size, on_stage, cancel)
    }

    fn default_scene_params() -> Self::Params {
        PlasterParams {
            seed: 42,
            ..Default::default()
        }
    }
}
