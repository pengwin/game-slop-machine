//! Inspector UI panels for the concrete wall material scene.

mod controls;
mod material_settings;
mod progress;

use bevy::prelude::*;

/// Adds UI panels shown for the concrete wall material scene.
pub struct ConcreteWallMaterialSceneInspectorUiPlugin;

impl Plugin for ConcreteWallMaterialSceneInspectorUiPlugin {
    fn build(&self, app: &mut App) {
        controls::plugin(app);
        material_settings::plugin(app);
        progress::plugin(app);
    }
}
