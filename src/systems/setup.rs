//! Setup systems for initializing the game

use bevy::prelude::*;

use crate::components::{Health, Player, Speed};

/// Spawns a 2D camera for the game
pub fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

/// Spawns the player entity with default components
pub fn spawn_player(mut commands: Commands) {
    commands.spawn((
        Player,
        Speed::default(),
        Health::default(),
        Transform::default(),
        Visibility::default(),
    ));
}
