use bevy::prelude::*;
use bevy::time::Timer;

#[derive(Component)]
pub(crate) struct Position {
    pub(crate) x: i8,
    pub(crate) y: i8,
}

#[derive(Component)]
pub(crate) struct Name(pub(crate) String);

#[derive(Component)]
pub(crate) struct Plant;

#[derive(Component)]
pub(crate) struct Value(i8);

#[derive(Bundle)]
pub(crate) struct PlantBundle {
    pub(crate) name: Name,
    pub(crate) position: Position,
    pub(crate) _type: Plant,
    pub(crate) value: Value,
    #[bundle]
    pub(crate) sprite: SpriteSheetBundle,
}

#[derive(Component)]
pub(crate) struct Terrain;

#[derive(Component)]
pub(crate) struct Tile(Position);

#[derive(Component)]
pub(crate) struct Map {
    pub(crate) map_vec: Vec<Vec<Tile>>
}

pub(crate) struct Entity(u64);

#[derive(Resource)]
pub(crate) struct GreetTimer(pub(crate) Timer);
