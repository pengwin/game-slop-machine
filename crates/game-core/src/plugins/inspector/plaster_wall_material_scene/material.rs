pub use texture_gen::PlasterGenerationStage;
use texture_gen::{PlasterMaterial, PlasterParams};

use super::super::{
    InspectorSceneState,
    material_scene::{
        self, MaterialEditableParams, MaterialGeneration, MaterialGenerationProgress,
        MaterialGenerationRequest, MaterialGenerationStatus, MaterialInspectorSpec,
    },
};

/// Material inspector scene specification for plaster wall materials.
pub struct PlasterWallMaterialSpec;

impl MaterialInspectorSpec for PlasterWallMaterialSpec {
    type Material = PlasterMaterial;

    const STATE: InspectorSceneState = InspectorSceneState::PlasterWallMaterial;
    const NAME: &'static str = "plaster wall";
}

/// Editable `StandardMaterial` settings for the plaster wall material.
pub type PlasterWallMaterialSettings = material_scene::MaterialSettings;

/// Editable plaster generator parameters for the inspector scene.
pub type PlasterWallEditableParams = MaterialEditableParams<PlasterWallMaterialSpec>;

/// Request to regenerate the plaster wall material from a parameter snapshot.
pub type PlasterWallGenerationRequest = MaterialGenerationRequest<PlasterWallMaterialSpec>;

/// Progress reported by the plaster wall material generator.
pub type PlasterWallGenerationProgress = MaterialGenerationProgress<PlasterWallMaterialSpec>;

/// User-facing plaster generation status.
pub type PlasterWallGenerationStatus = MaterialGenerationStatus<PlasterGenerationStage>;

/// Runtime state for the active plaster generation task.
pub type PlasterWallGeneration = MaterialGeneration<PlasterWallMaterialSpec>;

pub fn plugin(app: &mut bevy::prelude::App) {
    material_scene::plugin::<PlasterWallMaterialSpec>(app);
}

/// Returns the default editable plaster parameters for the debug scene.
#[must_use]
pub fn default_plaster_params() -> PlasterParams {
    material_scene::default_params::<PlasterWallMaterialSpec>()
}

pub fn start_plaster_generation(
    commands: &mut bevy::prelude::Commands<'_, '_>,
    material: bevy::prelude::Handle<bevy::prelude::StandardMaterial>,
    params: PlasterParams,
) {
    material_scene::start_generation::<PlasterWallMaterialSpec>(commands, material, params);
}
