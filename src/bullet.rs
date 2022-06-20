use crate::prelude::*;
use bevy::utils::HashSet;

pub struct BulletPlugin;

impl Plugin for BulletPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(bullet_fly).add_system(delete_bullet);
    }
}

fn delete_bullet(mut commands: Commands, mut events: EventReader<CollisionEvent>) {
    let mut entities_to_despawn = HashSet::new();

    for event in events.iter().filter(|e| e.is_started()) {
        let (entity_1, entity_2) = event.rigid_body_entities();
        let (layers_1, layers_2) = event.collision_layers();

        if layers_1.contains_group(Layer::Bullet) && !layers_1.contains_group(Layer::Player) {
            entities_to_despawn.insert(entity_1);
        }

        if layers_2.contains_group(Layer::Bullet) && !layers_1.contains_group(Layer::Player) {
            entities_to_despawn.insert(entity_2);
        }
    }

    entities_to_despawn
        .into_iter()
        .for_each(|ent| commands.entity(ent).despawn());
}

fn bullet_fly(mut bullets: Query<(&mut Transform, &Bullet)>, time: Res<Time>) {
    for (mut transform, bullet) in bullets.iter_mut() {
        transform.translation += bullet.direction.extend(0.0) * time.delta_seconds();
    }
}
