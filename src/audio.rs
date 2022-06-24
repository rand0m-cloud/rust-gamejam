use crate::prelude::*;
use bevy_kira_audio::{Audio, AudioApp, AudioChannel, AudioPlugin, AudioSource};

pub struct GameAudioPlugin;

pub struct AudioState {
    global_volume: f32,
}

struct Background;

impl Plugin for GameAudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(AudioPlugin)
            .add_system_set(SystemSet::on_exit(GameState::Splash).with_system(create_audio_state))
            .add_system_set(
                SystemSet::on_exit(GameState::Splash)
                    .with_system(play_background_music.after(create_audio_state)),
            )
            .add_system_set(SystemSet::on_update(GameState::MainMenu).with_system(update_volume))
            .add_audio_channel::<Background>();
    }
}

fn create_audio_state(mut commands: Commands, bgm_channel: Res<AudioChannel<Background>>) {
    let global_volume = 0.5;

    bgm_channel.set_volume(global_volume);

    commands.insert_resource(AudioState { global_volume });
}

fn update_volume(bgm_channel: Res<AudioChannel<Background>>, state: Res<AudioState>) {
    if state.is_changed() {
        bgm_channel.set_volume(state.global_volume);
    }
}

fn play_background_music(
    background_channel: Res<AudioChannel<Background>>,
    assets: Res<OurAssets>,
) {
    background_channel.play(assets.background_music.clone());
}
