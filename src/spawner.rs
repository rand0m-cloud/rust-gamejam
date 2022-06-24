use crate::{
    assets::ChickWalkFrames,
    minion::MinionBundle,
    prelude::*,
    world_ui::{spawn_quad, BarMaterial, Percentage},
};

#[derive(Component)]
struct MinionParentTag;

pub struct SpawnerPlugin;
impl Plugin for SpawnerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(GameState::GamePlay).with_system(spawn_initial_spawners),
        )
        .add_system_set(
            SystemSet::on_update(GameState::GamePlay)
                .with_system(minions_spawner_ai)
                .with_system(spawner_capture_ai)
                .with_system(spawner_win_con),
        )
        .register_type::<Spawner>();
    }
}

pub fn spawn_initial_spawners(
    mut commands: Commands,
    our_assets: Res<OurAssets>,

    mut mesh_assets: ResMut<Assets<Mesh>>,
    mut my_material_assets: ResMut<Assets<BarMaterial>>,

    map: Res<Assets<Map>>,
) {
    let map = map.get(our_assets.map.clone()).unwrap();

    let (chicken_spawner_locations, dog_spawner_locations) = map
        .spawn_locations
        .iter()
        .partition::<Vec<_>, _>(|(_, minion_type)| *minion_type == ChickenOrDog::Chicken);
    let chicken_spawner_locations = chicken_spawner_locations
        .into_iter()
        .map(|(location, _)| location)
        .collect();
    let dog_spawner_locations = dog_spawner_locations
        .into_iter()
        .map(|(location, _)| location)
        .collect();

    let mut spawners = Vec::new();

    spawners.extend(spawn_minion_spawners(
        &mut commands,
        &our_assets,
        ChickenOrDog::Chicken,
        chicken_spawner_locations,
        &mut mesh_assets,
        &mut my_material_assets,
    ));

    spawners.extend(spawn_minion_spawners(
        &mut commands,
        &our_assets,
        ChickenOrDog::Dog,
        dog_spawner_locations,
        &mut mesh_assets,
        &mut my_material_assets,
    ));

    commands
        .spawn_bundle(TransformBundle::default())
        .insert(Name::new("Spawners"))
        .push_children(&spawners);

    commands
        .spawn_bundle(TransformBundle::default())
        .insert(Name::new("Minions"))
        .insert(MinionParentTag);
}

fn spawn_minion_spawners(
    commands: &mut Commands,
    assets: &Res<OurAssets>,
    minion_type: ChickenOrDog,
    spawn_locations: Vec<Vec2>,
    mesh_assets: &mut ResMut<Assets<Mesh>>,
    my_material_assets: &mut ResMut<Assets<BarMaterial>>,
) -> Vec<Entity> {
    let (color, texture) = match minion_type {
        ChickenOrDog::Chicken => (Color::GREEN, assets.chicken_spawner.clone()),
        ChickenOrDog::Dog => (Color::SALMON, assets.dog_spawner.clone()),
    };

    let mut spawned = Vec::new();

    for spawn_location in spawn_locations {
        let ui = spawn_quad(commands, mesh_assets, my_material_assets);
        spawned.push(
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
                        200.0,
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
                .insert(Name::new("Spawner"))
                .add_child(ui)
                .id(),
        );
    }
    spawned
}

fn minions_spawner_ai(
    mut commands: Commands,
    assets: Res<OurAssets>,
    mut spawners_query: Query<(&mut Spawner, &GlobalTransform, &ChickenOrDog), With<Minion>>,
    chick_walk: Res<ChickWalkFrames>,
    parent: Query<Entity, With<MinionParentTag>>,
    time: Res<Time>,
) {
    let parent = parent.single();

    let mut spawned = Vec::new();
    for (mut spawner, transform, chicken_or_dog) in spawners_query.iter_mut() {
        spawner.spawn_timer.tick(time.delta());
        if spawner.spawn_timer.just_finished() {
            match chicken_or_dog {
                ChickenOrDog::Chicken => {
                    let ent = MinionBundle::spawn_chicken_minion(
                        &mut commands,
                        &assets,
                        transform.translation.truncate(),
                    )
                    .unwrap();
                    commands
                        .entity(ent)
                        .insert(chick_walk.texture.clone())
                        .insert(chick_walk.frames[0].clone())
                        .insert(Animation {
                            current_frame: 0,
                            frames: chick_walk.frames.iter().map(|f| f.index).collect(),
                            playing: true,
                            flip_x: false,
                            timer: Timer::from_seconds(1.0 / 10.0, true),
                        });
                    spawned.push(ent);
                }
                ChickenOrDog::Dog => {
                    let ent = MinionBundle::spawn_dog_minion(
                        &mut commands,
                        &assets,
                        transform.translation.truncate(),
                    )
                    .unwrap();
                    spawned.push(ent);
                }
            }
        }
    }
    commands.entity(parent).push_children(&spawned);
}

fn spawner_capture_ai(
    mut commands: Commands,
    mut spawners: Query<(&Collisions, &mut Spawner, Entity, &Children)>,
    mut ui_query: Query<&mut Percentage>,
    player: Query<&Player, Without<Minion>>,
    enemy: Query<&Enemy, Without<Minion>>,
    minions: Query<&ChickenOrDog, (With<Minion>, Without<Spawner>)>,
    time: Res<Time>,
) {
    for (collisions, mut spawner, spawner_ent, spawner_children) in spawners.iter_mut() {
        if collisions.is_empty() {
            continue;
        }

        let mut progress_multiplier = 0.0;

        if collisions.entities().any(|ent| player.get(ent).is_ok()) {
            progress_multiplier += 1.0;
        }

        progress_multiplier -= collisions
            .entities()
            .filter(|ent| enemy.get(*ent).is_ok())
            .count() as f32;

        let minion_advantage: f32 = collisions
            .entities()
            .filter_map(|ent| {
                let minion_type = minions.get(ent).ok()?;
                Some(match minion_type {
                    ChickenOrDog::Chicken => 0.2,
                    ChickenOrDog::Dog => -0.2,
                })
            })
            .sum();
        progress_multiplier += minion_advantage;

        let delta_progress = progress_multiplier * (time.delta_seconds() / spawner.capture_time);

        if (spawner.capture_progress <= -1.0 && delta_progress < 0.0)
            || (spawner.capture_progress >= 1.0 && delta_progress > 0.0)
        {
            continue;
        }

        spawner.capture_progress += delta_progress;

        for child in spawner_children.iter() {
            if let Ok(mut percentage) = ui_query.get_mut(*child) {
                percentage.value = spawner.capture_progress;
            }
        }

        if spawner.capture_progress <= -1.0 {
            commands.entity(spawner_ent).insert(ChickenOrDog::Dog);
        } else if spawner.capture_progress >= 1.0 {
            commands.entity(spawner_ent).insert(ChickenOrDog::Chicken);
        }
    }
}

fn spawner_win_con(
    spawners: Query<Option<&ChickenOrDog>, With<Spawner>>,
    mut state: ResMut<State<GameState>>,
) {
    let total_spawners = spawners.iter().count();
    let captured_spawners = spawners.iter().flatten().cloned();

    let (chicken_captured, dog_captured) =
        captured_spawners.partition::<Vec<_>, _>(|ty| *ty == ChickenOrDog::Chicken);

    if chicken_captured.len() == total_spawners {
        state.push(GameState::GameOver { won: true }).unwrap();
    } else if dog_captured.len() == total_spawners {
        state.push(GameState::GameOver { won: false }).unwrap();
    }
}
