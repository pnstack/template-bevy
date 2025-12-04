//! Game Systems
//!
//! This module contains all ECS systems that process components.
//! Systems are functions that run on entities with specific components.

mod movement;
mod setup;

pub use movement::*;
pub use setup::*;
