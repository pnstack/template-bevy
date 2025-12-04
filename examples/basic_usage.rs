//! Basic usage example for the Bevy game template
//!
//! This example demonstrates how to use the template's
//! components, resources, and systems.

use bevy::prelude::*;
use template_bevy::components::{Health, Player, Speed, Velocity};
use template_bevy::resources::{GameSettings, Score};
use template_bevy::states::GameState;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_state::<GameState>()
        .init_resource::<GameSettings>()
        .init_resource::<Score>()
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (update_player, print_score).run_if(in_state(GameState::Playing)),
        )
        .add_systems(OnEnter(GameState::Loading), start_game)
        .run();
}

fn start_game(mut next_state: ResMut<NextState<GameState>>) {
    next_state.set(GameState::Playing);
}

fn setup(mut commands: Commands) {
    // Spawn camera
    commands.spawn(Camera2d);

    // Spawn player with components
    commands.spawn((
        Player,
        Speed(300.0),
        Health::new(100.0),
        Velocity::default(),
        Sprite {
            color: Color::srgb(0.25, 0.75, 0.25),
            custom_size: Some(Vec2::new(50.0, 50.0)),
            ..default()
        },
        Transform::default(),
    ));

    println!("Game initialized!");
    println!("Use WASD or Arrow keys to move");
    println!("Press ESC to quit");
}

fn update_player(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut query: Query<(&Speed, &mut Transform), With<Player>>,
    mut score: ResMut<Score>,
) {
    for (speed, mut transform) in query.iter_mut() {
        let mut direction = Vec2::ZERO;

        if keyboard_input.pressed(KeyCode::KeyW) || keyboard_input.pressed(KeyCode::ArrowUp) {
            direction.y += 1.0;
        }
        if keyboard_input.pressed(KeyCode::KeyS) || keyboard_input.pressed(KeyCode::ArrowDown) {
            direction.y -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::KeyA) || keyboard_input.pressed(KeyCode::ArrowLeft) {
            direction.x -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::KeyD) || keyboard_input.pressed(KeyCode::ArrowRight) {
            direction.x += 1.0;
        }

        let movement = direction.normalize_or_zero() * speed.0 * time.delta_seconds();
        transform.translation.x += movement.x;
        transform.translation.y += movement.y;

        // Add score for moving
        if direction != Vec2::ZERO {
            score.add(1);
        }
    }
}

fn print_score(score: Res<Score>, mut last_score: Local<u32>) {
    if score.current != *last_score && score.current % 100 == 0 {
        println!(
            "Score: {} (High Score: {})",
            score.current, score.high_score
        );
        *last_score = score.current;
    }
}
