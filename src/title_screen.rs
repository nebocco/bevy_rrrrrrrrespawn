use crate::{
    sfx::{AudioControler, SfxHandles},
    state::GameState,
};
use bevy::prelude::*;

pub struct TitleScreenPlugin;

impl Plugin for TitleScreenPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Title), spawn_title_screen)
            .add_systems(Update, start_play.run_if(in_state(GameState::Title)))
            .add_systems(OnExit(GameState::Title), despawn_title_screen);
    }
}

#[derive(Component)]
struct Title;

fn spawn_title_screen(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((
            Title,
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..Default::default()
                },
                ..Default::default()
            },
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle {
                text: Text::from_section(
                    "Press Space to start",
                    TextStyle {
                        font: asset_server.load("fonts/PeaberryMono.ttf"),
                        font_size: 40.0,
                        color: Color::rgb(0.9, 0.9, 0.9),
                    },
                ),
                style: Style {
                    position_type: PositionType::Absolute,
                    bottom: Val::Percent(10.0),
                    ..Default::default()
                },
                ..Default::default()
            });
        });

    commands.spawn((
        Title,
        SpriteBundle {
            texture: asset_server.load("atlas/title.png"),
            ..Default::default()
        },
    ));
}

fn despawn_title_screen(mut commands: Commands, q: Query<Entity, With<Title>>) {
    for e in &q {
        commands.entity(e).despawn_recursive();
    }
}

fn start_play(
    mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    mut state: ResMut<NextState<GameState>>,
    sfxs: Res<SfxHandles>,
) {
    if keyboard_input.any_just_pressed([KeyCode::Space, KeyCode::Return]) {
        state.set(GameState::Load);
        commands.play_sfx(sfxs.select.clone());
    }
}
