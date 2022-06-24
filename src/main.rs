#![allow(clippy::type_complexity)]

use bevy::{asset::AssetServerSettings, render::camera::ScalingMode, window::PresentMode};
use bevy_asset_loader::AssetLoader;
use bevy_inspector_egui::{WorldInspectorParams, WorldInspectorPlugin};

pub const CLEAR: Color = Color::rgb(0.3, 0.3, 0.3);
pub const HEIGHT: f32 = 900.0;
pub const RESOLUTION: f32 = 16.0 / 9.0;

use rust_gamejam::{
    assets::GameAssetsPlugin, bullet::BulletPlugin, debug::DebugPlugin, enemy::EnemyPlugin,
    external::ExternalPlugin, map::MapPlugin, menus::MenuPlugin, minion::*, player::PlayerPlugin,
    prelude::*, spawner::SpawnerPlugin, world_ui::BarMaterialPlugin,
};

fn main() {
    let mut app = App::new();

    AssetLoader::new(GameState::Splash)
        .continue_to_state(GameState::MainMenu)
        .with_collection::<OurAssets>()
        .build(&mut app);

    app.add_state(GameState::Splash)
        .insert_resource(AssetServerSettings {
            watch_for_changes: true,
            ..default()
        })
        .insert_resource(ClearColor(Color::hex(&"24523b").unwrap()))
        .insert_resource(WindowDescriptor {
            width: HEIGHT * RESOLUTION,
            height: HEIGHT,
            title: "Bevy Template".to_string(),
            present_mode: PresentMode::Fifo,
            resizable: false,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .insert_resource(WorldInspectorParams {
            enabled: false,
            ..Default::default()
        })
        .add_plugin(PhysicsPlugin::default())
        .add_plugin(PlayerPlugin)
        .add_plugin(EnemyPlugin)
        .add_plugin(BulletPlugin)
        .add_plugin(MapPlugin)
        .add_plugin(GameAssetsPlugin)
        .add_plugin(MinionPlugin)
        .add_plugin(WorldInspectorPlugin::new())
        .add_plugin(ExternalPlugin)
        .add_plugin(SpawnerPlugin)
        .add_plugin(BarMaterialPlugin)
        .add_plugin(DebugPlugin)
        .add_plugin(MenuPlugin)
        .add_startup_system(spawn_camera)
        .add_system_set(SystemSet::on_enter(GameState::GamePlay).with_system(spawn_background))
        .add_system(toggle_inspector)
        .register_type::<Animation>()
        .run();
}

fn toggle_inspector(
    input: ResMut<Input<KeyCode>>,
    mut window_params: ResMut<WorldInspectorParams>,
) {
    if input.just_pressed(KeyCode::Grave) {
        window_params.enabled = !window_params.enabled
    }
}

fn spawn_background(mut commands: Commands, assets: Res<OurAssets>) {
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(6.1, 6.1)),
                ..default()
            },
            transform: Transform::from_xyz(2.0, 0.0, 0.0),
            texture: assets.background.clone(),
            ..default()
        })
        .insert(Name::new("Background"));
}

fn spawn_camera(mut commands: Commands) {
    let mut camera = OrthographicCameraBundle::new_2d();

    camera.orthographic_projection.right = 1.0 * RESOLUTION;
    camera.orthographic_projection.left = -1.0 * RESOLUTION;

    camera.orthographic_projection.top = 1.0;
    camera.orthographic_projection.bottom = -1.0;

    camera.orthographic_projection.scaling_mode = ScalingMode::None;

    commands.spawn_bundle(camera);
}
