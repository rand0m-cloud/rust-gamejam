pub use bevy::prelude::*;

pub use crate::GameState;
pub use crate::ImageAssets;

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
}

#[derive(Component)]
pub struct RectCollider;

#[derive(Component)]
pub struct CircleCollider;
