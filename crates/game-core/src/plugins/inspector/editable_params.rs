use bevy::prelude::*;
use ui_schema::{ControlsSchema, DelegatedControlsSchema};

/// Bevy resource wrapper for editable parameter structs owned by other crates.
#[derive(Resource, Clone, Debug)]
pub struct EditableParams<T: ControlsSchema> {
    /// Wrapped editable parameter value.
    pub value: T,
}

impl<T: ControlsSchema + Default> Default for EditableParams<T> {
    fn default() -> Self {
        Self {
            value: T::default(),
        }
    }
}

impl<T: ControlsSchema> EditableParams<T> {
    /// Creates an editable resource wrapper from a parameter value.
    #[must_use]
    pub const fn new(value: T) -> Self {
        Self { value }
    }
}

impl<T: ControlsSchema> DelegatedControlsSchema for EditableParams<T> {
    type Target = T;

    fn target(&self) -> &Self::Target {
        &self.value
    }

    fn target_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}
