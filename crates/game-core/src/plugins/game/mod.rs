use bevy::prelude::*;
use super::scene::ScenePlugin;

pub fn game_plugin(app: &mut App) {
    app.add_plugins(ScenePlugin);
}
