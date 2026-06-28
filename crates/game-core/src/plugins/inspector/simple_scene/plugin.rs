use bevy::prelude::*;

use super::{box_motion, camera, geometry, lighting, root, scene_sets};

/// Adds systems for spawning and despawning the simple preview scene.
pub struct SimpleScenePlugin;

impl Plugin for SimpleScenePlugin {
    fn build(&self, app: &mut App) {
        scene_sets::plugin(app);
        root::plugin(app);
        geometry::plugin(app);
        box_motion::plugin(app);
        camera::plugin(app);
        lighting::plugin(app);
    }
}
