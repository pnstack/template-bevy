//! Setup systems for initializing the game

use bevy::prelude::*;

use crate::components::{
    BoxCollider, Gravity, Grounded, Health, JumpConfig, Platform, Player, Speed, Velocity,
};

/// Spawns a 2D camera for the game
pub fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

/// Spawns the player entity with platformer components
pub fn spawn_player(mut commands: Commands) {
    let player_size = Vec2::new(40.0, 50.0);

    commands.spawn((
        Player,
        Speed(250.0),
        Health::default(),
        Velocity::default(),
        Gravity::default(),
        Grounded(false),
        JumpConfig::default(),
        BoxCollider::new(player_size.x, player_size.y),
        SpriteBundle {
            sprite: Sprite {
                color: Color::srgb(0.2, 0.6, 1.0), // Blue player
                custom_size: Some(player_size),
                ..default()
            },
            transform: Transform::from_xyz(0.0, 100.0, 0.0),
            ..default()
        },
    ));
}

/// Spawns the ground and platforms for the platformer
pub fn spawn_platforms(mut commands: Commands) {
    // Ground platform
    let ground_width = 800.0;
    let ground_height = 40.0;
    commands.spawn((
        Platform,
        BoxCollider::new(ground_width, ground_height),
        SpriteBundle {
            sprite: Sprite {
                color: Color::srgb(0.4, 0.3, 0.2), // Brown ground
                custom_size: Some(Vec2::new(ground_width, ground_height)),
                ..default()
            },
            transform: Transform::from_xyz(0.0, -250.0, 0.0),
            ..default()
        },
    ));

    // Floating platforms
    let platform_configs = [
        (-200.0, -100.0, 150.0, 20.0), // Left lower platform
        (150.0, -50.0, 120.0, 20.0),   // Right lower platform
        (-50.0, 50.0, 180.0, 20.0),    // Center middle platform
        (250.0, 120.0, 100.0, 20.0),   // Right upper platform
        (-250.0, 150.0, 100.0, 20.0),  // Left upper platform
    ];

    for (x, y, width, height) in platform_configs {
        commands.spawn((
            Platform,
            BoxCollider::new(width, height),
            SpriteBundle {
                sprite: Sprite {
                    color: Color::srgb(0.3, 0.5, 0.3), // Green platforms
                    custom_size: Some(Vec2::new(width, height)),
                    ..default()
                },
                transform: Transform::from_xyz(x, y, 0.0),
                ..default()
            },
        ));
    }
}
