//! Composable game plugins.

pub mod global_camera;
pub mod global_lighting;
pub mod simple_scene;

use bevy::prelude::*;

/// Composes the reusable game-core preview plugins.
pub struct GameCorePlugin;

impl Plugin for GameCorePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            global_camera::GlobalCameraPlugin,
            global_lighting::GlobalLightingPlugin,
            simple_scene::SimpleScenePlugin,
        ));
    }
}
