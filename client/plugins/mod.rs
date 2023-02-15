mod draft_plugin;
mod endgame_plugin;
pub(crate) mod game_plugin;
pub(crate) mod hello_plugin;
pub(crate) mod menu_plugin;
pub(crate) mod network_plugin;
pub(crate) mod splash_plugin;
pub(crate) mod state_handler_plugin;

use bevy::prelude::*;
use bevy::time::Timer;

#[derive(Component)]
pub(crate) struct Position {
    pub(crate) x: i8,
    pub(crate) y: i8,
}

#[derive(Component)]
pub(crate) struct Name(pub(crate) String);

#[derive(Component)]
pub(crate) struct Plant;

#[derive(Component)]
pub(crate) struct Value(pub(crate) i8);

#[derive(Bundle)]
pub(crate) struct PlantBundle {
    pub(crate) name: Name,
    pub(crate) position: Position,
    pub(crate) _type: Plant,
    pub(crate) value: Value,
    #[bundle]
    pub(crate) sprite: SpriteSheetBundle,
}

#[derive(Component)]
pub(crate) struct Terrain;

#[derive(Component)]
pub(crate) struct Tile(pub(crate) Position);

#[derive(Component)]
pub(crate) struct Map {
    pub(crate) map_vec: Vec<Vec<Tile>>,
}

pub(crate) struct Entity(u64);

#[derive(Resource)]
pub(crate) struct GreetTimer(pub(crate) Timer);

const TEXT_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);
const MENU_BACKGROUND_COLOR: Color = Color::DARK_GREEN;

// Enum that will be used as a global state for the game
#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash)]
enum GameState {
    #[default]
    Splash,
    Menu,
    Draft,
    Game,
    Endgame,
}

// One of the two settings that can be set through the menu. It will be a resource in the app
#[derive(Resource, Debug, Component, PartialEq, Eq, Clone, Copy)]
enum DisplayQuality {
    Low,
    Medium,
    High,
}

// One of the two settings that can be set through the menu. It will be a resource in the app
#[derive(Resource, Debug, Component, PartialEq, Eq, Clone, Copy)]
struct Volume(u32);

fn despawn_screen<T: bevy::prelude::Component>(
    to_despawn: Query<bevy::prelude::Entity, With<T>>,
    mut commands: Commands,
) {
    for entity in &to_despawn {
        commands.entity(entity).despawn_recursive();
    }
}
