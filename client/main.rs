use bevy::prelude::*;
use bevy::app::App;

#[derive(Component)]
struct Position { x: f32, y: f32 }

#[derive(Component)]
struct Name(String);

#[derive(Component)]
struct Plant;

struct Entity(u64);

fn print_position_system(query: Query<&Position>) {
    for position in query.iter() {
        println!("x: {:?}, y: {:?}", position.x, position.y);
    }
}

fn announce_plants(query: Query<&Name, With<Plant>>) {
    for name in query.iter() {
        println!("hello {}!", name.0);
    }
}

fn hello_world() {
    println!("hello world!");
}
fn add_plants(mut commands: Commands) {
    commands.spawn((Plant, Name("Carrot".to_string())));
    commands.spawn((Plant, Name("Potato".to_string())));
    commands.spawn((Plant, Name("Tomato".to_string())));
}

fn main() {
    App::new()
        .add_startup_system(add_plants)
        .add_system(hello_world)
        .add_system(print_position_system)
        .add_system(announce_plants)
        .run();
}