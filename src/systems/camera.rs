//! Camera follow system

use bevy::prelude::*;

use crate::components::{CameraFollow, MainCamera, Player};
use crate::game::constants::camera::TARGET_FRAMERATE;

/// Updates camera to follow the target entity smoothly
pub fn camera_follow_system(
    time: Res<Time>,
    player_query: Query<&Transform, (With<Player>, Without<MainCamera>)>,
    mut camera_query: Query<(&mut Transform, &CameraFollow), With<MainCamera>>,
) {
    let Ok(player_transform) = player_query.get_single() else {
        return;
    };

    for (mut camera_transform, camera_follow) in camera_query.iter_mut() {
        let target_position = player_transform.translation + camera_follow.offset;

        // Smooth camera movement using lerp (frame-rate independent)
        let lerp_factor = 1.0
            - camera_follow
                .smoothing
                .powf(time.delta_seconds() * TARGET_FRAMERATE);
        camera_transform.translation.x = camera_transform
            .translation
            .x
            .lerp(target_position.x, lerp_factor);
        camera_transform.translation.y = camera_transform
            .translation
            .y
            .lerp(target_position.y, lerp_factor);
    }
}

/// Sets up the camera to follow a specific target entity
pub fn setup_camera_follow(
    mut commands: Commands,
    camera_query: Query<Entity, With<MainCamera>>,
    player_query: Query<Entity, With<Player>>,
) {
    if let (Ok(camera_entity), Ok(player_entity)) =
        (camera_query.get_single(), player_query.get_single())
    {
        commands.entity(camera_entity).insert(
            CameraFollow::new(player_entity)
                .with_offset(Vec3::new(0.0, 50.0, 0.0))
                .with_smoothing(0.05),
        );
    }
}
