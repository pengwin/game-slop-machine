//! Inspector feature wiring.

mod camera_effects;
mod camera_presets;
mod global_light;
mod scene_selector;

use bevy::prelude::*;

/// Adds the inspector UI and scene selection systems.
pub struct InspectorPlugin;

impl Plugin for InspectorPlugin {
    fn build(&self, app: &mut App) {
        camera_effects::plugin(app);
        camera_presets::plugin(app);
        global_light::plugin(app);
        scene_selector::plugin(app);
    }
}
