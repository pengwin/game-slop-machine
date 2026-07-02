//! Generic material inspector scene systems.

mod poll_system;
mod resources;
mod settings_system;
mod spec;
mod tasks;

use bevy::prelude::*;
use texture_gen::TextureMaterial;

pub use resources::{
    MaterialEditableParams, MaterialGeneration, MaterialGenerationProgress,
    MaterialGenerationRequest, MaterialGenerationStatus, MaterialSettings,
};
pub use spec::{MaterialInspectorPlugin, MaterialInspectorSpec};
pub use tasks::start_generation;

use resources::reset_material_resources;
use settings_system::apply_material_settings_system;

/// Adds generic material generation systems for one material inspector scene.
pub fn plugin<S: MaterialInspectorSpec>(app: &mut App) {
    app.init_resource::<MaterialEditableParams<S>>()
        .init_resource::<MaterialSettings>()
        .add_systems(OnEnter(S::STATE), reset_material_resources::<S>)
        .add_systems(
            Update,
            (
                poll_system::poll_generation::<S>,
                apply_material_settings_system::<S>
                    .run_if(resource_exists_and_changed::<MaterialSettings>),
            )
                .run_if(in_state(S::STATE)),
        );
}

/// Returns the default editable parameters for a material inspector scene.
#[must_use]
pub fn default_params<S: MaterialInspectorSpec>()
-> <S::Material as texture_gen::TextureMaterial>::Params {
    S::Material::default_scene_params()
}
