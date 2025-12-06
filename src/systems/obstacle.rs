//! Obstacle spawning systems

use bevy::prelude::*;
use rand::Rng;

use crate::components::{AutoMove, BoxCollider, DamageOnContact, Obstacle};
use crate::game::constants::obstacles::{
    DESPAWN_X, HEIGHT_MAX, HEIGHT_MIN, SPAWN_X, SPAWN_Y_MAX, SPAWN_Y_MIN, SPEED_MAX, SPEED_MIN,
    WIDTH_MAX, WIDTH_MIN,
};
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
        let spawn_x = SPAWN_X;
        let spawn_y = rng.gen_range(SPAWN_Y_MIN..SPAWN_Y_MAX);

        // Random obstacle size
        let width = rng.gen_range(WIDTH_MIN..WIDTH_MAX);
        let height = rng.gen_range(HEIGHT_MIN..HEIGHT_MAX);

        // Random speed
        let speed = rng.gen_range(SPEED_MIN..SPEED_MAX);

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
        if transform.translation.x < DESPAWN_X {
            commands.entity(entity).despawn();
        }
    }
}
