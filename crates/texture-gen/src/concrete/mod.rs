mod generate;
mod params;

#[cfg(test)]
mod tests;

pub use generate::{
    ConcreteGenerationStage, ConcreteTextureSet, generate_concrete_set,
    generate_concrete_set_with_progress, generate_concrete_set_with_progress_and_cancellation,
};
pub use params::{ConcreteParams, ConcreteParamsSlider};

use crate::material::{PbrTextureSet, TextureMaterial};
use crate::TextureSize;

/// Concrete wall material implementation of `TextureMaterial`.
pub struct ConcreteMaterial;

impl TextureMaterial for ConcreteMaterial {
    type Params = ConcreteParams;
    type Stage = ConcreteGenerationStage;

    fn generate(
        params: &Self::Params,
        size: TextureSize,
        on_stage: impl FnMut(Self::Stage),
        cancel: impl Fn() -> bool,
    ) -> Option<PbrTextureSet> {
        generate_concrete_set_with_progress_and_cancellation(
            params,
            size,
            on_stage,
            cancel,
        )
    }

    fn default_scene_params() -> Self::Params {
        ConcreteParams {
            seed: 42,
            ..Default::default()
        }
    }
}
