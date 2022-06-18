use crate::prelude::*;

pub struct BulletPlugin;

impl Plugin for BulletPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(bullet_fly);
    }
}

fn bullet_fly(mut bullets: Query<(&mut Transform, &Bullet)>, time: Res<Time>) {
    for (mut transform, bullet) in bullets.iter_mut() {
        transform.translation += bullet.direction.extend(0.0) * time.delta_seconds();
    }
}
