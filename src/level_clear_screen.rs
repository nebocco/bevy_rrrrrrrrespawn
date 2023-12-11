use crate::{constants::LEVEL_IIDS, state::GameState};
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

pub struct LevelClearScreenPlugin;

impl Plugin for LevelClearScreenPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::LevelClear), spawn_level_clear_screen)
            .add_systems(
                Update,
                move_to_next_level.run_if(in_state(GameState::LevelClear)),
            )
            .add_systems(OnExit(GameState::LevelClear), despawn_level_clear_screen);

        #[cfg(debug_assertions)]
        app.add_systems(Update, move_to_specfic_level);
    }
}

#[derive(Component)]
struct LevelClearScreen;

fn spawn_level_clear_screen(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((
            LevelClearScreen,
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..Default::default()
                },
                background_color: Color::rgba(0.0, 0.0, 0.0, 0.6).into(),
                ..Default::default()
            },
        ))
        .with_children(|parent| {
            parent.spawn((TextBundle::from_section(
                "Level Clear",
                TextStyle {
                    font: asset_server.load("fonts/PeaberryMono.ttf"),
                    font_size: 40.0,
                    color: Color::rgb(0.9, 0.9, 0.9),
                },
            ),));
        });
}

fn despawn_level_clear_screen(mut commands: Commands, q: Query<Entity, With<LevelClearScreen>>) {
    for e in &q {
        commands.entity(e).despawn_recursive();
    }
}

fn move_to_next_level(
    input: Res<Input<KeyCode>>,
    mut commands: Commands,
    mut state: ResMut<NextState<GameState>>,
    ldtk_entity: Query<Entity, With<Handle<LdtkAsset>>>,
    mut level_selection: ResMut<LevelSelection>,
) {
    if !input.just_pressed(KeyCode::Space) {
        return;
    }
    if let LevelSelection::Iid(ref index) = *level_selection {
        let level_index = LEVEL_IIDS.iter().position(|x| x == index).unwrap();
        let e = ldtk_entity.single();
        if level_index + 1 < LEVEL_IIDS.len() {
            commands.entity(e).despawn_descendants();
            state.set(GameState::Spawn);
            *level_selection = LevelSelection::Iid(LEVEL_IIDS[level_index + 1].to_string());
        } else {
            commands.entity(e).despawn_recursive();
            state.set(GameState::Win);
            *level_selection = LevelSelection::Iid(LEVEL_IIDS[0].to_string());
        }
    } else {
        panic!();
    }
}

#[cfg(debug_assertions)]
fn move_to_specfic_level(
    input: Res<Input<KeyCode>>,
    mut commands: Commands,
    mut state: ResMut<NextState<GameState>>,
    ldtk_entity: Query<Entity, With<Handle<LdtkAsset>>>,
    mut level_selection: ResMut<LevelSelection>,
) {
    for (index, key) in [
        KeyCode::Key1,
        KeyCode::Key2,
        KeyCode::Key3,
        KeyCode::Key4,
        KeyCode::Key5,
        KeyCode::Key6,
        KeyCode::Key7,
        KeyCode::Key8,
        KeyCode::Key9,
        KeyCode::Key0,
    ]
    .iter()
    .enumerate()
    {
        if !input.just_pressed(*key) {
            continue;
        }
        let e = ldtk_entity.single();
        if index < LEVEL_IIDS.len() {
            commands.entity(e).despawn_descendants();
            state.set(GameState::Spawn);
            *level_selection = LevelSelection::Iid(LEVEL_IIDS[index].to_string());
        } else {
            commands.entity(e).despawn_recursive();
            state.set(GameState::Win);
            *level_selection = LevelSelection::Iid(LEVEL_IIDS[0].to_string());
        }
    }
}
