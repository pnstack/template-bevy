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

/// Gravity component for entities affected by gravity
#[derive(Component, Debug, Clone)]
pub struct Gravity(pub f32);

impl Default for Gravity {
    fn default() -> Self {
        Self(980.0) // Pixels per second squared
    }
}

/// Component tracking if entity is on the ground
#[derive(Component, Debug, Default, Clone)]
pub struct Grounded(pub bool);

/// Marker component for static platform entities
#[derive(Component, Debug, Default)]
pub struct Platform;

/// Box collider for AABB collision detection
#[derive(Component, Debug, Clone)]
pub struct BoxCollider {
    pub width: f32,
    pub height: f32,
}

impl BoxCollider {
    pub fn new(width: f32, height: f32) -> Self {
        Self { width, height }
    }
}

impl Default for BoxCollider {
    fn default() -> Self {
        Self {
            width: 50.0,
            height: 50.0,
        }
    }
}

/// Jump configuration component for Mario-like jumping
#[derive(Component, Debug, Clone)]
pub struct JumpConfig {
    pub jump_velocity: f32,
    pub jump_cut_multiplier: f32,
}

impl Default for JumpConfig {
    fn default() -> Self {
        Self {
            jump_velocity: 450.0,
            jump_cut_multiplier: 0.5,
        }
    }
}

/// Marker component for obstacle entities
#[derive(Component, Debug, Default)]
pub struct Obstacle;

/// Auto-movement component for entities that move automatically
#[derive(Component, Debug, Clone)]
pub struct AutoMove {
    /// Direction of movement (normalized)
    pub direction: Vec2,
    /// Movement speed
    pub speed: f32,
}

impl Default for AutoMove {
    fn default() -> Self {
        Self {
            direction: Vec2::new(-1.0, 0.0), // Move left by default
            speed: 150.0,
        }
    }
}

impl AutoMove {
    pub fn new(direction: Vec2, speed: f32) -> Self {
        Self {
            direction: direction.normalize_or_zero(),
            speed,
        }
    }

    /// Creates a left-moving auto-move component
    pub fn left(speed: f32) -> Self {
        Self::new(Vec2::new(-1.0, 0.0), speed)
    }

    /// Creates a right-moving auto-move component
    pub fn right(speed: f32) -> Self {
        Self::new(Vec2::new(1.0, 0.0), speed)
    }
}

/// Marker component for the main camera that follows the player
#[derive(Component, Debug, Default)]
pub struct MainCamera;

/// Camera follow configuration component
#[derive(Component, Debug, Clone)]
pub struct CameraFollow {
    /// Target entity to follow (usually the player)
    pub target: Option<Entity>,
    /// Offset from the target position
    pub offset: Vec3,
    /// Smoothing factor (0.0 = instant, 1.0 = very smooth/slow)
    pub smoothing: f32,
}

impl Default for CameraFollow {
    fn default() -> Self {
        Self {
            target: None,
            offset: Vec3::ZERO,
            smoothing: 0.1,
        }
    }
}

impl CameraFollow {
    pub fn new(target: Entity) -> Self {
        Self {
            target: Some(target),
            ..Default::default()
        }
    }

    pub fn with_offset(mut self, offset: Vec3) -> Self {
        self.offset = offset;
        self
    }

    pub fn with_smoothing(mut self, smoothing: f32) -> Self {
        self.smoothing = smoothing.clamp(0.01, 1.0);
        self
    }
}

/// Marker component for UI root node
#[derive(Component, Debug, Default)]
pub struct GameUI;

/// Marker component for score display UI
#[derive(Component, Debug, Default)]
pub struct ScoreDisplay;

/// Marker component for health bar UI
#[derive(Component, Debug, Default)]
pub struct HealthBar;

/// Marker component for health bar fill (the colored part)
#[derive(Component, Debug, Default)]
pub struct HealthBarFill;

/// Damage value component for obstacles
#[derive(Component, Debug, Clone)]
pub struct DamageOnContact {
    pub damage: f32,
}

impl Default for DamageOnContact {
    fn default() -> Self {
        Self { damage: 10.0 }
    }
}

impl DamageOnContact {
    pub fn new(damage: f32) -> Self {
        Self { damage }
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

    #[test]
    fn test_gravity_default() {
        let gravity = Gravity::default();
        assert_eq!(gravity.0, 980.0);
    }

    #[test]
    fn test_grounded_default() {
        let grounded = Grounded::default();
        assert!(!grounded.0);
    }

    #[test]
    fn test_box_collider_new() {
        let collider = BoxCollider::new(100.0, 50.0);
        assert_eq!(collider.width, 100.0);
        assert_eq!(collider.height, 50.0);
    }

    #[test]
    fn test_box_collider_default() {
        let collider = BoxCollider::default();
        assert_eq!(collider.width, 50.0);
        assert_eq!(collider.height, 50.0);
    }

    #[test]
    fn test_jump_config_default() {
        let config = JumpConfig::default();
        assert_eq!(config.jump_velocity, 450.0);
        assert_eq!(config.jump_cut_multiplier, 0.5);
    }

    #[test]
    fn test_auto_move_default() {
        let auto_move = AutoMove::default();
        assert_eq!(auto_move.direction, Vec2::new(-1.0, 0.0));
        assert_eq!(auto_move.speed, 150.0);
    }

    #[test]
    fn test_auto_move_new() {
        let auto_move = AutoMove::new(Vec2::new(2.0, 0.0), 200.0);
        assert!((auto_move.direction.x - 1.0).abs() < f32::EPSILON);
        assert_eq!(auto_move.speed, 200.0);
    }

    #[test]
    fn test_auto_move_left() {
        let auto_move = AutoMove::left(100.0);
        assert_eq!(auto_move.direction, Vec2::new(-1.0, 0.0));
        assert_eq!(auto_move.speed, 100.0);
    }

    #[test]
    fn test_auto_move_right() {
        let auto_move = AutoMove::right(100.0);
        assert_eq!(auto_move.direction, Vec2::new(1.0, 0.0));
        assert_eq!(auto_move.speed, 100.0);
    }

    #[test]
    fn test_camera_follow_default() {
        let follow = CameraFollow::default();
        assert!(follow.target.is_none());
        assert_eq!(follow.offset, Vec3::ZERO);
        assert!((follow.smoothing - 0.1).abs() < f32::EPSILON);
    }

    #[test]
    fn test_damage_on_contact_default() {
        let damage = DamageOnContact::default();
        assert_eq!(damage.damage, 10.0);
    }

    #[test]
    fn test_damage_on_contact_new() {
        let damage = DamageOnContact::new(25.0);
        assert_eq!(damage.damage, 25.0);
    }
}
