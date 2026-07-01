//! Inspector scene management: state and child scene plugins.

pub mod concrete_wall_material_scene;
mod editable_params;
pub mod plaster_wall_material_scene;
pub mod simple_scene;
mod state;
pub mod wall_material;

use bevy::prelude::*;

pub use concrete_wall_material_scene::{
    ConcreteWallDirtSettings, ConcreteWallEditableParams, ConcreteWallGenerationProgress,
    ConcreteWallGenerationRequest, ConcreteWallMaterialScenePlugin, ConcreteWallMaterialSceneRoot,
    ConcreteWallMaterialSettings, ConcreteWallUvSettings,
};
pub use editable_params::EditableParams;
pub use plaster_wall_material_scene::{
    PlasterWallDirtSettings, PlasterWallEditableParams, PlasterWallGenerationProgress,
    PlasterWallGenerationRequest, PlasterWallMaterialScenePlugin, PlasterWallMaterialSceneRoot,
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
