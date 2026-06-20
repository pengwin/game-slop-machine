use bevy::{
    anti_alias::taa::TemporalAntiAliasing,
    camera::RenderTarget,
    post_process::dof::{DepthOfFieldMode, DepthOfField},
    core_pipeline::prepass::{DepthPrepass, MotionVectorPrepass, NormalPrepass},
    core_pipeline::tonemapping::Tonemapping,
    light::ShadowFilteringMethod,
    pbr::ContactShadows,
    prelude::*,
    render::{
        camera::TemporalJitter,
        render_resource::TextureFormat,
        view::screenshot::{Screenshot, save_to_disk},
    },
};

use crate::{HeadlessFixture, fixtures};
use game_core::plugins::scene::camera_config::CameraConfig;

#[derive(Resource)]
pub struct ScreenshotConfig {
    pub path: String,
    pub width: u32,
    pub height: u32,
}

#[derive(Resource)]
pub struct RenderTargetHandle(Handle<Image>);

pub fn setup_screenshot(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut images: ResMut<Assets<Image>>,
    config: Res<ScreenshotConfig>,
    camera_config: Res<CameraConfig>,
    fixture: Res<HeadlessFixture>,
) {
    let mut image = Image::new_target_texture(
        config.width,
        config.height,
        TextureFormat::Rgba8UnormSrgb,
        None,
    );
    image.texture_descriptor.usage |= bevy::render::render_resource::TextureUsages::COPY_SRC;
    let handle = images.add(image);

    commands.insert_resource(RenderTargetHandle(handle.clone()));

    let mut camera = commands.spawn((
        Camera3d::default(),
        Msaa::Off,
        TemporalJitter::default(),
        TemporalAntiAliasing::default(),
        ShadowFilteringMethod::Gaussian,
        (
            DepthPrepass,
            NormalPrepass,
            MotionVectorPrepass,
        ),
        ContactShadows {
            linear_steps: 16,
            thickness: 0.1,
            length: 0.3,
        },
        DepthOfField {
            mode: DepthOfFieldMode::Bokeh,
            focal_distance: 15.0,
            aperture_f_stops: 1.2,
            ..default()
        },
        Tonemapping::AcesFitted,
        Camera {
            order: 1,
            ..default()
        },
        RenderTarget::Image(handle.into()),
        Projection::Orthographic(OrthographicProjection {
            scaling_mode: bevy::camera::ScalingMode::FixedVertical {
                viewport_height: camera_config.viewport_height,
            },
            ..OrthographicProjection::default_3d()
        }),
        Transform::from_translation(camera_config.position)
            .looking_at(camera_config.target, Vec3::Y),
    ));

    if fixtures::uses_studio_low_poly_render(&fixture.0) {
        camera.insert(EnvironmentMapLight {
            diffuse_map: asset_server.load("pisa_diffuse_rgb9e5_zstd.ktx2"),
            specular_map: asset_server.load("pisa_specular_rgb9e5_zstd.ktx2"),
            intensity: 1_520.0,
            ..default()
        });
    }
}

pub fn capture_and_exit(
    mut commands: Commands,
    config: Res<ScreenshotConfig>,
    handle: Res<RenderTargetHandle>,
    mut done: Local<bool>,
    mut frame_count: Local<u32>,
    mut exit: MessageWriter<AppExit>,
) {
    *frame_count += 1;

    if *frame_count >= 30 && !*done {
        let path = config.path.clone();
        println!("Capturing screenshot to: {}", path);
        commands
            .spawn(Screenshot::image(handle.0.clone()))
            .observe(save_to_disk(path));
        *done = true;
    }

    if *frame_count >= 45 {
        println!("Screenshot capture complete");
        exit.write(AppExit::Success);
    }
}
