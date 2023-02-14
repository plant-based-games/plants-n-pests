use bevy::prelude::*;

use crate::plugins::game_plugin::GamePlugin;
use crate::plugins::menu_plugin::MenuPlugin;
use crate::plugins::splash_plugin::SplashPlugin;

use super::{DisplayQuality, GameState, Volume};

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

pub struct StateHandlerPlugin;

impl Plugin for StateHandlerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup)
            .insert_resource(DisplayQuality::Medium)
            .insert_resource(Volume(7))
            .add_state::<GameState>(GameState::Splash)
            // Adds the plugins for each state
            .add_plugin(SplashPlugin)
            .add_plugin(MenuPlugin)
            .add_plugin(GamePlugin);
    }
}
