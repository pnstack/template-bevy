//! User Interface systems for displaying game information

use bevy::prelude::*;

use crate::components::{GameUI, Health, HealthBar, HealthBarFill, Player, ScoreDisplay};
use crate::resources::Score;

/// Spawns the game UI elements
pub fn spawn_game_ui(mut commands: Commands) {
    // Root UI node
    commands
        .spawn((
            GameUI,
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::SpaceBetween,
                    padding: UiRect::all(Val::Px(20.0)),
                    ..default()
                },
                ..default()
            },
        ))
        .with_children(|parent| {
            // Top bar with score and health
            parent
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        flex_direction: FlexDirection::Row,
                        justify_content: JustifyContent::SpaceBetween,
                        align_items: AlignItems::FlexStart,
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    // Score display
                    parent.spawn((
                        ScoreDisplay,
                        TextBundle::from_section(
                            "Score: 0",
                            TextStyle {
                                font_size: 32.0,
                                color: Color::WHITE,
                                ..default()
                            },
                        ),
                    ));

                    // Health bar container
                    parent
                        .spawn((
                            HealthBar,
                            NodeBundle {
                                style: Style {
                                    width: Val::Px(200.0),
                                    height: Val::Px(25.0),
                                    border: UiRect::all(Val::Px(2.0)),
                                    ..default()
                                },
                                background_color: Color::srgba(0.2, 0.2, 0.2, 0.8).into(),
                                border_color: Color::WHITE.into(),
                                ..default()
                            },
                        ))
                        .with_children(|parent| {
                            // Health bar fill
                            parent.spawn((
                                HealthBarFill,
                                NodeBundle {
                                    style: Style {
                                        width: Val::Percent(100.0),
                                        height: Val::Percent(100.0),
                                        ..default()
                                    },
                                    background_color: Color::srgb(0.2, 0.8, 0.2).into(),
                                    ..default()
                                },
                            ));
                        });
                });
        });
}

/// Updates the score display text
pub fn update_score_display(score: Res<Score>, mut query: Query<&mut Text, With<ScoreDisplay>>) {
    if score.is_changed() {
        for mut text in query.iter_mut() {
            text.sections[0].value = format!("Score: {}", score.current);
        }
    }
}

/// Updates the health bar fill based on player health
pub fn update_health_bar(
    player_query: Query<&Health, With<Player>>,
    mut health_bar_query: Query<(&mut Style, &mut BackgroundColor), With<HealthBarFill>>,
) {
    let Ok(health) = player_query.get_single() else {
        return;
    };

    for (mut style, mut background_color) in health_bar_query.iter_mut() {
        let percentage = health.percentage();
        style.width = Val::Percent(percentage * 100.0);

        // Change color based on health level
        let color = if percentage > 0.6 {
            Color::srgb(0.2, 0.8, 0.2) // Green
        } else if percentage > 0.3 {
            Color::srgb(0.8, 0.8, 0.2) // Yellow
        } else {
            Color::srgb(0.8, 0.2, 0.2) // Red
        };
        *background_color = color.into();
    }
}
