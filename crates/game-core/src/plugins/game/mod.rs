use bevy::prelude::*;
use super::scene::ScenePlugin;
use super::building::BuildingPlugin;

pub fn game_plugin(app: &mut App) {
    app.add_plugins(ScenePlugin);
    app.add_plugins(BuildingPlugin);
}
