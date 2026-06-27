use bevy::prelude::*;

use crate::plugins::{global_lighting::LightingPreset};
use super::super::InspectorSceneState;

pub fn plugin(app: &mut App) {
    app.add_systems(
        OnEnter(InspectorSceneState::Simple),
        configure_simple_scene_lighting,
    )
    .add_systems(
        OnExit(InspectorSceneState::Simple),
        restore_default_game_lighting,
    );
}

fn configure_simple_scene_lighting(mut preset: ResMut<'_, LightingPreset>) {
    *preset = LightingPreset::SimplePreview;
}

fn restore_default_game_lighting(mut preset: ResMut<'_, LightingPreset>) {
    *preset = LightingPreset::DefaultGame;
}
