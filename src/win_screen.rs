use crate::state::GameState;
use bevy::prelude::*;
use bevy_ecs_ldtk::LdtkAsset;

pub struct WinScreenPlugin;

impl Plugin for WinScreenPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Win), spawn_win_screen)
            .add_systems(OnExit(GameState::Win), despawn_win_screen);

        #[cfg(debug_assertions)]
        app.add_systems(Update, return_to_title.run_if(in_state(GameState::Win)));
    }
}

#[derive(Component)]
struct WinScreen;

fn spawn_win_screen(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut camera_query: Query<(&mut OrthographicProjection, &mut Transform)>,
) {
    commands
        .spawn((
            WinScreen,
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
                    "Thank you for playing!",
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
        WinScreen,
        SpriteBundle {
            texture: asset_server.load("atlas/win_screen.png"),
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 1.0)),
            ..Default::default()
        },
    ));

    let (mut orthographic_projection, mut camera_transform) = camera_query.single_mut();
    *orthographic_projection = Camera2dBundle::default().projection;
    *camera_transform = Transform::default();
}

fn despawn_win_screen(mut commands: Commands, q: Query<Entity, With<WinScreen>>) {
    for e in &q {
        commands.entity(e).despawn_recursive();
    }
}

#[cfg(debug_assertions)]
fn return_to_title(
    mut commands: Commands,
    input: Res<Input<KeyCode>>,
    mut state: ResMut<NextState<GameState>>,
    ldtk_query: Query<Entity, With<Handle<LdtkAsset>>>,
) {
    if input.just_pressed(KeyCode::Space) {
        for entity in &ldtk_query {
            commands.entity(entity).despawn_recursive();
        }
        state.set(GameState::Title)
    }
}
