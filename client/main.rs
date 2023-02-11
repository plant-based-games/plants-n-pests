use bevy::app::App;
use bevy::prelude::*;

#[derive(Component)]
struct Position {
    x: f32,
    y: f32,
}

#[derive(Component)]
struct Name(String);

#[derive(Component)]
struct Plant;

struct Entity(u64);

#[derive(Resource)]
struct GreetTimer(Timer);

pub struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GreetTimer(Timer::from_seconds(2.0, TimerMode::Repeating)))
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
    query: Query<&Name, With<Plant>>,
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
        Name("Carrot".to_string()),
        Position { x: 0.0, y: 0.0 },
    ));
    commands.spawn((
        Plant,
        Name("Potato".to_string()),
        Position { x: 0.0, y: 0.0 },
    ));
    commands.spawn((
        Plant,
        Name("Tomato".to_string()),
        Position { x: 0.0, y: 0.0 },
    ));
}

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                title: String::from("Plants & Pests"),
                resizable: true,
                fit_canvas_to_parent: true,
                ..default()
            },
            ..default()
        }))
        .add_plugin(HelloPlugin)
        .run();
}
