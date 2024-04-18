use crate::plugins::draft::DraftPlugin;
use crate::plugins::endgame::EndgamePlugin;
use bevy::prelude::*;

use crate::plugins::game::GamePlugin;
use crate::plugins::menu::MenuPlugin;
use crate::plugins::splash::SplashPlugin;

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
            .init_state::<GameState>()
            .add_plugins((
                SplashPlugin,
                MenuPlugin,
                DraftPlugin,
                GamePlugin,
                EndgamePlugin,
            ));
    }
}
