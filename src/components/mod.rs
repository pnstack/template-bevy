//! Game Components
//!
//! This module contains all ECS components used in the game.
//! Components are data containers attached to entities.

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

/// Marker component for the player entity
#[derive(Component, Debug, Default)]
pub struct Player;

/// Movement speed component
#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct Speed(pub f32);

impl Default for Speed {
    fn default() -> Self {
        Self(200.0)
    }
}

/// Health component for entities that can take damage
#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct Health {
    pub current: f32,
    pub max: f32,
}

impl Default for Health {
    fn default() -> Self {
        Self {
            current: 100.0,
            max: 100.0,
        }
    }
}

impl Health {
    pub fn new(max: f32) -> Self {
        Self { current: max, max }
    }

    pub fn take_damage(&mut self, amount: f32) {
        self.current = (self.current - amount).max(0.0);
    }

    pub fn heal(&mut self, amount: f32) {
        self.current = (self.current + amount).min(self.max);
    }

    pub fn is_dead(&self) -> bool {
        self.current <= 0.0
    }

    pub fn percentage(&self) -> f32 {
        self.current / self.max
    }
}

/// Velocity component for moving entities
#[derive(Component, Debug, Default, Clone)]
pub struct Velocity(pub Vec2);

/// Marker component for entities that can collide
#[derive(Component, Debug, Default)]
pub struct Collider {
    pub radius: f32,
}

impl Collider {
    pub fn new(radius: f32) -> Self {
        Self { radius }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_health_take_damage() {
        let mut health = Health::new(100.0);
        health.take_damage(30.0);
        assert_eq!(health.current, 70.0);
    }

    #[test]
    fn test_health_take_damage_clamps_to_zero() {
        let mut health = Health::new(100.0);
        health.take_damage(150.0);
        assert_eq!(health.current, 0.0);
    }

    #[test]
    fn test_health_heal() {
        let mut health = Health::new(100.0);
        health.take_damage(50.0);
        health.heal(30.0);
        assert_eq!(health.current, 80.0);
    }

    #[test]
    fn test_health_heal_clamps_to_max() {
        let mut health = Health::new(100.0);
        health.take_damage(20.0);
        health.heal(50.0);
        assert_eq!(health.current, 100.0);
    }

    #[test]
    fn test_health_is_dead() {
        let mut health = Health::new(100.0);
        assert!(!health.is_dead());
        health.take_damage(100.0);
        assert!(health.is_dead());
    }

    #[test]
    fn test_health_percentage() {
        let mut health = Health::new(100.0);
        health.take_damage(25.0);
        assert!((health.percentage() - 0.75).abs() < f32::EPSILON);
    }
}
