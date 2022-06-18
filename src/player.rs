use bevy::transform;

use crate::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::GamePlay).with_system(spawn_player))
            .add_system_set(
                SystemSet::on_update(GameState::GamePlay)
                    .with_system(player_movement)
                    .with_system(player_shoot),
            );
    }
}

fn player_shoot(
    mut commands: Commands,
    player: Query<&Transform, With<Player>>,
    keyboard: Res<Input<KeyCode>>,
    axis: Res<Axis<GamepadAxis>>,
    assets: Res<ImageAssets>,
) {
    let transform = player.single();

    let mut target_dir = Vec2::ZERO;

    for id in 0..16 {
        let axis_lx = GamepadAxis(Gamepad(id), GamepadAxisType::RightStickX);
        let axis_ly = GamepadAxis(Gamepad(id), GamepadAxisType::RightStickY);

        if let (Some(x), Some(y)) = (axis.get(axis_lx), axis.get(axis_ly)) {
            target_dir = Vec2::new(x, y);
        }
    }

    if keyboard.pressed(KeyCode::Left) {
        target_dir.x -= 1.0;
    }
    if keyboard.pressed(KeyCode::Right) {
        target_dir.x += 1.0;
    }
    if keyboard.pressed(KeyCode::Up) {
        target_dir.y += 1.0;
    }
    if keyboard.pressed(KeyCode::Down) {
        target_dir.y -= 1.0;
    }

    if target_dir.length() > 0.1 {
        target_dir = target_dir.normalize();

        let mut transform = *transform;
        transform.translation.z += 1.0;

        commands
            .spawn_bundle(SpriteBundle {
                sprite: Sprite {
                    color: Color::DARK_GREEN,
                    custom_size: Some(Vec2::splat(0.1)),
                    ..default()
                },
                texture: assets.placeholder.clone(),
                transform,
                ..default()
            })
            .insert(Bullet {
                speed: 0.2,
                direction: target_dir,
            });
    }
}

fn player_movement(
    mut player: Query<(&mut Transform, &MovementStats), With<Player>>,
    time: Res<Time>,
    keyboard: Res<Input<KeyCode>>,
    axis: Res<Axis<GamepadAxis>>,
) {
    let (mut transform, stats) = player.single_mut();

    for id in 0..16 {
        let axis_lx = GamepadAxis(Gamepad(id), GamepadAxisType::LeftStickX);
        let axis_ly = GamepadAxis(Gamepad(id), GamepadAxisType::LeftStickY);

        if let (Some(x), Some(y)) = (axis.get(axis_lx), axis.get(axis_ly)) {
            transform.translation.x += x * stats.speed * time.delta_seconds();
            transform.translation.y += y * stats.speed * time.delta_seconds();
        }
    }

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
