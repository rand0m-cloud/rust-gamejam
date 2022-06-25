use crate::prelude::*;

use bevy::{
    asset::{AssetLoader, LoadContext, LoadedAsset},
    reflect::TypeUuid,
    utils::BoxedFuture,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Rect {
    pub position: Vec2,
    pub size: Vec2,
    pub rotation: f32,
}

#[derive(Debug, Deserialize, Serialize, TypeUuid)]
#[uuid = "615963e9-3a3d-4eaa-bed3-76e8f05a1070"]
pub struct Map {
    pub rects: Vec<Rect>,
    pub spawn_locations: Vec<(Vec2, ChickenOrDog)>,
    pub player_spawn: Vec2,
    pub enemy_spawn: Vec2,
}

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_asset::<Map>()
            .init_asset_loader::<MapLoader>()
            .add_system_set(SystemSet::on_exit(GameState::Splash).with_system(create_map));
    }
}

fn create_map(map_assets: Res<Assets<Map>>, our_assets: Res<OurAssets>, mut commands: Commands) {
    let map = map_assets.get(our_assets.map.clone()).unwrap();
    let mut walls = Vec::new();
    for rect in &map.rects {
        walls.push(
            commands
                .spawn_bundle(SpriteBundle {
                    sprite: Sprite {
                        custom_size: Some(rect.size),
                        //Don't render walls anymore :)
                        color: Color::NONE,
                        ..default()
                    },
                    transform: Transform {
                        translation: rect.position.extend(10.0),
                        //rotation: Quat::from_axis_angle(Vec3::Z, rect.rotation),
                        rotation: Quat::from_euler(EulerRot::XYZ, 0.0, 0.0, rect.rotation),
                        ..Default::default()
                    },
                    ..default()
                })
                .insert(RigidBody::Static)
                .insert(CollisionShape::Cuboid {
                    half_extends: rect.size.extend(0.0) / 2.0,
                    border_radius: Some(0.0),
                })
                .insert(RotationConstraints::lock())
                .insert(CollisionLayers::all_masks::<Layer>().with_group(Layer::Wall))
                .insert(Name::new("Wall"))
                .id(),
        );
    }
    commands
        .spawn_bundle(TransformBundle {
            local: Transform::from_xyz(0.0, 0.0, 10.0),
            ..default()
        })
        .push_children(&walls)
        .insert(Name::new("Map"));
}

#[derive(Default)]
pub struct MapLoader;

impl AssetLoader for MapLoader {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, Result<(), anyhow::Error>> {
        Box::pin(async move {
            let custom_asset = ron::de::from_bytes::<Map>(bytes)?;
            load_context.set_default_asset(LoadedAsset::new(custom_asset));
            Ok(())
        })
    }

    fn extensions(&self) -> &[&str] {
        &["map"]
    }
}
