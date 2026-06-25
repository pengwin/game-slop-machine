use bevy::anti_alias::taa::TemporalAntiAliasing;
use bevy::camera::ScalingMode;
use bevy::core_pipeline::prepass::{DepthPrepass, MotionVectorPrepass, NormalPrepass};
use bevy::core_pipeline::tonemapping::Tonemapping;
use bevy::ecs::message::MessageReader;
use bevy::input::mouse::MouseWheel;
use bevy::light::ShadowFilteringMethod;
use bevy::pbr::{ContactShadows, ScreenSpaceAmbientOcclusion};
use bevy::post_process::dof::{DepthOfField, DepthOfFieldMode};
use bevy::prelude::*;
use bevy::render::camera::TemporalJitter;

use super::camera_config::CameraConfig;

#[derive(Component)]
pub struct MainCamera;

const PAN_SPEED: f32 = 20.0;
const ZOOM_SPEED: f32 = 1.5;
const MIN_ZOOM: f32 = 3.0;
const MAX_ZOOM: f32 = 200.0;

pub fn spawn_camera(
    mut commands: Commands,
    config: Res<CameraConfig>,
    asset_server: Res<AssetServer>,
) {
    commands.spawn((
        MainCamera,
        Name::new("Main Camera"),
        Camera3d::default(),
        Msaa::Off,
        ShadowFilteringMethod::Temporal,
        TemporalJitter::default(),
        TemporalAntiAliasing::default(),
        ContactShadows {
            linear_steps: 16,
            thickness: 0.1,
            length: 0.3,
        },
        ScreenSpaceAmbientOcclusion::default(),
        (DepthPrepass, NormalPrepass, MotionVectorPrepass),
        DepthOfField {
            mode: DepthOfFieldMode::Bokeh,
            focal_distance: 15.0,
            aperture_f_stops: 1.2,
            ..default()
        },
        Tonemapping::AcesFitted,
        Projection::Orthographic(OrthographicProjection {
            scaling_mode: ScalingMode::FixedVertical {
                viewport_height: config.viewport_height,
            },
            ..OrthographicProjection::default_3d()
        }),
        Transform::from_translation(config.position).looking_at(config.target, Vec3::Y),
        EnvironmentMapLight {
            diffuse_map: asset_server.load("pisa_diffuse_rgb9e5_zstd.ktx2"),
            specular_map: asset_server.load("pisa_specular_rgb9e5_zstd.ktx2"),
            intensity: 920.0,
            ..default()
        },
    ));
}

pub fn camera_controls(
    time: Res<Time>,
    input: Res<ButtonInput<KeyCode>>,
    mut scroll_events: MessageReader<MouseWheel>,
    mut cameras: Query<&mut Transform, With<MainCamera>>,
    mut projections: Query<&mut Projection, With<MainCamera>>,
) {
    let dt = time.delta_secs();

    // WASD pan
    let mut delta = Vec2::ZERO;
    if input.pressed(KeyCode::KeyW) || input.pressed(KeyCode::ArrowUp) {
        delta.y += 1.0;
    }
    if input.pressed(KeyCode::KeyS) || input.pressed(KeyCode::ArrowDown) {
        delta.y -= 1.0;
    }
    if input.pressed(KeyCode::KeyA) || input.pressed(KeyCode::ArrowLeft) {
        delta.x -= 1.0;
    }
    if input.pressed(KeyCode::KeyD) || input.pressed(KeyCode::ArrowRight) {
        delta.x += 1.0;
    }

    if delta != Vec2::ZERO {
        delta = delta.normalize() * PAN_SPEED * dt;
        for mut transform in &mut cameras {
            let forward = transform.forward().as_vec3();
            let right = transform.right().as_vec3();
            let move_dir = right * delta.x + forward * delta.y;
            transform.translation += move_dir;
        }
    }

    // Scroll zoom
    let mut scroll = 0.0;
    for event in scroll_events.read() {
        scroll += event.y;
    }

    if scroll != 0.0 {
        for mut projection in &mut projections {
            if let Projection::Orthographic(ref mut ortho) = *projection {
                let factor = 1.0 - scroll * ZOOM_SPEED * 0.01;
                ortho.scaling_mode = ScalingMode::FixedVertical {
                    viewport_height: (ortho.scale * factor).clamp(MIN_ZOOM, MAX_ZOOM),
                };
            }
        }
    }
}
