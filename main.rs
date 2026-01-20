extern crate otter_swag;
extern crate sdl2;

use otter_swag::*;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::mixer::{self, Channel, Chunk, AUDIO_S16LSB, DEFAULT_CHANNELS};
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{Canvas, Texture, TextureCreator};
use sdl2::video::{Window, WindowContext};
use std::collections::HashMap;
use std::path::Path;
use std::time::{Duration, Instant};

const FRAME_DELAY_MS: u64 = 1000 / FRAMES_PER_SECOND as u64;

/// Convert our ClipRect to SDL2 Rect
fn to_sdl_rect(clip: ClipRect) -> Rect {
    Rect::new(clip.x, clip.y, clip.w, clip.h)
}

/// Draw a score at the given position
/// Returns the width of the rendered score
fn draw_score(
    canvas: &mut Canvas<Window>,
    texture: &Texture,
    digits: &[u8],
    x: i32,
    y: i32,
) -> Result<i32, String> {
    let mut current_x = x;
    for &digit in digits {
        let clip = NUMBER_CLIPS[digit as usize];
        let dest = Rect::new(current_x, y, clip.w, clip.h);
        canvas.copy(texture, to_sdl_rect(clip), dest)?;
        current_x += DIGIT_SPACING;
    }
    Ok(current_x - x)
}

/// Load a BMP texture with magenta transparency
fn load_texture<'a>(
    texture_creator: &'a TextureCreator<WindowContext>,
    path: &str,
) -> Result<Texture<'a>, String> {
    let surface = sdl2::surface::Surface::load_bmp(Path::new(path))
        .map_err(|e| format!("Failed to load {}: {}", path, e))?;

    // Set color key for transparency (magenta: 255, 0, 255)
    let mut surface = surface;
    surface
        .set_color_key(true, Color::RGB(255, 0, 255))
        .map_err(|e| format!("Failed to set color key: {}", e))?;

    texture_creator
        .create_texture_from_surface(&surface)
        .map_err(|e| format!("Failed to create texture: {}", e))
}

/// Sound manager for playing game audio
struct SoundManager {
    sounds: HashMap<SoundEffect, Chunk>,
}

impl SoundManager {
    fn new() -> Result<Self, String> {
        let mut sounds = HashMap::new();

        // Try to load each sound, but don't fail if they're missing
        if let Ok(chunk) = Chunk::from_file("assets/sounds/coin.wav") {
            sounds.insert(SoundEffect::Coin, chunk);
        }
        if let Ok(chunk) = Chunk::from_file("assets/sounds/powerup.wav") {
            sounds.insert(SoundEffect::Powerup, chunk);
        }
        if let Ok(chunk) = Chunk::from_file("assets/sounds/boom.wav") {
            sounds.insert(SoundEffect::Boom, chunk);
        }

        Ok(Self { sounds })
    }

    fn play(&self, effect: SoundEffect) {
        if let Some(chunk) = self.sounds.get(&effect) {
            // Play on any available channel
            let _ = Channel::all().play(chunk, 0);
        }
    }
}

fn main() -> Result<(), String> {
    // Initialize SDL2
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    // Initialize audio
    let _audio = sdl_context.audio()?;
    mixer::open_audio(44100, AUDIO_S16LSB, DEFAULT_CHANNELS, 1024)?;
    mixer::allocate_channels(8);

    // Create window
    let window = video_subsystem
        .window("Otter Swag", SCREEN_WIDTH as u32, SCREEN_HEIGHT as u32)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
    let texture_creator = canvas.texture_creator();

    // Load textures
    let background_texture = load_texture(&texture_creator, "assets/background.bmp")?;
    let menu_texture = load_texture(&texture_creator, "assets/menuScreens.bmp")?;
    let otter_texture = load_texture(&texture_creator, "assets/otter.bmp")?;
    let missile_texture = load_texture(&texture_creator, "assets/missiles.bmp")?;
    let coin_texture = load_texture(&texture_creator, "assets/coins.bmp")?;
    let fish_texture = load_texture(&texture_creator, "assets/LoveFish.bmp")?;
    let numbers_texture = load_texture(&texture_creator, "assets/numbers.bmp")?;

    // Load sounds
    let sound_manager = SoundManager::new()?;

    // Load and play background music (keep _music alive for entire game loop)
    let _music = if Path::new("assets/sounds/swag.wav").exists() {
        match mixer::Music::from_file("assets/sounds/swag.wav") {
            Ok(music) => {
                let _ = music.play(-1); // Loop forever
                Some(music)
            }
            Err(e) => {
                eprintln!("Failed to load music: {}", e);
                None
            }
        }
    } else {
        None
    };

    // Initialize game
    let mut game = Game::new();
    let mut event_pump = sdl_context.event_pump()?;
    let mut space_held = false;

    'running: loop {
        let frame_start = Instant::now();

        // Handle events
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,

                Event::KeyDown {
                    keycode: Some(Keycode::Space),
                    repeat: false,
                    ..
                } => {
                    space_held = true;
                    game.handle_space_pressed();
                }

                Event::KeyUp {
                    keycode: Some(Keycode::Space),
                    ..
                } => {
                    space_held = false;
                    game.handle_space_released();
                }

                _ => {}
            }
        }

        // Keep swimming up while space is held
        if space_held && game.state == GameState::Playing {
            game.otter.swim_up();
        }

        // Update game
        game.update();

        // Play any pending sounds
        for sound in game.take_pending_sounds() {
            sound_manager.play(sound);
        }

        // Render
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        // Draw background
        canvas.copy(&background_texture, None, None)?;

        match game.state {
            GameState::Menu => {
                // Draw start menu (from menuScreens.bmp)
                let src = Rect::new(12, 32, SCREEN_WIDTH as u32, SCREEN_HEIGHT as u32);
                canvas.copy(&menu_texture, src, None)?;
            }

            GameState::Playing => {
                // Draw coins
                for coin in &game.coins {
                    let clip = coin.get_clip();
                    let dest = Rect::new(coin.x, coin.y, clip.w, clip.h);
                    canvas.copy(&coin_texture, to_sdl_rect(clip), dest)?;
                }

                // Draw fish
                for fish in &game.fish {
                    let clip = fish.get_clip();
                    let dest = Rect::new(fish.x, fish.y, clip.w, clip.h);
                    canvas.copy(&fish_texture, to_sdl_rect(clip), dest)?;
                }

                // Draw missiles
                for missile in &game.missiles {
                    let clip = missile.get_clip();
                    let dest = Rect::new(missile.x, missile.y, clip.w, clip.h);
                    canvas.copy(&missile_texture, to_sdl_rect(clip), dest)?;
                }

                // Draw otter
                // Use source clip dimensions but always render to 32x32 area
                // (matches original SDL1.2 behavior - no scaling)
                let otter_clip = game.otter.get_clip();
                let otter_dest = Rect::new(
                    game.otter.x,
                    game.otter.y,
                    otter_clip.w,
                    otter_clip.h,
                );
                // Don't set blend mode - just copy with color key transparency
                canvas.copy(&otter_texture, to_sdl_rect(otter_clip), otter_dest)?;

                // Draw score in top-right corner
                let score_digits = game.get_score_digits();
                let score_width = (score_digits.len() as i32) * DIGIT_SPACING;
                let score_x = SCREEN_WIDTH - score_width - 10;
                draw_score(&mut canvas, &numbers_texture, &score_digits, score_x, 5)?;
            }

            GameState::GameOver => {
                // Draw game over screen (x=505, y=32 in menuScreens.bmp)
                let src = Rect::new(505, 32, SCREEN_WIDTH as u32, SCREEN_HEIGHT as u32);
                canvas.copy(&menu_texture, src, None)?;

                // Draw final score centered on screen
                let score_digits = game.get_score_digits();
                let score_width = (score_digits.len() as i32) * DIGIT_SPACING;
                let score_x = (SCREEN_WIDTH - score_width) / 2;
                draw_score(&mut canvas, &numbers_texture, &score_digits, score_x, 135)?;

                // Draw high score below
                let high_score_digits = game.get_high_score_digits();
                let high_score_width = (high_score_digits.len() as i32) * DIGIT_SPACING;
                let high_score_x = (SCREEN_WIDTH - high_score_width) / 2;
                draw_score(&mut canvas, &numbers_texture, &high_score_digits, high_score_x, 175)?;
            }
        }

        canvas.present();

        // Frame rate limiting (10 FPS like original)
        let frame_time = frame_start.elapsed();
        if frame_time < Duration::from_millis(FRAME_DELAY_MS) {
            std::thread::sleep(Duration::from_millis(FRAME_DELAY_MS) - frame_time);
        }
    }

    Ok(())
}
