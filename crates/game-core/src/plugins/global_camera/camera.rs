use bevy::{
    anti_alias::taa::TemporalAntiAliasing,
    camera::{Hdr, ScalingMode},
    core_pipeline::{
        prepass::{DepthPrepass, MotionVectorPrepass, NormalPrepass},
        tonemapping::Tonemapping,
    },
    light::ShadowFilteringMethod,
    pbr::ScreenSpaceAmbientOcclusion,
    prelude::*,
    render::camera::TemporalJitter,
};
use ui_derive::Controls;

use super::{CameraPreset, SceneCameraProjection, SceneCameraSettings};

pub fn plugin(app: &mut App) {
    app.add_systems(Startup, spawn_global_camera).add_systems(
        Update,
        respawn_global_camera
            .run_if(resource_changed::<CameraPreset>.or_else(resource_changed::<CameraEffects>)),
    );
}

/// Marker for the single global 3D camera entity.
#[derive(Component, Clone, Default)]
pub struct GlobalCamera3d;

/// Optional rendering effects applied when the global camera is spawned.
#[allow(
    clippy::struct_excessive_bools,
    reason = "debug UI mirrors camera effect toggles"
)]
#[derive(Resource, Clone, Eq, PartialEq, Controls)]
pub struct CameraEffects {
    /// Disables multisample anti-aliasing for camera effects that require it.
    #[checkbox(label = "Msaa::Off")]
    pub msaa_off: bool,
    /// Enables HDR rendering on the camera.
    #[checkbox(label = "Hdr")]
    pub hdr: bool,
    /// Applies the ACES fitted tonemapper.
    #[checkbox(label = "Tonemapping::AcesFitted")]
    pub tonemapping_aces_fitted: bool,
    /// Enables the depth prepass.
    #[checkbox(label = "DepthPrepass")]
    pub depth_prepass: bool,
    /// Enables the normal prepass.
    #[checkbox(label = "NormalPrepass")]
    pub normal_prepass: bool,
    /// Enables the motion vector prepass.
    #[checkbox(label = "MotionVectorPrepass")]
    pub motion_vector_prepass: bool,
    /// Enables screen space ambient occlusion.
    #[checkbox(label = "ScreenSpaceAmbientOcclusion")]
    pub screen_space_ambient_occlusion: bool,
    /// Enables temporal jitter.
    #[checkbox(label = "TemporalJitter")]
    pub temporal_jitter: bool,
    /// Enables temporal anti-aliasing.
    #[checkbox(label = "TemporalAntiAliasing")]
    pub temporal_anti_aliasing: bool,
    /// Enables temporal shadow filtering (recommended with TAA).
    #[checkbox(label = "ShadowFilter::Temporal")]
    pub shadow_filter_temporal: bool,
}

impl Default for CameraEffects {
    fn default() -> Self {
        Self {
            msaa_off: true,
            hdr: true,
            tonemapping_aces_fitted: true,
            depth_prepass: true,
            normal_prepass: true,
            motion_vector_prepass: true,
            screen_space_ambient_occlusion: true,
            temporal_jitter: true,
            temporal_anti_aliasing: true,
            shadow_filter_temporal: true,
        }
    }
}

#[allow(clippy::needless_pass_by_value)]
fn spawn_global_camera(
    mut commands: Commands<'_, '_>,
    preset: Res<'_, CameraPreset>,
    effects: Res<'_, CameraEffects>,
) {
    spawn_camera(&mut commands, &preset.settings(), &effects);
}

#[allow(clippy::needless_pass_by_value)]
fn respawn_global_camera(
    mut commands: Commands<'_, '_>,
    preset: Res<'_, CameraPreset>,
    effects: Res<'_, CameraEffects>,
    cameras: Query<'_, '_, Entity, With<GlobalCamera3d>>,
) {
    for entity in &cameras {
        commands.entity(entity).despawn();
    }

    spawn_camera(&mut commands, &preset.settings(), &effects);
}

fn orthographic_projection(camera: &SceneCameraSettings) -> OrthographicProjection {
    let SceneCameraProjection::Orthographic { viewport_height } = camera.projection else {
        unreachable!("orthographic_projection called for a non-orthographic camera preset");
    };

    OrthographicProjection {
        scaling_mode: ScalingMode::FixedVertical { viewport_height },
        ..OrthographicProjection::default_3d()
    }
}

fn camera_projection(camera: &SceneCameraSettings) -> Projection {
    match camera.projection {
        SceneCameraProjection::Orthographic { .. } => {
            Projection::from(orthographic_projection(camera))
        }
        SceneCameraProjection::Perspective { fov } => {
            Projection::Perspective(PerspectiveProjection { fov, ..default() })
        }
    }
}

fn camera_transform(camera: &SceneCameraSettings) -> Transform {
    Transform::from_translation(camera.translation).looking_at(camera.target, Vec3::Y)
}

fn spawn_camera(
    commands: &mut Commands<'_, '_>,
    camera: &SceneCameraSettings,
    effects: &CameraEffects,
) {
    let mut spawned_camera = commands.spawn((
        Name::new("Global 3D Camera"),
        GlobalCamera3d,
        Camera3d::default(),
        Camera {
            clear_color: ClearColorConfig::Custom(camera.clear_color),
            ..default()
        },
        camera_projection(camera),
        camera_transform(camera),
    ));

    apply_camera_effects(&mut spawned_camera, effects);

    info!("Spawned GlobalCamera3d");
}

fn apply_camera_effects(camera: &mut EntityCommands<'_>, effects: &CameraEffects) {
    if effects.msaa_off {
        camera.insert(Msaa::Off);
    }
    if effects.hdr {
        camera.insert(Hdr);
    }
    if effects.tonemapping_aces_fitted {
        camera.insert(Tonemapping::AcesFitted);
    }
    if effects.depth_prepass {
        camera.insert(DepthPrepass);
    }
    if effects.normal_prepass {
        camera.insert(NormalPrepass);
    }
    if effects.motion_vector_prepass {
        camera.insert(MotionVectorPrepass);
    }
    if effects.screen_space_ambient_occlusion {
        camera.insert(ScreenSpaceAmbientOcclusion::default());
    }
    if effects.temporal_jitter {
        camera.insert(TemporalJitter::default());
    }
    if effects.temporal_anti_aliasing {
        camera.insert(TemporalAntiAliasing::default());
    }
    if effects.shadow_filter_temporal {
        camera.insert(ShadowFilteringMethod::Temporal);
    } else {
        camera.insert(ShadowFilteringMethod::Gaussian);
    }
}
