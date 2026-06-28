use bevy::prelude::*;

use super::super::InspectorSceneState;

use super::boxes;

pub const BOX_BOB_AMPLITUDE: f32 = boxes::BOX_SIZE.y * 0.25;

#[derive(Component, Clone, Copy, Default)]
pub struct SimpleSceneBoxMotion {
    pub base_y: f32,
    pub phase: f32,
}

pub fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        animate_simple_scene_boxes.run_if(in_state(InspectorSceneState::Simple)),
    );
}

#[allow(clippy::needless_pass_by_value)]
fn animate_simple_scene_boxes(
    time: Res<'_, Time>,
    mut boxes: Query<'_, '_, (&SimpleSceneBoxMotion, &mut Transform)>,
) {
    let elapsed = time.elapsed_secs();

    for (motion, mut transform) in &mut boxes {
        let wave = elapsed.mul_add(1.8, motion.phase).sin().mul_add(0.5, 0.5);
        transform.translation.y = motion.base_y + wave * BOX_BOB_AMPLITUDE;
    }
}
