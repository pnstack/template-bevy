//! Custom Bevy Plugins
//!
//! This module contains custom plugins that bundle related
//! systems, resources, and components together.

use bevy::prelude::*;

use crate::resources::{GameSettings, GameTimer, ObstacleSpawnTimer, Score};
use crate::states::GameState;
use crate::systems::{
    apply_auto_movement, apply_gravity, apply_velocity, camera_follow_system,
    check_obstacle_collisions, check_platform_collisions, despawn_offscreen_obstacles, player_jump,
    player_movement, setup_camera, setup_camera_follow, spawn_game_ui, spawn_obstacles,
    spawn_platforms, spawn_player, update_health_bar, update_score_display,
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
            .init_resource::<ObstacleSpawnTimer>()
            // Setup systems (run once on startup)
            .add_systems(
                Startup,
                (setup_camera, spawn_player, spawn_platforms, spawn_game_ui),
            )
            // Post-startup setup for camera follow (after player is spawned)
            .add_systems(OnEnter(GameState::Playing), setup_camera_follow)
            // Update systems (run every frame during Playing state)
            // Order: input -> physics -> collision -> movement -> camera -> UI
            .add_systems(
                Update,
                (
                    // Player input systems
                    player_movement,
                    player_jump,
                    // Physics systems
                    apply_gravity,
                    apply_velocity,
                    apply_auto_movement,
                    // Collision systems
                    check_platform_collisions,
                    check_obstacle_collisions,
                    // Obstacle spawning and cleanup
                    spawn_obstacles,
                    despawn_offscreen_obstacles,
                    // Camera system
                    camera_follow_system,
                    // UI systems
                    update_score_display,
                    update_health_bar,
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
