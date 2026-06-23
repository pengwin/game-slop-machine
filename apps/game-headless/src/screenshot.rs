use bevy::{
    anti_alias::taa::TemporalAntiAliasing,
    camera::RenderTarget,
    core_pipeline::prepass::{DepthPrepass, MotionVectorPrepass, NormalPrepass},
    core_pipeline::tonemapping::Tonemapping,
    light::ShadowFilteringMethod,
    pbr::ContactShadows,
    post_process::dof::{DepthOfField, DepthOfFieldMode},
    prelude::*,
    render::{
        camera::TemporalJitter,
        render_resource::TextureFormat,
        view::screenshot::{Screenshot, save_to_disk},
    },
};

use crate::{HeadlessFixture, fixtures};
use game_core::plugins::building::procedural_texture::ProceduralTextures;
use game_core::plugins::scene::camera_config::CameraConfig;
use std::path::Path;

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
    println!(
        "Headless screenshot setup: fixture={}, output={}, size={}x{}",
        fixture.0, config.path, config.width, config.height
    );
    ensure_parent_dir(&config.path);

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
        ShadowFilteringMethod::Gaussian,
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

    if !fixtures::is_texture_fixture(&fixture.0) {
        camera.insert((
            TemporalJitter::default(),
            TemporalAntiAliasing::default(),
            (DepthPrepass, NormalPrepass, MotionVectorPrepass),
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
        ));
    }

    if fixtures::uses_studio_low_poly_render(&fixture.0)
        && !fixtures::is_texture_fixture(&fixture.0)
    {
        camera.insert(EnvironmentMapLight {
            diffuse_map: asset_server.load("pisa_diffuse_rgb9e5_zstd.ktx2"),
            specular_map: asset_server.load("pisa_specular_rgb9e5_zstd.ktx2"),
            intensity: 920.0,
            ..default()
        });
    }
}

pub fn capture_and_exit(
    mut commands: Commands,
    config: Res<ScreenshotConfig>,
    handle: Res<RenderTargetHandle>,
    fixture: Res<HeadlessFixture>,
    mut done: Local<bool>,
    mut frame_count: Local<u32>,
    mut last_pending_textures: Local<Option<usize>>,
    mut procedural_textures_ready_frame: Local<Option<u32>>,
    mut exit: MessageWriter<AppExit>,
    textures: Option<Res<ProceduralTextures>>,
) {
    *frame_count += 1;

    if *frame_count >= 2 && !*done {
        if let Some(textures) = textures.as_deref() {
            let pending = textures.pending_count();
            if pending > 0 {
                let should_log =
                    last_pending_textures.as_ref() != Some(&pending) || *frame_count % 30 == 0;
                if should_log {
                    println!(
                        "headless frame {} fixture={} waiting for {} procedural textures",
                        *frame_count, fixture.0, pending
                    );
                    *last_pending_textures = Some(pending);
                }
                return;
            }

            let ready_frame = procedural_textures_ready_frame.get_or_insert(*frame_count);
            if *frame_count < *ready_frame + 30 {
                return;
            }
        }

        let path = config.path.clone();
        ensure_parent_dir(&path);
        let _ = std::fs::remove_file(&path);
        println!(
            "headless frame {} fixture={} triggering capture",
            *frame_count, fixture.0
        );
        commands
            .spawn(Screenshot::image(handle.0.clone()))
            .observe(save_to_disk(path));
        *done = true;
    }

    if *done {
        let path = config.path.clone();
        if std::path::Path::new(&path).exists() {
            // Attempt to read the image. It might fail if the file is partially written.
            if let Ok(img) = image::open(&path) {
                let img = img.to_rgba8();
                if let Some(first_pixel) = img.pixels().next() {
                    let mut has_other_color = false;
                    for pixel in img.pixels() {
                        if pixel != first_pixel {
                            has_other_color = true;
                            break;
                        }
                    }

                    if has_other_color {
                        println!("Screenshot capture complete (rendered successfully)");
                        exit.write(AppExit::Success);
                    } else {
                        println!("Screenshot was a solid color, retrying...");
                        let _ = std::fs::remove_file(&path);
                        *done = false;
                    }
                } else {
                    // Empty image
                    let _ = std::fs::remove_file(&path);
                    *done = false;
                }
            }
        }
    }
}

fn ensure_parent_dir(path: &str) {
    if let Some(parent) = Path::new(path).parent() {
        if !parent.as_os_str().is_empty() {
            if let Err(err) = std::fs::create_dir_all(parent) {
                eprintln!(
                    "Could not create screenshot directory {:?}: {}",
                    parent, err
                );
            }
        }
    }
}
