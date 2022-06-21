#![allow(clippy::type_complexity)]

use std::fs;

use bevy::{
    input::mouse::MouseWheel, prelude::*, render::camera::ScalingMode, window::PresentMode,
};
use bevy_inspector_egui::{WorldInspectorParams, WorldInspectorPlugin};
use ron::ser::{to_string_pretty, PrettyConfig};

pub const CLEAR: Color = Color::rgb(0.3, 0.3, 0.3);
pub const HEIGHT: f32 = 900.0;
pub const RESOLUTION: f32 = 16.0 / 9.0;

use rust_gamejam::map::{Map, Rect};

#[derive(Component)]
pub struct MapSquare;

fn save_map(map: Query<&Transform, With<MapSquare>>, input: Res<Input<KeyCode>>) {
    if input.just_pressed(KeyCode::Return) {
        let mut rects = Vec::new();
        for transform in map.iter() {
            rects.push(Rect {
                position: transform.translation.truncate(),
                size: transform.scale.truncate(),
                rotation: transform.rotation.to_euler(EulerRot::XYZ).2,
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
fn change_selection(
    changed: Query<Entity, (Changed<Transform>, With<MapSquare>)>,
    mut selection: ResMut<Selection>,
) {
    for ent in changed.iter() {
        selection.entity = ent;
    }
}

fn load_map(mut commands: Commands, mut selection: ResMut<Selection>) {
    if let Ok(map) =
        ron::de::from_str::<Map>(&fs::read_to_string("assets/main.map").unwrap_or_default())
    {
        for rect in &map.rects {
            selection.entity = commands
                .spawn_bundle(SpriteBundle {
                    transform: Transform {
                        translation: rect.position.extend(0.0),
                        //rotation: Quat::from_axis_angle(Vec3::Z, rect.rotation),
                        rotation: Quat::from_euler(EulerRot::XYZ, 0.0, 0.0, rect.rotation),
                        scale: rect.size.extend(0.0),
                    },
                    ..default()
                })
                .insert(MapSquare)
                .insert(Name::new("Wall"))
                .id();
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
        .insert_resource(Selection {
            entity: Entity::from_raw(0),
        })
        .add_plugin(WorldInspectorPlugin::new())
        .add_startup_system(spawn_camera)
        .add_startup_system(load_map)
        .add_system(toggle_inspector)
        .add_system(save_map)
        .add_system(fly_camera)
        .add_system(create_square)
        .add_system(move_selected)
        .add_system(color_selected)
        .add_system(change_selection)
        .run();
}

pub struct Selection {
    entity: Entity,
}

fn create_square(
    mut commands: Commands,
    input: Res<Input<KeyCode>>,
    mut selection: ResMut<Selection>,
) {
    if input.just_pressed(KeyCode::Space) {
        selection.entity = commands
            .spawn_bundle(SpriteBundle::default())
            .insert(MapSquare)
            .id();
    }
}

fn color_selected(mut sprites: Query<(Entity, &mut Sprite)>, selection: Res<Selection>) {
    for (ent, mut sprite) in sprites.iter_mut() {
        if ent == selection.entity {
            sprite.color = Color::RED;
        } else {
            sprite.color = Color::WHITE;
        }
    }
}

fn move_selected(
    mut transform: Query<&mut Transform>,
    selection: Res<Selection>,
    input: Res<Input<KeyCode>>,
) {
    if let Ok(mut trans) = transform.get_mut(selection.entity) {
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
