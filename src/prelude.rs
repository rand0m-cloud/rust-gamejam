pub use bevy::prelude::*;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Enemy;

#[derive(Component)]
pub struct MovementStats {
    speed: f32,
}

#[derive(Component)]
pub struct Bullet {
    speed: f32,
}

#[derive(Component)]
pub struct RectCollider;

#[derive(Component)]
pub struct CircleCollider;
