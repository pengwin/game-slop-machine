use bevy::prelude::*;

use super::{camera, geometry, lighting, material, root, scene_sets};

/// Adds systems for the plaster wall material debug scene.
pub struct PlasterWallMaterialScenePlugin;

impl Plugin for PlasterWallMaterialScenePlugin {
    fn build(&self, app: &mut App) {
        scene_sets::plugin(app);
        root::plugin(app);
        geometry::plugin(app);
        material::plugin(app);
        camera::plugin(app);
        lighting::plugin(app);
    }
}
