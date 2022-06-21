use crate::{minion::spawn_minion, prelude::*};

pub struct SpawnerPlugin;
impl Plugin for SpawnerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(GameState::GamePlay).with_system(spawn_initial_spawners),
        )
        .add_system_set(
            SystemSet::on_update(GameState::GamePlay)
                .with_system(minions_spawner_ai)
                .with_system(spawner_capture_ai),
        )
        .register_type::<Spawner>();
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
            .insert(Minion)
            .insert(Spawner::default())
            .insert(RigidBody::Sensor)
            .insert(CollisionShape::Sphere { radius: 0.2 })
            .insert(
                CollisionLayers::none()
                    .with_group(Layer::CaptureArea)
                    .with_masks(&[Layer::Player, Layer::Enemy]),
            )
            .insert(crate::external::collisions::Collisions::default())
            .insert(Name::new("Spawner"));
    }
}

pub fn minions_spawner_ai(
    mut commands: Commands,
    assets: Res<OurAssets>,
    mut spawners_query: Query<(&mut Spawner, &GlobalTransform, &ChickenOrDog), With<Minion>>,
    time: Res<Time>,
) {
    for (mut spawner, transform, chicken_or_dog) in spawners_query.iter_mut() {
        spawner.spawn_timer.tick(time.delta());
        if spawner.spawn_timer.just_finished() {
            spawn_minion(
                &mut commands,
                &assets,
                *chicken_or_dog,
                transform.translation.truncate(),
            );
        }
    }
}

fn spawner_capture_ai(
    mut commands: Commands,
    mut spawners: Query<(&Collisions, &mut Spawner, Entity)>,
    player: Query<&Player, Without<Minion>>,
    enemy: Query<&Enemy, Without<Minion>>,
    time: Res<Time>,
) {
    for (collisions, mut spawner, spawner_ent) in spawners.iter_mut() {
        if collisions.is_empty() {
            continue;
        }

        let progress_multiplier = if collisions.entities().any(|ent| player.get(ent).is_ok()) {
            1.0
        } else if collisions.entities().any(|ent| enemy.get(ent).is_ok()) {
            -1.0
        } else {
            0.0
        };

        let delta_progress = progress_multiplier * (time.delta_seconds() / spawner.capture_time);

        if (spawner.capture_progress <= -1.0 && delta_progress < 0.0)
            || (spawner.capture_progress >= 1.0 && delta_progress > 0.0)
        {
            continue;
        }

        spawner.capture_progress += delta_progress;

        if spawner.capture_progress <= -1.0 {
            commands.entity(spawner_ent).insert(ChickenOrDog::Dog);
        } else if spawner.capture_progress >= 1.0 {
            commands.entity(spawner_ent).insert(ChickenOrDog::Chicken);
        }
    }
}
