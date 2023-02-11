mod plugins;
mod shared;

use crate::plugins::hello_plugin::HelloPlugin;
use bevy::app::App;
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                title: String::from("Plants & Pests"),
                resizable: true,
                fit_canvas_to_parent: true,
                ..default()
            },
            ..default()
        }))
        .add_plugin(HelloPlugin)
        .run();
}
