use crate::prelude::*;
use serde::{Deserialize, Serialize};

pub struct MinionPlugin;
impl Plugin for MinionPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_update(GameState::GamePlay).with_system(minions_ai))
            .register_type::<Spawner>();
    }
}

#[derive(Bundle)]
pub struct MinionBundle {
    #[bundle]
    sprite: SpriteSheetBundle,
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
    pub fn spawn_dog_minion(
        commands: &mut Commands,
        assets: &Res<OurAssets>,
        spawn_location: Vec2,
    ) -> anyhow::Result<Entity> {
        let config: DogMinionConfig = ron::de::from_reader(
            std::fs::File::open("assets/config/dog_minion.ron")
                .context("failed to open assets/config/dog_minion.ron")?,
        )
        .context("failed to deserialize DogMinionConfig")?;
        let size = 0.25;

        let ent = commands
            .spawn_bundle(MinionBundle {
                sprite: SpriteSheetBundle {
                    texture_atlas: assets.placeholder_atlas.clone(),
                    sprite: TextureAtlasSprite {
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
            .insert(Name::new("Puppy"))
            .id();
        Ok(ent)
    }

    pub fn spawn_chicken_minion(
        commands: &mut Commands,
        assets: &Res<OurAssets>,
        spawn_location: Vec2,
    ) -> anyhow::Result<Entity> {
        let config: ChickenMinionConfig = ron::de::from_reader(
            std::fs::File::open("assets/config/chicken_minion.ron")
                .context("failed to open assets/config/chicken_minion.ron")?,
        )
        .context("failed to deserialize ChickenMinionConfig")?;
        let size = 0.25;

        let ent = commands
            .spawn_bundle(MinionBundle {
                sprite: SpriteSheetBundle {
                    texture_atlas: assets.placeholder_atlas.clone(),
                    sprite: TextureAtlasSprite {
                        color: Color::GREEN,
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
                minion_type: ChickenOrDog::Chicken,
                minion: Minion,
                hp: Health(config.hp),
                rigid_body: RigidBody::Dynamic,
                collision_shape: CollisionShape::Sphere { radius: size / 2.0 },
                rotation_constraints: RotationConstraints::lock(),
                collision_layer: CollisionLayers::all_masks::<Layer>().with_group(Layer::Enemy),
            })
            .insert(Name::new("Chick"))
            .id();
        Ok(ent)
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
