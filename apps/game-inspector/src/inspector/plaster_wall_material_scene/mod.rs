//! Inspector UI panels for the plaster wall material scene.

mod controls;
mod material_settings;
mod progress;

use bevy::prelude::*;

/// Adds UI panels shown for the plaster wall material scene.
pub struct PlasterWallMaterialSceneInspectorUiPlugin;

impl Plugin for PlasterWallMaterialSceneInspectorUiPlugin {
    fn build(&self, app: &mut App) {
        controls::plugin(app);
        material_settings::plugin(app);
        progress::plugin(app);
    }
}
