use bevy::prelude::*;

/// Active scene selected in the inspector.
#[derive(States, Debug, Clone, Default, Eq, PartialEq, Hash)]
pub enum InspectorSceneState {
    /// No inspector scene is currently active.
    #[default]
    None,
    /// The simple preview scene is currently active.
    Simple,
    /// The plaster wall material debug scene is currently active.
    PlasterWallMaterial,
    /// The concrete wall material debug scene is currently active.
    ConcreteWallMaterial,
}

impl InspectorSceneState {
    /// Returns the human-readable label shown in inspector UI.
    #[must_use]
    pub const fn label(&self) -> &'static str {
        match self {
            Self::None => "None",
            Self::Simple => "Simple scene",
            Self::PlasterWallMaterial => "Plaster wall material",
            Self::ConcreteWallMaterial => "Concrete wall material",
        }
    }
}
