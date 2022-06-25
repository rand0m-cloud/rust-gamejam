use crate::{assets::DogWalkFrames, prelude::*};

pub struct EnemyPlugin;
impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::GamePlay).with_system(spawn_enemy))
            .add_system_set(
                SystemSet::on_update(GameState::GamePlay)
                    .with_system(enemy_ai)
                    .with_system(enemy_shoot),
            );
    }
}

#[derive(Component)]
struct BulletParentTag;

pub fn spawn_enemy(
    mut commands: Commands,
    map: Res<Assets<Map>>,
    dog_walk: Res<DogWalkFrames>,
    our_assets: Res<OurAssets>,
) {
    let map = map.get(our_assets.map.clone()).unwrap();
    let size = 0.25;

    commands
        .spawn_bundle(SpriteSheetBundle {
            sprite: dog_walk.frames[0].clone(),
            texture_atlas: dog_walk.texture.clone(),
            transform: Transform::from_translation(map.enemy_spawn.extend(800.0)),
            ..default()
        })
        .insert(Animation {
            current_frame: 0,
            frames: dog_walk.frames.iter().map(|f| f.index).collect(),
            alt_frames: Some(dog_walk.alt_frames.iter().map(|f| f.index).collect()),
            playing_alt: false,
            playing: true,
            flip_x: true,
            timer: Timer::from_seconds(2.0 / 10.0, true),
        })
        .insert(Enemy {
            bullet_cooldown: Timer::from_seconds(0.6, true),
        })
        .insert(Health(PLAYER_HP))
        .insert(MovementStats { speed: 0.2 })
        .insert(RigidBody::Dynamic)
        .insert(CollisionShape::Sphere { radius: size / 2.0 })
        .insert(RotationConstraints::lock())
        .insert(ChickenOrDog::Dog)
        .insert(CollisionLayers::all_masks::<Layer>().with_group(Layer::Enemy))
        .insert(Name::new("Enemy"));

    commands
        .spawn_bundle(TransformBundle::default())
        .insert(BulletParentTag)
        .insert(Name::new("Enemy Bullets"));
}

fn enemy_ai(
    mut minion_query: Query<(&GlobalTransform, &mut Transform, &MovementStats), With<Enemy>>,
    targets_query: Query<
        (&GlobalTransform, Option<&ChickenOrDog>),
        Or<(With<Spawner>, With<Player>, With<Enemy>)>,
    >,
    player_query: Query<&GlobalTransform, With<Player>>,

    time: Res<Time>,
) {
    for (global_transform, mut transform, movement_stats) in minion_query.iter_mut() {
        let position = global_transform.translation.truncate();

        let enemy_targets = targets_query
            .iter()
            .filter_map(|(transform, target_minion_type)| match target_minion_type {
                None => Some(*transform),
                Some(ChickenOrDog::Chicken) => Some(*transform),
                _ => None,
            });

        let target_position = {
            if let Some(closest_target) = find_closest(position, enemy_targets) {
                closest_target
            } else {
                player_query.single().translation.truncate()
            }
        };

        let dir = target_position - position;
        let dir = dir.try_normalize().unwrap_or_default().extend(0.0);
        transform.translation += dir * movement_stats.speed * time.delta_seconds();
    }
}

fn enemy_shoot(
    mut commands: Commands,
    mut enemies: Query<(&mut Enemy, &GlobalTransform, &Transform)>,
    targets: Query<(&GlobalTransform, &ChickenOrDog), Or<(With<Player>, With<Minion>)>>,

    parent: Query<Entity, With<BulletParentTag>>,
    time: Res<Time>,

    assets: Res<OurAssets>,
) {
    let parent = parent.single();
    let delta = time.delta();

    for (mut enemy, global_transform, transform) in enemies.iter_mut() {
        if !enemy.bullet_cooldown.finished() {
            enemy.bullet_cooldown.tick(delta);
            continue;
        }

        let position = global_transform.translation.truncate();

        let enemy_targets = targets
            .iter()
            .filter_map(|(transform, team)| {
                if *team == ChickenOrDog::Chicken {
                    Some(transform)
                } else {
                    None
                }
            })
            .cloned();

        enemy.bullet_cooldown.tick(time.delta());

        if let Some(target) = find_closest(position, enemy_targets) {
            let target_dir = (target - position).normalize();

            let mut transform = *transform;
            transform.translation.z += 1.0;

            let size = 0.1;

            let bullet = commands
                .spawn_bundle(SpriteBundle {
                    sprite: Sprite {
                        color: Color::DARK_GREEN,
                        custom_size: Some(Vec2::splat(size)),
                        ..default()
                    },
                    texture: assets.placeholder.clone(),
                    transform,
                    ..default()
                })
                .insert(Bullet {
                    speed: 0.2,
                    direction: target_dir,
                })
                .insert(ChickenOrDog::Dog)
                .insert(RigidBody::Sensor)
                .insert(CollisionShape::Sphere { radius: size / 2.0 })
                .insert(RotationConstraints::lock())
                .insert(
                    CollisionLayers::all_masks::<Layer>()
                        .with_group(Layer::Bullet)
                        .without_mask(Layer::Bullet)
                        .without_mask(Layer::Enemy),
                )
                .insert(Collisions::default())
                .insert(Name::new("Enemy Bullet"))
                .id();
            commands.entity(parent).add_child(bullet);
        }
    }
}
