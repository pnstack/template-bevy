//! Game States
//!
//! This module defines all possible game states.
//! States control the flow of the game (menu, playing, paused, etc).

use bevy::prelude::*;

/// Main game state
#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum GameState {
    /// Loading assets and initializing
    #[default]
    Loading,
    /// Main menu screen
    MainMenu,
    /// Active gameplay
    Playing,
    /// Game is paused
    Paused,
    /// Game over screen
    GameOver,
}

/// In-game sub-state for more granular control
#[derive(SubStates, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
#[source(GameState = GameState::Playing)]
pub enum PlayingState {
    /// Normal gameplay
    #[default]
    Running,
    /// Showing a cutscene
    Cutscene,
    /// Player is in dialogue
    Dialogue,
}
