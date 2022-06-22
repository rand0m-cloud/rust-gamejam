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

    //    for (_bullet, entity) in bullet_collisions {
    //        if let Ok(mut health) = enemies.get_mut(entity) {
    //            health.0 -= 1.0;
    //        }
    //    }
}

fn delete_bullet(mut commands: Commands, bullets: Query<&Collisions, With<Bullet>>) {
    let bullets_to_delete = bullets
        .iter()
        .map(|collisions| collisions.entities())
        .flatten()
        .collect::<HashSet<_>>();

    if bullets_to_delete.len() > 0 {
        info!("bullets query: {:?}", bullets.iter().collect::<Vec<_>>());
        info!("deleting bullets: {bullets_to_delete:?}");
    }
    bullets_to_delete
        .into_iter()
        .inspect(|ent_id| info!("deleting bullet {ent_id:?}"))
        .for_each(|ent| commands.entity(ent).despawn());
}

fn bullet_fly(mut bullets: Query<(&mut Transform, &Bullet)>, time: Res<Time>) {
    for (mut transform, bullet) in bullets.iter_mut() {
        transform.translation += bullet.direction.extend(0.0) * time.delta_seconds();
    }
}
