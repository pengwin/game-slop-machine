use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Game Slop Machine".into(),
                    ..default()
                }),
                ..default()
            }),
            game_core::plugins::GamePlugin,
        ))
        .run();
}
