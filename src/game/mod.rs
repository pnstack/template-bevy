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
    pub const GAME_TITLE: &str = "2D Mario-Style Platformer";

    /// Camera smoothing constants
    pub mod camera {
        /// Target framerate for smoothing calculations
        pub const TARGET_FRAMERATE: f32 = 60.0;
        /// Minimum smoothing factor (instant follow)
        pub const MIN_SMOOTHING: f32 = 0.01;
        /// Maximum smoothing factor (slowest follow)
        pub const MAX_SMOOTHING: f32 = 1.0;
    }

    /// Obstacle spawning constants
    pub mod obstacles {
        /// X position where obstacles spawn (right side of screen)
        pub const SPAWN_X: f32 = 700.0;
        /// Minimum Y position for obstacle spawn
        pub const SPAWN_Y_MIN: f32 = -200.0;
        /// Maximum Y position for obstacle spawn
        pub const SPAWN_Y_MAX: f32 = 150.0;
        /// Minimum obstacle width
        pub const WIDTH_MIN: f32 = 30.0;
        /// Maximum obstacle width
        pub const WIDTH_MAX: f32 = 60.0;
        /// Minimum obstacle height
        pub const HEIGHT_MIN: f32 = 30.0;
        /// Maximum obstacle height
        pub const HEIGHT_MAX: f32 = 60.0;
        /// Minimum obstacle speed
        pub const SPEED_MIN: f32 = 100.0;
        /// Maximum obstacle speed
        pub const SPEED_MAX: f32 = 250.0;
        /// X position at which obstacles are despawned (left side of screen)
        pub const DESPAWN_X: f32 = -800.0;
    }

    /// Scoring constants
    pub mod scoring {
        /// Points awarded for surviving obstacle collision
        pub const OBSTACLE_SURVIVE_POINTS: u32 = 10;
    }
}
