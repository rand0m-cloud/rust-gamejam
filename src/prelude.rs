pub use bevy::prelude::*;
pub use heron::prelude::*;

pub use crate::{assets::OurAssets, map::Map, GameState};

#[derive(Component)]
pub struct Player;

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
    //None must exist to easily check against default (default is all for some cursed reason)
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

//Physics helpers
pub fn layer_contains_group(layer: &CollisionLayers, group: &Layer) -> bool {
    layer.contains_group(group) && !layer.contains_group(Layer::None)
}

pub fn check_both_entitys<T, F>(entities: (T, T), mut f: F)
where
    F: FnMut(T),
{
    f(entities.0);
    f(entities.1);
}
