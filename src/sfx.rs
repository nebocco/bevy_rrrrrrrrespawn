use crate::state::GameState;
use bevy::prelude::*;

pub struct SfxPlugin;
impl Plugin for SfxPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Title), setup);
    }
}

#[derive(Resource)]
pub struct SfxHandles {
    pub jump: Handle<AudioSource>,
    pub split: Handle<AudioSource>,
    pub switch: Handle<AudioSource>,
    pub star: Handle<AudioSource>,
    pub select: Handle<AudioSource>,
    pub clear: Handle<AudioSource>,
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let handles = SfxHandles {
        jump: asset_server.load("sfx/jump.wav"),
        split: asset_server.load("sfx/split.wav"),
        switch: asset_server.load("sfx/switch.wav"),
        star: asset_server.load("sfx/star.wav"),
        select: asset_server.load("sfx/select.wav"),
        clear: asset_server.load("sfx/clear.wav"),
    };

    commands.insert_resource(handles);

    let bgm = asset_server.load("music/Abstraction - Three Red Hearts - Puzzle Pieces.wav");
    commands.play_bgm(bgm);
}

pub(crate) trait AudioControler {
    fn play_sfx(&mut self, handle: Handle<AudioSource>);
    fn play_bgm(&mut self, handle: Handle<AudioSource>);
}

impl<'w, 's> AudioControler for Commands<'w, 's> {
    fn play_sfx(&mut self, source: Handle<AudioSource>) {
        self.spawn(AudioBundle {
            source,
            settings: PlaybackSettings {
                volume: bevy::audio::Volume::new_relative(0.2),
                ..PlaybackSettings::ONCE
            },
        });
    }

    fn play_bgm(&mut self, source: Handle<AudioSource>) {
        self.spawn(AudioBundle {
            source,
            settings: PlaybackSettings {
                volume: bevy::audio::Volume::new_relative(0.12),
                ..PlaybackSettings::LOOP
            },
        });
    }
}
