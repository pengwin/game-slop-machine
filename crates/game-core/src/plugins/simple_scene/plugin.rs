use bevy::prelude::*;

use super::{camera, geometry, lighting, root, scene_sets, InspectorScene};

/// Adds systems for spawning and despawning the simple preview scene.
pub struct SimpleScenePlugin;

impl Plugin for SimpleScenePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<InspectorScene>();

        scene_sets::plugin(app);
        root::plugin(app);
        geometry::plugin(app);
        camera::plugin(app);
        lighting::plugin(app);
    }
}
