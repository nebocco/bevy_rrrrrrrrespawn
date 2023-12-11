use crate::{components::*, state::GameState};
use bevy::{asset::LoadState, prelude::*};
use bevy_ecs_ldtk::prelude::*;

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(LevelSelection::Index(0));

        // Load
        app.add_systems(OnExit(GameState::Title), setup_ldtk_world)
            .add_systems(Update, check_load_status.run_if(in_state(GameState::Load)));

        // Spawn
        app.add_systems(
            Update,
            move_to_play_state.run_if(in_state(GameState::Spawn)),
        );

        // Play
        app.add_systems(
            Update,
            move_to_level_clear_state.run_if(in_state(GameState::Play).and_then(level_clear)),
        )
        .add_systems(
            Update,
            restart_level
                .run_if(in_state(GameState::Play).or_else(in_state(GameState::LevelClear))),
        )
        .add_systems(Update, camera_fit_inside_current_level);
    }
}

fn move_to_play_state(mut state: ResMut<NextState<GameState>>) {
    state.set(GameState::Play);
}

fn move_to_level_clear_state(mut state: ResMut<NextState<GameState>>) {
    state.set(GameState::LevelClear);
}

fn setup_ldtk_world(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(LdtkWorldBundle {
        ldtk_handle: asset_server.load("levels.ldtk"),
        ..Default::default()
    });
}

fn check_load_status(
    ldtk_handle: Query<&Handle<LdtkAsset>>,
    asset_server: Res<AssetServer>,
    mut state: ResMut<NextState<GameState>>,
) {
    if let Ok(handle) = ldtk_handle.get_single() {
        if asset_server.get_load_state(handle.clone()) == LoadState::Loaded {
            state.set(GameState::Spawn);
        }
    } else {
        println!("wait for loading...");
    }
}

fn level_clear(q: Query<(), With<Star>>) -> bool {
    q.is_empty()
}

fn restart_level(
    mut commands: Commands,
    level_query: Query<Entity, With<Handle<LdtkLevel>>>,
    artificial_query: Query<Entity, With<Artificial>>,
    input: Res<Input<KeyCode>>,
    mut state: ResMut<NextState<GameState>>,
) {
    if input.just_pressed(KeyCode::R) {
        for e in artificial_query.iter() {
            commands.entity(e).despawn();
        }
        for level_entity in &level_query {
            commands.entity(level_entity).insert(Respawn);
        }
        state.set(GameState::Spawn);
    }
}

#[allow(clippy::type_complexity)]
fn camera_fit_inside_current_level(
    mut camera_query: Query<(&mut OrthographicProjection, &mut Transform)>,
    level_query: Query<(&Transform, &Handle<LdtkLevel>), Without<OrthographicProjection>>,
    level_selection: Res<LevelSelection>,
    ldtk_levels: Res<Assets<LdtkLevel>>,
) {
    let (mut orthographic_projection, mut camera_transform) = camera_query.single_mut();

    for (level_transform, level_handle) in &level_query {
        let Some(ldtk_level) = ldtk_levels.get(level_handle) else {
            continue;
        };
        let level = &ldtk_level.level;
        if !level_selection.is_match(&0, level) {
            continue;
        }

        let window_size = (level.px_hei as f32)
            .max(level.px_wid as f32)
            .clamp(360.0, 960.0);
        orthographic_projection.scaling_mode = bevy::render::camera::ScalingMode::Fixed {
            width: window_size,
            height: window_size,
        };

        camera_transform.translation.x = level_transform.translation.x + level.px_wid as f32 / 2.;
        camera_transform.translation.y = level_transform.translation.y + level.px_hei as f32 / 2.;
    }
}
