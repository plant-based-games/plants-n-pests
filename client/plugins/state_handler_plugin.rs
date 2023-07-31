use crate::plugins::draft_plugin::DraftPlugin;
use crate::plugins::endgame_plugin::EndgamePlugin;
use bevy::prelude::*;

use crate::plugins::game_plugin::GamePlugin;
use crate::plugins::menu_plugin::MenuPlugin;
use crate::plugins::splash_plugin::SplashPlugin;

use super::{DisplayQuality, GameState, Volume};

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

pub struct ScreenHandlerPlugin;

impl Plugin for ScreenHandlerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .insert_resource(DisplayQuality::Medium)
            .insert_resource(Volume(7))
            .add_state::<GameState>()
            .add_plugins((SplashPlugin, MenuPlugin, DraftPlugin, GamePlugin, EndgamePlugin));
    }
}
