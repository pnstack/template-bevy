//! Auto-movement systems for entities that move automatically

use bevy::prelude::*;

use crate::components::AutoMove;

/// Applies automatic movement to entities with AutoMove component
pub fn apply_auto_movement(time: Res<Time>, mut query: Query<(&AutoMove, &mut Transform)>) {
    for (auto_move, mut transform) in query.iter_mut() {
        let movement = auto_move.direction * auto_move.speed * time.delta_seconds();
        transform.translation.x += movement.x;
        transform.translation.y += movement.y;
    }
}
