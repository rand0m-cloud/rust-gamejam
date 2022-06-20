use crate::prelude::*;

pub struct EnemyPlugin;
impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::GamePlay).with_system(spawn_enemies))
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

pub fn spawn_enemies(mut commands: Commands, assets: Res<OurAssets>) {
    let spawn_locations = [(-0.5, 0.5), (0.5, 0.5), (0.0, 1.0)]
        .into_iter()
        .map(Vec2::from);

    let size = 0.25;

    for spawn_location in spawn_locations {
        commands
            .spawn_bundle(SpriteBundle {
                texture: assets.placeholder.clone(),
                sprite: Sprite {
                    color: Color::BLUE,
                    custom_size: Some(Vec2::splat(size)),
                    ..default()
                },
                transform: Transform::from_translation(Vec3::new(
                    spawn_location.x,
                    spawn_location.y,
                    1.0,
                )),
                ..default()
            })
            .insert(Enemy)
            .insert(Health(3.0))
            .insert(MovementStats { speed: 0.1 })
            .insert(RigidBody::Dynamic)
            .insert(CollisionShape::Sphere { radius: size / 2.0 })
            .insert(RotationConstraints::lock())
            .insert(CollisionLayers::all_masks::<Layer>().with_group(Layer::Enemy));
    }
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
        let dir = player_translation - enemy_global_transform.translation;
        let dir = dir.try_normalize().unwrap_or_default();

        enemy_transform.translation += dir * movement_stats.speed * time.delta_seconds();
    }
}
