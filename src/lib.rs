//! Otter Swag - A Rust port of the classic game
//!
//! This module contains the core game logic, separated from rendering
//! to allow for testing and different rendering backends.

use rand::Rng;

// =============================================================================
// Constants (from original C++ source)
// =============================================================================

pub const SCREEN_WIDTH: i32 = 480;
pub const SCREEN_HEIGHT: i32 = 320;
pub const FRAMES_PER_SECOND: u32 = 10;

// Otter constants (from otter.cpp)
pub const OTTER_WIDTH: i32 = 32;
pub const OTTER_HEIGHT: i32 = 32;
pub const OTTER_START_X: i32 = 50;
pub const OTTER_START_Y: i32 = -35;
pub const OTTER_VELOCITY: i32 = 16; // width / 2

// Boundary constants
pub const OTTER_MIN_Y: i32 = 20;
pub const OTTER_WALK_Y: i32 = 280;

// Missile constants (from missile.cpp)
pub const MISSILE_VELOCITY_X: i32 = 20;
pub const MISSILE_EXPLODE_FRAMES: usize = 13;

// Coin constants (from coin.cpp)
pub const COIN_START_Y: i32 = -50;
pub const COIN_VELOCITY_Y: i32 = 7;
pub const COIN_VELOCITY_X_WATER: i32 = -10;
pub const COIN_WATER_THRESHOLD: i32 = 50;
pub const COIN_BOTTOM_THRESHOLD: i32 = 298;
pub const COIN_SCORE: u32 = 100;

// Score constants (from main.cpp)
pub const SCORE_PER_FRAME: u32 = 7;        // Added every frame during gameplay
pub const MISSILE_DESTROY_SCORE: u32 = 200; // Bonus for destroying missile while invincible
pub const INVINCIBILITY_SCORE_DURATION: u32 = 2000; // Invincibility lasts until score increases by this

// Fish constants (from fish.cpp)
pub const FISH_VELOCITY_X: i32 = 10;

// Spawn rates (from main.cpp)
pub const COIN_SPAWN_CHANCE: i32 = 50;   // out of 1000 (5%)
pub const FISH_SPAWN_CHANCE: i32 = 3;    // out of 1000 (0.3%)

// =============================================================================
// Sprite Clip Data (from original .cpp files)
// =============================================================================

/// A simple rectangle for sprite clipping
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ClipRect {
    pub x: i32,
    pub y: i32,
    pub w: u32,
    pub h: u32,
}

impl ClipRect {
    pub const fn new(x: i32, y: i32, w: u32, h: u32) -> Self {
        Self { x, y, w, h }
    }
}

// Otter sprite clips (from otter.cpp set_clips)
pub const OTTER_CLIPS_DOWN: &[ClipRect] = &[
    ClipRect::new(215, 0, 33, 32),
    ClipRect::new(248, 0, 32, 32),
];

pub const OTTER_CLIPS_UP: &[ClipRect] = &[
    ClipRect::new(411, 0, 32, 32),
    ClipRect::new(411, 0, 32, 32),
];

pub const OTTER_CLIPS_WALK: &[ClipRect] = &[
    ClipRect::new(312, 0, 33, 32),
    ClipRect::new(345, 0, 33, 32),
];

pub const OTTER_CLIPS_ROLL: &[ClipRect] = &[
    ClipRect::new(64, 0, 33, 32),
    ClipRect::new(97, 0, 26, 32),
    ClipRect::new(123, 0, 33, 32),
    ClipRect::new(156, 0, 26, 32),
];

// Missile sprite clips (from missile.cpp)
pub const MISSILE_CLIPS_SHOOT: &[ClipRect] = &[
    ClipRect::new(13, 16, 33, 8),
    ClipRect::new(58, 16, 32, 8),
];

pub const MISSILE_CLIPS_EXPLODE: &[ClipRect] = &[
    ClipRect::new(49, 67, 35, 28),
    ClipRect::new(148, 57, 52, 42),
    ClipRect::new(236, 24, 92, 78),
    ClipRect::new(334, 24, 105, 78),
    ClipRect::new(458, 32, 80, 70),
    ClipRect::new(568, 41, 76, 61),
    ClipRect::new(28, 124, 73, 62),
    ClipRect::new(137, 124, 74, 62),
    ClipRect::new(242, 117, 78, 69),
    ClipRect::new(349, 117, 81, 70),
    ClipRect::new(459, 117, 79, 69),
    ClipRect::new(567, 117, 77, 69),
    ClipRect::new(13, 16, 1, 1), // Final frame (invisible)
];

// Coin sprite clips (from coin.cpp)
pub const COIN_CLIPS_SPIN: &[ClipRect] = &[
    ClipRect::new(6, 3, 14, 16),
    ClipRect::new(23, 3, 12, 16),
    ClipRect::new(38, 3, 11, 16),
    ClipRect::new(52, 3, 9, 16),
    ClipRect::new(65, 3, 4, 16),
    ClipRect::new(73, 3, 9, 16),
    ClipRect::new(85, 3, 11, 16),
    ClipRect::new(99, 3, 12, 16),
    ClipRect::new(114, 3, 14, 16),
    ClipRect::new(130, 3, 16, 16),
];

// Fish sprite clips (from fish.cpp)
pub const FISH_CLIPS_SWIM: &[ClipRect] = &[
    ClipRect::new(281, 15, 31, 17),
    ClipRect::new(241, 13, 30, 18),
    ClipRect::new(323, 10, 30, 23),
    ClipRect::new(405, 5, 30, 26),
    ClipRect::new(364, 5, 30, 27),
];

// Number sprite clips (from scoreCounter.cpp) - for score display
// Each digit 0-9 from numbers.bmp
pub const NUMBER_CLIPS: &[ClipRect] = &[
    ClipRect::new(287, 0, 21, 28), // 0
    ClipRect::new(0, 0, 20, 28),   // 1
    ClipRect::new(30, 0, 24, 28),  // 2
    ClipRect::new(62, 0, 23, 28),  // 3
    ClipRect::new(93, 0, 26, 28),  // 4
    ClipRect::new(126, 0, 25, 28), // 5
    ClipRect::new(157, 0, 26, 28), // 6
    ClipRect::new(189, 0, 26, 28), // 7
    ClipRect::new(221, 0, 26, 28), // 8
    ClipRect::new(253, 0, 26, 28), // 9
];

// Digit spacing for score display
pub const DIGIT_SPACING: i32 = 28;

// =============================================================================
// Game Types
// =============================================================================

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GameState {
    Menu,
    Playing,
    GameOver,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OtterState {
    SwimmingDown,
    SwimmingUp,
    Walking,
    Rolling, // Invincible
}

impl OtterState {
    pub fn get_clips(&self) -> &'static [ClipRect] {
        match self {
            OtterState::SwimmingDown => OTTER_CLIPS_DOWN,
            OtterState::SwimmingUp => OTTER_CLIPS_UP,
            OtterState::Walking => OTTER_CLIPS_WALK,
            OtterState::Rolling => OTTER_CLIPS_ROLL,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MissileState {
    Shooting,
    Exploding,
}

/// Sound effect events that the renderer should play
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum SoundEffect {
    Coin,
    Powerup,
    Boom,
}

// =============================================================================
// Game Entities
// =============================================================================

#[derive(Clone, Debug)]
pub struct Otter {
    pub x: i32,
    pub y: i32,
    pub velocity_y: i32,
    pub state: OtterState,
    pub frame: usize,
    pub is_invincible: bool,
}

impl Otter {
    pub fn new() -> Self {
        Self {
            x: OTTER_START_X,
            y: OTTER_START_Y,
            velocity_y: OTTER_VELOCITY,
            state: OtterState::SwimmingDown,
            frame: 0,
            is_invincible: false,
        }
    }

    pub fn reset(&mut self) {
        *self = Self::new();
    }

    fn set_state(&mut self, new_state: OtterState) {
        if self.state != new_state {
            self.state = new_state;
            self.frame = 0; // Reset frame on state change to prevent index out of bounds
        }
    }

    pub fn swim_up(&mut self) {
        self.velocity_y = -OTTER_VELOCITY;
        if self.is_invincible {
            self.set_state(OtterState::Rolling);
        } else {
            self.set_state(OtterState::SwimmingUp);
        }
    }

    pub fn swim_down(&mut self) {
        self.velocity_y = OTTER_VELOCITY;
        if self.is_invincible {
            self.set_state(OtterState::Rolling);
        } else {
            self.set_state(OtterState::SwimmingDown);
        }
    }

    pub fn update(&mut self) {
        // Apply velocity
        self.y += self.velocity_y;

        // Boundary checks (from otter.cpp)
        if self.y > OTTER_WALK_Y {
            // Only switch to Walking if not invincible (invincible stays Rolling)
            if !self.is_invincible {
                self.set_state(OtterState::Walking);
            }
            self.y = OTTER_WALK_Y;
        }

        if self.y + OTTER_HEIGHT > SCREEN_HEIGHT {
            self.y = SCREEN_HEIGHT - OTTER_HEIGHT;
        }

        if self.y < OTTER_MIN_Y {
            self.y = OTTER_MIN_Y;
        }

        // Note: Invincibility is now managed by Game based on score, not timer

        // Update animation frame
        let clips = self.state.get_clips();
        self.frame = (self.frame + 1) % clips.len();
    }

    pub fn activate_invincibility(&mut self) {
        self.is_invincible = true;
        self.set_state(OtterState::Rolling);
    }

    pub fn deactivate_invincibility(&mut self) {
        self.is_invincible = false;
        self.set_state(OtterState::SwimmingDown);
    }

    pub fn get_clip(&self) -> ClipRect {
        let clips = self.state.get_clips();
        clips[self.frame % clips.len()] // Safety: modulo ensures valid index
    }

    pub fn get_collision_rect(&self) -> (i32, i32, u32, u32) {
        // State and frame-dependent collision boxes (from otter.cpp check_collision)
        match self.state {
            OtterState::SwimmingUp => {
                if self.frame == 0 {
                    // Top: y+7, Bottom: y+31, Left: x, Right: x+32
                    (self.x, self.y + 7, 32, 24)
                } else {
                    // Top: y+3, Bottom: y+31, Left: x, Right: x+32
                    (self.x, self.y + 3, 32, 28)
                }
            }
            OtterState::SwimmingDown => {
                // Top: y+1, Bottom: y+31, Left: x, Right: x+32
                (self.x, self.y + 1, 32, 30)
            }
            OtterState::Walking => {
                // Top: y+13, Bottom: y+31, Left: x, Right: x+31
                (self.x, self.y + 13, 31, 18)
            }
            OtterState::Rolling => {
                if self.frame == 0 || self.frame == 2 {
                    // Top: y+6, Bottom: y+32, Left: x, Right: x+33
                    (self.x, self.y + 6, 33, 26)
                } else {
                    // Top: y, Bottom: y+32, Left: x, Right: x+26
                    (self.x, self.y, 26, 32)
                }
            }
        }
    }
}

impl Default for Otter {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Debug)]
pub struct Missile {
    pub x: i32,
    pub y: i32,
    pub state: MissileState,
    pub frame: usize,
    pub active: bool,
}

impl Missile {
    pub fn new(y: i32) -> Self {
        Self {
            x: SCREEN_WIDTH,
            y,
            state: MissileState::Shooting,
            frame: 0,
            active: true,
        }
    }

    pub fn update(&mut self) {
        match self.state {
            MissileState::Shooting => {
                self.x -= MISSILE_VELOCITY_X;
                self.frame = (self.frame + 1) % MISSILE_CLIPS_SHOOT.len();

                if self.x < -50 {
                    self.active = false;
                }
            }
            MissileState::Exploding => {
                self.frame += 1;
                if self.frame >= MISSILE_EXPLODE_FRAMES {
                    self.active = false;
                }
            }
        }
    }

    pub fn explode(&mut self) {
        self.state = MissileState::Exploding;
        self.frame = 0;
    }

    pub fn get_clip(&self) -> ClipRect {
        match self.state {
            MissileState::Shooting => {
                MISSILE_CLIPS_SHOOT[self.frame % MISSILE_CLIPS_SHOOT.len()]
            }
            MissileState::Exploding => {
                let idx = self.frame.min(MISSILE_CLIPS_EXPLODE.len() - 1);
                MISSILE_CLIPS_EXPLODE[idx]
            }
        }
    }

    pub fn get_collision_rect(&self) -> (i32, i32, u32, u32) {
        // From otter.cpp: obstacle hitbox is 24x10
        (self.x, self.y, 24, 10)
    }
}

#[derive(Clone, Debug)]
pub struct Coin {
    pub x: i32,
    pub y: i32,
    pub velocity_x: i32,
    pub frame: usize,
    pub collected: bool,
    pub active: bool,
}

impl Coin {
    pub fn new(x: i32) -> Self {
        Self {
            x,
            y: COIN_START_Y,
            velocity_x: 0,
            frame: 0,
            collected: false,
            active: true,
        }
    }

    pub fn update(&mut self) {
        self.y += COIN_VELOCITY_Y;

        // Water drift (from coin.cpp)
        if self.y > COIN_WATER_THRESHOLD {
            self.velocity_x = COIN_VELOCITY_X_WATER;
        }
        self.x += self.velocity_x;

        // Animation
        self.frame = (self.frame + 1) % COIN_CLIPS_SPIN.len();

        // Deactivate at bottom or off-screen
        if self.y > COIN_BOTTOM_THRESHOLD || self.x < -20 {
            self.active = false;
        }
    }

    pub fn get_clip(&self) -> ClipRect {
        COIN_CLIPS_SPIN[self.frame % COIN_CLIPS_SPIN.len()]
    }

    pub fn get_collision_rect(&self) -> (i32, i32, u32, u32) {
        (self.x, self.y, 16, 16)
    }
}

#[derive(Clone, Debug)]
pub struct Fish {
    pub x: i32,
    pub y: i32,
    pub frame: usize,
    pub active: bool,
}

impl Fish {
    pub fn new(y: i32) -> Self {
        Self {
            x: SCREEN_WIDTH,
            y,
            frame: 0,
            active: true,
        }
    }

    pub fn update(&mut self) {
        self.x -= FISH_VELOCITY_X;
        self.frame = (self.frame + 1) % FISH_CLIPS_SWIM.len();

        if self.x < -40 {
            self.active = false;
        }
    }

    pub fn get_clip(&self) -> ClipRect {
        FISH_CLIPS_SWIM[self.frame % FISH_CLIPS_SWIM.len()]
    }

    pub fn get_collision_rect(&self) -> (i32, i32, u32, u32) {
        (self.x, self.y, 30, 20)
    }
}

// =============================================================================
// Collision Detection
// =============================================================================

pub fn rects_collide(a: (i32, i32, u32, u32), b: (i32, i32, u32, u32)) -> bool {
    let (ax, ay, aw, ah) = a;
    let (bx, by, bw, bh) = b;

    ax < bx + bw as i32
        && ax + aw as i32 > bx
        && ay < by + bh as i32
        && ay + ah as i32 > by
}

// =============================================================================
// Main Game Struct
// =============================================================================

#[derive(Clone)]
pub struct Game {
    pub state: GameState,
    pub otter: Otter,
    pub missiles: Vec<Missile>,
    pub coins: Vec<Coin>,
    pub fish: Vec<Fish>,
    pub score: u32,
    pub high_score: u32,
    pub obstacle_timer: f32,
    pub obstacle_spawn_rate: f32,
    /// Score threshold at which invincibility ends (None = not invincible)
    pub invincibility_check_score: Option<u32>,
    pending_sounds: Vec<SoundEffect>,
}

impl Game {
    pub fn new() -> Self {
        Self {
            state: GameState::Menu,
            otter: Otter::new(),
            missiles: Vec::new(),
            coins: Vec::new(),
            fish: Vec::new(),
            score: 0,
            high_score: 0,
            obstacle_timer: 0.0,
            obstacle_spawn_rate: 50.0,
            invincibility_check_score: None,
            pending_sounds: Vec::new(),
        }
    }

    pub fn reset(&mut self) {
        self.otter.reset();
        self.missiles.clear();
        self.coins.clear();
        self.fish.clear();
        self.score = 0;
        self.obstacle_timer = 0.0;
        self.obstacle_spawn_rate = 50.0;
        self.invincibility_check_score = None;
        self.pending_sounds.clear();
        self.state = GameState::Playing;
    }

    pub fn start(&mut self) {
        if self.state == GameState::Menu || self.state == GameState::GameOver {
            self.reset();
        }
    }

    /// Take any pending sound effects (renderer should play these)
    pub fn take_pending_sounds(&mut self) -> Vec<SoundEffect> {
        std::mem::take(&mut self.pending_sounds)
    }

    /// Get the digits of the score for rendering
    /// Returns a vec of digits from most significant to least significant
    /// e.g., 1234 -> [1, 2, 3, 4]
    pub fn get_score_digits(&self) -> Vec<u8> {
        if self.score == 0 {
            return vec![0];
        }

        let mut digits = Vec::new();
        let mut n = self.score;
        while n > 0 {
            digits.push((n % 10) as u8);
            n /= 10;
        }
        digits.reverse();
        digits
    }

    /// Get the digits of the high score for rendering
    pub fn get_high_score_digits(&self) -> Vec<u8> {
        if self.high_score == 0 {
            return vec![0];
        }

        let mut digits = Vec::new();
        let mut n = self.high_score;
        while n > 0 {
            digits.push((n % 10) as u8);
            n /= 10;
        }
        digits.reverse();
        digits
    }

    pub fn handle_space_pressed(&mut self) {
        match self.state {
            GameState::Menu | GameState::GameOver => {
                self.start();
            }
            GameState::Playing => {
                self.otter.swim_up();
            }
        }
    }

    pub fn handle_space_released(&mut self) {
        if self.state == GameState::Playing {
            self.otter.swim_down();
        }
    }

    pub fn update(&mut self) {
        if self.state != GameState::Playing {
            return;
        }

        // Add per-frame score (from original main.cpp: score += 7)
        self.score += SCORE_PER_FRAME;

        // Check if invincibility should end (score-based, from original)
        if self.otter.is_invincible {
            if let Some(check_score) = self.invincibility_check_score {
                if self.score >= check_score {
                    self.otter.deactivate_invincibility();
                    self.invincibility_check_score = None;
                }
            }
        }

        // Update otter
        self.otter.update();

        // Update missiles
        for missile in &mut self.missiles {
            missile.update();
        }
        self.missiles.retain(|m| m.active);

        // Update coins
        for coin in &mut self.coins {
            coin.update();
        }
        self.coins.retain(|c| c.active && !c.collected);

        // Update fish
        for fish in &mut self.fish {
            fish.update();
        }
        self.fish.retain(|f| f.active);

        // Spawn entities
        self.spawn_entities();

        // Check collisions
        self.check_collisions();

        // Increase difficulty based on score
        self.obstacle_spawn_rate = (50.0 - (self.score as f32 / 100.0)).max(10.0);
    }

    fn spawn_entities(&mut self) {
        let mut rng = rand::thread_rng();

        // Spawn missiles (dynamic rate)
        self.obstacle_timer += 1.0;
        if self.obstacle_timer > self.obstacle_spawn_rate {
            self.obstacle_timer = 0.0;
            let y = rng.gen_range(30..280);
            self.missiles.push(Missile::new(y));
        }

        // Spawn coins (5% chance per frame)
        if rng.gen_range(0..1000) < COIN_SPAWN_CHANCE {
            let x = rng.gen_range(50..400);
            self.coins.push(Coin::new(x));
        }

        // Spawn fish (0.3% chance - rare)
        if rng.gen_range(0..1000) < FISH_SPAWN_CHANCE {
            let y = rng.gen_range(50..250);
            self.fish.push(Fish::new(y));
        }
    }

    fn check_collisions(&mut self) {
        let otter_rect = self.otter.get_collision_rect();

        // Check missile collisions
        for missile in &mut self.missiles {
            if missile.state == MissileState::Shooting
                && rects_collide(otter_rect, missile.get_collision_rect())
            {
                if self.otter.is_invincible {
                    // Destroy missile when invincible and get bonus score
                    missile.explode();
                    self.score += MISSILE_DESTROY_SCORE;
                    self.pending_sounds.push(SoundEffect::Boom);
                } else {
                    // Game over
                    missile.explode();
                    self.pending_sounds.push(SoundEffect::Boom);
                    if self.score > self.high_score {
                        self.high_score = self.score;
                    }
                    self.state = GameState::GameOver;
                    return;
                }
            }
        }

        // Check coin collisions
        for coin in &mut self.coins {
            if !coin.collected && rects_collide(otter_rect, coin.get_collision_rect()) {
                coin.collected = true;
                self.score += COIN_SCORE;
                self.pending_sounds.push(SoundEffect::Coin);
            }
        }

        // Check fish collisions (activates score-based invincibility)
        for fish in &mut self.fish {
            if fish.active && rects_collide(otter_rect, fish.get_collision_rect()) {
                fish.active = false;
                self.otter.activate_invincibility();
                // Invincibility lasts until score increases by INVINCIBILITY_SCORE_DURATION
                self.invincibility_check_score = Some(self.score + INVINCIBILITY_SCORE_DURATION);
                self.pending_sounds.push(SoundEffect::Powerup);
            }
        }
    }
}

impl Default for Game {
    fn default() -> Self {
        Self::new()
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_otter_initialization() {
        let otter = Otter::new();
        assert_eq!(otter.x, OTTER_START_X);
        assert_eq!(otter.y, OTTER_START_Y);
        assert_eq!(otter.state, OtterState::SwimmingDown);
        assert!(!otter.is_invincible);
    }

    #[test]
    fn test_otter_swim_up() {
        let mut otter = Otter::new();
        otter.swim_up();
        assert_eq!(otter.velocity_y, -OTTER_VELOCITY);
        assert_eq!(otter.state, OtterState::SwimmingUp);
    }

    #[test]
    fn test_otter_swim_down() {
        let mut otter = Otter::new();
        otter.swim_up(); // First go up
        otter.swim_down(); // Then down
        assert_eq!(otter.velocity_y, OTTER_VELOCITY);
        assert_eq!(otter.state, OtterState::SwimmingDown);
    }

    #[test]
    fn test_otter_state_change_resets_frame() {
        let mut otter = Otter::new();
        otter.frame = 5; // Set to invalid frame
        otter.set_state(OtterState::Rolling);
        assert_eq!(otter.frame, 0); // Should reset
    }

    #[test]
    fn test_otter_frame_bounds() {
        let mut otter = Otter::new();
        // Update many times to cycle through frames
        for _ in 0..100 {
            otter.update();
            let clips = otter.state.get_clips();
            assert!(otter.frame < clips.len(), "Frame {} out of bounds for {:?}", otter.frame, otter.state);
        }
    }

    #[test]
    fn test_otter_invincibility() {
        let mut otter = Otter::new();
        otter.activate_invincibility();
        assert!(otter.is_invincible);
        assert_eq!(otter.state, OtterState::Rolling);
    }

    #[test]
    fn test_otter_invincibility_deactivate() {
        let mut otter = Otter::new();
        otter.activate_invincibility();
        assert!(otter.is_invincible);

        otter.deactivate_invincibility();
        assert!(!otter.is_invincible);
        assert_eq!(otter.state, OtterState::SwimmingDown);
    }

    #[test]
    fn test_game_invincibility_score_based() {
        let mut game = Game::new();
        game.start();
        game.score = 1000;

        // Simulate eating a fish
        game.otter.activate_invincibility();
        game.invincibility_check_score = Some(game.score + INVINCIBILITY_SCORE_DURATION);

        assert!(game.otter.is_invincible);
        assert_eq!(game.invincibility_check_score, Some(1000 + INVINCIBILITY_SCORE_DURATION));

        // Set score to well before threshold (accounting for SCORE_PER_FRAME being added each update)
        // Threshold is 3000 (1000 + 2000), so set to 2990 - after update it becomes 2997, still < 3000
        game.score = 1000 + INVINCIBILITY_SCORE_DURATION - 10;
        game.update();
        assert!(game.otter.is_invincible, "Should still be invincible before threshold");

        // Now set score so that after SCORE_PER_FRAME is added, we exceed threshold
        // Set to 2994, after update it becomes 3001, which exceeds 3000
        game.score = 1000 + INVINCIBILITY_SCORE_DURATION - SCORE_PER_FRAME + 1;
        game.update();
        assert!(!game.otter.is_invincible, "Should no longer be invincible after exceeding threshold");
    }

    #[test]
    fn test_otter_boundary_top() {
        let mut otter = Otter::new();
        otter.y = 0;
        otter.velocity_y = -OTTER_VELOCITY;
        otter.update();
        assert!(otter.y >= OTTER_MIN_Y);
    }

    #[test]
    fn test_otter_boundary_bottom() {
        let mut otter = Otter::new();
        otter.y = SCREEN_HEIGHT;
        otter.velocity_y = OTTER_VELOCITY;
        otter.update();
        assert!(otter.y + OTTER_HEIGHT <= SCREEN_HEIGHT);
    }

    #[test]
    fn test_missile_movement() {
        let mut missile = Missile::new(100);
        let initial_x = missile.x;
        missile.update();
        assert_eq!(missile.x, initial_x - MISSILE_VELOCITY_X);
    }

    #[test]
    fn test_missile_explode() {
        let mut missile = Missile::new(100);
        missile.explode();
        assert_eq!(missile.state, MissileState::Exploding);
        assert_eq!(missile.frame, 0);
    }

    #[test]
    fn test_missile_deactivates_offscreen() {
        let mut missile = Missile::new(100);
        missile.x = -60;
        missile.update();
        assert!(!missile.active);
    }

    #[test]
    fn test_coin_movement() {
        let mut coin = Coin::new(200);
        let initial_y = coin.y;
        coin.update();
        assert_eq!(coin.y, initial_y + COIN_VELOCITY_Y);
    }

    #[test]
    fn test_coin_water_drift() {
        let mut coin = Coin::new(200);
        coin.y = COIN_WATER_THRESHOLD + 1;
        coin.update();
        assert_eq!(coin.velocity_x, COIN_VELOCITY_X_WATER);
    }

    #[test]
    fn test_fish_movement() {
        let mut fish = Fish::new(100);
        let initial_x = fish.x;
        fish.update();
        assert_eq!(fish.x, initial_x - FISH_VELOCITY_X);
    }

    #[test]
    fn test_collision_detection() {
        // Overlapping rects
        assert!(rects_collide((0, 0, 10, 10), (5, 5, 10, 10)));

        // Non-overlapping rects
        assert!(!rects_collide((0, 0, 10, 10), (20, 20, 10, 10)));

        // Edge touching (not overlapping)
        assert!(!rects_collide((0, 0, 10, 10), (10, 0, 10, 10)));
    }

    #[test]
    fn test_game_initialization() {
        let game = Game::new();
        assert_eq!(game.state, GameState::Menu);
        assert_eq!(game.score, 0);
        assert!(game.missiles.is_empty());
        assert!(game.coins.is_empty());
        assert!(game.fish.is_empty());
    }

    #[test]
    fn test_game_start() {
        let mut game = Game::new();
        game.start();
        assert_eq!(game.state, GameState::Playing);
    }

    #[test]
    fn test_game_reset() {
        let mut game = Game::new();
        game.score = 1000;
        game.missiles.push(Missile::new(100));
        game.reset();
        assert_eq!(game.score, 0);
        assert!(game.missiles.is_empty());
        assert_eq!(game.state, GameState::Playing);
    }

    #[test]
    fn test_high_score_preserved() {
        let mut game = Game::new();
        game.high_score = 500;
        game.reset();
        assert_eq!(game.high_score, 500);
    }

    #[test]
    fn test_all_clip_arrays_not_empty() {
        assert!(!OTTER_CLIPS_DOWN.is_empty());
        assert!(!OTTER_CLIPS_UP.is_empty());
        assert!(!OTTER_CLIPS_WALK.is_empty());
        assert!(!OTTER_CLIPS_ROLL.is_empty());
        assert!(!MISSILE_CLIPS_SHOOT.is_empty());
        assert!(!MISSILE_CLIPS_EXPLODE.is_empty());
        assert!(!COIN_CLIPS_SPIN.is_empty());
        assert!(!FISH_CLIPS_SWIM.is_empty());
        assert!(!NUMBER_CLIPS.is_empty());
        assert_eq!(NUMBER_CLIPS.len(), 10); // 0-9 digits
    }

    #[test]
    fn test_score_digits() {
        let mut game = Game::new();

        // Test zero
        game.score = 0;
        assert_eq!(game.get_score_digits(), vec![0]);

        // Test single digit
        game.score = 5;
        assert_eq!(game.get_score_digits(), vec![5]);

        // Test multiple digits
        game.score = 1234;
        assert_eq!(game.get_score_digits(), vec![1, 2, 3, 4]);

        // Test large number
        game.score = 98765;
        assert_eq!(game.get_score_digits(), vec![9, 8, 7, 6, 5]);
    }
}
