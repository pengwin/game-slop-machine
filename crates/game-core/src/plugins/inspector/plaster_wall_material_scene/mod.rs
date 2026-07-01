//! Debug scene for the generated plaster wall PBR material.

mod camera;
mod geometry;
mod lighting;
mod material;
mod plugin;
mod root;
mod scene_sets;

pub use geometry::{PlasterWallDirtSettings, PlasterWallUvSettings};
pub use material::{
    PlasterGenerationStage, PlasterWallEditableParams, PlasterWallGenerationProgress,
    PlasterWallGenerationRequest, PlasterWallGenerationStatus, PlasterWallMaterialSettings,
    default_plaster_params,
};
pub use plugin::PlasterWallMaterialScenePlugin;
pub use root::PlasterWallMaterialSceneRoot;
