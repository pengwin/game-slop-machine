//! Pure procedural texture generation.

mod material;
mod concrete;
mod mip;
mod plaster;
mod surface;
mod texture;

pub use ui_derive::Sliders;

pub use material::{PbrTextureSet, TextureMaterial, TextureStage};
pub use concrete::{
    ConcreteGenerationStage, ConcreteMaterial, ConcreteParams, ConcreteParamsSlider,
    ConcreteTextureSet, generate_concrete_set, generate_concrete_set_with_progress,
    generate_concrete_set_with_progress_and_cancellation,
};
pub use mip::{GeneratedMipTexture, MipGenerationKind, generate_mip_chain};
pub use plaster::{
    PlasterGenerationStage, PlasterMaterial, PlasterParams, PlasterParamsSlider,
    PlasterTextureSet, generate_plaster_set, generate_plaster_set_with_progress,
    generate_plaster_set_with_progress_and_cancellation,
};
pub use texture::{
    GeneratedTexture, RUNTIME_TEXTURE_SIZE, TEST_TEXTURE_SIZE, TextureColorSpace, TextureSize,
};
