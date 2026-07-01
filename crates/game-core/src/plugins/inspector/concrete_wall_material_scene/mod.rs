//! Debug scene for the generated concrete wall PBR material.

mod camera;
mod geometry;
mod lighting;
mod material;
mod plugin;
mod root;
mod scene_sets;

pub use geometry::{ConcreteWallDirtSettings, ConcreteWallUvSettings};
pub use material::{
    ConcreteGenerationStage, ConcreteWallEditableParams, ConcreteWallGenerationProgress,
    ConcreteWallGenerationRequest, ConcreteWallGenerationStatus, ConcreteWallMaterialSettings,
    default_concrete_params,
};
pub use plugin::ConcreteWallMaterialScenePlugin;
pub use root::ConcreteWallMaterialSceneRoot;
