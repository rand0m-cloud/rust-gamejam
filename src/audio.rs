use crate::prelude::*;
use bevy_kira_audio::{AudioApp, AudioChannel, AudioPlugin};

pub struct GameAudioPlugin;

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

fn create_audio_state(bgm_channel: Res<AudioChannel<Background>>) {
    let global_volume = 0.5;

    bgm_channel.set_volume(global_volume);
}

fn update_volume(bgm_channel: Res<AudioChannel<Background>>, volume: Res<f32>) {
    if volume.is_changed() {
        bgm_channel.set_volume(*volume);
    }
}

fn play_background_music(
    background_channel: Res<AudioChannel<Background>>,
    assets: Res<OurAssets>,
) {
    background_channel.play(assets.background_music.clone());
}
