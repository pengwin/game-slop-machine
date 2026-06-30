use bevy::prelude::*;

use super::super::InspectorSceneState;

pub fn plugin(app: &mut App) {
    app.configure_sets(
        OnEnter(InspectorSceneState::ConcreteWallMaterial),
        (
            ConcreteWallMaterialSceneSet::Root,
            ConcreteWallMaterialSceneSet::Content,
        )
            .chain(),
    );
}

#[derive(SystemSet, Debug, Clone, Eq, PartialEq, Hash)]
pub enum ConcreteWallMaterialSceneSet {
    Root,
    Content,
}
