pub mod collisions;

use bevy::{app::Plugin, prelude::App};

pub struct ExternalPlugin;

impl Plugin for ExternalPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(collisions::update_collisions_system)
            .register_type::<collisions::Collisions>();
    }
}
