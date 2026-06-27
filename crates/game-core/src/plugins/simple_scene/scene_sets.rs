use bevy::prelude::*;

use crate::plugins::inspector_scene::InspectorScene;

pub fn plugin(app: &mut App) {
    app.configure_sets(
        OnEnter(InspectorScene::Simple),
        (SimpleSceneSet::Root, SimpleSceneSet::Content).chain(),
    );
}

#[derive(SystemSet, Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum SimpleSceneSet {
    Root,
    Content,
}
