//! Custom Bevy Plugins
//!
//! This module contains custom plugins that bundle related
//! systems, resources, and components together.

use bevy::prelude::*;

use crate::resources::{GameSettings, GameTimer, Score};
use crate::states::GameState;
use crate::systems::{
    apply_gravity, apply_velocity, check_platform_collisions, player_jump, player_movement,
    setup_camera, spawn_platforms, spawn_player,
};

/// Main game plugin that sets up all game systems
pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            // Initialize states
            .init_state::<GameState>()
            // Initialize resources
            .init_resource::<GameSettings>()
            .init_resource::<Score>()
            .init_resource::<GameTimer>()
            // Setup systems (run once on startup)
            .add_systems(Startup, (setup_camera, spawn_player, spawn_platforms))
            // Update systems (run every frame during Playing state)
            // Order: input -> physics -> collision -> movement
            .add_systems(
                Update,
                (
                    player_movement,
                    player_jump,
                    apply_gravity,
                    apply_velocity,
                    check_platform_collisions,
                )
                    .chain()
                    .run_if(in_state(GameState::Playing)),
            )
            // Transition from Loading to MainMenu after startup
            .add_systems(OnEnter(GameState::Loading), transition_to_menu);
    }
}

/// System to transition from Loading to MainMenu
fn transition_to_menu(mut next_state: ResMut<NextState<GameState>>) {
    // In a real game, you would wait for assets to load
    // For now, transition immediately to Playing for demo purposes
    next_state.set(GameState::Playing);
}

/// Debug plugin for development
pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        #[cfg(debug_assertions)]
        {
            app.add_systems(Update, debug_info);
        }
    }
}

#[cfg(debug_assertions)]
fn debug_info(keyboard_input: Res<ButtonInput<KeyCode>>) {
    if keyboard_input.just_pressed(KeyCode::F1) {
        info!("Debug: F1 pressed - Add your debug functionality here");
    }
}
