//! Inspector feature wiring.

mod scene_selector;

use bevy::prelude::*;

/// Adds the inspector UI and scene selection systems.
pub struct InspectorPlugin;

impl Plugin for InspectorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_ui_camera);
        scene_selector::plugin(app);
    }
}

fn spawn_ui_camera(mut commands: Commands<'_, '_>) {
    commands.spawn((
        Camera2d,
        Camera {
            order: 10,
            clear_color: ClearColorConfig::None,
            ..default()
        },
    ));
}
