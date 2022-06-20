use crate::prelude::*;
use bevy::utils::Duration;
use serde::{Deserialize, Serialize};

pub struct MinionPlugin;
impl Plugin for MinionPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(GameState::GamePlay).with_system(spawn_initial_spawners),
        )
        .add_system_set(
            SystemSet::on_update(GameState::GamePlay)
                .with_system(minions_spawner_ai)
                .with_system(minions_ai),
        );
    }
}

pub fn spawn_initial_spawners(mut commands: Commands, assets: Res<OurAssets>) {
    let chicken_spawner_locations = [(-1.0, 0.0), (-0.8, 0.1)]
        .into_iter()
        .map(Vec2::from)
        .collect();
    let dog_spawner_locations = [(1.0, 0.0), (0.8, 0.1)]
        .into_iter()
        .map(Vec2::from)
        .collect();

    spawn_minion_spawners(
        &mut commands,
        &assets,
        ChickenOrDog::Chicken,
        chicken_spawner_locations,
    );
    spawn_minion_spawners(
        &mut commands,
        &assets,
        ChickenOrDog::Dog,
        dog_spawner_locations,
    );
}

#[derive(Bundle)]
pub struct MinionBundle {
    #[bundle]
    sprite: SpriteBundle,
    movement_stats: MovementStats,
    minion_type: ChickenOrDog,
    minion: Minion,
    hp: Health,
    rigid_body: RigidBody,
    collision_shape: CollisionShape,
    rotation_constraints: RotationConstraints,
    collision_layer: CollisionLayers,
}

#[derive(Serialize, Deserialize)]
pub struct DogMinionConfig {
    speed: f32,
    hp: f32,
}

#[derive(Serialize, Deserialize)]
pub struct ChickenMinionConfig {
    speed: f32,
    hp: f32,
}

impl MinionBundle {
    pub fn create_dog_minion(
        assets: &Res<OurAssets>,
        spawn_location: Vec2,
    ) -> anyhow::Result<Self> {
        let config: DogMinionConfig = ron::de::from_reader(
            std::fs::File::open("assets/config/dog_minion.ron")
                .context("failed to open assets/config/dog_minion.ron")?,
        )
        .context("failed to deserialize DogMinionConfig")?;
        let size = 0.25;

        Ok(Self {
            sprite: SpriteBundle {
                texture: assets.dog_spawner.clone(),
                sprite: Sprite {
                    color: Color::SALMON,
                    custom_size: Some(Vec2::splat(size)),
                    ..default()
                },
                transform: Transform::from_translation(Vec3::new(
                    spawn_location.x,
                    spawn_location.y,
                    1.0,
                )),
                ..default()
            },
            movement_stats: MovementStats {
                speed: config.speed,
            },
            minion_type: ChickenOrDog::Dog,
            minion: Minion,
            hp: Health(config.hp),
            rigid_body: RigidBody::Dynamic,
            collision_shape: CollisionShape::Sphere { radius: size / 2.0 },
            rotation_constraints: RotationConstraints::lock(),
            collision_layer: CollisionLayers::all_masks::<Layer>().with_group(Layer::Enemy),
        })
    }

    pub fn create_chicken_minion(
        assets: &Res<OurAssets>,
        spawn_location: Vec2,
    ) -> anyhow::Result<Self> {
        let config: ChickenMinionConfig = ron::de::from_reader(
            std::fs::File::open("assets/config/chicken_minion.ron")
                .context("failed to open assets/config/chicken_minion.ron")?,
        )
        .context("failed to deserialize ChickenMinionConfig")?;
        let size = 0.25;

        Ok(Self {
            sprite: SpriteBundle {
                texture: assets.chicken_spawner.clone(),
                sprite: Sprite {
                    color: Color::SALMON,
                    custom_size: Some(Vec2::splat(size)),
                    ..default()
                },
                transform: Transform::from_translation(Vec3::new(
                    spawn_location.x,
                    spawn_location.y,
                    1.0,
                )),
                ..default()
            },
            movement_stats: MovementStats {
                speed: config.speed,
            },
            minion_type: ChickenOrDog::Dog,
            minion: Minion,
            hp: Health(config.hp),
            rigid_body: RigidBody::Dynamic,
            collision_shape: CollisionShape::Sphere { radius: size / 2.0 },
            rotation_constraints: RotationConstraints::lock(),
            collision_layer: CollisionLayers::all_masks::<Layer>().with_group(Layer::Enemy),
        })
    }
}

fn spawn_minion(
    commands: &mut Commands,
    assets: &Res<OurAssets>,
    minion_type: ChickenOrDog,
    spawn_location: Vec2,
) {
    let minion = match minion_type {
        ChickenOrDog::Chicken => MinionBundle::create_chicken_minion(assets, spawn_location),
        ChickenOrDog::Dog => MinionBundle::create_dog_minion(assets, spawn_location),
    }
    .unwrap();

    commands.spawn_bundle(minion);
}

fn spawn_minion_spawners(
    commands: &mut Commands,
    assets: &Res<OurAssets>,
    minion_type: ChickenOrDog,
    spawn_locations: Vec<Vec2>,
) {
    let (color, texture) = match minion_type {
        ChickenOrDog::Chicken => (Color::GREEN, assets.chicken_spawner.clone()),
        ChickenOrDog::Dog => (Color::SALMON, assets.dog_spawner.clone()),
    };

    for spawn_location in spawn_locations {
        commands
            .spawn_bundle(SpriteBundle {
                texture: texture.clone(),
                sprite: Sprite {
                    color,
                    custom_size: Some(Vec2::splat(0.25)),
                    ..default()
                },
                transform: Transform::from_translation(Vec3::new(
                    spawn_location.x,
                    spawn_location.y,
                    1.0,
                )),
                ..default()
            })
            .insert(MovementStats { speed: 0.1 })
            .insert(minion_type)
            .insert(Minion)
            .insert(Spawner {
                timer: Timer::new(Duration::from_secs_f32(5.0), true),
            });
    }
}

pub fn minions_ai(
    mut minion_query: Query<
        (
            &ChickenOrDog,
            &GlobalTransform,
            &mut Transform,
            &MovementStats,
        ),
        (With<Minion>, Without<Spawner>),
    >,
    player_query: Query<&GlobalTransform, With<Player>>,
    enemy_query: Query<&GlobalTransform, With<Enemy>>,

    time: Res<Time>,
) {
    for (minion_type, global_transform, mut transform, movement_stats) in minion_query.iter_mut() {
        let position = global_transform.translation;
        let target_position = if *minion_type == ChickenOrDog::Chicken {
            if let Some(closest_enemy) = enemy_query.iter().min_by(|transform, other_transform| {
                (position - transform.translation)
                    .length()
                    .partial_cmp(&(position - other_transform.translation).length())
                    .unwrap()
            }) {
                closest_enemy.translation
            } else {
                //XXX gross
                player_query.single().translation
            }
        } else {
            player_query.single().translation
        };

        let dir = target_position - position;
        let dir = dir.try_normalize().unwrap_or_default();
        transform.translation += dir * movement_stats.speed * time.delta_seconds();
    }
}

pub fn minions_spawner_ai(
    mut commands: Commands,
    assets: Res<OurAssets>,
    mut spawners_query: Query<(&mut Spawner, &GlobalTransform, &ChickenOrDog), With<Minion>>,
    time: Res<Time>,
) {
    for (mut spawner, transform, chicken_or_dog) in spawners_query.iter_mut() {
        spawner.timer.tick(time.delta());
        if spawner.timer.just_finished() {
            spawn_minion(
                &mut commands,
                &assets,
                *chicken_or_dog,
                transform.translation.truncate(),
            );
        }
    }
}
