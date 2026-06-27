//! Inspector scene management: state and child scene plugins.

pub mod simple_scene;

use bevy::prelude::*;

pub use simple_scene::{SimpleScenePlugin, SimpleSceneRoot};

/// Registers the active inspector scene state and child scene plugins.
pub struct InspectorScenePlugin;

impl Plugin for InspectorScenePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<InspectorSceneState>()
            .add_plugins(SimpleScenePlugin);
    }
}

/// Active scene selected in the inspector.
#[derive(States, Debug, Clone, Default, Eq, PartialEq, Hash)]
pub enum InspectorSceneState {
    /// No inspector scene is currently active.
    #[default]
    None,
    /// The simple preview scene is currently active.
    Simple,
}

impl InspectorSceneState {
    /// Returns the human-readable label shown in inspector UI.
    #[must_use]
    pub const fn label(&self) -> &'static str {
        match self {
            Self::None => "None",
            Self::Simple => "Simple scene",
        }
    }
}
