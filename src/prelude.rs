pub use crate::external::collisions::Collisions;
pub use anyhow::Context;
pub use bevy::prelude::*;
use bevy::utils::Duration;
pub use heron::prelude::*;
use serde::{Deserialize, Serialize};

pub use crate::{assets::OurAssets, map::Map, GameState};

#[derive(Component)]
pub struct Player {
    pub bullet_cooldown: Timer,
}

#[derive(Component)]
pub struct Enemy {
    pub bullet_cooldown: Timer,
}

#[derive(Component, Debug, Reflect, Default)]
#[reflect(Component)]
pub struct Health(pub f32);

#[derive(PhysicsLayer, Copy, Clone)]
pub enum Layer {
    Bullet,
    Enemy,
    Player,
    Wall,
    CaptureArea,

    // only for sanity checks, default physics layers is all layers and masks
    None,
}

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
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

#[derive(Copy, Clone, Debug, Component, PartialEq, Eq, Serialize, Deserialize)]
pub enum ChickenOrDog {
    Chicken,
    Dog,
}

impl Default for ChickenOrDog {
    fn default() -> Self {
        ChickenOrDog::Chicken
    }
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct Spawner {
    pub spawn_timer: Timer,
    pub capture_time: f32,
    // -1.0 < progress < 1.0
    // negative means the enemy won the objective
    pub capture_progress: f32,
}

impl Default for Spawner {
    fn default() -> Self {
        Self {
            spawn_timer: Timer::new(Duration::from_secs_f32(5.0), true),
            capture_progress: 0.0,
            capture_time: 5.0,
        }
    }
}

/// Checks if a collision event contains a bullet. If so, return the entities with the bullet as the first entity
pub fn is_bullet_collision(event: &CollisionEvent) -> Option<(Entity, Entity)> {
    is_layer_collision(event, Layer::Bullet)
}

/// Checks if a collision event contains the specific physics layer. If so, return the entities with the chosen layer as the first entity
pub fn is_layer_collision(event: &CollisionEvent, layer: Layer) -> Option<(Entity, Entity)> {
    let entities = event.rigid_body_entities();
    let layers = event.collision_layers();

    // assert that neither layer is uninitialized
    assert!(![layers.0, layers.1]
        .into_iter()
        .any(|layer| layer.contains_group(Layer::None)));

    match [layers.0, layers.1]
        .into_iter()
        .position(|l| l.contains_group(layer))
    {
        Some(0) => Some(entities),
        Some(1) => Some((entities.1, entities.0)),
        _ => None,
    }
}

pub fn find_closest(position: Vec2, iter: impl Iterator<Item = GlobalTransform>) -> Option<Vec2> {
    iter.min_by(|transform, other_transform| {
        (position - transform.translation.truncate())
            .length()
            .partial_cmp(&(position - other_transform.translation.truncate()).length())
            .unwrap()
    })
    .map(|transform| transform.translation.truncate())
}
