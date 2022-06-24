use crate::prelude::*;
use bevy::{render::camera::Camera2d, utils::Duration};
use bevy_tweening::{
    lens::{TransformPositionLens, TransformScaleLens},
    *,
};

pub struct GameOverPlugin;

impl Plugin for GameOverPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(GameState::GameOver { won: true }).with_system(camera_animation),
        );
        app.add_system_set(
            SystemSet::on_enter(GameState::GameOver { won: false }).with_system(camera_animation),
        );
    }
}

fn camera_animation(
    mut commands: Commands,
    camera_query: Query<(&GlobalTransform, Entity), (With<Camera2d>, Without<Player>)>,
) {
    let (camera_transform, camera) = camera_query.single();
    let animation_duration = Duration::from_secs(2);

    let transform_tween = Tween::new(
        EaseFunction::QuadraticInOut,
        TweeningType::Once,
        animation_duration,
        TransformPositionLens {
            start: camera_transform.translation,
            end: Vec3::new(1.9, 0.1, 999.0),
        },
    );

    let scale_tween = Tween::new(
        EaseFunction::QuadraticInOut,
        TweeningType::Once,
        animation_duration,
        TransformScaleLens {
            start: Vec3::splat(1.0),
            end: Vec3::new(3.5, 3.5, 1.0),
        },
    );

    let tween = Tracks::new([scale_tween, transform_tween]);

    commands.entity(camera).insert(Animator::new(tween));
    //.insert(Animator::new(transform_tween));
}
