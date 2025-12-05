//! Basic usage example for the Bevy game template
//!
//! This example demonstrates how to use the template's
//! components, resources, and systems for a 2D platformer.

use bevy::prelude::*;
use template_bevy::components::{
    BoxCollider, Gravity, Grounded, Health, JumpConfig, Platform, Player, Speed, Velocity,
};
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
            (
                player_horizontal_movement,
                player_jump,
                apply_gravity,
                apply_velocity,
                check_collisions,
                print_score,
            )
                .chain()
                .run_if(in_state(GameState::Playing)),
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

    // Spawn player with platformer components
    let player_size = Vec2::new(40.0, 50.0);
    commands.spawn((
        Player,
        Speed(250.0),
        Health::new(100.0),
        Velocity::default(),
        Gravity::default(),
        Grounded(false),
        JumpConfig::default(),
        BoxCollider::new(player_size.x, player_size.y),
        Sprite {
            color: Color::srgb(0.2, 0.6, 1.0),
            custom_size: Some(player_size),
            ..default()
        },
        Transform::from_xyz(0.0, 100.0, 0.0),
    ));

    // Spawn ground
    let ground_size = Vec2::new(800.0, 40.0);
    commands.spawn((
        Platform,
        BoxCollider::new(ground_size.x, ground_size.y),
        Sprite {
            color: Color::srgb(0.4, 0.3, 0.2),
            custom_size: Some(ground_size),
            ..default()
        },
        Transform::from_xyz(0.0, -250.0, 0.0),
    ));

    // Spawn floating platforms
    for (x, y, width) in [
        (-200.0, -100.0, 150.0),
        (150.0, 0.0, 120.0),
        (-50.0, 100.0, 180.0),
    ] {
        let size = Vec2::new(width, 20.0);
        commands.spawn((
            Platform,
            BoxCollider::new(size.x, size.y),
            Sprite {
                color: Color::srgb(0.3, 0.5, 0.3),
                custom_size: Some(size),
                ..default()
            },
            Transform::from_xyz(x, y, 0.0),
        ));
    }

    println!("2D Platformer initialized!");
    println!("Use A/D or Arrow keys to move left/right");
    println!("Press SPACE to jump (only when on ground)");
}

fn player_horizontal_movement(
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

fn player_jump(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Velocity, &Grounded, &JumpConfig), With<Player>>,
) {
    for (mut velocity, grounded, jump_config) in query.iter_mut() {
        if keyboard_input.just_pressed(KeyCode::Space) && grounded.0 {
            velocity.0.y = jump_config.jump_velocity;
        }
        if keyboard_input.just_released(KeyCode::Space) && velocity.0.y > 0.0 {
            velocity.0.y *= jump_config.jump_cut_multiplier;
        }
    }
}

fn apply_gravity(time: Res<Time>, mut query: Query<(&Gravity, &mut Velocity, &Grounded)>) {
    for (gravity, mut velocity, grounded) in query.iter_mut() {
        if !grounded.0 {
            velocity.0.y -= gravity.0 * time.delta_seconds();
        }
    }
}

fn apply_velocity(time: Res<Time>, mut query: Query<(&Velocity, &mut Transform)>) {
    for (velocity, mut transform) in query.iter_mut() {
        transform.translation.x += velocity.0.x * time.delta_seconds();
        transform.translation.y += velocity.0.y * time.delta_seconds();
    }
}

#[allow(clippy::type_complexity)]
fn check_collisions(
    mut player_query: Query<
        (&mut Transform, &mut Velocity, &BoxCollider, &mut Grounded),
        With<Player>,
    >,
    platform_query: Query<(&Transform, &BoxCollider), (With<Platform>, Without<Player>)>,
) {
    for (mut player_tf, mut velocity, player_col, mut grounded) in player_query.iter_mut() {
        let mut is_grounded = false;
        let ph_w = player_col.width / 2.0;
        let ph_h = player_col.height / 2.0;

        for (platform_tf, platform_col) in platform_query.iter() {
            let plat_h_w = platform_col.width / 2.0;
            let plat_h_h = platform_col.height / 2.0;

            let p_left = player_tf.translation.x - ph_w;
            let p_right = player_tf.translation.x + ph_w;
            let p_bottom = player_tf.translation.y - ph_h;
            let p_top = player_tf.translation.y + ph_h;

            let plat_left = platform_tf.translation.x - plat_h_w;
            let plat_right = platform_tf.translation.x + plat_h_w;
            let plat_top = platform_tf.translation.y + plat_h_h;

            let h_overlap = p_left < plat_right && p_right > plat_left;

            if h_overlap && velocity.0.y <= 0.0 {
                let threshold = 10.0;
                if p_bottom <= plat_top && p_bottom >= plat_top - threshold && p_top > plat_top {
                    player_tf.translation.y = plat_top + ph_h;
                    velocity.0.y = 0.0;
                    is_grounded = true;
                }
            }
        }
        grounded.0 = is_grounded;
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
