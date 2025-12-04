use template_bevy::components::{Health, Speed};
use template_bevy::resources::{GameSettings, GameTimer, Score};

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
