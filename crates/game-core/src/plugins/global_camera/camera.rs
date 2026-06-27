use bevy::{camera::ScalingMode, prelude::*};

use super::{CameraPreset, SceneCameraSettings};

pub fn plugin(app: &mut App) {
    app.add_systems(Startup, spawn_global_camera).add_systems(
        Update,
        apply_camera_preset.run_if(resource_changed::<CameraPreset>),
    );
}

/// Marker for the single global 3D camera entity.
#[derive(Component, Clone, Default)]
pub struct GlobalCamera3d;

fn spawn_global_camera(mut commands: Commands<'_, '_>, preset: Res<'_, CameraPreset>) {
    let camera = preset.into_inner().settings();

    commands.queue_spawn_scene(camera_scene(&camera));
}

fn apply_camera_preset(
    preset: Res<'_, CameraPreset>,
    mut camera: Query<'_, '_, (&mut Camera, &mut Projection, &mut Transform), With<GlobalCamera3d>>,
) {
    let camera_config = preset.into_inner().settings();

    let Ok((mut camera, mut projection, mut transform)) = camera.single_mut() else {
        warn!("GlobalCamera3d is missing");
        return;
    };

    camera.clear_color = ClearColorConfig::Custom(camera_config.clear_color);
    *projection = Projection::from(orthographic_projection(&camera_config));
    *transform = camera_transform(&camera_config);
}

fn orthographic_projection(camera: &SceneCameraSettings) -> OrthographicProjection {
    OrthographicProjection {
        scaling_mode: ScalingMode::FixedVertical {
            viewport_height: camera.orthographic_viewport_height,
        },
        ..OrthographicProjection::default_3d()
    }
}

fn camera_transform(camera: &SceneCameraSettings) -> Transform {
    Transform::from_translation(camera.translation).looking_at(camera.target, Vec3::Y)
}

fn camera_scene(camera: &SceneCameraSettings) -> impl Scene {
    let clear_color = camera.clear_color;
    let projection = Projection::from(orthographic_projection(camera));
    let transform = camera_transform(camera);

    bsn! {
        (
            Name::new("Global 3D Camera")
            GlobalCamera3d
            Camera3d
            Camera {
                clear_color: { ClearColorConfig::Custom(clear_color) }
            }
            template_value(projection)
            template_value(transform)
        )
    }
}
