use bevy::prelude::*;

use super::super::InspectorSceneState;
use crate::plugins::global_lighting::LightingPreset;

pub fn plugin(app: &mut App) {
    app.add_systems(
        OnEnter(InspectorSceneState::ConcreteWallMaterial),
        configure_concrete_wall_material_scene_lighting,
    )
    .add_systems(
        OnExit(InspectorSceneState::ConcreteWallMaterial),
        restore_default_game_lighting,
    );
}

fn configure_concrete_wall_material_scene_lighting(mut preset: ResMut<'_, LightingPreset>) {
    *preset = LightingPreset::SimplePreview;
}

fn restore_default_game_lighting(mut preset: ResMut<'_, LightingPreset>) {
    *preset = LightingPreset::DefaultGame;
}
