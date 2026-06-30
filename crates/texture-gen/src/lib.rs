//! Pure procedural texture generation.

mod mip;
mod plaster;
mod texture;

pub use mip::{GeneratedMipTexture, MipGenerationKind, generate_mip_chain};
pub use plaster::{
    PlasterGenerationStage, PlasterParams, PlasterTextureSet, generate_plaster_set,
    generate_plaster_set_with_progress, generate_plaster_set_with_progress_and_cancellation,
};
pub use texture::{
    GeneratedTexture, RUNTIME_TEXTURE_SIZE, TEST_TEXTURE_SIZE, TextureColorSpace, TextureSize,
};
