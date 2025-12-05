//! Game Systems
//!
//! This module contains all ECS systems that process components.
//! Systems are functions that run on entities with specific components.

mod movement;
mod setup;

// Re-export specific systems for clarity
pub use movement::{
    apply_gravity, apply_velocity, check_platform_collisions, player_jump, player_movement,
};
pub use setup::{setup_camera, spawn_platforms, spawn_player};
