use crate::prelude::*;

pub struct EnemyPlugin;
impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::GamePlay).with_system(spawn_enemy))
            .add_system_set(SystemSet::on_update(GameState::GamePlay).with_system(enemy_ai))
            .add_system(enemy_death);
    }
}

//seperate system in case there's other ways to die in the future
//or an effect :)
pub fn enemy_death(enemies: Query<(Entity, &Health)>, mut commands: Commands) {
    for (ent, health) in enemies.iter() {
        if health.0 <= 0.0 {
            commands.entity(ent).despawn_recursive();
        }
    }
}

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
                1.0,
            )),
            ..default()
        })
        .insert(Enemy)
        .insert(Health(10.0))
        .insert(MovementStats { speed: 0.2 })
        .insert(RigidBody::Dynamic)
        .insert(CollisionShape::Sphere { radius: size / 2.0 })
        .insert(RotationConstraints::lock())
        .insert(ChickenOrDog::Dog)
        .insert(CollisionLayers::all_masks::<Layer>().with_group(Layer::Enemy))
        .insert(Name::new("Enemy"));
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
