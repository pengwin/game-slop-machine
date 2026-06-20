mod building_preview;
mod district_render;
mod fixtures;
mod furniture_preview;
mod screenshot;

use bevy::{
    app::ScheduleRunnerPlugin,
    asset::AssetPlugin,
    light::{CascadeShadowConfigBuilder, DirectionalLightShadowMap},
    prelude::*,
    window::ExitCondition,
    winit::WinitPlugin,
};
use game_core::plugins::GamePlugin;
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
            (generate_building, screenshot::setup_screenshot).chain(),
        )
        .add_systems(
            Update,
            (prune_studio_default_lights, screenshot::capture_and_exit).chain(),
        )
        .run();
}

#[derive(Resource)]
pub(crate) struct HeadlessFixture(pub(crate) String);

fn generate_building(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    fixture: Res<HeadlessFixture>,
) {
    if fixtures::is_district_fixture(&fixture.0) {
        district_render::spawn_district(&mut commands, &mut meshes, &mut materials, &fixture.0);
        return;
    }

    if fixtures::uses_studio_low_poly_render(&fixture.0) {
        setup_studio_low_poly_render(&mut commands);
    }

    if fixtures::is_furniture_fixture(&fixture.0) {
        commands.insert_resource(fixtures::furniture_camera_for_fixture(&fixture.0));
        let ground_size = if fixture.0 == "all-furniture" {
            14.0
        } else {
            7.0
        };
        commands.insert_resource(game_core::plugins::scene::scene_config::SceneConfig {
            ground_size,
            ground_color: fixtures::studio_ground_color(),
        });
        furniture_preview::spawn_furniture_preview(
            &mut commands,
            &mut meshes,
            &mut materials,
            &fixture.0,
        );
        return;
    }

    if let Some(cam) = fixtures::building_camera_for_fixture(&fixture.0) {
        commands.insert_resource(cam);
    }

    if fixtures::uses_studio_low_poly_render(&fixture.0) {
        commands.insert_resource(game_core::plugins::scene::scene_config::SceneConfig {
            ground_size: 20.0,
            ground_color: fixtures::studio_ground_color(),
        });
    }

    let config = fixtures::config_for_fixture(&fixture.0);
    building_preview::spawn_building_preview(
        &mut commands,
        &mut meshes,
        &mut materials,
        &config,
        &fixture.0,
    );
}

fn setup_studio_low_poly_render(commands: &mut Commands) {
    commands.insert_resource(ClearColor(Color::srgb(0.86, 0.86, 0.84)));
    commands.insert_resource(DirectionalLightShadowMap { size: 4096 });
    commands.insert_resource(GlobalAmbientLight {
        color: Color::srgb(1.0, 0.96, 0.90),
        brightness: 1.62,
        ..default()
    });
    commands.spawn((
        Name::new("Studio Low Poly Key Light"),
        DirectionalLight {
            color: Color::srgb(1.0, 0.95, 0.85),
            illuminance: 1_550.0,
            shadow_maps_enabled: true,
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
            illuminance: 520.0,
            shadow_maps_enabled: false,
            ..default()
        },
        Transform::from_rotation(Quat::from_euler(EulerRot::XYZ, -0.6, 2.3, 0.0)),
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
                "Studio Low Poly Key Light" | "Studio Low Poly Fill Light"
            )
        });
        if !keep {
            commands.entity(entity).despawn();
        }
    }

    *done = true;
}
