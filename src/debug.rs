use crate::prelude::*;
use std::time::Duration;

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        if std::env::var("SLOWDOWN").is_ok() {
            app.add_system(slow_down);
        }
        app.register_type::<Health>();
    }
}

fn slow_down() {
    let amount = std::env::var("SLOWDOWN").unwrap();
    let amount = amount
        .parse::<f32>()
        .expect("environment variable SLOWDOWN to be a float");
    std::thread::sleep(Duration::from_secs_f32(amount));
}
