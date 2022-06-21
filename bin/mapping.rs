#![allow(clippy::type_complexity)]

use std::fs;

use bevy::{
    input::mouse::MouseWheel, prelude::*, render::camera::ScalingMode,
    sprite::MaterialMesh2dBundle, window::PresentMode,
};
use bevy_inspector_egui::{WorldInspectorParams, WorldInspectorPlugin};
use ron::ser::{to_string_pretty, PrettyConfig};

use bevy_mod_picking::*;

pub const CLEAR: Color = Color::rgb(0.3, 0.3, 0.3);
pub const HEIGHT: f32 = 900.0;
pub const RESOLUTION: f32 = 16.0 / 9.0;

use rust_gamejam::map::{Map, Rect};

#[derive(Component)]
struct WallSquare;
#[derive(Component)]
struct PlayerSpawn;
#[derive(Component)]
struct Spawner;

fn save_map(
    map: Query<&Transform, With<WallSquare>>,
    spawners_query: Query<&Transform, With<Spawner>>,
    player_spawn: Query<&Transform, With<PlayerSpawn>>,
    input: Res<Input<KeyCode>>,
) {
    if input.just_pressed(KeyCode::Return) {
        let mut rects = Vec::new();
        for transform in map.iter() {
            rects.push(Rect {
                position: transform.translation.truncate(),
                size: transform.scale.truncate(),
                rotation: transform.rotation.to_euler(EulerRot::XYZ).2,
            });
        }

        let mut spawn_locations = Vec::new();
        for transform in spawners_query.iter() {
            spawn_locations.push(transform.translation.truncate());
        }
        let player_spawn = player_spawn.single().translation.truncate();

        let data = Map {
            rects,
            spawn_locations,
            player_spawn,
        };

        let pretty = PrettyConfig::new()
            .depth_limit(2)
            .separate_tuple_members(true)
            .enumerate_arrays(true);
        let s = to_string_pretty(&data, pretty).expect("Serialization failed");

        fs::write("assets/main.map", s).expect("Unable to write file");
        println!("SAVED");
    }
}

fn load_map(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    if let Ok(map) =
        ron::de::from_str::<Map>(&fs::read_to_string("assets/main.map").unwrap_or_default())
    {
        for rect in &map.rects {
            commands
                .spawn_bundle(MaterialMesh2dBundle {
                    mesh: meshes.add(Mesh::from(shape::Quad::default())).into(),
                    material: materials.add(ColorMaterial::from(Color::WHITE)),
                    transform: Transform {
                        translation: rect.position.extend(1.0),
                        //rotation: Quat::from_axis_angle(Vec3::Z, rect.rotation),
                        rotation: Quat::from_euler(EulerRot::XYZ, 0.0, 0.0, rect.rotation),
                        scale: rect.size.extend(1.0),
                    },
                    ..default()
                })
                .insert(WallSquare)
                .insert(Name::new("Wall"))
                .insert_bundle(PickableBundle::default());
        }
        for spawners in &map.spawn_locations {
            commands
                .spawn_bundle(MaterialMesh2dBundle {
                    mesh: meshes.add(Mesh::from(shape::Quad::default())).into(),
                    material: materials.add(ColorMaterial::from(Color::PURPLE)),
                    transform: Transform {
                        translation: spawners.extend(2.2),
                        //rotation: Quat::from_axis_angle(Vec3::Z, rect.rotation),
                        scale: Vec3::splat(0.1),
                        ..Default::default()
                    },
                    ..default()
                })
                .insert(Spawner)
                .insert(Name::new("Wall"))
                .insert_bundle(PickableBundle::default());
        }
        commands
            .spawn_bundle(MaterialMesh2dBundle {
                mesh: meshes.add(Mesh::from(shape::Quad::default())).into(),
                material: materials.add(ColorMaterial::from(Color::GREEN)),
                transform: Transform {
                    translation: map.player_spawn.extend(0.1),
                    scale: Vec3::splat(0.1),
                    //rotation: Quat::from_axis_angle(Vec3::Z, rect.rotation),
                    ..Default::default()
                },
                ..default()
            })
            .insert(PlayerSpawn)
            .insert(Name::new("Wall"))
            .insert_bundle(PickableBundle::default());
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
        .add_plugins(DefaultPickingPlugins)
        .add_plugin(DebugEventsPickingPlugin)
        .add_system(create_square)
        .add_system(move_selected)
        .run();
}

fn create_square(
    mut commands: Commands,
    input: Res<Input<KeyCode>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    if input.just_pressed(KeyCode::Space) && input.pressed(KeyCode::LControl) {
        commands
            .spawn_bundle(MaterialMesh2dBundle {
                mesh: meshes.add(Mesh::from(shape::Quad::default())).into(),
                material: materials.add(ColorMaterial::from(Color::PURPLE)),
                transform: Transform::from_scale(Vec3::splat(0.1)),
                ..default()
            })
            .insert(Spawner)
            .insert(Name::new("Spawner"))
            .insert_bundle(PickableBundle::default());
    } else if input.just_pressed(KeyCode::Space) {
        commands
            .spawn_bundle(MaterialMesh2dBundle {
                mesh: meshes.add(Mesh::from(shape::Quad::default())).into(),
                material: materials.add(ColorMaterial::from(Color::WHITE)),
                ..default()
            })
            .insert(WallSquare)
            .insert(Name::new("Wall"))
            .insert_bundle(PickableBundle::default());
    }
}

fn move_selected(mut transform: Query<(&mut Transform, &Selection)>, input: Res<Input<KeyCode>>) {
    for (mut trans, selected) in transform.iter_mut() {
        if selected.selected() {
            if input.pressed(KeyCode::I) {
                trans.translation.y += 0.02;
            }
            if input.pressed(KeyCode::K) {
                trans.translation.y -= 0.02;
            }
            if input.pressed(KeyCode::J) {
                trans.translation.x -= 0.02;
            }
            if input.pressed(KeyCode::L) {
                trans.translation.x += 0.02;
            }
            if input.pressed(KeyCode::T) {
                trans.scale.y += 0.02;
            }
            if input.pressed(KeyCode::G) {
                trans.scale.y -= 0.02;
            }
            if input.pressed(KeyCode::Y) {
                trans.scale.x += 0.02;
            }
            if input.pressed(KeyCode::H) {
                trans.scale.x -= 0.02;
            }
            if input.pressed(KeyCode::U) {
                trans.rotation *= Quat::from_axis_angle(Vec3::Z, 0.03);
            }
            if input.pressed(KeyCode::O) {
                trans.rotation *= Quat::from_axis_angle(Vec3::Z, -0.03);
            }
        }
    }
}

fn fly_camera(
    mut transform: Query<(&mut OrthographicProjection, &mut Transform), With<Camera>>,
    input: Res<Input<KeyCode>>,
    mut scroll_evr: EventReader<MouseWheel>,
) {
    let (mut camera, mut trans) = transform.single_mut();

    for ev in scroll_evr.iter() {
        camera.scale -= 0.1 * ev.y;
    }

    if input.pressed(KeyCode::W) {
        trans.translation.y += 0.1;
    }
    if input.pressed(KeyCode::S) {
        trans.translation.y -= 0.1;
    }
    if input.pressed(KeyCode::A) {
        trans.translation.x -= 0.1;
    }
    if input.pressed(KeyCode::D) {
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

    commands
        .spawn_bundle(camera)
        .insert_bundle(PickingCameraBundle::default());
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
