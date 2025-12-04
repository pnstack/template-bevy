//! Template Bevy - Game Development Template
//!
//! A Bevy game template designed for indie game developers.
//! This template provides a solid foundation with organized modules
//! for components, systems, resources, states, and plugins.

pub mod components;
pub mod game;
pub mod plugins;
pub mod resources;
pub mod states;
pub mod systems;

pub use components::*;
pub use game::*;
pub use resources::*;
pub use states::*;
