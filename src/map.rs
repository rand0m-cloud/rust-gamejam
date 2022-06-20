use crate::prelude::*;

use bevy::{
    asset::{AssetLoader, LoadContext, LoadedAsset},
    reflect::TypeUuid,
    utils::BoxedFuture,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Rect {
    position: Vec2,
    size: Vec2,
    rotation: f32,
}

#[derive(Debug, Deserialize, TypeUuid)]
#[uuid = "615963e9-3a3d-4eaa-bed3-76e8f05a1070"]
pub struct Map {
    rects: Vec<Rect>,
}

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_asset::<Map>()
            .init_asset_loader::<MapLoader>()
            .add_system_set(SystemSet::on_enter(GameState::GamePlay).with_system(create_map));
    }
}

fn create_map(map_assets: Res<Assets<Map>>, our_assets: Res<OurAssets>, mut commands: Commands) {
    let map = map_assets.get(our_assets.map.clone()).unwrap();
    for rect in &map.rects {
        commands
            .spawn_bundle(SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(rect.size),
                    ..default()
                },
                transform: Transform {
                    translation: rect.position.extend(0.0),
                    rotation: Quat::from_axis_angle(Vec3::Z, rect.rotation.to_radians()),
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
            .insert(Name::new("Wall"));
    }
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