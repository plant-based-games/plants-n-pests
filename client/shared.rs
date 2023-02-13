use bevy::prelude::*;
use bevy::time::Timer;

#[derive(Component)]
pub(crate) struct Position {
    pub(crate) x: f32,
    pub(crate) y: f32,
}

#[derive(Component)]
pub(crate) struct Name(pub(crate) String);

#[derive(Component)]
pub(crate) struct Plant;

#[derive(Bundle)]
pub(crate) struct PlantBundle {
    pub(crate) name: Name,
    pub(crate) position: Position,
    pub(crate) _p: Plant,
    #[bundle]
    pub(crate) sprite: SpriteSheetBundle,
}

pub(crate) struct Entity(u64);

#[derive(Resource)]
pub(crate) struct GreetTimer(pub(crate) Timer);
