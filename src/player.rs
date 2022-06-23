use bevy::render::camera::Camera2d;

use crate::{assets::ChickenWalkFrames, prelude::*};

pub struct PlayerPlugin;

#[derive(Component)]
struct BulletParentTag;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::GamePlay).with_system(spawn_player))
            .add_system_set(
                SystemSet::on_update(GameState::GamePlay)
                    .with_system(player_movement)
                    .with_system(camera_follow.after(player_movement))
                    .with_system(player_shoot),
            );
    }
}

fn camera_follow(
    player_query: Query<&Transform, With<Player>>,
    mut camera_query: Query<&mut Transform, (With<Camera2d>, Without<Player>)>,
) {
    let mut camera_translation = camera_query.single_mut();
    let player_translation = player_query.single().translation;
    camera_translation.translation.x = player_translation.x;
    camera_translation.translation.y = player_translation.y;
}

fn player_shoot(
    mut commands: Commands,
    mut player: Query<(&Transform, &mut Player)>,
    parent: Query<Entity, With<BulletParentTag>>,

    keyboard: Res<Input<KeyCode>>,
    axis: Res<Axis<GamepadAxis>>,
    time: Res<Time>,

    assets: Res<OurAssets>,
) {
    let parent = parent.single();
    let (transform, mut player) = player.single_mut();

    if !player.bullet_cooldown.finished() {
        player.bullet_cooldown.tick(time.delta());
        return;
    }

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
        transform.translation.z += 100.0;

        let size = 0.1;

        player.bullet_cooldown.tick(time.delta());

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
            .insert(ChickenOrDog::Chicken)
            .insert(RigidBody::Sensor)
            .insert(CollisionShape::Sphere { radius: size / 2.0 })
            .insert(RotationConstraints::lock())
            .insert(
                CollisionLayers::all_masks::<Layer>()
                    .with_group(Layer::Bullet)
                    .without_mask(Layer::Bullet)
                    .without_mask(Layer::Player),
            )
            .insert(Collisions::default())
            .insert(Name::new("Player Bullet"))
            .id();
        commands.entity(parent).add_child(bullet);
    }
}

fn player_movement(
    mut player: Query<(&mut Transform, &mut Animation, &MovementStats), With<Player>>,
    time: Res<Time>,
    keyboard: Res<Input<KeyCode>>,
    axis: Res<Axis<GamepadAxis>>,
) {
    let (mut transform, mut animation, stats) = player.single_mut();

    animation.playing = false;
    for id in 0..16 {
        let axis_lx = GamepadAxis(Gamepad(id), GamepadAxisType::LeftStickX);
        let axis_ly = GamepadAxis(Gamepad(id), GamepadAxisType::LeftStickY);

        if let (Some(x), Some(y)) = (axis.get(axis_lx), axis.get(axis_ly)) {
            if x.abs() > 0.01 || y.abs() > 0.01 {
                animation.playing = true;
            }
            transform.translation.x += x * stats.speed * time.delta_seconds();
            transform.translation.y += y * stats.speed * time.delta_seconds();
        }
    }

    if keyboard.pressed(KeyCode::D) {
        transform.translation.x += time.delta_seconds() * stats.speed;
        animation.playing = true;
        animation.flip_x = true;
    }
    if keyboard.pressed(KeyCode::A) {
        transform.translation.x -= time.delta_seconds() * stats.speed;
        animation.playing = true;
        animation.flip_x = false;
    }
    if keyboard.pressed(KeyCode::W) {
        transform.translation.y += time.delta_seconds() * stats.speed;
        animation.playing = true;
    }
    if keyboard.pressed(KeyCode::S) {
        transform.translation.y -= time.delta_seconds() * stats.speed;
        animation.playing = true;
    }
}

fn spawn_player(
    mut commands: Commands,
    chicken_walk: Res<ChickenWalkFrames>,
    map: Res<Assets<Map>>,
    our_assets: Res<OurAssets>,
) {
    let size = chicken_walk.frames[0].custom_size.unwrap().x;
    let map = map.get(our_assets.map.clone()).unwrap();

    commands
        .spawn_bundle(SpriteSheetBundle {
            sprite: chicken_walk.frames[0].clone(),
            texture_atlas: chicken_walk.texture.clone(),
            transform: Transform::from_translation(map.player_spawn.extend(800.0)),
            ..default()
        })
        .insert(Player {
            bullet_cooldown: Timer::from_seconds(0.3, true),
        })
        .insert(MovementStats { speed: 0.5 })
        .insert(RigidBody::Dynamic)
        .insert(CollisionShape::Sphere { radius: size / 2.0 })
        .insert(RotationConstraints::lock())
        .insert(
            CollisionLayers::all_masks::<Layer>()
                .with_group(Layer::Player)
                .without_mask(Layer::Bullet),
        )
        .insert(Animation {
            current_frame: 0,
            frames: chicken_walk.frames.iter().map(|f| f.index).collect(),
            playing: false,
            flip_x: false,
            timer: Timer::from_seconds(1.0 / 10.0, true),
        })
        .insert(Name::new("Player"));

    commands
        .spawn_bundle(TransformBundle::default())
        .insert(BulletParentTag)
        .insert(Name::new("Bullets"));
}
