//! Inspector scene management: state and child scene plugins.

pub mod concrete_wall_material_scene;
pub mod plaster_wall_material_scene;
pub mod simple_scene;
mod state;

use bevy::prelude::*;

pub use concrete_wall_material_scene::{
    ConcreteWallDirtSettings, ConcreteWallGenerationProgress, ConcreteWallGenerationRequest,
    ConcreteWallMaterialControls, ConcreteWallMaterialScenePlugin, ConcreteWallMaterialSceneRoot,
    ConcreteWallMaterialSettings, ConcreteWallUvSettings,
};
pub use plaster_wall_material_scene::{
    PlasterWallDirtSettings, PlasterWallGenerationProgress, PlasterWallGenerationRequest,
    PlasterWallMaterialControls, PlasterWallMaterialScenePlugin, PlasterWallMaterialSceneRoot,
    PlasterWallMaterialSettings, PlasterWallUvSettings,
};
pub use simple_scene::{SimpleScenePlugin, SimpleSceneRoot};
pub use state::InspectorSceneState;

/// Registers the active inspector scene state and child scene plugins.
pub struct InspectorScenePlugin;

impl Plugin for InspectorScenePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<InspectorSceneState>().add_plugins((
            SimpleScenePlugin,
            PlasterWallMaterialScenePlugin,
            ConcreteWallMaterialScenePlugin,
        ));
    }
}
