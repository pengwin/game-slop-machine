//! Shared inspector scene selection state.

use bevy::prelude::*;

/// Registers the active inspector scene state.
pub struct InspectorScenePlugin;

impl Plugin for InspectorScenePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<InspectorScene>();
    }
}

/// Active scene selected in the inspector.
#[derive(States, Debug, Clone, Default, Eq, PartialEq, Hash)]
pub enum InspectorScene {
    /// No inspector scene is currently active.
    #[default]
    None,
    /// The simple preview scene is currently active.
    Simple,
}

impl InspectorScene {
    /// Returns the human-readable label shown in inspector UI.
    #[must_use]
    pub const fn label(&self) -> &'static str {
        match self {
            Self::None => "None",
            Self::Simple => "Simple scene",
        }
    }
}
