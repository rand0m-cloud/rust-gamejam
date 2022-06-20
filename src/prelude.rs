pub use bevy::prelude::*;
pub use heron::prelude::*;

pub use crate::{map::Map, GameState, OurAssets};

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Enemy;

#[derive(PhysicsLayer)]
pub enum Layer {
    Bullet,
    Enemy,
    Player,
    Wall,
}

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

#[derive(Copy, Clone, Debug, Component, PartialEq, Eq)]
pub enum ChickenOrDog {
    Chicken,
    Dog,
}

#[derive(Component)]
pub struct Spawner {
    pub timer: Timer,
}
