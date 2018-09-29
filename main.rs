extern crate sdl2;

#[macro_use]
mod models;

use models::menu::{Menu, MenuState};
use sdl2::event::Event;
use sdl2::image::{INIT_JPG, INIT_PNG};
use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;
use sdl2::render::Texture;
use sdl2::surface::Surface;
use std::path::Path;

pub struct OtterSwag<'r> {
    menu: Menu,
    menu_texture: &'r Texture<'r>,
}

impl<'r> OtterSwag<'r> {
    pub fn new(menu: Menu, menu_texture: &'r Texture) -> Self {
        return OtterSwag {
            menu: menu,
            menu_texture: menu_texture,
        };
    }

    pub fn start(&mut self) {
        self.menu.to_playing();
    }
    pub fn state(&self) -> MenuState {
        self.menu.state
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

    let background_path = Path::new("assets/background.bmp");

    // Load the background image
    let background_surface = Surface::load_bmp(background_path).unwrap();
    let background_texture = texture_creator
        .create_texture_from_surface(&background_surface)
        .unwrap();

    let menu_path = Path::new("assets/menuScreens.bmp");
    let menu_surface = Surface::load_bmp(menu_path).unwrap();
    let menu_texture = texture_creator
        .create_texture_from_surface(&menu_surface)
        .unwrap();

    let menu_tile_size = (480, 320);

    let menu = Menu::new();

    // Start Menu Screen
    let dst_rect_start = Rect::new(0, 0, menu_tile_size.0, menu_tile_size.1);
    let mut game = OtterSwag::new(menu, &menu_texture);

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
                        MenuState::StartScreen { .. } => {
                            // Start Game loop.
                            println!("We're in the start screen still.");
                            game.start();
                        }
                        MenuState::Playing { .. } => {
                            // Start Game loop.
                            println!("We're in the play screen")
                            // TODO: The otter should start moving up.
                        }
                        MenuState::GameOver { .. } => {
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
        canvas
            .copy(&background_texture, None, None)//, 0.0, None, false, false)
            .unwrap();

        // If our menu is visible, we're going to draw that onto our canvas.
        if game.menu.is_visible() {
            canvas
                .copy(
                    game.menu_texture,
                    game.menu.get_source_rect(),
                    dst_rect_start,
                )
                .unwrap();
        }
        // TODO: Draw the otter on top of here too.
        canvas.present();
    }
}
