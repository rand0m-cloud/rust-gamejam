use crate::prelude::*;
pub struct BulletPlugin;

impl Plugin for BulletPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(bullet_fly)
            .add_system(delete_bullet)
            .add_system(bullet_damage);
    }
}

pub fn bullet_damage(
    mut entities: Query<(&mut Health, &ChickenOrDog)>,
    bullets: Query<(&Collisions, &Bullet)>,
) {
    // have to collect the iter to drop the entities query
    #[allow(clippy::needless_collect)]
    let entities_to_damage = bullets
        .iter()
        .flat_map(|(collisions, bullet)| {
            collisions
                .entities()
                .map(move |collision| (collision, bullet))
        })
        .filter_map(|(entity, bullet)| {
            let (_, entity_team) = entities.get(entity).ok()?;
            if bullet.origin_team != *entity_team {
                Some(entity)
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    for entity in entities_to_damage {
        let (mut health, _) = entities.get_mut(entity).unwrap();
        health.0 -= 1.0;
    }
}

fn delete_bullet(mut commands: Commands, bullets: Query<(&Collisions, Entity), With<Bullet>>) {
    let bullets_to_delete = bullets.iter().filter_map(|(collisions, bullet_ent)| {
        if !collisions.is_empty() {
            Some(bullet_ent)
        } else {
            None
        }
    });

    bullets_to_delete.for_each(|ent| commands.entity(ent).despawn());
}

fn bullet_fly(mut bullets: Query<(&mut Transform, &Bullet)>, time: Res<Time>) {
    for (mut transform, bullet) in bullets.iter_mut() {
        transform.translation += bullet.direction.extend(0.0) * time.delta_seconds();
    }
}
