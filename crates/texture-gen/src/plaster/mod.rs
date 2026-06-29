//! Plaster texture generation.

mod generate;
mod params;
#[cfg(test)]
mod tests;

pub use generate::{
    PlasterGenerationStage, PlasterTextureSet, generate_plaster_set,
    generate_plaster_set_with_progress,
};
pub use params::PlasterParams;
