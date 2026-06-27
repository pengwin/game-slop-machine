use bevy::prelude::*;

use super::super::InspectorSceneState;

pub fn plugin(app: &mut App) {
    app.configure_sets(
        OnEnter(InspectorSceneState::Simple),
        (SimpleSceneSet::Root, SimpleSceneSet::Content).chain(),
    );
}

#[derive(SystemSet, Debug, Clone, Eq, PartialEq, Hash)]
pub enum SimpleSceneSet {
    Root,
    Content,
}
