//! Plaster texture generation.

mod generate;
mod params;
#[cfg(test)]
mod tests;

pub use generate::{
    PlasterTextureSet, TextureKind, generate_plaster_channel, generate_plaster_set, plaster_height,
};
pub use params::{
    PlasterAlbedoParams, PlasterAlbedoShadeParams, PlasterAlbedoTintParams, PlasterHeightParams,
    PlasterNormalParams, PlasterOrmParams, PlasterParams,
};
