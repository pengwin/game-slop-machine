use bevy::prelude::*;

use super::super::InspectorSceneState;

pub fn plugin(app: &mut App) {
    app.configure_sets(
        OnEnter(InspectorSceneState::PlasterWallMaterial),
        (
            PlasterWallMaterialSceneSet::Root,
            PlasterWallMaterialSceneSet::Content,
        )
            .chain(),
    );
}

#[derive(SystemSet, Debug, Clone, Eq, PartialEq, Hash)]
pub enum PlasterWallMaterialSceneSet {
    Root,
    Content,
}
