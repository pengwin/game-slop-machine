use bevy::prelude::*;

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
