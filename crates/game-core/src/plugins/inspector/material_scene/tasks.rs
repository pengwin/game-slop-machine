use bevy::{prelude::*, tasks::AsyncComputeTaskPool};
use std::{
    marker::PhantomData,
    sync::{
        Arc, Mutex,
        atomic::{AtomicBool, Ordering},
        mpsc::{Sender, channel},
    },
};
use texture_gen::{RUNTIME_TEXTURE_SIZE, TextureMaterial};

use super::{
    resources::{GenerationMessage, MaterialGeneration, MaterialGenerationProgress},
    spec::MaterialInspectorSpec,
};

/// Starts a material generation task for a material inspector scene.
pub fn start_generation<S: MaterialInspectorSpec>(
    commands: &mut Commands<'_, '_>,
    material: Handle<StandardMaterial>,
    params: <S::Material as TextureMaterial>::Params,
) {
    let (sender, receiver) = channel();
    let cancellation = Arc::new(AtomicBool::new(false));
    spawn_generation_task::<S>(sender, 0, params, Arc::clone(&cancellation));

    commands.insert_resource(MaterialGeneration::<S> {
        receiver: Mutex::new(receiver),
        material,
        active_id: 0,
        next_id: 1,
        cancellation,
        albedo: None,
        normal: None,
        orm: None,
        applied: false,
        marker: PhantomData,
    });
    commands.insert_resource(MaterialGenerationProgress::<S>::default());

    info!("Started {} material generation", S::NAME);
}

pub(super) fn spawn_generation_task<S: MaterialInspectorSpec>(
    sender: Sender<GenerationMessage<S>>,
    id: u64,
    params: <S::Material as TextureMaterial>::Params,
    cancellation: Arc<AtomicBool>,
) {
    AsyncComputeTaskPool::get()
        .spawn(async move {
            let progress_sender = sender.clone();
            let progress_cancellation = Arc::clone(&cancellation);
            let texture_set = S::Material::generate(
                &params,
                RUNTIME_TEXTURE_SIZE,
                |stage| {
                    if progress_cancellation.load(Ordering::Relaxed) {
                        return;
                    }
                    let _ = progress_sender.send(GenerationMessage::StageFinished(id, stage));
                },
                || cancellation.load(Ordering::Relaxed),
            );
            let Some(texture_set) = texture_set else {
                return;
            };
            let _ = sender.send(GenerationMessage::Finished(id, texture_set));
        })
        .detach();
}
