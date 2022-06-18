use crate::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::GamePlay).with_system(spawn_player))
            .add_system_set(SystemSet::on_update(GameState::GamePlay).with_system(player_movement));
    }
}

fn player_movement(
    mut player: Query<(&mut Transform, &MovementStats), With<Player>>,
    time: Res<Time>,
    keyboard: Res<Input<KeyCode>>,
) {
    let (mut transform, stats) = player.single_mut();
    if keyboard.pressed(KeyCode::D) {
        transform.translation.x += time.delta_seconds() * stats.speed;
    }
    if keyboard.pressed(KeyCode::A) {
        transform.translation.x -= time.delta_seconds() * stats.speed;
    }
    if keyboard.pressed(KeyCode::W) {
        transform.translation.y += time.delta_seconds() * stats.speed;
    }
    if keyboard.pressed(KeyCode::S) {
        transform.translation.y -= time.delta_seconds() * stats.speed;
    }
}

fn spawn_player(mut commands: Commands, assets: Res<ImageAssets>) {
    commands
        .spawn_bundle(SpriteBundle {
            texture: assets.placeholder.clone(),
            sprite: Sprite {
                custom_size: Some(Vec2::splat(1.0)),
                ..default()
            },
            ..default()
        })
        .insert(Player)
        .insert(MovementStats { speed: 0.5 });
}
