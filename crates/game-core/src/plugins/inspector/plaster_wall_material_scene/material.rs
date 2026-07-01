use bevy::{prelude::*, tasks::AsyncComputeTaskPool};
use std::sync::{
    Arc, Mutex,
    atomic::{AtomicBool, Ordering},
    mpsc::{Receiver, Sender, channel},
};
pub use texture_gen::PlasterGenerationStage;
use texture_gen::{
    MipGenerationKind, PlasterParams, PlasterTextureSet, RUNTIME_TEXTURE_SIZE, generate_mip_chain,
    generate_plaster_set_with_progress_and_cancellation,
};

use super::super::wall_material::{WallMaterialSettings, apply_material_settings, bevy_image};
use super::super::{EditableParams, InspectorSceneState};

/// Editable `StandardMaterial` settings for the plaster wall material.
pub type PlasterWallMaterialSettings = WallMaterialSettings;

/// Editable plaster generator parameters for the inspector scene.
pub type PlasterWallEditableParams = EditableParams<PlasterParams>;

pub fn plugin(app: &mut App) {
    app.init_resource::<PlasterWallEditableParams>()
        .init_resource::<PlasterWallMaterialSettings>()
        .add_systems(
            OnEnter(InspectorSceneState::PlasterWallMaterial),
            reset_plaster_material_resources,
        )
        .add_systems(
            Update,
            (
                poll_plaster_generation,
                apply_plaster_wall_material_settings
                    .run_if(resource_exists_and_changed::<PlasterWallMaterialSettings>),
            )
                .run_if(in_state(InspectorSceneState::PlasterWallMaterial)),
        );
}

/// Request to regenerate the plaster wall material from a parameter snapshot.
#[derive(Resource, Clone, Debug)]
pub struct PlasterWallGenerationRequest {
    /// Parameters to use for the next generation.
    pub params: PlasterParams,
}

/// Returns the default editable plaster parameters for the debug scene.
#[must_use]
pub fn default_plaster_params() -> PlasterParams {
    PlasterParams {
        seed: 42,
        ..default()
    }
}

fn reset_plaster_material_resources(
    mut params: ResMut<'_, PlasterWallEditableParams>,
    mut settings: ResMut<'_, PlasterWallMaterialSettings>,
) {
    *params = PlasterWallEditableParams::new(default_plaster_params());
    *settings = PlasterWallMaterialSettings::default();
}

/// Progress reported by the plaster wall material generator.
#[derive(Resource, Clone, Debug)]
pub struct PlasterWallGenerationProgress {
    /// Current generation status.
    pub status: PlasterWallGenerationStatus,
    /// Progress value in the inclusive `0.0..=1.0` range.
    pub fraction: f32,
}

impl Default for PlasterWallGenerationProgress {
    fn default() -> Self {
        Self {
            status: PlasterWallGenerationStatus::Queued,
            fraction: 0.0,
        }
    }
}

/// User-facing generation status.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PlasterWallGenerationStatus {
    /// Generation task is queued.
    Queued,
    /// One pipeline stage has just completed.
    Generating(PlasterGenerationStage),
    /// All channels are ready and applied.
    Ready,
}

impl PlasterWallGenerationStatus {
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

/// Runtime state for the active plaster generation task.
#[derive(Resource)]
pub struct PlasterWallGeneration {
    receiver: Mutex<Receiver<PlasterGenerationMessage>>,
    material: Handle<StandardMaterial>,
    active_id: u64,
    next_id: u64,
    cancellation: Arc<AtomicBool>,
    albedo: Option<Handle<Image>>,
    normal: Option<Handle<Image>>,
    orm: Option<Handle<Image>>,
    applied: bool,
}

enum PlasterGenerationMessage {
    StageFinished(u64, PlasterGenerationStage),
    Finished(u64, PlasterTextureSet),
}

pub fn start_plaster_generation(
    commands: &mut Commands<'_, '_>,
    material: Handle<StandardMaterial>,
    params: PlasterParams,
) {
    let (sender, receiver) = channel();
    let cancellation = Arc::new(AtomicBool::new(false));
    spawn_generation_task(sender, 0, params, Arc::clone(&cancellation));

    commands.insert_resource(PlasterWallGeneration {
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
    commands.insert_resource(PlasterWallGenerationProgress::default());

    info!("Started plaster wall material generation");
}

fn spawn_generation_task(
    sender: Sender<PlasterGenerationMessage>,
    id: u64,
    params: PlasterParams,
    cancellation: Arc<AtomicBool>,
) {
    AsyncComputeTaskPool::get()
        .spawn(async move {
            let progress_sender = sender.clone();
            let progress_cancellation = Arc::clone(&cancellation);
            let texture_set = generate_plaster_set_with_progress_and_cancellation(
                &params,
                RUNTIME_TEXTURE_SIZE,
                |stage| {
                    if progress_cancellation.load(Ordering::Relaxed) {
                        return;
                    }
                    let _ =
                        progress_sender.send(PlasterGenerationMessage::StageFinished(id, stage));
                },
                || cancellation.load(Ordering::Relaxed),
            );
            let Some(texture_set) = texture_set else {
                return;
            };
            let _ = sender.send(PlasterGenerationMessage::Finished(id, texture_set));
        })
        .detach();
}

#[allow(clippy::needless_pass_by_value)]
fn poll_plaster_generation(
    mut commands: Commands<'_, '_>,
    mut generation: Option<ResMut<'_, PlasterWallGeneration>>,
    mut progress: Option<ResMut<'_, PlasterWallGenerationProgress>>,
    request: Option<Res<'_, PlasterWallGenerationRequest>>,
    settings: Res<'_, PlasterWallMaterialSettings>,
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
        request_plaster_generation(generation, progress, request.params.clone());
        commands.remove_resource::<PlasterWallGenerationRequest>();
    }

    loop {
        let message = {
            let Ok(receiver) = generation.receiver.lock() else {
                warn!("Plaster generation receiver lock is poisoned");
                return;
            };
            receiver.try_recv()
        };

        let Ok(message) = message else {
            break;
        };

        match message {
            PlasterGenerationMessage::StageFinished(id, stage) => {
                if id != generation.active_id {
                    continue;
                }
                progress.status = PlasterWallGenerationStatus::Generating(stage);
                progress.fraction = stage.fraction();
            }
            PlasterGenerationMessage::Finished(id, texture_set) => {
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

    progress.status = PlasterWallGenerationStatus::Ready;
    generation.applied = true;
}

#[allow(clippy::needless_pass_by_value)]
fn apply_plaster_wall_material_settings(
    generation: Option<Res<'_, PlasterWallGeneration>>,
    settings: Res<'_, PlasterWallMaterialSettings>,
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

fn request_plaster_generation(
    generation: &mut PlasterWallGeneration,
    progress: &mut PlasterWallGenerationProgress,
    params: PlasterParams,
) {
    begin_next_plaster_generation(generation, progress, params);
}

fn begin_next_plaster_generation(
    generation: &mut PlasterWallGeneration,
    progress: &mut PlasterWallGenerationProgress,
    params: PlasterParams,
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
    progress.status = PlasterWallGenerationStatus::Queued;
    progress.fraction = 0.0;

    spawn_generation_task(
        generation.sender(),
        id,
        params,
        Arc::clone(&generation.cancellation),
    );
}

impl PlasterWallGeneration {
    fn sender(&self) -> Sender<PlasterGenerationMessage> {
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
