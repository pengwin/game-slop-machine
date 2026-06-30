//! Pure procedural texture generation.

mod common;
mod plaster;

pub use common::{
    GeneratedTexture, RUNTIME_TEXTURE_SIZE, TEST_TEXTURE_SIZE, TextureColorSpace, TextureSize,
};
pub use plaster::{
    PlasterGenerationStage, PlasterParams, PlasterTextureSet, generate_plaster_set,
    generate_plaster_set_with_progress, generate_plaster_set_with_progress_and_cancellation,
};
