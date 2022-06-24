use crate::prelude::*;

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
    assets: Res<OurAssets>,
    map: Res<Assets<Map>>,
    our_assets: Res<OurAssets>,
) {
    let map = map.get(our_assets.map.clone()).unwrap();
    let size = 0.25;

    commands
        .spawn_bundle(SpriteBundle {
            texture: assets.placeholder.clone(),
            sprite: Sprite {
                color: Color::BLUE,
                custom_size: Some(Vec2::splat(size)),
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(
                map.enemy_spawn.x,
                map.enemy_spawn.y,
                500.0,
            )),
            ..default()
        })
        .insert(Enemy {
            bullet_cooldown: Timer::from_seconds(0.6, true),
        })
        .insert(Health(10.0))
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

pub fn enemy_ai(
    mut query: ParamSet<(
        Query<&GlobalTransform, With<Player>>,
        Query<(&GlobalTransform, &mut Transform, &MovementStats), With<Enemy>>,
    )>,
    time: Res<Time>,
) {
    let player_translation = query.p0().single().translation;
    for (enemy_global_transform, mut enemy_transform, movement_stats) in query.p1().iter_mut() {
        let dir = player_translation.truncate() - enemy_global_transform.translation.truncate();
        let dir = dir.try_normalize().unwrap_or_default().extend(0.0);

        enemy_transform.translation += dir * movement_stats.speed * time.delta_seconds();
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
