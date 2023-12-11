use crate::state::GameState;
use bevy::{prelude::*, utils::HashMap};

pub struct AnimationPlugin;

impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Title), setup)
            .add_systems(Update, add_animation_timer)
            .add_systems(Update, animate_sprite);
    }
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub enum Animation {
    TwinkleStar,
    DespawnStar,
    DespawnDoor,
}

#[derive(Resource)]
pub struct Animations {
    pub animations:
        HashMap<Animation, (Handle<TextureAtlas>, TextureAtlasSprite, AnimationSetting)>,
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let mut animations = HashMap::new();
    // twinkle_star
    let texture_handle = asset_server.load("atlas/twinkle_star.png");
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::new(16.0, 16.0), 2, 2, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    animations.insert(
        Animation::TwinkleStar,
        (
            texture_atlas_handle.clone(),
            TextureAtlasSprite::new(0),
            AnimationSetting {
                indices: vec![0, 1, 2, 3],
                fps: 12.0,
                looped: true,
            },
        ),
    );

    // despawn_star
    let texture_handle = asset_server.load("atlas/despawn_star.png");
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::new(32.0, 32.0), 2, 2, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    animations.insert(
        Animation::DespawnStar,
        (
            texture_atlas_handle.clone(),
            TextureAtlasSprite::new(1),
            AnimationSetting {
                indices: vec![1, 2],
                fps: 18.0,
                looped: false,
            },
        ),
    );

    // despawn_door
    let texture_handle = asset_server.load("atlas/despawn_door.png");
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::new(32.0, 32.0), 2, 2, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    animations.insert(
        Animation::DespawnDoor,
        (
            texture_atlas_handle.clone(),
            TextureAtlasSprite::new(1),
            AnimationSetting {
                indices: vec![1, 2],
                fps: 18.0,
                looped: false,
            },
        ),
    );

    commands.insert_resource(Animations { animations });
}

#[derive(Component, Clone)]
pub struct AnimationSetting {
    indices: Vec<usize>,
    fps: f32,
    looped: bool,
}

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

fn add_animation_timer(
    mut commands: Commands,
    mut query: Query<(Entity, &AnimationSetting), Without<AnimationTimer>>,
) {
    for (e, animation) in &mut query {
        commands
            .entity(e)
            .insert(AnimationTimer(Timer::from_seconds(
                1.0 / animation.fps,
                TimerMode::Repeating,
            )));
    }
}

fn animate_sprite(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(
        Entity,
        &AnimationSetting,
        &mut AnimationTimer,
        &mut TextureAtlasSprite,
    )>,
) {
    for (e, animation, mut timer, mut sprite) in &mut query {
        timer.tick(time.delta());
        if timer.just_finished() {
            let i: usize = animation
                .indices
                .iter()
                .position(|&x| x == sprite.index)
                .unwrap_or(0);
            if i == animation.indices.len() - 1 && !animation.looped {
                commands.entity(e).despawn();
                continue;
            }
            let next_index = animation.indices[(i + 1) % animation.indices.len()];
            sprite.index = next_index;
        }
    }
}
