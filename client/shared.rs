use bevy::prelude::*;
use bevy::time::Timer;

#[derive(Component)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

#[derive(Component)]
pub struct Name(pub String);

#[derive(Component)]
pub struct Plant;

// pub struct Entity(u64);

#[derive(Resource)]
pub struct GreetTimer(pub Timer);
