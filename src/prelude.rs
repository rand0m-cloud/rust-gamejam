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
pub struct Enemy;

#[derive(Component, Debug)]
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
    pub origin_team: ChickenOrDog,
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
