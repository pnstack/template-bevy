//! Obstacle spawning systems

use bevy::prelude::*;
use rand::Rng;

use crate::components::{AutoMove, BoxCollider, DamageOnContact, Obstacle};
use crate::resources::ObstacleSpawnTimer;

/// Spawns obstacles at regular intervals
pub fn spawn_obstacles(
    mut commands: Commands,
    time: Res<Time>,
    mut spawn_timer: ResMut<ObstacleSpawnTimer>,
) {
    spawn_timer.timer.tick(time.delta());

    if spawn_timer.timer.just_finished() {
        let mut rng = rand::thread_rng();

        // Random spawn position on the right side of the screen
        let spawn_x = 700.0;
        let spawn_y = rng.gen_range(-200.0..150.0);

        // Random obstacle size
        let width = rng.gen_range(30.0..60.0);
        let height = rng.gen_range(30.0..60.0);

        // Random speed
        let speed = rng.gen_range(100.0..250.0);

        commands.spawn((
            Obstacle,
            AutoMove::left(speed),
            BoxCollider::new(width, height),
            DamageOnContact::default(),
            SpriteBundle {
                sprite: Sprite {
                    color: Color::srgb(0.8, 0.2, 0.2), // Red obstacles
                    custom_size: Some(Vec2::new(width, height)),
                    ..default()
                },
                transform: Transform::from_xyz(spawn_x, spawn_y, 0.0),
                ..default()
            },
        ));
    }
}

/// Despawns obstacles that have moved off screen
pub fn despawn_offscreen_obstacles(
    mut commands: Commands,
    query: Query<(Entity, &Transform), With<Obstacle>>,
) {
    for (entity, transform) in query.iter() {
        // Despawn if too far left
        if transform.translation.x < -800.0 {
            commands.entity(entity).despawn();
        }
    }
}
