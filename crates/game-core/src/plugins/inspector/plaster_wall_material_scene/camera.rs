use bevy::prelude::*;

use super::super::InspectorSceneState;
use crate::plugins::global_camera::CameraPreset;

pub fn plugin(app: &mut App) {
    app.add_systems(
        OnEnter(InspectorSceneState::PlasterWallMaterial),
        configure_plaster_wall_material_scene_camera,
    )
    .add_systems(
        OnExit(InspectorSceneState::PlasterWallMaterial),
        restore_default_game_camera,
    );
}

fn configure_plaster_wall_material_scene_camera(mut preset: ResMut<'_, CameraPreset>) {
    *preset = CameraPreset::DefaultGameIsometricPerspective;
}

fn restore_default_game_camera(mut preset: ResMut<'_, CameraPreset>) {
    *preset = CameraPreset::DefaultGame;
}
