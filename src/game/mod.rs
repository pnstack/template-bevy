//! Core Game Logic
//!
//! This module contains core game logic that doesn't fit
//! into components, systems, or resources.

/// Game configuration constants
pub mod constants {
    /// Default window width
    pub const WINDOW_WIDTH: f32 = 1280.0;
    /// Default window height
    pub const WINDOW_HEIGHT: f32 = 720.0;
    /// Game title
    pub const GAME_TITLE: &str = "Template Bevy Game";
}
