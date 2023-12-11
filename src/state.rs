use bevy::prelude::*;

#[derive(States, SystemSet, Default, Debug, Hash, PartialEq, Eq, Clone)]
pub enum GameState {
    #[default]
    Title,
    Load,
    Spawn,
    Play,
    LevelClear,
    Win,
}
