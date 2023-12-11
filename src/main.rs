// This example shows off a more in-depth implementation of a game with `bevy_ecs_ldtk`.
// Please run with `--release`.

use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

use bevy_rapier2d::prelude::*;
use constants::{LEVEL_IIDS, WINDOW_SIZE};
use state::GameState;

mod animation;
mod components;
mod constants;
mod ground;
mod level;
mod level_clear_screen;
mod player;
mod sfx;
mod state;
mod switch;
mod title_screen;
mod ui;
mod win_screen;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        resolution: WINDOW_SIZE.into(),
                        ..Default::default()
                    }),
                    ..Default::default()
                }),
        )
        .add_plugins((
            LdtkPlugin,
            RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0),
        ))
        .add_plugins((
            animation::AnimationPlugin,
            ground::GroundPlugin,
            level::LevelPlugin,
            level_clear_screen::LevelClearScreenPlugin,
            player::PlayerPlugin,
            sfx::SfxPlugin,
            switch::SwitchPlugin,
            title_screen::TitleScreenPlugin,
            ui::UiPlugin,
            win_screen::WinScreenPlugin,
        ))
        .insert_resource(Msaa::Off)
        .insert_resource(RapierConfiguration {
            gravity: Vec2::new(0.0, -800.0),
            ..Default::default()
        })
        .insert_resource(LdtkSettings {
            level_spawn_behavior: LevelSpawnBehavior::UseZeroTranslation,
            set_clear_color: SetClearColor::FromLevelBackground,
            ..Default::default()
        })
        .insert_resource(LevelSelection::Iid(LEVEL_IIDS[0].to_string()))
        .add_state::<GameState>()
        .add_systems(Startup, setup)
        .register_ldtk_int_cell::<components::WallBundle>(1)
        .register_ldtk_entity::<components::PlayerBundle>("Player")
        .register_ldtk_entity::<components::StarBundle>("Star")
        .register_ldtk_entity::<components::SwitchBundle>("Switch")
        .register_ldtk_entity::<components::DoorBundle>("Horizontal_Door")
        .register_ldtk_entity::<components::DoorBundle>("Vertical_Door")
        .register_ldtk_entity::<components::UiDataBundle>("Ui_data")
        .register_ldtk_entity::<components::UiDataBundle>("Ui_long_data")
        .run();
}

pub fn setup(mut commands: Commands) {
    let camera = Camera2dBundle::default();
    commands.spawn(camera);
}
