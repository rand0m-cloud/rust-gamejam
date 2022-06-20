use crate::prelude::*;
use bevy::utils::HashSet;

pub struct BulletPlugin;

impl Plugin for BulletPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(bullet_fly)
            .add_system(delete_bullet)
            .add_system(bullet_damage);
    }
}

pub fn bullet_damage(mut enemies: Query<&mut Health>, mut events: EventReader<CollisionEvent>) {
    let bullet_collisions = events
        .iter()
        .filter(|e| e.is_started())
        .filter_map(is_bullet_collision);
    for (_bullet, entity) in bullet_collisions {
        if let Ok(mut health) = enemies.get_mut(entity) {
            health.0 -= 1.0;
        }
    }
}

fn delete_bullet(
    mut commands: Commands,
    bullets: Query<&Bullet>,
    mut events: EventReader<CollisionEvent>,
) {
    let bullet_collisions = events
        .iter()
        .filter(|e| e.is_started())
        .filter_map(is_bullet_collision);

    let mut bullets_to_despawn = HashSet::new();

    for (bullet, entity) in bullet_collisions {
        bullets_to_despawn.insert(bullet);
        if bullets.get(entity).is_ok() {
            bullets_to_despawn.insert(entity);
        }
    }

    bullets_to_despawn
        .into_iter()
        .for_each(|ent| commands.entity(ent).despawn());
}

fn bullet_fly(mut bullets: Query<(&mut Transform, &Bullet)>, time: Res<Time>) {
    for (mut transform, bullet) in bullets.iter_mut() {
        transform.translation += bullet.direction.extend(0.0) * time.delta_seconds();
    }
}
