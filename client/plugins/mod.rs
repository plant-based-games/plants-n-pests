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
pub(crate) struct PlayerId(pub(crate) String);

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
pub(crate) struct Player;

#[derive(Bundle)]
pub(crate) struct PlayerBundle {
    pub(crate) name: Name,
    pub(crate) id: PlayerId,
    pub(crate) _type: Player,
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
const BASEURL: &str = "localhost:8080";

// Enum that will be used as a global state for the game
#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
enum GameState {
    #[default]
    Splash,
    Menu,
    Draft,
    Game,
    Endgame,
}

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const HOVERED_PRESSED_BUTTON: Color = Color::rgb(0.25, 0.65, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

#[derive(Component)]
struct SelectedOption;

fn button_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, Option<&SelectedOption>),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut color, selected) in &mut interaction_query {
        *color = match (*interaction, selected) {
            (Interaction::Clicked, _) | (Interaction::None, Some(_)) => PRESSED_BUTTON.into(),
            (Interaction::Hovered, Some(_)) => HOVERED_PRESSED_BUTTON.into(),
            (Interaction::Hovered, None) => HOVERED_BUTTON.into(),
            (Interaction::None, None) => NORMAL_BUTTON.into(),
        }
    }
}

#[derive(Resource, Debug, Component, PartialEq, Eq, Clone, Copy)]
enum DisplayQuality {
    Low,
    Medium,
    High,
}

#[derive(Resource, Debug, Component, PartialEq, Eq, Clone, Copy)]
struct Volume(u32);

fn despawn<T: bevy::prelude::Component>(
    to_despawn: Query<bevy::prelude::Entity, With<T>>,
    mut commands: Commands,
) {
    for entity in &to_despawn {
        commands.entity(entity).despawn_recursive();
    }
}
