#![forbid(unsafe_code)]

mod browser_client;
mod plugins;

use crate::plugins::hello_plugin::HelloPlugin;
use crate::plugins::network_plugin::NetworkPlugin;
use crate::plugins::state_handler_plugin::ScreenHandlerPlugin;
use bevy::app::App;
use bevy::prelude::*;

#[cfg(not(target_arch = "wasm32"))]
use reqwest::blocking::Client;

#[cfg(target_arch = "wasm32")]
use reqwest::Client;
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

const BASEURL: &str = "localhost:8080";

#[derive(Resource)]
struct PlayerSettings {
    cookie_secret: String,
}

#[derive(Resource)]
struct HTTPClient(Client);

#[cfg(not(target_arch = "wasm32"))]
impl FromWorld for PlayerSettings {
    fn from_world(world: &mut World) -> Self {
        if let Some(client) = world.get_resource::<HTTPClient>() {
            if let Ok(res) = client
                .0
                .post(format!("{}{}", BASEURL, "/login"))
                .body("hi")
                .send()
            {
                let mut cookies = res.cookies();
                if let Some(cookie) = cookies.find(|c| c.name() == "cookie") {
                    println!("SUCCESSFUL LOGIN");
                    PlayerSettings {
                        cookie_secret: cookie.value().to_string(),
                    }
                } else {
                    println!("ERROR LOGGING IN 1");
                    PlayerSettings {
                        cookie_secret: String::from("NOCOOKIEFOUND"),
                    }
                }
            } else {
                println!("ERROR LOGGING IN 2");
                PlayerSettings {
                    cookie_secret: String::from("NOCOOKIEFOUND"),
                }
            }
        } else {
            println!("ERROR LOGGING IN 3");
            PlayerSettings {
                cookie_secret: String::from("NOCOOKIEFOUND"),
            }
        }
    }
}

#[cfg(target_arch = "wasm32")]
impl FromWorld for PlayerSettings {
    fn from_world(world: &mut World) -> Self {
        if let Some(client) = world.get_resource::<HTTPClient>() {
            if let Ok(res) = client
                .0
                .post(format!("{}{}", BASEURL, "/login"))
                .body("hi")
                .send()
            {
                let mut cookies = res.cookies();
                if let Some(cookie) = cookies.find(|c| c.name() == "cookie") {
                    println!("SUCCESSFUL LOGIN");
                    PlayerSettings {
                        cookie_secret: cookie.value().to_string(),
                    }
                } else {
                    println!("ERROR LOGGING IN 1");
                    PlayerSettings {
                        cookie_secret: String::from("NOCOOKIEFOUND"),
                    }
                }
            } else {
                println!("ERROR LOGGING IN 2");
                PlayerSettings {
                    cookie_secret: String::from("NOCOOKIEFOUND"),
                }
            }
        } else {
            println!("ERROR LOGGING IN 3");
            PlayerSettings {
                cookie_secret: String::from("NOCOOKIEFOUND"),
            }
        }
    }
}

fn main() {
    let mut app = App::new();

    app.insert_resource(HTTPClient(Client::new()))
        .init_resource::<PlayerSettings>()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: String::from("Plants & Pests"),
                resizable: true,
                fit_canvas_to_parent: true,
                ..default()
            }),
            ..default()
        }))
        .add_plugin(HelloPlugin)
        .add_plugin(NetworkPlugin)
        .add_plugin(ScreenHandlerPlugin);

    #[cfg(target_arch = "wasm32")]
    app.add_system(handle_browser_resize);

    app.run();
}
