use crate::prelude::*;

pub struct BulletPlugin;

impl Plugin for BulletPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(bullet_fly)
            .add_system(delete_bullet)
            .add_system(bullet_damage);
    }
}

pub fn bullet_damage(mut enemies: Query<&mut Health>, mut events: EventReader<CollisionEvent>) {
    for event in events.iter().filter(|e| e.is_started()) {
        let (entity_1, entity_2) = event.rigid_body_entities();
        let (layers_1, layers_2) = event.collision_layers();

        //maybe this is too fancy of a way to do both pairs...
        //probably should just use clearer repeated code
        check_both_entitys(
            (
                (entity_1, layers_1, layers_2),
                (entity_2, layers_2, layers_1),
            ),
            |(entity, layer_1, layer_2)| {
                // is there a better way to get an interaction between 2 specifc layers...
                if layer_contains_group(&layer_2, &Layer::Bullet)
                    && layer_contains_group(&layer_1, &Layer::Enemy)
                {
                    //ouch
                    if let Ok(mut health) = enemies.get_mut(entity) {
                        health.0 -= 1.0;
                    }
                }
            },
        );
    }
}

fn delete_bullet(mut commands: Commands, mut events: EventReader<CollisionEvent>) {
    // Tracking which bullets have already been despawned to prevent the edge case of 1 bullet hitting 2 things in 1 frame
    // Double despawn isn't a crash but it's an annoying warning
    let mut despawned = Vec::new();

    for event in events.iter().filter(|e| e.is_started()) {
        let (entity_1, entity_2) = event.rigid_body_entities();
        let (layers_1, layers_2) = event.collision_layers();

        //Am i being too fancy here
        // I probably should just dupe the code
        check_both_entitys(
            ((entity_1, layers_1), (entity_2, layers_2)),
            |(entity, layer)| {
                if layer_contains_group(&layer, &Layer::Bullet) && !despawned.contains(&entity) {
                    despawned.push(entity);
                    commands.entity(entity).despawn();
                }
            },
        );
    }
}

fn bullet_fly(mut bullets: Query<(&mut Transform, &Bullet)>, time: Res<Time>) {
    for (mut transform, bullet) in bullets.iter_mut() {
        transform.translation += bullet.direction.extend(0.0) * time.delta_seconds();
    }
}
