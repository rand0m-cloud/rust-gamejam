pub use anyhow::Context;
pub use bevy::prelude::*;
pub use heron::prelude::*;

pub use crate::{assets::OurAssets, map::Map, GameState};

#[derive(Component)]
pub struct Player {
    pub bullet_cooldown: Timer,
}

#[derive(Component)]
pub struct Enemy;

#[derive(Component)]
pub struct Health(pub f32);

#[derive(PhysicsLayer)]
pub enum Layer {
    Bullet,
    Enemy,
    Player,
    Wall,

    // only for sanity checks, default physics layers is all layers and masks
    None,
}

#[derive(Component)]
pub struct Animation {
    pub current_frame: usize,
    pub frames: Vec<usize>,
    pub playing: bool,
    pub flip_x: bool,
    pub timer: Timer,
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

/// Checks if a collision event contains a bullet. If so, return the entities with the bullet as the first entity
pub fn is_bullet_collision(event: &CollisionEvent) -> Option<(Entity, Entity)> {
    let entities = event.rigid_body_entities();
    let layers = event.collision_layers();

    // assert that neither layer is uninitialized
    assert!(![layers.0, layers.1]
        .into_iter()
        .any(|layer| layer.contains_group(Layer::None)));

    match [layers.0, layers.1]
        .into_iter()
        .position(|layer| layer.contains_group(Layer::Bullet))
    {
        Some(0) => Some(entities),
        Some(1) => Some((entities.1, entities.0)),
        _ => None,
    }
}
