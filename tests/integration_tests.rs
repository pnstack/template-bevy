use bevy::math::Vec2;
use template_bevy::components::{AutoMove, DamageOnContact, Health, Obstacle, Speed};
use template_bevy::resources::{GameSettings, GameTimer, ObstacleSpawnTimer, Score};

#[test]
fn test_health_creation() {
    let health = Health::new(100.0);
    assert_eq!(health.current, 100.0);
    assert_eq!(health.max, 100.0);
    assert!(!health.is_dead());
}

#[test]
fn test_health_damage_and_heal() {
    let mut health = Health::new(100.0);

    health.take_damage(30.0);
    assert_eq!(health.current, 70.0);

    health.heal(20.0);
    assert_eq!(health.current, 90.0);
}

#[test]
fn test_speed_default() {
    let speed = Speed::default();
    assert_eq!(speed.0, 200.0);
}

#[test]
fn test_game_settings_default() {
    let settings = GameSettings::default();
    assert_eq!(settings.master_volume, 1.0);
    assert_eq!(settings.music_volume, 0.7);
    assert_eq!(settings.sfx_volume, 1.0);
    assert!(!settings.fullscreen);
}

#[test]
fn test_score_tracking() {
    let mut score = Score::default();
    assert_eq!(score.current, 0);
    assert_eq!(score.high_score, 0);

    score.add(100);
    assert_eq!(score.current, 100);
    assert_eq!(score.high_score, 100);

    score.reset();
    assert_eq!(score.current, 0);
    assert_eq!(score.high_score, 100);
}

#[test]
fn test_game_timer() {
    let mut timer = GameTimer::default();
    assert_eq!(timer.elapsed, 0.0);
    assert!(!timer.paused);

    timer.tick(0.016);
    assert!(timer.elapsed > 0.0);

    timer.pause();
    let elapsed_before = timer.elapsed;
    timer.tick(0.016);
    assert_eq!(timer.elapsed, elapsed_before);

    timer.resume();
    timer.tick(0.016);
    assert!(timer.elapsed > elapsed_before);
}

#[test]
fn test_auto_move_creation() {
    let auto_move = AutoMove::new(Vec2::new(1.0, 0.0), 200.0);
    assert_eq!(auto_move.speed, 200.0);
    assert!((auto_move.direction.x - 1.0).abs() < f32::EPSILON);
}

#[test]
fn test_auto_move_helpers() {
    let left = AutoMove::left(100.0);
    assert_eq!(left.direction.x, -1.0);
    assert_eq!(left.speed, 100.0);

    let right = AutoMove::right(150.0);
    assert_eq!(right.direction.x, 1.0);
    assert_eq!(right.speed, 150.0);
}

#[test]
fn test_damage_on_contact() {
    let default_damage = DamageOnContact::default();
    assert_eq!(default_damage.damage, 10.0);

    let custom_damage = DamageOnContact::new(25.0);
    assert_eq!(custom_damage.damage, 25.0);
}

#[test]
fn test_obstacle_spawn_timer() {
    let default_timer = ObstacleSpawnTimer::default();
    assert_eq!(default_timer.timer.duration().as_secs_f32(), 2.0);

    let custom_timer = ObstacleSpawnTimer::new(3.5);
    assert_eq!(custom_timer.timer.duration().as_secs_f32(), 3.5);
}

#[test]
fn test_obstacle_marker() {
    // Test that Obstacle is a valid marker component (can be constructed)
    let _obstacle = Obstacle;
}
