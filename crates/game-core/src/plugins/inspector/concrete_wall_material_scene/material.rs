use bevy::{
    prelude::*,
    tasks::AsyncComputeTaskPool,
};
use std::sync::{
    Arc, Mutex,
    atomic::{AtomicBool, Ordering},
    mpsc::{Receiver, Sender, channel},
};
pub use texture_gen::ConcreteGenerationStage;
use texture_gen::{
    ConcreteParams, ConcreteTextureSet, MipGenerationKind,
    RUNTIME_TEXTURE_SIZE, generate_concrete_set_with_progress_and_cancellation,
    generate_mip_chain,
};

use super::super::InspectorSceneState;
use super::super::wall_material::{
    WallMaterialSettings, apply_material_settings, bevy_image,
};

/// Editable `StandardMaterial` settings for the concrete wall material.
pub type ConcreteWallMaterialSettings = WallMaterialSettings;

pub fn plugin(app: &mut App) {
    app.init_resource::<ConcreteWallMaterialControls>()
        .init_resource::<ConcreteWallMaterialSettings>()
        .add_systems(
            OnEnter(InspectorSceneState::ConcreteWallMaterial),
            reset_concrete_material_resources,
        )
        .add_systems(
            Update,
            (
                poll_concrete_generation,
                apply_concrete_wall_material_settings
                    .run_if(resource_exists_and_changed::<ConcreteWallMaterialSettings>),
            )
                .run_if(in_state(InspectorSceneState::ConcreteWallMaterial)),
        );
}

/// Editable controls for the concrete wall material generator.
#[derive(Resource, Clone, Debug)]
pub struct ConcreteWallMaterialControls {
    /// Current concrete generation parameters.
    pub params: ConcreteParams,
}

impl Default for ConcreteWallMaterialControls {
    fn default() -> Self {
        Self {
            params: default_concrete_params(),
        }
    }
}

/// Request to regenerate the concrete wall material from a parameter snapshot.
#[derive(Resource, Clone, Debug)]
pub struct ConcreteWallGenerationRequest {
    /// Parameters to use for the next generation.
    pub params: ConcreteParams,
}

/// Returns the default editable concrete parameters for the debug scene.
#[must_use]
pub fn default_concrete_params() -> ConcreteParams {
    ConcreteParams {
        seed: 42,
        ..default()
    }
}

fn reset_concrete_material_resources(
    mut controls: ResMut<'_, ConcreteWallMaterialControls>,
    mut settings: ResMut<'_, ConcreteWallMaterialSettings>,
) {
    *controls = ConcreteWallMaterialControls::default();
    *settings = ConcreteWallMaterialSettings::default();
}

/// Progress reported by the concrete wall material generator.
#[derive(Resource, Clone, Debug)]
pub struct ConcreteWallGenerationProgress {
    /// Current generation status.
    pub status: ConcreteWallGenerationStatus,
    /// Progress value in the inclusive `0.0..=1.0` range.
    pub fraction: f32,
}

impl Default for ConcreteWallGenerationProgress {
    fn default() -> Self {
        Self {
            status: ConcreteWallGenerationStatus::Queued,
            fraction: 0.0,
        }
    }
}

/// User-facing generation status.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ConcreteWallGenerationStatus {
    /// Generation task is queued.
    Queued,
    /// One pipeline stage has just completed.
    Generating(ConcreteGenerationStage),
    /// All channels are ready and applied.
    Ready,
}

impl ConcreteWallGenerationStatus {
    /// Returns the short status label shown in inspector UI.
    #[must_use]
    pub const fn label(&self) -> &'static str {
        match self {
            Self::Queued => "Queued",
            Self::Generating(stage) => stage.label(),
            Self::Ready => "Ready",
        }
    }
}

/// Runtime state for the active concrete generation task.
#[derive(Resource)]
pub struct ConcreteWallGeneration {
    receiver: Mutex<Receiver<ConcreteGenerationMessage>>,
    material: Handle<StandardMaterial>,
    active_id: u64,
    next_id: u64,
    cancellation: Arc<AtomicBool>,
    albedo: Option<Handle<Image>>,
    normal: Option<Handle<Image>>,
    orm: Option<Handle<Image>>,
    applied: bool,
}

enum ConcreteGenerationMessage {
    StageFinished(u64, ConcreteGenerationStage),
    Finished(u64, ConcreteTextureSet),
}

pub fn start_concrete_generation(
    commands: &mut Commands<'_, '_>,
    material: Handle<StandardMaterial>,
    params: ConcreteParams,
) {
    let (sender, receiver) = channel();
    let cancellation = Arc::new(AtomicBool::new(false));
    spawn_generation_task(sender, 0, params, Arc::clone(&cancellation));

    commands.insert_resource(ConcreteWallGeneration {
        receiver: Mutex::new(receiver),
        material,
        active_id: 0,
        next_id: 1,
        cancellation,
        albedo: None,
        normal: None,
        orm: None,
        applied: false,
    });
    commands.insert_resource(ConcreteWallGenerationProgress::default());

    info!("Started concrete wall material generation");
}

fn spawn_generation_task(
    sender: Sender<ConcreteGenerationMessage>,
    id: u64,
    params: ConcreteParams,
    cancellation: Arc<AtomicBool>,
) {
    AsyncComputeTaskPool::get()
        .spawn(async move {
            let progress_sender = sender.clone();
            let progress_cancellation = Arc::clone(&cancellation);
            let texture_set = generate_concrete_set_with_progress_and_cancellation(
                &params,
                RUNTIME_TEXTURE_SIZE,
                |stage| {
                    if progress_cancellation.load(Ordering::Relaxed) {
                        return;
                    }
                    let _ =
                        progress_sender.send(ConcreteGenerationMessage::StageFinished(id, stage));
                },
                || cancellation.load(Ordering::Relaxed),
            );
            let Some(texture_set) = texture_set else {
                return;
            };
            let _ = sender.send(ConcreteGenerationMessage::Finished(id, texture_set));
        })
        .detach();
}

#[allow(clippy::needless_pass_by_value)]
fn poll_concrete_generation(
    mut commands: Commands<'_, '_>,
    mut generation: Option<ResMut<'_, ConcreteWallGeneration>>,
    mut progress: Option<ResMut<'_, ConcreteWallGenerationProgress>>,
    request: Option<Res<'_, ConcreteWallGenerationRequest>>,
    settings: Res<'_, ConcreteWallMaterialSettings>,
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
        request_concrete_generation(generation, progress, request.params.clone());
        commands.remove_resource::<ConcreteWallGenerationRequest>();
    }

    loop {
        let message = {
            let Ok(receiver) = generation.receiver.lock() else {
                warn!("Concrete generation receiver lock is poisoned");
                return;
            };
            receiver.try_recv()
        };

        let Ok(message) = message else {
            break;
        };

        match message {
            ConcreteGenerationMessage::StageFinished(id, stage) => {
                if id != generation.active_id {
                    continue;
                }
                progress.status = ConcreteWallGenerationStatus::Generating(stage);
                progress.fraction = stage.fraction();
            }
            ConcreteGenerationMessage::Finished(id, texture_set) => {
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

    progress.status = ConcreteWallGenerationStatus::Ready;
    generation.applied = true;
}

#[allow(clippy::needless_pass_by_value)]
fn apply_concrete_wall_material_settings(
    generation: Option<Res<'_, ConcreteWallGeneration>>,
    settings: Res<'_, ConcreteWallMaterialSettings>,
    mut materials: ResMut<'_, Assets<StandardMaterial>>,
) {
    let Some(generation) = generation.as_deref() else {
        return;
    };
    let Some(mut material) = materials.get_mut(&generation.material) else {
        return;
    };

    apply_material_settings(&mut material, &settings);
}

fn request_concrete_generation(
    generation: &mut ConcreteWallGeneration,
    progress: &mut ConcreteWallGenerationProgress,
    params: ConcreteParams,
) {
    begin_next_concrete_generation(generation, progress, params);
}

fn begin_next_concrete_generation(
    generation: &mut ConcreteWallGeneration,
    progress: &mut ConcreteWallGenerationProgress,
    params: ConcreteParams,
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
    progress.status = ConcreteWallGenerationStatus::Queued;
    progress.fraction = 0.0;

    spawn_generation_task(
        generation.sender(),
        id,
        params,
        Arc::clone(&generation.cancellation),
    );
}

impl ConcreteWallGeneration {
    fn sender(&self) -> Sender<ConcreteGenerationMessage> {
        let (sender, replacement_receiver) = channel();
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
