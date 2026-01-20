//! Integration tests for Otter Swag game logic

use otter_swag::*;

/// Helper to create a game ready for collision testing
/// Sets otter at a stable position that won't move too far
fn setup_collision_test() -> Game {
    let mut game = Game::new();
    game.start();
    // Position otter in middle of screen, at the boundary so it won't move
    game.otter.y = OTTER_MIN_Y; // Will stay here when swimming up
    game.otter.velocity_y = -OTTER_VELOCITY; // Swimming up
    game
}

#[test]
fn test_full_game_cycle() {
    let mut game = Game::new();

    // Start in menu
    assert_eq!(game.state, GameState::Menu);

    // Press space to start
    game.handle_space_pressed();
    assert_eq!(game.state, GameState::Playing);

    // Simulate some gameplay
    for _ in 0..100 {
        game.update();
    }

    // Game should still be playing (unless hit by missile)
    // Score might have increased from coins
}

#[test]
fn test_otter_stays_in_bounds_during_gameplay() {
    let mut game = Game::new();
    game.start();

    // Simulate holding space (swim up)
    for _ in 0..200 {
        game.otter.swim_up();
        game.otter.update();
        assert!(game.otter.y >= OTTER_MIN_Y, "Otter went above min Y");
    }

    // Simulate releasing space (swim down)
    for _ in 0..200 {
        game.otter.swim_down();
        game.otter.update();
        assert!(
            game.otter.y + OTTER_HEIGHT <= SCREEN_HEIGHT,
            "Otter went below screen"
        );
    }
}

#[test]
fn test_missile_spawning_and_cleanup() {
    let mut game = Game::new();
    game.start();

    // Run many frames to spawn missiles
    for _ in 0..500 {
        game.update();
    }

    // Missiles should have spawned
    // Some might still be active, some cleaned up
    // Just verify no panics occurred
}

#[test]
fn test_coin_collection_increases_score() {
    let mut game = setup_collision_test();

    // Get otter collision rect and place coin overlapping it
    let (ox, oy, _ow, _oh) = game.otter.get_collision_rect();

    // Place coin directly overlapping otter's collision rect
    let mut coin = Coin::new(ox);
    coin.y = oy;
    game.coins.push(coin);

    let initial_score = game.score;

    // Verify collision would occur
    let otter_rect = game.otter.get_collision_rect();
    let coin_rect = game.coins[0].get_collision_rect();
    assert!(
        rects_collide(otter_rect, coin_rect),
        "Coin should overlap otter. Otter: {:?}, Coin: {:?}",
        otter_rect,
        coin_rect
    );

    game.update();

    assert!(
        game.score > initial_score,
        "Score didn't increase after collecting coin. Score: {}, Initial: {}",
        game.score,
        initial_score
    );
}

#[test]
fn test_fish_grants_invincibility() {
    let mut game = setup_collision_test();

    // Get otter collision rect
    let (ox, oy, _, _) = game.otter.get_collision_rect();

    // Place fish overlapping otter
    let mut fish = Fish::new(oy);
    fish.x = ox;
    game.fish.push(fish);

    assert!(!game.otter.is_invincible);

    // Verify collision
    assert!(
        rects_collide(game.otter.get_collision_rect(), game.fish[0].get_collision_rect()),
        "Fish should overlap otter"
    );

    game.update();

    assert!(
        game.otter.is_invincible,
        "Otter should be invincible after eating fish"
    );
}

#[test]
fn test_missile_collision_without_invincibility_ends_game() {
    let mut game = setup_collision_test();

    // Get otter collision rect
    let (ox, oy, _, _) = game.otter.get_collision_rect();

    // Ensure not invincible
    assert!(!game.otter.is_invincible);

    // Place missile overlapping otter
    let mut missile = Missile::new(oy);
    missile.x = ox;
    game.missiles.push(missile);

    // Verify collision
    assert!(
        rects_collide(game.otter.get_collision_rect(), game.missiles[0].get_collision_rect()),
        "Missile should overlap otter"
    );

    game.update();

    assert_eq!(
        game.state,
        GameState::GameOver,
        "Game should be over after missile collision"
    );
}

#[test]
fn test_missile_collision_with_invincibility_destroys_missile() {
    let mut game = setup_collision_test();

    // Get otter collision rect
    let (ox, oy, _, _) = game.otter.get_collision_rect();

    // Make otter invincible
    game.otter.activate_invincibility();

    // Place missile overlapping otter
    let mut missile = Missile::new(oy);
    missile.x = ox;
    game.missiles.push(missile);

    // Verify collision
    assert!(
        rects_collide(game.otter.get_collision_rect(), game.missiles[0].get_collision_rect()),
        "Missile should overlap otter"
    );

    game.update();

    // Game should still be playing
    assert_eq!(game.state, GameState::Playing);

    // Missile should be exploding
    assert_eq!(game.missiles[0].state, MissileState::Exploding);
}

#[test]
fn test_high_score_updates_on_game_over() {
    let mut game = setup_collision_test();

    // Get otter collision rect
    let (ox, oy, _, _) = game.otter.get_collision_rect();

    // Give some score
    game.score = 500;

    // Place missile to trigger game over
    let mut missile = Missile::new(oy);
    missile.x = ox;
    game.missiles.push(missile);

    game.update();

    assert_eq!(game.state, GameState::GameOver);
    // Score gets +7 per frame before game over, so high score is 507
    assert_eq!(game.high_score, 500 + SCORE_PER_FRAME, "High score should be updated on game over");
}

#[test]
fn test_high_score_preserved_across_resets() {
    let mut game = Game::new();
    game.high_score = 1000;

    game.reset();

    assert_eq!(game.high_score, 1000, "High score should be preserved");
    assert_eq!(game.score, 0, "Current score should reset");
}

#[test]
fn test_sound_effects_generated() {
    let mut game = setup_collision_test();

    // Get otter collision rect
    let (ox, oy, _, _) = game.otter.get_collision_rect();

    // Place coin overlapping otter
    let mut coin = Coin::new(ox);
    coin.y = oy;
    game.coins.push(coin);

    game.update();

    let sounds = game.take_pending_sounds();
    assert!(
        sounds.contains(&SoundEffect::Coin),
        "Coin sound should be triggered. Got sounds: {:?}",
        sounds
    );
}

#[test]
fn test_difficulty_increases_with_score() {
    let mut game = Game::new();
    game.start();

    let initial_spawn_rate = game.obstacle_spawn_rate;
    game.score = 1000;
    game.update();

    assert!(
        game.obstacle_spawn_rate < initial_spawn_rate,
        "Spawn rate should decrease (more missiles) as score increases"
    );
}

#[test]
fn test_invincibility_score_based_expiry() {
    let mut game = setup_collision_test();
    game.score = 500;

    // Get otter collision rect and place fish overlapping
    let (ox, oy, _, _) = game.otter.get_collision_rect();
    let mut fish = Fish::new(oy);
    fish.x = ox;
    game.fish.push(fish);

    // Verify collision would occur
    assert!(
        rects_collide(game.otter.get_collision_rect(), game.fish[0].get_collision_rect()),
        "Fish should overlap otter"
    );

    // Collect the fish to activate invincibility
    game.update();
    assert!(game.otter.is_invincible, "Should be invincible after eating fish");

    // invincibility_check_score should be: score after update (507) + INVINCIBILITY_SCORE_DURATION (2000) = 2507
    let check_score = game.invincibility_check_score.expect("Should have check score set");

    // Set score so that after update (which adds SCORE_PER_FRAME), we're still below threshold
    game.score = check_score - SCORE_PER_FRAME - 1;
    game.update();
    assert!(game.otter.is_invincible, "Should still be invincible before threshold");

    // Set score so that after update we exceed threshold
    game.score = check_score - SCORE_PER_FRAME + 1;
    game.update();
    assert!(!game.otter.is_invincible, "Should no longer be invincible after score threshold");
}

#[test]
fn test_multiple_coins_collection() {
    let mut game = setup_collision_test();

    // Get otter collision rect
    let (ox, oy, _, _) = game.otter.get_collision_rect();

    // Add multiple coins at otter position
    for _ in 0..5 {
        let mut coin = Coin::new(ox);
        coin.y = oy;
        game.coins.push(coin);
    }

    game.update();

    // Score is: SCORE_PER_FRAME (7) + 5 * COIN_SCORE (100 each)
    assert_eq!(game.score, SCORE_PER_FRAME + 5 * COIN_SCORE, "Should collect all 5 coins plus per-frame score");
}

#[test]
fn test_collision_detection_function() {
    // Exact overlap
    assert!(rects_collide((0, 0, 10, 10), (0, 0, 10, 10)));

    // Partial overlap
    assert!(rects_collide((0, 0, 10, 10), (5, 5, 10, 10)));

    // No overlap - to the right
    assert!(!rects_collide((0, 0, 10, 10), (20, 0, 10, 10)));

    // No overlap - below
    assert!(!rects_collide((0, 0, 10, 10), (0, 20, 10, 10)));

    // Edge touching (should not collide)
    assert!(!rects_collide((0, 0, 10, 10), (10, 0, 10, 10)));
}
