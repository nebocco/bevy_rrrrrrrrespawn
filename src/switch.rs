use crate::{
    animation::{Animation, Animations},
    components::*,
    sfx::{AudioControler, SfxHandles},
    state::GameState,
};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub struct SwitchPlugin;

impl Plugin for SwitchPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Title), store_map_texture_handle);

        app.add_systems(Update, add_switch_pieds);
        app.add_systems(
            Update,
            (send_switch_pushed_event, switch_pushed_motion).run_if(in_state(GameState::Play)),
        );
        app.add_systems(
            Update,
            (open_door, remove_door).run_if(in_state(GameState::Play)),
        );
    }
}

#[derive(Resource)]
struct MapTexture {
    _handle: Handle<TextureAtlas>,
}

fn store_map_texture_handle(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.load("atlas/player.png");
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, (32.0, 32.0).into(), 6, 5, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    commands.insert_resource(MapTexture {
        _handle: texture_atlas_handle,
    });
}

fn add_switch_pieds(
    mut commands: Commands,
    switch_query: Query<Entity, (With<Switch>, Without<Children>)>,
) {
    for entity in &switch_query {
        let pieds_shape = Collider::cuboid(8., 3.);
        let pieds_translation = Vec3::new(0., -8., 0.);

        let Some(mut e) = commands.get_entity(entity) else {
            continue;
        };

        e.with_children(|switch| {
            switch.spawn_empty().insert((
                pieds_shape,
                RigidBody::Fixed,
                Friction::new(1.0),
                Transform::from_translation(pieds_translation),
                GlobalTransform::default(),
            ));
        });
    }
}

#[derive(Event)]
struct SwitchPushed(Entity);

#[derive(Component)]
struct Pushed;

#[allow(clippy::type_complexity)]
fn send_switch_pushed_event(
    mut commands: Commands,
    switch_query: Query<Entity, (With<Switch>, Without<Player>, Without<Pushed>)>,
    mut collisions: EventReader<CollisionEvent>,
) {
    for collision_event in collisions.iter() {
        if let CollisionEvent::Started(e1, e2, _) = collision_event {
            if switch_query.contains(*e1) {
                commands.entity(*e1).insert(Pushed);
            } else if switch_query.contains(*e2) {
                commands.entity(*e2).insert(Pushed);
            }
        }
    }
}

fn switch_pushed_motion(
    mut pushed_switch_query: Query<&mut TextureAtlasSprite, (With<Switch>, Added<Pushed>)>,
) {
    const PUSHED_SWITCH_INDEX: usize = 14;
    for mut texture_sprite in pushed_switch_query.iter_mut() {
        *texture_sprite = TextureAtlasSprite::new(PUSHED_SWITCH_INDEX);
    }
}

#[derive(Component)]
struct Open;

fn open_door(
    mut commands: Commands,
    door_query: Query<(Entity, &Door), Without<Open>>,
    pushed_switch_query: Query<&RelatedDoor, (With<Switch>, Added<Pushed>)>,
    sfxs: Res<SfxHandles>,
) {
    for related_door in pushed_switch_query.iter() {
        for (door_entity, door) in door_query.iter() {
            if door.0 == related_door.0 {
                commands.entity(door_entity).insert(Open);
                commands.play_sfx(sfxs.switch.clone());
            }
        }
    }
}

#[allow(clippy::type_complexity)]
fn remove_door(
    mut commands: Commands,
    opened_door_query: Query<(Entity, &Transform), (With<Door>, Added<Open>)>,
    animations: Res<Animations>,
) {
    for (e, transform) in opened_door_query.iter() {
        commands.entity(e).despawn();

        let (texture_atlas, sprite, animation_setting) = animations
            .animations
            .get(&Animation::DespawnDoor)
            .expect("Door animation not found")
            .clone();

        commands.spawn((
            SpriteSheetBundle {
                texture_atlas,
                transform: *transform,
                sprite,
                ..Default::default()
            },
            animation_setting,
        ));
    }
}
