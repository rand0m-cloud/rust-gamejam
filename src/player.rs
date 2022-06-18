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
