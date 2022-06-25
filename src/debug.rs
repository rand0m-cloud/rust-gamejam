use crate::prelude::*;
use bevy_inspector_egui::RegisterInspectable;
use std::time::Duration;

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        if std::env::var("SLOWDOWN").is_ok() {
            app.add_system(slow_down);
        }
        app.register_type::<Health>()
            .register_type::<RespawnTimer>()
            .register_type::<DamageFlash>()
            .register_type::<Player>()
            .register_type::<Enemy>()
            .register_type::<Enemy>()
            .register_type::<Animation>()
            .register_type::<MovementStats>()
            .register_type::<Bullet>()
            .register_type::<RectCollider>()
            .register_type::<CircleCollider>()
            .register_type::<Minion>()
            .register_inspectable::<ChickenOrDog>()
            .register_type::<Spawner>();
    }
}

fn slow_down() {
    let amount = std::env::var("SLOWDOWN").unwrap();
    let amount = amount
        .parse::<f32>()
        .expect("environment variable SLOWDOWN to be a float");
    std::thread::sleep(Duration::from_secs_f32(amount));
}
