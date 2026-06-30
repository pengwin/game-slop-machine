//! Shared types and helpers for wall material scenes.

mod image_convert;
mod settings;

pub use image_convert::{bevy_image, repeating_linear_sampler};
pub use settings::{WallMaterialSettings, apply_material_settings};
