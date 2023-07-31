use bevy::prelude::*;

use super::{GreetTimer, Name, Plant, PlantBundle, Position, Value};
pub struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GreetTimer(Timer::from_seconds(5.0, TimerMode::Repeating)))
            .add_systems(Startup, add_plants)
            .add_systems(Update, announce_plants);
    }
}

fn announce_plants(
    time: Res<Time>,
    mut timer: ResMut<GreetTimer>,
    query: Query<(&Name, &Position), With<Plant>>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        for (name, position) in query.iter() {
            println!("hello {}!", name.0);
            println!("x: {:?}, y: {:?}", position.x, position.y);
        }
    }
}

fn add_plants(mut commands: Commands) {
    commands.spawn(PlantBundle {
        name: Name("Carrot".to_string()),
        position: Position { x: 0, y: 0 },
        _type: Plant,
        value: Value(1),
        sprite: SpriteSheetBundle {
            sprite: Default::default(),
            texture_atlas: Default::default(),
            transform: Default::default(),
            global_transform: Default::default(),
            visibility: Default::default(),
            computed_visibility: Default::default(),
        },
    });
    commands.spawn(PlantBundle {
        name: Name("Potato".to_string()),
        position: Position { x: 0, y: 0 },
        _type: Plant,
        value: Value(1),
        sprite: SpriteSheetBundle {
            sprite: Default::default(),
            texture_atlas: Default::default(),
            transform: Default::default(),
            global_transform: Default::default(),
            visibility: Default::default(),
            computed_visibility: Default::default(),
        },
    });
    commands.spawn(PlantBundle {
        name: Name("Tomato".to_string()),
        position: Position { x: 0, y: 0 },
        _type: Plant,
        value: Value(1),
        sprite: SpriteSheetBundle {
            sprite: Default::default(),
            texture_atlas: Default::default(),
            transform: Default::default(),
            global_transform: Default::default(),
            visibility: Default::default(),
            computed_visibility: Default::default(),
        },
    });
}
