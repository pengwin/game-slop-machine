use bevy::camera::ScalingMode;
use bevy::light::ShadowFilteringMethod;
use bevy::prelude::*;

use super::camera_config::CameraConfig;

#[derive(Component)]
pub struct MainCamera;

pub fn spawn_camera(mut commands: Commands, config: Res<CameraConfig>) {
    commands.spawn((
        MainCamera,
        Camera3d::default(),
        ShadowFilteringMethod::Gaussian,
        Projection::Orthographic(OrthographicProjection {
            scaling_mode: ScalingMode::FixedVertical {
                viewport_height: config.viewport_height,
            },
            ..OrthographicProjection::default_3d()
        }),
        Transform::from_translation(config.position).looking_at(config.target, Vec3::Y),
    ));
}
