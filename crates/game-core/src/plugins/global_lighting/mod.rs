//! Global lighting for game and inspector scenes.

mod light_controls;
mod preset;
mod presets;
mod scene_lighting_settings;
mod sun;

use bevy::prelude::*;

pub use light_controls::{GlobalLightControls, GlobalLightControlsSlider};
pub use preset::LightingPreset;
pub use scene_lighting_settings::{SceneLightingSettings, SceneShadowCascadeSettings};

/// Owns the single global sun entity and applies scene lighting presets.
pub struct GlobalLightingPlugin;

impl Plugin for GlobalLightingPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<LightingPreset>()
            .init_resource::<GlobalLightControls>();
        sun::plugin(app);
    }
}
