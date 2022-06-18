use crate::prelude::*;

pub struct MinionPlugin;
impl Plugin for MinionPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::GamePlay).with_system(spawn_minions))
            .add_system_set(SystemSet::on_update(GameState::GamePlay).with_system(minions_ai));
    }
}

pub fn spawn_minions(mut commands: Commands, assets: Res<ImageAssets>) {
    spawn_minions_spawner(commands, assets, ChickenOrDog::Chicken);
}

fn spawn_minions_spawner(
    mut commands: Commands,
    assets: Res<ImageAssets>,
    minion_type: ChickenOrDog,
) {
    let spawn_locations = [(-0.5, 0.5), (0.5, 0.5), (0.0, 1.0)]
        .into_iter()
        .map(Vec2::from);

    let texture = match minion_type {
        ChickenOrDog::Chicken => assets.chicken_spawner.clone(),
        ChickenOrDog::Dog => assets.dog_spawner.clone(),
    };

    for spawn_location in spawn_locations {
        commands
            .spawn_bundle(SpriteBundle {
                texture: texture.clone(),
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
            .insert(MovementStats { speed: 0.1 })
            .insert(minion_type)
            .insert(Minion)
            .insert(Spawner);
    }
}

pub fn minions_ai() {}
