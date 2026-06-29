//! Debug scene for the generated plaster wall PBR material.

mod camera;
mod geometry;
mod lighting;
mod material;
mod plugin;
mod root;
mod scene_sets;

pub use material::{
    PlasterGenerationStage, PlasterWallGenerationProgress, PlasterWallGenerationRequest,
    PlasterWallGenerationStatus, PlasterWallMaterialControls, PlasterWallMaterialSettings,
};
pub use plugin::PlasterWallMaterialScenePlugin;
pub use root::PlasterWallMaterialSceneRoot;
