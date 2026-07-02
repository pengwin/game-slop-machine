use bevy::prelude::*;
use std::sync::{
    Arc,
    atomic::{AtomicBool, Ordering},
    mpsc::Sender,
};
use texture_gen::{MipGenerationKind, TextureMaterial, TextureStage, generate_mip_chain};

use super::{
    resources::{
        GenerationMessage, MaterialGeneration, MaterialGenerationProgress,
        MaterialGenerationRequest, MaterialGenerationStatus, MaterialSettings,
    },
    spec::MaterialInspectorSpec,
    tasks::spawn_generation_task,
};
use crate::plugins::inspector::wall_material::{apply_material_settings, bevy_image};

#[allow(clippy::needless_pass_by_value)]
pub(super) fn poll_generation<S: MaterialInspectorSpec>(
    mut commands: Commands<'_, '_>,
    mut generation: Option<ResMut<'_, MaterialGeneration<S>>>,
    mut progress: Option<ResMut<'_, MaterialGenerationProgress<S>>>,
    request: Option<Res<'_, MaterialGenerationRequest<S>>>,
    settings: Res<'_, MaterialSettings>,
    mut images: ResMut<'_, Assets<Image>>,
    mut materials: ResMut<'_, Assets<StandardMaterial>>,
) {
    let Some(generation) = generation.as_deref_mut() else {
        return;
    };
    let Some(progress) = progress.as_deref_mut() else {
        return;
    };

    if let Some(request) = request.as_deref() {
        begin_next_generation(generation, progress, request.params.clone());
        commands.remove_resource::<MaterialGenerationRequest<S>>();
    }

    loop {
        let message = {
            let Ok(receiver) = generation.receiver.lock() else {
                warn!("{} generation receiver lock is poisoned", S::NAME);
                return;
            };
            receiver.try_recv()
        };

        let Ok(message) = message else {
            break;
        };

        match message {
            GenerationMessage::StageFinished(id, stage) => {
                if id != generation.active_id {
                    continue;
                }
                progress.status = MaterialGenerationStatus::Generating(stage);
                progress.fraction = stage.fraction();
            }
            GenerationMessage::Finished(id, texture_set) => {
                if id != generation.active_id {
                    continue;
                }
                generation.albedo = Some(images.add(bevy_image(generate_mip_chain(
                    &texture_set.albedo,
                    MipGenerationKind::Color,
                ))));
                generation.normal = Some(images.add(bevy_image(generate_mip_chain(
                    &texture_set.normal,
                    MipGenerationKind::Normal,
                ))));
                generation.orm = Some(images.add(bevy_image(generate_mip_chain(
                    &texture_set.orm,
                    MipGenerationKind::Color,
                ))));
                progress.fraction = 1.0;
            }
        }
    }

    if generation.applied {
        return;
    }

    let (Some(albedo), Some(normal), Some(orm)) = (
        generation.albedo.clone(),
        generation.normal.clone(),
        generation.orm.clone(),
    ) else {
        return;
    };

    if let Some(mut material) = materials.get_mut(&generation.material) {
        material.base_color_texture = Some(albedo);
        material.normal_map_texture = Some(normal);
        material.metallic_roughness_texture = Some(orm.clone());
        material.occlusion_texture = Some(orm);
        apply_material_settings(&mut material, &settings);
    }

    progress.status = MaterialGenerationStatus::Ready;
    generation.applied = true;
}

fn begin_next_generation<S: MaterialInspectorSpec>(
    generation: &mut MaterialGeneration<S>,
    progress: &mut MaterialGenerationProgress<S>,
    params: <S::Material as TextureMaterial>::Params,
) {
    let id = generation.next_id;
    generation.cancel_active();
    generation.next_id = generation.next_id.saturating_add(1);
    generation.active_id = id;
    generation.cancellation = Arc::new(AtomicBool::new(false));
    generation.albedo = None;
    generation.normal = None;
    generation.orm = None;
    generation.applied = false;
    progress.status = MaterialGenerationStatus::Queued;
    progress.fraction = 0.0;

    spawn_generation_task::<S>(
        generation.sender(),
        id,
        params,
        Arc::clone(&generation.cancellation),
    );
}

impl<S: MaterialInspectorSpec> MaterialGeneration<S> {
    fn sender(&self) -> Sender<GenerationMessage<S>> {
        let (sender, replacement_receiver) = std::sync::mpsc::channel();
        let Ok(mut receiver) = self.receiver.lock() else {
            return sender;
        };
        let old_receiver = std::mem::replace(&mut *receiver, replacement_receiver);
        drop(old_receiver);
        sender
    }

    fn cancel_active(&self) {
        self.cancellation.store(true, Ordering::Relaxed);
    }
}
