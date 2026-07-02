use bevy::prelude::*;
use std::{
    marker::PhantomData,
    sync::{Arc, Mutex, atomic::AtomicBool, mpsc::Receiver},
};
use texture_gen::{PbrTextureSet, TextureMaterial, TextureStage};

use super::{default_params, spec::MaterialInspectorSpec};
use crate::plugins::inspector::{EditableParams, wall_material::WallMaterialSettings};

/// Editable material generator parameters for a material inspector scene.
pub type MaterialEditableParams<S> =
    EditableParams<<<S as MaterialInspectorSpec>::Material as TextureMaterial>::Params>;

/// Editable `StandardMaterial` settings for a material inspector scene.
pub type MaterialSettings = WallMaterialSettings;

/// Request to regenerate a material from a parameter snapshot.
#[derive(Resource, Clone, Debug)]
pub struct MaterialGenerationRequest<S: MaterialInspectorSpec> {
    /// Parameters to use for the next generation.
    pub params: <S::Material as TextureMaterial>::Params,
    marker: PhantomData<S>,
}

impl<S: MaterialInspectorSpec> MaterialGenerationRequest<S> {
    /// Creates a material generation request.
    #[must_use]
    pub const fn new(params: <S::Material as TextureMaterial>::Params) -> Self {
        Self {
            params,
            marker: PhantomData,
        }
    }
}

/// Progress reported by a material generator.
#[derive(Resource, Clone, Debug)]
pub struct MaterialGenerationProgress<S: MaterialInspectorSpec> {
    /// Current generation status.
    pub status: MaterialGenerationStatus<<S::Material as TextureMaterial>::Stage>,
    /// Progress value in the inclusive `0.0..=1.0` range.
    pub fraction: f32,
    marker: PhantomData<S>,
}

impl<S: MaterialInspectorSpec> Default for MaterialGenerationProgress<S> {
    fn default() -> Self {
        Self {
            status: MaterialGenerationStatus::Queued,
            fraction: 0.0,
            marker: PhantomData,
        }
    }
}

/// User-facing material generation status.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum MaterialGenerationStatus<Stage> {
    /// Generation task is queued.
    Queued,
    /// One pipeline stage has just completed.
    Generating(Stage),
    /// All channels are ready and applied.
    Ready,
}

impl<Stage: TextureStage> MaterialGenerationStatus<Stage> {
    /// Returns the short status label shown in inspector UI.
    #[must_use]
    pub fn label(&self) -> &'static str {
        match self {
            Self::Queued => "Queued",
            Self::Generating(stage) => stage.label(),
            Self::Ready => "Ready",
        }
    }
}

/// Runtime state for the active material generation task.
#[derive(Resource)]
pub struct MaterialGeneration<S: MaterialInspectorSpec> {
    pub(super) receiver: Mutex<Receiver<GenerationMessage<S>>>,
    pub(super) material: Handle<StandardMaterial>,
    pub(super) active_id: u64,
    pub(super) next_id: u64,
    pub(super) cancellation: Arc<AtomicBool>,
    pub(super) albedo: Option<Handle<Image>>,
    pub(super) normal: Option<Handle<Image>>,
    pub(super) orm: Option<Handle<Image>>,
    pub(super) applied: bool,
    pub(super) marker: PhantomData<S>,
}

pub(super) enum GenerationMessage<S: MaterialInspectorSpec> {
    StageFinished(u64, <S::Material as TextureMaterial>::Stage),
    Finished(u64, PbrTextureSet),
}

pub(super) fn reset_material_resources<S: MaterialInspectorSpec>(
    mut params: ResMut<'_, MaterialEditableParams<S>>,
    mut settings: ResMut<'_, MaterialSettings>,
) {
    *params = MaterialEditableParams::<S>::new(default_params::<S>());
    *settings = MaterialSettings::default();
}
