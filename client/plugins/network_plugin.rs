use bevy::prelude::*;

use super::{Name, Player, PlayerBundle, PlayerId};
pub struct NetworkPlugin;

#[derive(Resource)]
struct HeartbeatTimer(Timer);

impl Plugin for NetworkPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(HeartbeatTimer(Timer::from_seconds(
            5.0,
            TimerMode::Repeating,
        )))
        .add_startup_system(add_defaults)
        .add_system(heartbeat);
    }
}

fn heartbeat(
    time: Res<Time>,
    mut timer: ResMut<HeartbeatTimer>,
    query: Query<&Name, With<Player>>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        for name in query.iter() {
            println!("hello {}!", name.0);
            //TODO: Send heartbeat request
        }
    }
}

fn add_defaults(mut commands: Commands) {
    commands.spawn(PlayerBundle {
        name: Name(String::from("jordan")),
        id: PlayerId(String::from("jordan")),
        _type: Player,
    });
}
