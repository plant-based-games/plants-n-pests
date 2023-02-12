mod plugins;
mod shared;

use crate::plugins::hello_plugin::HelloPlugin;
use crate::plugins::state_handler_plugin::StateHandlerPlugin;
use bevy::app::App;
use bevy::prelude::*;
#[cfg(target_arch = "wasm32")]
use web_sys;

#[cfg(target_arch = "wasm32")]
fn handle_browser_resize(mut windows: ResMut<Windows>) {
    let window = windows.get_primary_mut().unwrap();
    let wasm_window = web_sys::window().unwrap();
    let (target_width, target_height) = (
        wasm_window.inner_width().unwrap().as_f64().unwrap() as f32,
        wasm_window.inner_height().unwrap().as_f64().unwrap() as f32,
    );
    if window.width() != target_width || window.height() != target_height {
        window.set_resolution(target_width, target_height);
    }
}

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        window: WindowDescriptor {
            title: String::from("Plants & Pests"),
            resizable: true,
            fit_canvas_to_parent: true,
            ..default()
        },
        ..default()
    }))
    .add_plugin(HelloPlugin)
    .add_plugin(StateHandlerPlugin);

    #[cfg(target_arch = "wasm32")]
    app.add_system(handle_browser_resize);

    app.run();
}
