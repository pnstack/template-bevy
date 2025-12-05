//! Movement systems for entity motion

use bevy::prelude::*;

use crate::components::{
    BoxCollider, Gravity, Grounded, JumpConfig, Platform, Player, Speed, Velocity,
};

/// Handles player horizontal movement input (A/D or Left/Right arrows)
pub fn player_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&Speed, &mut Velocity), With<Player>>,
) {
    for (speed, mut velocity) in query.iter_mut() {
        let mut direction = 0.0;

        if keyboard_input.pressed(KeyCode::KeyA) || keyboard_input.pressed(KeyCode::ArrowLeft) {
            direction -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::KeyD) || keyboard_input.pressed(KeyCode::ArrowRight) {
            direction += 1.0;
        }

        velocity.0.x = direction * speed.0;
    }
}

/// Handles player jump input (Spacebar) - only when grounded
pub fn player_jump(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Velocity, &Grounded, &JumpConfig), With<Player>>,
) {
    for (mut velocity, grounded, jump_config) in query.iter_mut() {
        // Jump when spacebar is pressed and player is on the ground
        if keyboard_input.just_pressed(KeyCode::Space) && grounded.0 {
            velocity.0.y = jump_config.jump_velocity;
        }

        // Variable jump height: cut velocity when spacebar is released mid-jump
        if keyboard_input.just_released(KeyCode::Space) && velocity.0.y > 0.0 {
            velocity.0.y *= jump_config.jump_cut_multiplier;
        }
    }
}

/// Applies gravity to entities with Gravity and Velocity components
pub fn apply_gravity(time: Res<Time>, mut query: Query<(&Gravity, &mut Velocity, &Grounded)>) {
    for (gravity, mut velocity, grounded) in query.iter_mut() {
        if !grounded.0 {
            velocity.0.y -= gravity.0 * time.delta_seconds();
        }
    }
}

/// Applies velocity to entity transforms
pub fn apply_velocity(time: Res<Time>, mut query: Query<(&Velocity, &mut Transform)>) {
    for (velocity, mut transform) in query.iter_mut() {
        transform.translation.x += velocity.0.x * time.delta_seconds();
        transform.translation.y += velocity.0.y * time.delta_seconds();
    }
}

/// Checks for collisions between player and platforms
#[allow(clippy::type_complexity)]
pub fn check_platform_collisions(
    mut player_query: Query<
        (&mut Transform, &mut Velocity, &BoxCollider, &mut Grounded),
        With<Player>,
    >,
    platform_query: Query<(&Transform, &BoxCollider), (With<Platform>, Without<Player>)>,
) {
    for (mut player_transform, mut velocity, player_collider, mut grounded) in
        player_query.iter_mut()
    {
        let mut is_grounded = false;

        let player_half_width = player_collider.width / 2.0;
        let player_half_height = player_collider.height / 2.0;

        for (platform_transform, platform_collider) in platform_query.iter() {
            let platform_half_width = platform_collider.width / 2.0;
            let platform_half_height = platform_collider.height / 2.0;

            // Calculate AABB bounds
            let player_left = player_transform.translation.x - player_half_width;
            let player_right = player_transform.translation.x + player_half_width;
            let player_bottom = player_transform.translation.y - player_half_height;
            let player_top = player_transform.translation.y + player_half_height;

            let platform_left = platform_transform.translation.x - platform_half_width;
            let platform_right = platform_transform.translation.x + platform_half_width;
            let _platform_bottom = platform_transform.translation.y - platform_half_height;
            let platform_top = platform_transform.translation.y + platform_half_height;

            // Check for horizontal overlap
            let horizontal_overlap = player_left < platform_right && player_right > platform_left;

            // Check if player is landing on top of platform (falling down)
            if horizontal_overlap && velocity.0.y <= 0.0 {
                // Check if player's bottom is at or slightly below platform top
                // and player was above platform before
                let landing_threshold = 10.0;
                if player_bottom <= platform_top
                    && player_bottom >= platform_top - landing_threshold
                    && player_top > platform_top
                {
                    // Land on platform
                    player_transform.translation.y = platform_top + player_half_height;
                    velocity.0.y = 0.0;
                    is_grounded = true;
                }
            }
        }

        grounded.0 = is_grounded;
    }
}
