use crate::{
    animation::{Animation, AnimationSetting, Animations},
    components::*,
    sfx::{AudioControler, SfxHandles},
    state::GameState,
};
use bevy::prelude::*;
use bevy_ecs_ldtk::{ldtk::ldtk_fields::LdtkFields, LdtkAsset, LdtkLevel};
use bevy_rapier2d::prelude::*;

const PLAYER_VELOCITY: f32 = 120.0;
const PLAYER_JUMP_VELOCITY: f32 = 300.0;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Title), store_player_texture_handle);

        app.add_systems(
            Update,
            (
                movement,
                allow_moving_spawned_player,
                count_spawn_timer,
                // split_into_two,
                star_set_twinkle_animation,
            )
                .run_if(in_state(GameState::Play)),
        )
        .add_systems(
            Update,
            stop_movement.run_if(in_state(GameState::LevelClear)),
        )
        .add_systems(PostUpdate, star_despawn.run_if(in_state(GameState::Play)))
        .add_systems(PreUpdate, split_into_two.run_if(in_state(GameState::Play)))
        .add_systems(OnEnter(GameState::Play), store_maximum_split_in_level)
        .add_systems(
            Update,
            update_max_split_ui.run_if(in_state(GameState::Play)),
        );
    }
}

#[derive(Copy, Clone, PartialEq, Debug, Default, Component)]
pub struct SpawnTimer(f32);

#[derive(Copy, Clone, PartialEq, Debug, Default, Component)]
pub struct Locked;

#[derive(Resource)]
struct PlayerTexture {
    handle: Handle<TextureAtlas>,
}

fn store_player_texture_handle(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.load("atlas/player.png");
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, (32.0, 32.0).into(), 6, 5, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    commands.insert_resource(PlayerTexture {
        handle: texture_atlas_handle,
    });
}

#[allow(clippy::type_complexity)]
fn movement(
    mut commands: Commands,
    input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Velocity, &GroundDetection), (With<Player>, Without<Locked>)>,
    sfxs: Res<SfxHandles>,
) {
    let mut any_jumped = false;
    for (mut velocity, ground_detection) in &mut query {
        let right = if input.any_pressed([KeyCode::D, KeyCode::Right]) {
            1.
        } else {
            0.
        };
        let left = if input.any_pressed([KeyCode::A, KeyCode::Left]) {
            1.
        } else {
            0.
        };

        velocity.linvel.x = (right - left) * PLAYER_VELOCITY;

        if input.just_pressed(KeyCode::Space) && ground_detection.on_ground {
            velocity.linvel.y = PLAYER_JUMP_VELOCITY;
            any_jumped = true;
        }
    }
    if any_jumped {
        commands.play_sfx(sfxs.jump.clone());
    }
}

fn stop_movement(mut query: Query<(&mut Velocity, &GroundDetection), With<Player>>) {
    for (mut velocity, ground_detection) in query.iter_mut() {
        if ground_detection.on_ground {
            velocity.linvel = Vec2::ZERO;
        }
    }
}

fn split_into_two(
    input: Res<Input<KeyCode>>,
    mut commands: Commands,
    player_texture: Res<PlayerTexture>,
    player_query: Query<(Entity, &Transform, &Player)>,
    ldtk_query: Query<Entity, With<Handle<LdtkAsset>>>,
    mut maximum_split: Option<ResMut<MaximumSplit>>,
    sfxs: Res<SfxHandles>,
) {
    if !input.just_pressed(KeyCode::X) {
        return;
    }

    let Some(maximum_split) = maximum_split.as_mut() else {
        return;
    };

    if maximum_split.current * 2 > maximum_split.max {
        return;
    }
    maximum_split.current *= 2;

    for (e, transform, player) in player_query.iter() {
        let next_player_level = player.level + 1;
        let collider_size = convert_player_level_to_collider_size(next_player_level);
        let players = [
            commands
                .spawn((
                    PlayerBundle {
                        sprite_bundle: SpriteSheetBundle {
                            texture_atlas: player_texture.handle.clone(),
                            sprite: TextureAtlasSprite::new(next_player_level.min(5) as usize),
                            transform: (*transform).with_translation(
                                transform.translation + Vec3::new(collider_size * 1.05, 0.0, 0.0),
                            ),
                            ..Default::default()
                        },
                        collider_bundle: ColliderBundle {
                            collider: Collider::cuboid(collider_size, collider_size),
                            rigid_body: RigidBody::Dynamic,
                            velocity: Velocity::linear(
                                (PLAYER_VELOCITY, PLAYER_JUMP_VELOCITY * 2. / 3.).into(),
                            ),
                            rotation_constraints: LockedAxes::ROTATION_LOCKED,
                            friction: Friction {
                                coefficient: 0.0,
                                combine_rule: CoefficientCombineRule::Min,
                            },
                            ..Default::default()
                        },
                        player: Player {
                            level: next_player_level,
                        },
                        ..Default::default()
                    },
                    SpawnTimer(0.2),
                    Locked,
                    Artificial,
                ))
                .id(),
            commands
                .spawn((
                    PlayerBundle {
                        sprite_bundle: SpriteSheetBundle {
                            texture_atlas: player_texture.handle.clone(),
                            sprite: TextureAtlasSprite::new(next_player_level.min(5) as usize),
                            transform: (*transform).with_translation(
                                transform.translation + Vec3::new(-collider_size * 1.05, 0.0, 0.0),
                            ),
                            ..Default::default()
                        },
                        collider_bundle: ColliderBundle {
                            collider: Collider::cuboid(collider_size, collider_size),
                            rigid_body: RigidBody::Dynamic,
                            velocity: Velocity::linear(
                                (-PLAYER_VELOCITY, PLAYER_JUMP_VELOCITY * 2. / 3.).into(),
                            ),
                            rotation_constraints: LockedAxes::ROTATION_LOCKED,
                            friction: Friction {
                                coefficient: 0.0,
                                combine_rule: CoefficientCombineRule::Min,
                            },
                            ..Default::default()
                        },
                        player: Player {
                            level: next_player_level,
                        },
                        ..Default::default()
                    },
                    SpawnTimer(0.1),
                    Locked,
                    Artificial,
                    ActiveEvents::empty(),
                ))
                .id(),
        ];
        commands.entity(e).despawn();
        let ldtk_entity = ldtk_query.single();
        commands.entity(ldtk_entity).push_children(&players);
    }

    commands.play_sfx(sfxs.split.clone());
}

fn count_spawn_timer(
    mut commands: Commands,
    mut query: Query<(Entity, &mut SpawnTimer), With<Player>>,
    timer: Res<Time>,
) {
    for (e, mut spawn_timer) in query.iter_mut() {
        spawn_timer.0 -= timer.delta_seconds();
        if spawn_timer.0 < 0.0 {
            if let Some(mut entity) = commands.get_entity(e) {
                entity
                    .remove::<SpawnTimer>()
                    .insert(ActiveEvents::COLLISION_EVENTS);
            }
        }
    }
}

#[allow(clippy::type_complexity)]
fn allow_moving_spawned_player(
    mut commands: Commands,
    mut collisions: EventReader<CollisionEvent>,
    locked_query: Query<Entity, (With<Player>, With<Locked>, Without<SpawnTimer>)>,
) {
    for collision_event in collisions.iter() {
        if let CollisionEvent::Started(e1, e2, _) = collision_event {
            if locked_query.contains(*e1) {
                commands.entity(*e1).remove::<(Locked, ActiveEvents)>();
            }
            if locked_query.contains(*e2) {
                commands.entity(*e2).remove::<(Locked, ActiveEvents)>();
            }
        }
    }
}

fn star_set_twinkle_animation(
    mut commands: Commands,
    query: Query<Entity, (With<Star>, Without<AnimationSetting>)>,
    animation: Res<Animations>,
) {
    for e in query.iter() {
        let (texture_atlas, _sprite, animation_setting) = animation
            .animations
            .get(&Animation::TwinkleStar)
            .unwrap()
            .clone();
        commands.entity(e).insert((
            texture_atlas,
            TextureAtlasSprite::new(e.index() as usize & 3), // randomize
            Visibility::Visible,
            animation_setting,
        ));
    }
}

#[allow(clippy::type_complexity)]
fn star_despawn(
    mut commands: Commands,
    player_query: Query<Entity, With<Player>>,
    star_query: Query<(Entity, &Transform), (With<Star>, Without<Player>)>,
    mut collisions: EventReader<CollisionEvent>,
    sfxs: Res<SfxHandles>,
    animation: Res<Animations>,
) {
    let mut removed_stars = Vec::new();
    for collision_event in collisions.iter() {
        if let CollisionEvent::Started(e1, e2, _) = collision_event {
            if player_query.contains(*e1) && star_query.contains(*e2) && !removed_stars.contains(e2)
            {
                removed_stars.push(*e2);
                commands.entity(*e1).despawn();
                commands.entity(*e2).despawn();

                commands.play_sfx(sfxs.star.clone());

                let transform: &Transform = star_query.get_component(*e2).unwrap();
                let (texture_atlas, sprite, animation_setting) = animation
                    .animations
                    .get(&Animation::DespawnStar)
                    .unwrap()
                    .clone();
                commands.spawn((
                    SpriteSheetBundle {
                        texture_atlas,
                        sprite,
                        transform: *transform,
                        ..Default::default()
                    },
                    animation_setting,
                ));
            } else if player_query.contains(*e2)
                && star_query.contains(*e1)
                && !removed_stars.contains(e1)
            {
                removed_stars.push(*e1);
                commands.entity(*e1).despawn();
                commands.entity(*e2).despawn();

                commands.play_sfx(sfxs.star.clone());

                let transform: &Transform = star_query.get_component(*e1).unwrap();
                let (texture_atlas, sprite, animation_setting) = animation
                    .animations
                    .get(&Animation::DespawnStar)
                    .unwrap()
                    .clone();
                commands.spawn((
                    SpriteSheetBundle {
                        texture_atlas,
                        sprite,
                        transform: *transform,
                        ..Default::default()
                    },
                    animation_setting,
                ));
            }
        }
    }
}

fn convert_player_level_to_collider_size(level: u8) -> f32 {
    match level {
        0 => 16.0,
        1 => 13.0,
        2 => 10.0,
        3 => 7.0,
        4 => 5.0,
        5 => 3.0,
        _ => 3.0,
    }
}

#[derive(Clone, Copy, Default, Debug, Resource)]
pub struct MaximumSplit {
    max: i32,
    current: i32,
}

#[derive(Component)]
struct MaximumSplitUi;

fn store_maximum_split_in_level(
    mut commands: Commands,
    player_query: Query<(), With<Player>>,
    level_query: Query<(Entity, &Handle<LdtkLevel>)>,
    level_asset: Res<Assets<LdtkLevel>>,
    asset_server: Res<AssetServer>,
) {
    let Ok((e, handle)) = level_query.get_single() else {
        return;
    };
    let Some(level): Option<&LdtkLevel> = level_asset.get(handle) else {
        return;
    };
    let max_split = level.level.get_int_field("max_split").cloned().unwrap_or(8);

    let maximum_split = MaximumSplit {
        max: max_split,
        current: player_query.iter().count() as i32,
    };

    commands.insert_resource(maximum_split);

    // ui
    commands.entity(e).with_children(|level| {
        level.spawn_empty().insert((
            Text2dBundle {
                text: Text::from_section(
                    format!("{}/{}", maximum_split.current, maximum_split.max),
                    TextStyle {
                        font: asset_server.load("fonts/PeaberryMono.ttf"),
                        font_size: 24.0,
                        color: Color::WHITE,
                    },
                ),
                text_anchor: bevy::sprite::Anchor::BottomLeft,
                transform: Transform::from_translation(Vec3::new(5.0, 3.0, 6.0))
                    .with_scale(Vec3::splat(0.5)),
                ..Default::default()
            },
            MaximumSplitUi,
        ));
    });
}

fn update_max_split_ui(
    mut query: Query<&mut Text, With<MaximumSplitUi>>,
    maximum_split: Res<MaximumSplit>,
) {
    if !maximum_split.is_changed() {
        return;
    }
    if let Ok(mut text) = query.get_single_mut() {
        text.sections[0].value = format!("{}/{}", maximum_split.current, maximum_split.max);
    }
}
