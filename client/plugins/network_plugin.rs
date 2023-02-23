use crate::HTTPClient;
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
        .add_startup_system(add_defaults)
        .add_system(heartbeat);
    }
}

fn heartbeat(
    time: Res<Time>,
    client: Res<HTTPClient>,
    mut timer: ResMut<HeartbeatTimer>,
    query: Query<&Name, With<Player>>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        for name in query.iter() {
            println!("hello {}!", name.0);
        }
        if let Ok(res) = client
            .0
            .post(format!("{}{}", BASEURL, "/heartbeat"))
            .body("hi")
            .send()
        {
            println!("response status: {}!", res.status());
        } else {
            println!("HEARTBEAT ERROR");
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
