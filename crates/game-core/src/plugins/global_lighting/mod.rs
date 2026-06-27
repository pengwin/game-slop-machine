//! Global lighting for game and inspector scenes.

mod preset;
mod presets;
mod scene_lighting_settings;
mod sun;

use bevy::prelude::*;

pub use preset::LightingPreset;
pub use scene_lighting_settings::SceneLightingSettings;

/// Owns the single global sun entity and applies scene lighting presets.
pub struct GlobalLightingPlugin;

impl Plugin for GlobalLightingPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<LightingPreset>();
        sun::plugin(app);
    }
}
