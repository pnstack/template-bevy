//! Game Systems
//!
//! This module contains all ECS systems that process components.
//! Systems are functions that run on entities with specific components.

mod auto_movement;
mod camera;
mod collision;
mod movement;
mod obstacle;
mod setup;
mod ui;

// Re-export specific systems for clarity
pub use auto_movement::apply_auto_movement;
pub use camera::{camera_follow_system, setup_camera_follow};
pub use collision::check_obstacle_collisions;
pub use movement::{
    apply_gravity, apply_velocity, check_platform_collisions, player_jump, player_movement,
};
pub use obstacle::{despawn_offscreen_obstacles, spawn_obstacles};
pub use setup::{setup_camera, spawn_platforms, spawn_player};
pub use ui::{spawn_game_ui, update_health_bar, update_score_display};
