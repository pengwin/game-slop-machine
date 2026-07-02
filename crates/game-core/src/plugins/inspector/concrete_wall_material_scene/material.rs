pub use texture_gen::ConcreteGenerationStage;
use texture_gen::{ConcreteMaterial, ConcreteParams};

use super::super::{
    InspectorSceneState,
    material_scene::{
        self, MaterialEditableParams, MaterialGeneration, MaterialGenerationProgress,
        MaterialGenerationRequest, MaterialGenerationStatus, MaterialInspectorSpec,
    },
};

/// Material inspector scene specification for concrete wall materials.
pub struct ConcreteWallMaterialSpec;

impl MaterialInspectorSpec for ConcreteWallMaterialSpec {
    type Material = ConcreteMaterial;

    const STATE: InspectorSceneState = InspectorSceneState::ConcreteWallMaterial;
    const NAME: &'static str = "concrete wall";
}

/// Editable `StandardMaterial` settings for the concrete wall material.
pub type ConcreteWallMaterialSettings = material_scene::MaterialSettings;

/// Editable concrete generator parameters for the inspector scene.
pub type ConcreteWallEditableParams = MaterialEditableParams<ConcreteWallMaterialSpec>;

/// Request to regenerate the concrete wall material from a parameter snapshot.
pub type ConcreteWallGenerationRequest = MaterialGenerationRequest<ConcreteWallMaterialSpec>;

/// Progress reported by the concrete wall material generator.
pub type ConcreteWallGenerationProgress = MaterialGenerationProgress<ConcreteWallMaterialSpec>;

/// User-facing concrete generation status.
pub type ConcreteWallGenerationStatus = MaterialGenerationStatus<ConcreteGenerationStage>;

/// Runtime state for the active concrete generation task.
pub type ConcreteWallGeneration = MaterialGeneration<ConcreteWallMaterialSpec>;

pub fn plugin(app: &mut bevy::prelude::App) {
    material_scene::plugin::<ConcreteWallMaterialSpec>(app);
}

/// Returns the default editable concrete parameters for the debug scene.
#[must_use]
pub fn default_concrete_params() -> ConcreteParams {
    material_scene::default_params::<ConcreteWallMaterialSpec>()
}

pub fn start_concrete_generation(
    commands: &mut bevy::prelude::Commands<'_, '_>,
    material: bevy::prelude::Handle<bevy::prelude::StandardMaterial>,
    params: ConcreteParams,
) {
    material_scene::start_generation::<ConcreteWallMaterialSpec>(commands, material, params);
}
