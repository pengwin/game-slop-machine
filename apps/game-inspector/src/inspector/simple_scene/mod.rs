//! Inspector UI panels for the simple scene.

mod camera_effects;
mod camera_presets;
mod global_light;

use bevy::prelude::*;

/// Adds UI panels shown for the simple inspector scene.
pub struct SimpleSceneInspectorUiPlugin;

impl Plugin for SimpleSceneInspectorUiPlugin {
    fn build(&self, app: &mut App) {
        camera_effects::plugin(app);
        camera_presets::plugin(app);
        global_light::plugin(app);
    }
}
