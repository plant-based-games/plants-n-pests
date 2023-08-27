use bevy::prelude::*;

use super::{Name, Player, PlayerBundle, PlayerId, BASEURL};
pub struct NetworkPlugin;

#[derive(Resource)]
struct HeartbeatTimer(Timer);

impl Plugin for NetworkPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(HeartbeatTimer(Timer::from_seconds(
            5.0,
            TimerMode::Repeating,
        )))
        .add_systems(Startup, add_defaults);
    }
}

fn add_defaults(mut commands: Commands) {
    commands.spawn(PlayerBundle {
        name: Name(String::from("jordan")),
        id: PlayerId(String::from("jordan")),
        _type: Player,
    });
}
