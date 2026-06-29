//! Pure procedural texture generation.

mod builders;
mod common;
mod noise;
mod plaster;

pub use common::{
    GeneratedTexture, RUNTIME_TEXTURE_SIZE, TEST_TEXTURE_SIZE, TextureColorSpace, TextureSize,
};
pub use plaster::{
    PlasterAlbedoParams, PlasterAlbedoShadeParams, PlasterAlbedoTintParams, PlasterHeightParams,
    PlasterNormalParams, PlasterOrmParams, PlasterParams, PlasterTextureSet, TextureKind,
    generate_plaster_channel, generate_plaster_set, plaster_height,
};
