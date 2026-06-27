use bevy::prelude::*;

use crate::plugins::global_camera::CameraPreset;

use super::InspectorScene;

pub fn plugin(app: &mut App) {
    app.add_systems(
        OnEnter(InspectorScene::Simple),
        configure_simple_scene_camera,
    )
    .add_systems(OnExit(InspectorScene::Simple), restore_default_game_camera);
}

fn configure_simple_scene_camera(mut preset: ResMut<'_, CameraPreset>) {
    *preset = CameraPreset::SimplePreview;
}

fn restore_default_game_camera(mut preset: ResMut<'_, CameraPreset>) {
    *preset = CameraPreset::DefaultGame;
}
