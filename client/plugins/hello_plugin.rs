use crate::shared;
use bevy::prelude::*;

use crate::shared::*;

pub struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GreetTimer(Timer::from_seconds(5.0, TimerMode::Repeating)))
            .add_startup_system(add_plants)
            .add_system(hello_world)
            .add_system(print_position_system)
            .add_system(announce_plants);
    }
}

fn print_position_system(time: Res<Time>, mut timer: ResMut<GreetTimer>, query: Query<&Position>) {
    if timer.0.tick(time.delta()).just_finished() {
        for position in query.iter() {
            println!("x: {:?}, y: {:?}", position.x, position.y);
        }
    }
}

fn announce_plants(
    time: Res<Time>,
    mut timer: ResMut<GreetTimer>,
    query: Query<&shared::Name, With<Plant>>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        for name in query.iter() {
            println!("hello {}!", name.0);
        }
    }
}

fn hello_world(time: Res<Time>, mut timer: ResMut<GreetTimer>) {
    if timer.0.tick(time.delta()).just_finished() {
        println!("hello world!");
    }
}

fn add_plants(mut commands: Commands) {
    commands.spawn((
        Plant,
        shared::Name("Carrot".to_string()),
        Position { x: 0.0, y: 0.0 },
    ));
    commands.spawn((
        Plant,
        shared::Name("Potato".to_string()),
        Position { x: 0.0, y: 0.0 },
    ));
    commands.spawn((
        Plant,
        shared::Name("Tomato".to_string()),
        Position { x: 0.0, y: 0.0 },
    ));
}
