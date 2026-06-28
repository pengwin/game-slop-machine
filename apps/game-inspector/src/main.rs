//! Inspector app for previewing generated game assets and scenes.

mod inspector;

use bevy::{
    feathers::{dark_theme::create_dark_theme, theme::UiTheme, FeathersPlugins},
    prelude::*,
};
use game_core::plugins::GameCorePlugin;

use inspector::InspectorPlugin;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Game Inspector".into(),
                    ..default()
                }),
                ..default()
            }),
            FeathersPlugins,
            GameCorePlugin,
            InspectorPlugin,
        ))
        .insert_resource(UiTheme(create_dark_theme()))
        .run();
}
