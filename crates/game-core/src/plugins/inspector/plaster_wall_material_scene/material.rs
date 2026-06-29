use bevy::{
    asset::RenderAssetUsages,
    image::{ImageAddressMode, ImageFilterMode, ImageSampler, ImageSamplerDescriptor},
    prelude::*,
    render::render_resource::{Extent3d, Face, TextureDimension, TextureFormat},
    tasks::AsyncComputeTaskPool,
};
use std::sync::{
    Mutex,
    mpsc::{Receiver, Sender, channel},
};
use texture_gen::{
    GeneratedTexture, PlasterParams, RUNTIME_TEXTURE_SIZE, TextureColorSpace, TextureKind,
    generate_plaster_channel,
};

use super::super::InspectorSceneState;

pub fn plugin(app: &mut App) {
    app.init_resource::<PlasterWallMaterialControls>()
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

/// Editable controls for the plaster wall material generator.
#[derive(Resource, Clone, Debug)]
pub struct PlasterWallMaterialControls {
    /// Current plaster generation parameters.
    pub params: PlasterParams,
}

impl Default for PlasterWallMaterialControls {
    fn default() -> Self {
        Self {
            params: default_plaster_params(),
        }
    }
}

/// Editable `StandardMaterial` settings for the plaster wall material.
#[derive(Resource, Clone, Debug, PartialEq)]
pub struct PlasterWallMaterialSettings {
    /// Red tint multiplier.
    pub tint_r: f32,
    /// Green tint multiplier.
    pub tint_g: f32,
    /// Blue tint multiplier.
    pub tint_b: f32,
    /// Roughness scalar multiplied with the ORM roughness channel.
    pub perceptual_roughness: f32,
    /// Metallic scalar multiplied with the ORM metallic channel.
    pub metallic: f32,
    /// Specular intensity for the non-metal surface.
    pub reflectance: f32,
    /// Enables two-sided lighting in the PBR shader.
    pub double_sided: bool,
    /// Disables backface culling when true.
    pub cull_none: bool,
    /// Shows base color only, ignoring lighting and maps.
    pub unlit: bool,
}

impl Default for PlasterWallMaterialSettings {
    fn default() -> Self {
        Self {
            tint_r: 1.0,
            tint_g: 1.0,
            tint_b: 1.0,
            perceptual_roughness: 1.0,
            metallic: 1.0,
            reflectance: 0.5,
            double_sided: false,
            cull_none: false,
            unlit: false,
        }
    }
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
    mut controls: ResMut<'_, PlasterWallMaterialControls>,
    mut settings: ResMut<'_, PlasterWallMaterialSettings>,
) {
    *controls = PlasterWallMaterialControls::default();
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
    /// One channel is currently being generated.
    Generating(TextureKind),
    /// All channels are ready and applied.
    Ready,
}

impl PlasterWallGenerationStatus {
    /// Returns the short status label shown in inspector UI.
    #[must_use]
    pub const fn label(&self) -> &'static str {
        match self {
            Self::Queued => "Queued",
            Self::Generating(kind) => kind.label(),
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
    pending_params: Option<PlasterParams>,
    albedo: Option<Handle<Image>>,
    normal: Option<Handle<Image>>,
    orm: Option<Handle<Image>>,
    applied: bool,
}

enum PlasterGenerationMessage {
    Started(u64, TextureKind),
    Finished(u64, TextureKind, GeneratedTexture),
}

pub(super) fn start_plaster_generation(
    commands: &mut Commands<'_, '_>,
    material: Handle<StandardMaterial>,
    params: PlasterParams,
) {
    let (sender, receiver) = channel();
    spawn_generation_task(sender, 0, params);

    commands.insert_resource(PlasterWallGeneration {
        receiver: Mutex::new(receiver),
        material,
        active_id: 0,
        next_id: 1,
        pending_params: None,
        albedo: None,
        normal: None,
        orm: None,
        applied: false,
    });
    commands.insert_resource(PlasterWallGenerationProgress::default());

    info!("Started plaster wall material generation");
}

fn spawn_generation_task(sender: Sender<PlasterGenerationMessage>, id: u64, params: PlasterParams) {
    AsyncComputeTaskPool::get()
        .spawn(async move {
            for kind in [TextureKind::Albedo, TextureKind::Normal, TextureKind::Orm] {
                let _ = sender.send(PlasterGenerationMessage::Started(id, kind));
                let texture = generate_plaster_channel(&params, kind, RUNTIME_TEXTURE_SIZE);
                let _ = sender.send(PlasterGenerationMessage::Finished(id, kind, texture));
            }
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
            PlasterGenerationMessage::Started(id, kind) => {
                if id != generation.active_id {
                    continue;
                }
                progress.status = PlasterWallGenerationStatus::Generating(kind);
            }
            PlasterGenerationMessage::Finished(id, kind, texture) => {
                if id != generation.active_id {
                    continue;
                }
                let handle = images.add(bevy_image(texture));
                match kind {
                    TextureKind::Albedo => {
                        generation.albedo = Some(handle);
                        progress.fraction = 1.0 / 3.0;
                    }
                    TextureKind::Normal => {
                        generation.normal = Some(handle);
                        progress.fraction = 2.0 / 3.0;
                    }
                    TextureKind::Orm => {
                        generation.orm = Some(handle);
                        progress.fraction = 1.0;
                    }
                }
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

    if let Some(params) = generation.pending_params.take() {
        begin_next_plaster_generation(generation, progress, params);
    }
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

#[allow(clippy::missing_const_for_fn)]
pub(super) fn apply_material_settings(
    material: &mut StandardMaterial,
    settings: &PlasterWallMaterialSettings,
) {
    material.base_color = Color::srgba(
        settings.tint_r.clamp(0.0, 2.0),
        settings.tint_g.clamp(0.0, 2.0),
        settings.tint_b.clamp(0.0, 2.0),
        1.0,
    );
    material.perceptual_roughness = settings.perceptual_roughness.clamp(0.0, 1.0);
    material.metallic = settings.metallic.clamp(0.0, 1.0);
    material.reflectance = settings.reflectance.clamp(0.0, 1.0);
    material.double_sided = settings.double_sided;
    material.cull_mode = if settings.cull_none {
        None
    } else {
        Some(Face::Back)
    };
    material.unlit = settings.unlit;
}

fn request_plaster_generation(
    generation: &mut PlasterWallGeneration,
    progress: &mut PlasterWallGenerationProgress,
    params: PlasterParams,
) {
    if generation.applied {
        begin_next_plaster_generation(generation, progress, params);
    } else {
        generation.pending_params = Some(params);
        progress.status = PlasterWallGenerationStatus::Queued;
        progress.fraction = 0.0;
    }
}

fn begin_next_plaster_generation(
    generation: &mut PlasterWallGeneration,
    progress: &mut PlasterWallGenerationProgress,
    params: PlasterParams,
) {
    let id = generation.next_id;
    generation.next_id = generation.next_id.saturating_add(1);
    generation.active_id = id;
    generation.albedo = None;
    generation.normal = None;
    generation.orm = None;
    generation.applied = false;
    progress.status = PlasterWallGenerationStatus::Queued;
    progress.fraction = 0.0;

    spawn_generation_task(generation.sender(), id, params);
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
}

fn bevy_image(texture: GeneratedTexture) -> Image {
    let format = match texture.color_space {
        TextureColorSpace::Srgb => TextureFormat::Rgba8UnormSrgb,
        TextureColorSpace::Linear => TextureFormat::Rgba8Unorm,
    };
    let mut image = Image::new(
        Extent3d {
            width: texture.width,
            height: texture.height,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        texture.data,
        format,
        RenderAssetUsages::RENDER_WORLD | RenderAssetUsages::MAIN_WORLD,
    );
    image.sampler = ImageSampler::Descriptor(repeating_linear_sampler());
    image
}

fn repeating_linear_sampler() -> ImageSamplerDescriptor {
    ImageSamplerDescriptor {
        address_mode_u: ImageAddressMode::Repeat,
        address_mode_v: ImageAddressMode::Repeat,
        address_mode_w: ImageAddressMode::Repeat,
        mag_filter: ImageFilterMode::Linear,
        min_filter: ImageFilterMode::Linear,
        mipmap_filter: ImageFilterMode::Linear,
        ..default()
    }
}
