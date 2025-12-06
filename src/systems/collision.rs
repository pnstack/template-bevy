//! Collision systems for detecting and handling collisions between entities

use bevy::prelude::*;

use crate::components::{BoxCollider, DamageOnContact, Health, Obstacle, Player};
use crate::game::constants::scoring::OBSTACLE_SURVIVE_POINTS;
use crate::resources::Score;

/// Checks for collisions between player and obstacles
#[allow(clippy::type_complexity)]
pub fn check_obstacle_collisions(
    mut commands: Commands,
    mut player_query: Query<(&Transform, &BoxCollider, &mut Health), With<Player>>,
    obstacle_query: Query<
        (Entity, &Transform, &BoxCollider, &DamageOnContact),
        (With<Obstacle>, Without<Player>),
    >,
    mut score: ResMut<Score>,
) {
    let Ok((player_transform, player_collider, mut player_health)) = player_query.get_single_mut()
    else {
        return;
    };

    let player_half_width = player_collider.width / 2.0;
    let player_half_height = player_collider.height / 2.0;

    for (obstacle_entity, obstacle_transform, obstacle_collider, damage) in obstacle_query.iter() {
        let obstacle_half_width = obstacle_collider.width / 2.0;
        let obstacle_half_height = obstacle_collider.height / 2.0;

        // AABB collision check
        let player_left = player_transform.translation.x - player_half_width;
        let player_right = player_transform.translation.x + player_half_width;
        let player_bottom = player_transform.translation.y - player_half_height;
        let player_top = player_transform.translation.y + player_half_height;

        let obstacle_left = obstacle_transform.translation.x - obstacle_half_width;
        let obstacle_right = obstacle_transform.translation.x + obstacle_half_width;
        let obstacle_bottom = obstacle_transform.translation.y - obstacle_half_height;
        let obstacle_top = obstacle_transform.translation.y + obstacle_half_height;

        // Check for overlap
        let horizontal_overlap = player_left < obstacle_right && player_right > obstacle_left;
        let vertical_overlap = player_bottom < obstacle_top && player_top > obstacle_bottom;

        if horizontal_overlap && vertical_overlap {
            // Apply damage to player
            player_health.take_damage(damage.damage);

            // Remove the obstacle
            commands.entity(obstacle_entity).despawn();

            // Add score for surviving collision
            score.add(OBSTACLE_SURVIVE_POINTS);
        }
    }
}
