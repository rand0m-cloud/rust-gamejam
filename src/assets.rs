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
        if !animation.playing && !animation.playing_alt {
            continue;
        }

        animation.timer.tick(time.delta());

        if animation.timer.just_finished() {
            animation.current_frame += 1;
            let frames_len = if animation.playing_alt {
                //If playing alt the frames must exist
                animation.alt_frames.as_ref().unwrap().len()
            } else {
                animation.frames.len()
            };
            if animation.current_frame >= frames_len {
                animation.current_frame = 0;
                animation.playing_alt = false;
            }
        }
        if animation.playing_alt {
            sprite.index = animation.alt_frames.as_ref().unwrap()[animation.current_frame];
        } else {
            sprite.index = animation.frames[animation.current_frame];
        }
        sprite.flip_x = animation.flip_x;
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

    #[asset(texture_atlas(tile_size_x = 512., tile_size_y = 512., columns = 1, rows = 1))]
    #[asset(path = "awesome.png")]
    pub placeholder_atlas: Handle<TextureAtlas>,

    #[asset(path = "dog.png")]
    pub dog: Handle<Image>,

    #[asset(path = "awesome.png")]
    pub dog_spawner: Handle<Image>,

    #[asset(path = "awesome.png")]
    pub chicken_spawner: Handle<Image>,

    #[asset(path = "background.png")]
    pub background: Handle<Image>,

    #[asset(path = "main.map")]
    pub map: Handle<Map>,

    #[asset(path = "bip-bop.ogg")]
    pub background_music: Handle<bevy_kira_audio::AudioSource>,
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
    pub alt_frames: Vec<TextureAtlasSprite>,
    pub texture: Handle<TextureAtlas>,
}

pub struct DogWalkFrames {
    pub frames: Vec<TextureAtlasSprite>,
    pub alt_frames: Vec<TextureAtlasSprite>,
    pub texture: Handle<TextureAtlas>,
}

pub struct ChickWalkFrames {
    pub frames: Vec<TextureAtlasSprite>,
    pub alt_frames: Vec<TextureAtlasSprite>,
    pub texture: Handle<TextureAtlas>,
}

fn parse_animation(file_contents: &str, atlas: &mut TextureAtlas) -> Vec<TextureAtlasSprite> {
    let desc: GraphicsDesc = ron::from_str(file_contents).unwrap_or_else(|e| {
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
    assets: ResMut<OurAssets>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    images: Res<Assets<Image>>,
) {
    //unwrap safe because asset loader
    let chicken_image = images.get(assets.chicken.clone()).unwrap();
    let mut chicken_atlas = TextureAtlas::new_empty(assets.chicken.clone(), chicken_image.size());

    let chick_image = images.get(assets.chicken_minion.clone()).unwrap();
    let mut chick_atlas =
        TextureAtlas::new_empty(assets.chicken_minion.clone(), chick_image.size());

    let dog_image = images.get(assets.dog.clone()).unwrap();
    let mut dog_atlas = TextureAtlas::new_empty(assets.dog.clone(), dog_image.size());

    let chicken_walk = parse_animation(
        include_str!("../assets/chicken_walk.ron"),
        &mut chicken_atlas,
    );
    let chicken_shoot = parse_animation(
        include_str!("../assets/chicken_shoot.ron"),
        &mut chicken_atlas,
    );

    let chick_walk = parse_animation(include_str!("../assets/chick_walk.ron"), &mut chick_atlas);
    let chick_attack =
        parse_animation(include_str!("../assets/chick_attack.ron"), &mut chick_atlas);

    let dog_walk = parse_animation(include_str!("../assets/dog_walk.ron"), &mut dog_atlas);
    let dog_attack = parse_animation(include_str!("../assets/dog_shoot.ron"), &mut dog_atlas);

    let chicken_handle = texture_atlases.add(chicken_atlas);
    let chick_handle = texture_atlases.add(chick_atlas);
    let dog_handle = texture_atlases.add(dog_atlas);

    commands.insert_resource(ChickenWalkFrames {
        frames: chicken_walk,
        alt_frames: chicken_shoot,
        texture: chicken_handle,
    });

    commands.insert_resource(ChickWalkFrames {
        frames: chick_walk,
        alt_frames: chick_attack,
        texture: chick_handle,
    });

    commands.insert_resource(DogWalkFrames {
        frames: dog_walk,
        alt_frames: dog_attack,
        texture: dog_handle,
    });
}
