mod building_preview;
mod district_render;
mod fixtures;
mod mesh_util;
mod screenshot;

use bevy::{app::ScheduleRunnerPlugin, prelude::*, window::ExitCondition, winit::WinitPlugin};
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
        .add_systems(Update, screenshot::capture_and_exit)
        .run();
}

#[derive(Resource)]
struct HeadlessFixture(String);

fn generate_building(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    fixture: Res<HeadlessFixture>,
) {
    if fixtures::is_district_fixture(&fixture.0) {
        district_render::spawn_district(
            &mut commands,
            &mut meshes,
            &mut materials,
            &fixture.0,
        );
        return;
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
