use crate::shared;
use bevy::prelude::*;

use crate::shared::*;

pub struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GreetTimer(Timer::from_seconds(5.0, TimerMode::Repeating)))
            .add_startup_system(add_plants)
            .add_system(announce_plants);
    }
}

fn announce_plants(
    time: Res<Time>,
    mut timer: ResMut<GreetTimer>,
    query: Query<(&shared::Name, &Position), With<Plant>>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        for (name, position) in query.iter() {
            println!("hello {}!", name.0);
            println!("x: {:?}, y: {:?}", position.x, position.y);
        }
    }
}

fn add_plants(mut commands: Commands) {
    commands.spawn(
        PlantBundle {
            name: shared::Name("Carrot".to_string()),
            position: Position { x: 0.0, y: 0.0 },
            _p: Plant,
            sprite: SpriteSheetBundle {
                sprite: Default::default(),
                texture_atlas: Default::default(),
                transform: Default::default(),
                global_transform: Default::default(),
                visibility: Default::default(),
                computed_visibility: Default::default(),
            }
        }
    );
    commands.spawn(
        PlantBundle {
            name: shared::Name("Potato".to_string()),
            position: Position { x: 0.0, y: 0.0 },
            _p: Plant,
            sprite: SpriteSheetBundle {
                sprite: Default::default(),
                texture_atlas: Default::default(),
                transform: Default::default(),
                global_transform: Default::default(),
                visibility: Default::default(),
                computed_visibility: Default::default(),
            }
        }
    );
    commands.spawn(
        PlantBundle {
            name: shared::Name("Tomato".to_string()),
            position: Position { x: 0.0, y: 0.0 },
            _p: Plant,
            sprite: SpriteSheetBundle {
                sprite: Default::default(),
                texture_atlas: Default::default(),
                transform: Default::default(),
                global_transform: Default::default(),
                visibility: Default::default(),
                computed_visibility: Default::default(),
            }
        }
    );
}
