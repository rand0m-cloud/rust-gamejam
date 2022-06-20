#![allow(clippy::type_complexity)]

use std::fs;

use bevy::{prelude::*, render::camera::ScalingMode, window::PresentMode};
use bevy_inspector_egui::{WorldInspectorParams, WorldInspectorPlugin};
use ron::ser::{to_string_pretty, PrettyConfig};

pub const CLEAR: Color = Color::rgb(0.3, 0.3, 0.3);
pub const HEIGHT: f32 = 900.0;
pub const RESOLUTION: f32 = 16.0 / 9.0;

use rust_gamejam::map::{Map, Rect};

#[derive(Component)]
pub struct MapSquare;

fn save_map(map: Query<&Transform, With<MapSquare>>, input: Res<Input<KeyCode>>) {
    if input.just_pressed(KeyCode::S) {
        let mut rects = Vec::new();
        for transform in map.iter() {
            rects.push(Rect {
                position: transform.translation.truncate(),
                size: transform.scale.truncate(),
                rotation: transform.rotation.to_axis_angle().1.to_degrees(),
            });
        }

        let data = Map { rects };

        let pretty = PrettyConfig::new()
            .depth_limit(2)
            .separate_tuple_members(true)
            .enumerate_arrays(true);
        let s = to_string_pretty(&data, pretty).expect("Serialization failed");

        fs::write("assets/main.map", s).expect("Unable to write file");
        println!("SAVED");
    }
}

fn load_map(mut commands: Commands) {
    if let Ok(map) =
        ron::de::from_str::<Map>(&fs::read_to_string("assets/main.map").unwrap_or_default())
    {
        for rect in &map.rects {
            commands
                .spawn_bundle(SpriteBundle {
                    transform: Transform {
                        translation: rect.position.extend(0.0),
                        rotation: Quat::from_axis_angle(Vec3::Z, rect.rotation.to_radians()),
                        scale: rect.size.extend(0.0),
                    },
                    ..default()
                })
                .insert(MapSquare)
                .insert(Name::new("Wall"));
        }
    }
}

fn main() {
    App::new()
        .insert_resource(ClearColor(CLEAR))
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
            despawnable_entities: true,
            ..Default::default()
        })
        .add_plugin(WorldInspectorPlugin::new())
        .add_startup_system(spawn_camera)
        .add_startup_system(load_map)
        .add_system(toggle_inspector)
        .add_system(save_map)
        .add_system(fly_camera)
        .add_system(create_square)
        .run();
}

fn create_square(mut commands: Commands, input: Res<Input<KeyCode>>) {
    if input.just_pressed(KeyCode::Space) {
        commands
            .spawn_bundle(SpriteBundle::default())
            .insert(MapSquare);
    }
}
fn fly_camera(mut transform: Query<&mut Transform, With<Camera>>, input: Res<Input<KeyCode>>) {
    let mut trans = transform.single_mut();
    if input.pressed(KeyCode::Up) {
        trans.translation.y += 0.1;
    }
    if input.pressed(KeyCode::Down) {
        trans.translation.y -= 0.1;
    }
    if input.pressed(KeyCode::Left) {
        trans.translation.x -= 0.1;
    }
    if input.pressed(KeyCode::Right) {
        trans.translation.x += 0.1;
    }
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

fn toggle_inspector(
    input: ResMut<Input<KeyCode>>,
    mut window_params: ResMut<WorldInspectorParams>,
) {
    if input.just_pressed(KeyCode::Grave) {
        window_params.enabled = !window_params.enabled
    }
}

#[allow(dead_code)]
fn slow_down() {
    std::thread::sleep(std::time::Duration::from_secs_f32(1.000));
}
