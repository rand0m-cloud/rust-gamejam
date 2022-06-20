use std::fs;

use bevy_asset_loader::AssetCollection;
use serde::Deserialize;

use crate::prelude::*;

pub struct GameAssetsPlugin;

pub const PIXEL_SIZE: f32 = 200.00;

impl Plugin for GameAssetsPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_exit(GameState::Splash).with_system(load_graphics))
            .add_system(animate_frames);
    }
}

fn animate_frames(mut graphics: Query<(&mut TextureAtlasSprite, &mut Animation)>, time: Res<Time>) {
    for (mut sprite, mut animation) in graphics.iter_mut() {
        if !animation.playing {
            continue;
        }
        animation.timer.tick(time.delta());
        if animation.timer.just_finished() {
            animation.current_frame += 1;
            if animation.current_frame >= animation.frames.len() {
                animation.current_frame = 0;
            }
            sprite.index = animation.frames[animation.current_frame];
            sprite.flip_x = animation.flip_x;
        }
    }
}

#[derive(AssetCollection)]
pub struct OurAssets {
    #[asset(path = "awesome.png")]
    pub placeholder: Handle<Image>,
    #[asset(path = "chicken.png")]
    pub chicken: Handle<Image>,
    #[asset(path = "chicken_minion.png")]
    pub chicken_minion: Handle<Image>,
    #[asset(path = "awesome.png")]
    pub enemy_placeholder: Handle<Image>,
    #[asset(path = "awesome.png")]
    pub chicken_spawner: Handle<Image>,
    #[asset(path = "awesome.png")]
    pub dog_spawner: Handle<Image>,
    #[asset(path = "main.map")]
    pub map: Handle<Map>,
}

#[derive(Default, Clone, Copy, Debug, Reflect, Deserialize)]
pub struct SpriteDesc {
    pub pos: (f32, f32),
    pub size: (f32, f32),
}

impl SpriteDesc {
    pub fn to_atlas_rect(self) -> bevy::sprite::Rect {
        bevy::sprite::Rect {
            //A tiny amount is clipped off the sides of the rectangle
            //to stop contents of other sprites from bleeding through
            min: Vec2::new(self.pos.0 + 0.15, self.pos.1 + 0.15),
            max: Vec2::new(
                self.pos.0 + self.size.0 - 0.15,
                self.pos.1 + self.size.1 - 0.15,
            ),
        }
    }
}

#[derive(Deserialize)]
pub struct GraphicsDesc {
    frames: Vec<SpriteDesc>,
}

pub struct ChickenWalkFrames {
    pub frames: Vec<TextureAtlasSprite>,
    pub texture: Handle<TextureAtlas>,
}

pub struct ChickWalkFrames {
    pub frames: Vec<TextureAtlasSprite>,
    pub texture: Handle<TextureAtlas>,
}

fn parse_animation(file_name: &str, atlas: &mut TextureAtlas) -> Vec<TextureAtlasSprite> {
    let desc = fs::read_to_string(file_name).unwrap();
    let desc: GraphicsDesc = ron::from_str(&desc).unwrap_or_else(|e| {
        panic!("Failed to load config: {}", e);
    });

    let mut frames = Vec::new();

    for desc in desc.frames.iter() {
        let mut sprite = TextureAtlasSprite::new(atlas.add_texture(desc.to_atlas_rect()));
        //Set the size to be proportional to the source rectangle
        sprite.custom_size = Some(Vec2::new(
            desc.size.0 / PIXEL_SIZE,
            desc.size.1 / PIXEL_SIZE,
        ));

        frames.push(sprite);
    }

    frames
}

fn load_graphics(
    mut commands: Commands,
    assets: Res<OurAssets>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    images: Res<Assets<Image>>,
) {
    //unwrap safe because asset loader
    let chicken_image = images.get(assets.chicken.clone()).unwrap();
    let mut chicken_atlas = TextureAtlas::new_empty(assets.chicken.clone(), chicken_image.size());

    let chicken_walk = parse_animation("assets/chicken_walk.ron", &mut chicken_atlas);
    let chicken_shoot = parse_animation("assets/chicken_walk.ron", &mut chicken_atlas);

    let handle = texture_atlases.add(chicken_atlas);

    commands.insert_resource(ChickenWalkFrames {
        frames: chicken_walk,
        texture: handle,
    });
}
