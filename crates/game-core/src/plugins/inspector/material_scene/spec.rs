use bevy::prelude::*;
use std::{fmt::Debug, marker::PhantomData};
use texture_gen::TextureMaterial;
use ui_schema::ControlsSchema;

use super::plugin;
use crate::plugins::inspector::InspectorSceneState;

/// Bevy plugin wrapper for one generic material inspector subsystem.
pub struct MaterialInspectorPlugin<S: MaterialInspectorSpec>(PhantomData<S>);

impl<S: MaterialInspectorSpec> Default for MaterialInspectorPlugin<S> {
    fn default() -> Self {
        Self(PhantomData)
    }
}

impl<S: MaterialInspectorSpec> Plugin for MaterialInspectorPlugin<S> {
    fn build(&self, app: &mut App) {
        plugin::<S>(app);
    }
}

/// Static configuration for a material inspector scene.
pub trait MaterialInspectorSpec: Send + Sync + 'static
where
    <Self::Material as TextureMaterial>::Params: ControlsSchema + Debug,
    <Self::Material as TextureMaterial>::Stage: Debug,
{
    /// Procedural material generator used by this scene.
    type Material: TextureMaterial;

    /// Inspector scene state that owns this material scene.
    const STATE: InspectorSceneState;
    /// Human-readable material name used in logs.
    const NAME: &'static str;
}
