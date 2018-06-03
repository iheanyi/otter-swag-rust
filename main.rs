extern crate sdl2;

use std::path::Path;
use sdl2::event::Event;
use sdl2::image::{LoadTexture, INIT_PNG, INIT_JPG};
use sdl2::keyboard::Keycode;

mod otter_swag {
    #[derive(Copy, Clone)]
    pub enum State {
        Paused,
        Playing,
        Stopped,
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
            }
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

    let window = video_subsystem.window("Otter Swag", 480, 320)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    let backgorund_path = Path::new("background.bmp");
    let mut canvas = window.into_canvas().build().unwrap();
    let texture_creator = canvas.texture_creator();
    // Load the background image
    let background_texture = texture_creator.load_texture(backgorund_path).unwrap();
    canvas.copy(&background_texture, None, None).expect("Render failed");
    canvas.present();

    let mut game = otter_swag::OtterSwag::new();

    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut frame : u32 = 0;
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                Event::KeyDown { keycode: Some(Keycode::Space), repeat: false, .. } => {
                    match game.state() {
                        otter_swag::State::StartScreen => {
                            // Start Game loop.
                            game.start();
                        },
                        _ => {}
                    }
                    // TODO: Update state of the otter here.
                },
                Event::KeyDown { keycode: Some(Keycode::Space), repeat: true, .. } => {
                    // TODO: Update state of the otter here.
                },
                _ => {}
            }
        }

        // Update game loop
        if frame >= 30 {
            // TODO: Update game state here.
            frame = 0;
        }
        // canvas.copy the menu screen here if the game is paused.
        canvas.copy(&background_texture, None, None).expect("Render failed");
        // TODO: Draw the otter on top of here too.
        canvas.present();
        // The rest of the game loop goes here...
        if let otter_swag::State::Playing = game.state() {
            frame += 1;
        }
    }
}
