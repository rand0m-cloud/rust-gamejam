pub use bevy::prelude::*;

pub use crate::{GameState, ImageAssets};

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Enemy;

#[derive(Component)]
pub struct MovementStats {
    pub speed: f32,
}

#[derive(Component)]
pub struct Bullet {
    pub speed: f32,
    pub direction: Vec2,
}

#[derive(Component)]
pub struct RectCollider;

#[derive(Component)]
pub struct CircleCollider;

#[derive(Component)]
pub struct Minion;

#[derive(Copy, Clone, Debug, Component)]
pub enum ChickenOrDog {
    Chicken,
    Dog,
}

#[derive(Component)]
pub struct Spawner;
