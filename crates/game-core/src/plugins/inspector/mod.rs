//! Inspector scene management: state and child scene plugins.

pub mod plaster_wall_material_scene;
pub mod simple_scene;
mod state;

use bevy::prelude::*;

pub use plaster_wall_material_scene::{
    PlasterWallGenerationProgress, PlasterWallGenerationRequest, PlasterWallMaterialControls,
    PlasterWallMaterialScenePlugin, PlasterWallMaterialSceneRoot, PlasterWallMaterialSettings,
};
pub use simple_scene::{SimpleScenePlugin, SimpleSceneRoot};
pub use state::InspectorSceneState;

/// Registers the active inspector scene state and child scene plugins.
pub struct InspectorScenePlugin;

impl Plugin for InspectorScenePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<InspectorSceneState>()
            .add_plugins((SimpleScenePlugin, PlasterWallMaterialScenePlugin));
    }
}
