//! Global camera for game and inspector scenes.

mod camera;
mod preset;
mod presets;
mod scene_camera_settings;

use bevy::prelude::*;

pub use preset::CameraPreset;
pub use scene_camera_settings::SceneCameraSettings;

/// Owns the single global 3D camera and applies scene camera presets.
pub struct GlobalCameraPlugin;

impl Plugin for GlobalCameraPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CameraPreset>();
        camera::plugin(app);
    }
}
