use crate::prelude::*;

pub struct BulletPlugin;

impl Plugin for BulletPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(bullet_fly).add_system(delete_bullet);
    }
}

fn delete_bullet(mut commands: Commands, mut events: EventReader<CollisionEvent>) {
    // Tracking which bullets have already been despawned to prevent the edge case of 1 bullet hitting 2 things in 1 frame
    // Double despawn isn't a crash but it's an annoying warning
    let mut despawned = Vec::new();

    for event in events.iter().filter(|e| e.is_started()) {
        let (entity_1, entity_2) = event.rigid_body_entities();
        let (layers_1, layers_2) = event.collision_layers();

        if layers_1.contains_group(Layer::Bullet)
            && !layers_1.contains_group(Layer::Player)
            && !despawned.contains(&entity_1)
        {
            despawned.push(entity_1);
            commands.entity(entity_1).despawn()
        }

        if layers_2.contains_group(Layer::Bullet)
            && !layers_1.contains_group(Layer::Player)
            && !despawned.contains(&entity_2)
        {
            despawned.push(entity_2);
            commands.entity(entity_2).despawn()
        }
    }
}

fn bullet_fly(mut bullets: Query<(&mut Transform, &Bullet)>, time: Res<Time>) {
    for (mut transform, bullet) in bullets.iter_mut() {
        transform.translation += bullet.direction.extend(0.0) * time.delta_seconds();
    }
}
