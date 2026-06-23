mod building_preview;
mod district_render;
mod fixtures;
mod furniture_preview;
mod screenshot;
mod texture_preview;

use bevy::{
    app::ScheduleRunnerPlugin,
    asset::AssetPlugin,
    light::{CascadeShadowConfigBuilder, DirectionalLightShadowMap},
    prelude::*,
    window::ExitCondition,
    winit::WinitPlugin,
};
use game_core::plugins::GamePlugin;
use game_core::plugins::building::procedural_texture::ProceduralTextures;
use game_core::plugins::scene::camera_config::CameraConfig;
use game_core::plugins::scene::scene_config::SceneConfig;
use std::time::Duration;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let output_path = args
        .get(1)
        .cloned()
        .unwrap_or_else(|| "output.png".to_string());
    let fixture = args
        .get(2)
        .cloned()
        .unwrap_or_else(|| "procedural".to_string());

    App::new()
        .add_plugins((
            DefaultPlugins
                .set(AssetPlugin {
                    file_path: "../../assets".to_string(),
                    ..default()
                })
                .set(WindowPlugin {
                    primary_window: None,
                    exit_condition: ExitCondition::DontExit,
                    ..default()
                })
                .disable::<WinitPlugin>(),
            ScheduleRunnerPlugin::run_loop(Duration::from_millis(100)),
            GamePlugin,
        ))
        .insert_resource(screenshot::ScreenshotConfig {
            path: output_path,
            width: 1280,
            height: 1024,
        })
        .insert_resource(HeadlessFixture(fixture))
        .add_systems(
            Startup,
            (
                configure_fixture_resources,
                generate_building,
                screenshot::setup_screenshot,
            )
                .chain(),
        )
        .add_systems(
            Update,
            (prune_studio_default_lights, screenshot::capture_and_exit).chain(),
        )
        .run();
}

#[derive(Resource)]
pub(crate) struct HeadlessFixture(pub(crate) String);

fn configure_fixture_resources(
    fixture: Res<HeadlessFixture>,
    mut camera: ResMut<CameraConfig>,
    mut scene: ResMut<SceneConfig>,
) {
    if fixtures::is_district_fixture(&fixture.0) {
        *camera = fixtures::district_camera_for_fixture(&fixture.0);
        scene.ground_size = fixtures::district_ground_size_for_fixture(&fixture.0);
        return;
    }

    if fixtures::is_texture_fixture(&fixture.0) {
        *camera = fixtures::texture_camera_for_fixture(&fixture.0);
        scene.ground_size = if fixture.0 == "texture-plaster-wall" {
            20.0
        } else {
            5.0
        };
        scene.ground_color = fixtures::studio_ground_color();
        return;
    }

    if fixtures::is_furniture_fixture(&fixture.0) {
        *camera = fixtures::furniture_camera_for_fixture(&fixture.0);
        scene.ground_size = if fixture.0 == "all-furniture" {
            14.0
        } else {
            7.0
        };
        scene.ground_color = fixtures::studio_ground_color();
        return;
    }

    if let Some(building_camera) = fixtures::building_camera_for_fixture(&fixture.0) {
        *camera = building_camera;
    }

    if fixtures::uses_studio_low_poly_render(&fixture.0) {
        scene.ground_size = 20.0;
        scene.ground_color = fixtures::studio_ground_color();
    }
}

fn generate_building(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut images: ResMut<Assets<Image>>,
    fixture: Res<HeadlessFixture>,
    mut textures: ResMut<ProceduralTextures>,
) {
    if fixtures::is_district_fixture(&fixture.0) {
        district_render::spawn_district(
            &mut commands,
            &mut meshes,
            &mut materials,
            &mut textures,
            &mut images,
            &fixture.0,
        );
        return;
    }

    if fixtures::is_texture_fixture(&fixture.0) {
        setup_texture_fixture_render(&mut commands);
    } else if fixtures::uses_studio_low_poly_render(&fixture.0) {
        setup_studio_low_poly_render(&mut commands);
    }

    if fixtures::is_texture_fixture(&fixture.0) {
        texture_preview::spawn_texture_preview(
            &mut commands,
            &mut meshes,
            &mut materials,
            &mut textures,
            &mut images,
            &fixture.0,
        );
        return;
    }

    if fixtures::is_furniture_fixture(&fixture.0) {
        furniture_preview::spawn_furniture_preview(
            &mut commands,
            &mut meshes,
            &mut materials,
            &mut textures,
            &mut images,
            &fixture.0,
        );
        return;
    }

    let config = fixtures::config_for_fixture(&fixture.0);
    building_preview::spawn_building_preview(
        &mut commands,
        &mut meshes,
        &mut materials,
        &mut textures,
        &mut images,
        &config,
        &fixture.0,
    );
}

fn setup_studio_low_poly_render(commands: &mut Commands) {
    commands.insert_resource(ClearColor(Color::srgb(0.86, 0.86, 0.84)));
    commands.insert_resource(DirectionalLightShadowMap { size: 4096 });
    commands.insert_resource(GlobalAmbientLight {
        color: Color::srgb(0.94, 0.91, 0.86),
        brightness: 1.02,
        ..default()
    });
    commands.spawn((
        Name::new("Studio Low Poly Key Light"),
        DirectionalLight {
            color: Color::srgb(1.0, 0.95, 0.85),
            illuminance: 2_650.0,
            shadow_maps_enabled: true,
            contact_shadows_enabled: true,
            ..default()
        },
        CascadeShadowConfigBuilder {
            num_cascades: 3,
            maximum_distance: 24.0,
            ..default()
        }
        .build(),
        Transform::from_rotation(Quat::from_euler(EulerRot::XYZ, -1.50, -0.22, 0.0)),
    ));
    commands.spawn((
        Name::new("Studio Low Poly Fill Light"),
        DirectionalLight {
            color: Color::srgb(0.95, 0.97, 1.0),
            illuminance: 320.0,
            shadow_maps_enabled: false,
            ..default()
        },
        Transform::from_rotation(Quat::from_euler(EulerRot::XYZ, -0.6, 2.3, 0.0)),
    ));
}

fn setup_texture_fixture_render(commands: &mut Commands) {
    commands.insert_resource(ClearColor(Color::srgb(0.46, 0.47, 0.47)));
    commands.insert_resource(DirectionalLightShadowMap { size: 2048 });
    commands.insert_resource(GlobalAmbientLight {
        color: Color::srgb(0.96, 0.97, 1.0),
        brightness: 1.55,
        ..default()
    });
    commands.spawn((
        Name::new("Texture Fixture Raking Light"),
        DirectionalLight {
            color: Color::srgb(1.0, 0.98, 0.94),
            illuminance: 2_050.0,
            shadow_maps_enabled: true,
            contact_shadows_enabled: true,
            ..default()
        },
        Transform::from_rotation(Quat::from_euler(EulerRot::XYZ, -0.42, -0.95, 0.0)),
    ));
    commands.spawn((
        Name::new("Texture Fixture Cool Fill"),
        DirectionalLight {
            color: Color::srgb(0.78, 0.84, 1.0),
            illuminance: 2_400.0,
            shadow_maps_enabled: false,
            ..default()
        },
        Transform::from_rotation(Quat::from_euler(EulerRot::XYZ, -0.55, 2.35, 0.0)),
    ));
    commands.spawn((
        Name::new("Texture Fixture Camera Fill"),
        PointLight {
            color: Color::srgb(0.94, 0.96, 1.0),
            intensity: 420.0,
            range: 5.0,
            shadow_maps_enabled: false,
            ..default()
        },
        Transform::from_xyz(1.9, 1.7, 1.7),
    ));
}

fn prune_studio_default_lights(
    mut commands: Commands,
    fixture: Res<HeadlessFixture>,
    lights: Query<(Entity, Option<&Name>), With<DirectionalLight>>,
    mut done: Local<bool>,
) {
    if *done || !fixtures::uses_studio_low_poly_render(&fixture.0) {
        return;
    }

    for (entity, name) in &lights {
        let keep = name.is_some_and(|name| {
            matches!(
                name.as_str(),
                "Studio Low Poly Key Light"
                    | "Studio Low Poly Fill Light"
                    | "Texture Fixture Raking Light"
                    | "Texture Fixture Cool Fill"
                    | "Texture Fixture Camera Fill"
            )
        });
        if !keep {
            commands.entity(entity).despawn();
        }
    }

    *done = true;
}
