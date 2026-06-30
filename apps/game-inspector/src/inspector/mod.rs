//! Inspector feature wiring.

mod concrete_wall_material_scene;
mod consts;
mod despawn_ui;
mod plaster_wall_material_scene;
mod scene_selector;
mod simple_scene;

use bevy::prelude::*;

use concrete_wall_material_scene::ConcreteWallMaterialSceneInspectorUiPlugin;
use plaster_wall_material_scene::PlasterWallMaterialSceneInspectorUiPlugin;
use simple_scene::SimpleSceneInspectorUiPlugin;

/// Adds the inspector UI and scene selection systems.
pub struct InspectorPlugin;

impl Plugin for InspectorPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            SimpleSceneInspectorUiPlugin,
            PlasterWallMaterialSceneInspectorUiPlugin,
            ConcreteWallMaterialSceneInspectorUiPlugin,
        ));
        scene_selector::plugin(app);
    }
}
