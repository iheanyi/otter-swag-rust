extern crate sdl2;

use sdl2::event::Event;
use sdl2::image::{INIT_JPG, INIT_PNG};
use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;
use sdl2::surface::Surface;
use std::path::Path;

mod otter_swag {
    #[derive(Copy, Clone)]
    pub enum State {
        Playing,
        GameOver,
        StartScreen,
    }

    #[derive(Copy, Clone)]
    pub enum OtterState {
        Normal,
        Super,
        Dead,
    }

    pub struct OtterSwag {
        state: State,
    }

    impl OtterSwag {
        pub fn new() -> OtterSwag {
            return OtterSwag {
                state: State::StartScreen,
            };
        }

        pub fn start(&mut self) {
            self.state = State::Playing
        }

        pub fn state(&self) -> State {
            self.state
        }
    }
}

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let _image_context = sdl2::image::init(INIT_PNG | INIT_JPG).unwrap();

    let window = video_subsystem
        .window("Otter Swag", 480, 320)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    let texture_creator = canvas.texture_creator();

    let background_path = Path::new("background.bmp");
    // Load the background image
    let background_surface = Surface::load_bmp(background_path).unwrap();
    let background_texture = texture_creator
        .create_texture_from_surface(&background_surface)
        .unwrap();

    let menu_path = Path::new("menuScreens.bmp");
    let menu_surface = Surface::load_bmp(menu_path).unwrap();
    let menu_texture = texture_creator
        .create_texture_from_surface(&menu_surface)
        .unwrap();

    let menu_tile_size = (480, 320);

    // Start Menu Screen
    let mut source_rect_start = Rect::new(0, 0, menu_tile_size.0, menu_tile_size.1);
    let dst_rect_start = Rect::new(0, 0, menu_tile_size.0, menu_tile_size.1);

    canvas.clear();
    canvas
        .copy_ex(&background_texture, None, None, 0.0, None, false, false)
        .unwrap();

    canvas
        .copy_ex(
            &menu_texture,
            source_rect_start,
            dst_rect_start,
            0.0,
            None,
            false,
            false,
        )
        .unwrap();
    // In the starting state, we want to copy the starting menu screen as well
    // onto the menu as well.
    canvas.present();

    let mut game = otter_swag::OtterSwag::new();

    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut frame: u32 = 0;

    'running: loop {
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
                    match game.state() {
                        otter_swag::State::StartScreen => {
                            // Start Game loop.
                            println!("We're in the start screen still.");
                            game.start();
                        }
                        otter_swag::State::Playing => {
                            // Start Game loop.
                            println!("We're in the play screen")
                            // TODO: The otter should start moving up.
                        }
                        otter_swag::State::GameOver => {
                            println!("We're in the end game state")
                            // TODO: Reset the game world and start the game over again.
                        }
                    }
                    // TODO: Update state of the otter here.
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Space),
                    repeat: true,
                    ..
                } => {
                    // TODO: Update state of the otter here to be swimming.
                }
                _ => {}
            }
        }

        // Update game loop
        if frame >= 60 {
            // TODO: Update game state here.
            frame = 0;
        }
        // canvas.copy the menu screen here if the game is paused.

        canvas.clear();
        match game.state() {
            otter_swag::State::StartScreen => {
                source_rect_start.set_x(12);
                source_rect_start.set_y(32);

                canvas
                    .copy(&background_texture, None, None)//, 0.0, None, false, false)
                    .unwrap();

                canvas
                    .copy(&menu_texture, source_rect_start, dst_rect_start)
                    .unwrap();
            }
            otter_swag::State::Playing => {
                canvas
                    .copy(&background_texture, None, None)//, 0.0, None, false, false)
                    .unwrap();

                // TODO: Render the freaking otter here through copying it.
            }
            otter_swag::State::GameOver => {
                source_rect_start.set_x(505);
                source_rect_start.set_y(32);

                canvas
                    .copy(&background_texture, None, None)//, 0.0, None, false, false)
                    .unwrap();

                canvas
                    .copy(&menu_texture, source_rect_start, dst_rect_start)
                    .unwrap();
            }
        }
        // TODO: Draw the otter on top of here too.
        canvas.present();
        // The rest of the game loop goes here...
        if let otter_swag::State::Playing = game.state() {
            frame += 1;
        }
    }
}
