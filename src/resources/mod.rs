//! Game Resources
//!
//! This module contains all game resources (global state).
//! Resources are unique data that exists independently of entities.

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

/// Game settings resource
#[derive(Resource, Debug, Clone, Serialize, Deserialize)]
pub struct GameSettings {
    pub master_volume: f32,
    pub music_volume: f32,
    pub sfx_volume: f32,
    pub fullscreen: bool,
}

impl Default for GameSettings {
    fn default() -> Self {
        Self {
            master_volume: 1.0,
            music_volume: 0.7,
            sfx_volume: 1.0,
            fullscreen: false,
        }
    }
}

/// Score tracking resource
#[derive(Resource, Debug, Clone, Default)]
pub struct Score {
    pub current: u32,
    pub high_score: u32,
}

impl Score {
    pub fn add(&mut self, points: u32) {
        self.current += points;
        if self.current > self.high_score {
            self.high_score = self.current;
        }
    }

    pub fn reset(&mut self) {
        self.current = 0;
    }
}

/// Game timer resource
#[derive(Resource, Debug, Clone)]
pub struct GameTimer {
    pub elapsed: f32,
    pub paused: bool,
}

impl Default for GameTimer {
    fn default() -> Self {
        Self {
            elapsed: 0.0,
            paused: false,
        }
    }
}

impl GameTimer {
    pub fn tick(&mut self, delta: f32) {
        if !self.paused {
            self.elapsed += delta;
        }
    }

    pub fn pause(&mut self) {
        self.paused = true;
    }

    pub fn resume(&mut self) {
        self.paused = false;
    }

    pub fn reset(&mut self) {
        self.elapsed = 0.0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_score_add() {
        let mut score = Score::default();
        score.add(100);
        assert_eq!(score.current, 100);
        assert_eq!(score.high_score, 100);
    }

    #[test]
    fn test_score_high_score_update() {
        let mut score = Score::default();
        score.add(50);
        score.reset();
        score.add(30);
        assert_eq!(score.current, 30);
        assert_eq!(score.high_score, 50);
    }

    #[test]
    fn test_game_timer_tick() {
        let mut timer = GameTimer::default();
        timer.tick(1.0);
        assert!((timer.elapsed - 1.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_game_timer_pause() {
        let mut timer = GameTimer::default();
        timer.tick(1.0);
        timer.pause();
        timer.tick(1.0);
        assert!((timer.elapsed - 1.0).abs() < f32::EPSILON);
    }
}
