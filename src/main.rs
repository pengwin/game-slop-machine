mod plugins;

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            plugins::GamePlugin,
        ))
        .run();
}
