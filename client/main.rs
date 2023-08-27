#![forbid(unsafe_code)]

mod browser_client;
mod plugins;

use crate::plugins::hello_plugin::HelloPlugin;
use crate::plugins::network_plugin::NetworkPlugin;
use crate::plugins::state_handler_plugin::ScreenHandlerPlugin;
use bevy::app::App;
use bevy::prelude::*;

#[cfg(target_arch = "wasm32")]
use bevy::window::{PrimaryWindow, WindowResolution};

#[cfg(target_arch = "wasm32")]
fn handle_browser_resize(mut window_query: Query<&Window, With<PrimaryWindow>>) {
    let Ok(window) = window_query.get_single_mut() else {
        return;
    };
    let wasm_window = web_sys::window().unwrap();
    let resolution = WindowResolution::new(
        wasm_window.inner_width().unwrap().as_f64().unwrap() as f32,
        wasm_window.inner_height().unwrap().as_f64().unwrap() as f32,
    );
    if window.width() != resolution.width() || window.height() != resolution.height() {
        window.resolution = resolution;
    }
}

fn main() {
    let mut app = App::new();

    app.add_plugins((DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: String::from("Plants & Pests"),
                resizable: true,
                fit_canvas_to_parent: true,
                ..default()
            }),
            ..default()
        }), HelloPlugin, NetworkPlugin, ScreenHandlerPlugin));

    #[cfg(target_arch = "wasm32")]
    app.add_system(handle_browser_resize);

    app.run();
}
