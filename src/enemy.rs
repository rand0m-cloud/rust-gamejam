use crate::prelude::*;

pub struct EnemyPlugin;
impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::GamePlay).with_system(spawn_enemies))
            .add_system_set(SystemSet::on_update(GameState::GamePlay).with_system(enemy_ai));
    }
}

pub fn spawn_enemies(mut commands: Commands, assets: Res<ImageAssets>) {
    let spawn_locations = [(-0.5, 0.5), (0.5, 0.5), (0.0, 1.0)]
        .into_iter()
        .map(Vec2::from);

    for spawn_location in spawn_locations {
        commands
            .spawn_bundle(SpriteBundle {
                texture: assets.placeholder.clone(),
                sprite: Sprite {
                    color: Color::RED,
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
            .insert(Enemy {})
            .insert(MovementStats { speed: 0.1 });
    }
}

pub fn enemy_ai(
    mut query: ParamSet<(
        Query<&GlobalTransform, With<Player>>,
        Query<(&GlobalTransform, &mut Transform, &MovementStats), With<Enemy>>,
    )>,
) {
    let player_translation = query.p0().single().translation;
    for (enemy_global_transform, mut enemy_transform, movement_stats) in query.p1().iter_mut() {
        let dir = player_translation - enemy_global_transform.translation;
        let dir = dir.try_normalize().unwrap_or_default();

        enemy_transform.translation += dir * movement_stats.speed;
    }
}
