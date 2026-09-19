use bevy::prelude::*;

#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub enum GameState {
    #[default]
    Loading,
    MainMenu,
    InGame,
    GameOver,
}

#[derive(SubStates, Debug, Clone, PartialEq, Eq, Hash, Default)]
#[source(GameState = GameState::InGame)]
pub enum InGameState {
    #[default]
    Playing,
    Paused,
}
