extern crate sdl2;

#[macro_use]
mod models;

use models::menu::{Menu, MenuState};
use models::otter::Otter;
use sdl2::event::Event;
use sdl2::image::{INIT_JPG, INIT_PNG};
use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;
use sdl2::render::{Texture, WindowCanvas};
use sdl2::surface::Surface;
use std::path::Path;

pub struct OtterSwag<'r> {
    menu: Menu,
    otter: Otter,
    dst_rect: Rect,
    menu_texture: &'r Texture<'r>,
    bg_texture: &'r Texture<'r>,
    canvas: WindowCanvas,
}

impl<'r> OtterSwag<'r> {
    pub fn new(
        canvas: WindowCanvas,
        menu: Menu,
        otter: Otter,
        menu_texture: &'r Texture,
        bg_texture: &'r Texture,
    ) -> Self {
        let menu_tile_size = (480, 320);

        return OtterSwag {
            canvas: canvas,
            menu: menu,
            otter: otter,
            dst_rect: Rect::new(0, 0, menu_tile_size.0, menu_tile_size.1),
            menu_texture: menu_texture,
            bg_texture: bg_texture,
        };
    }

    pub fn start(&mut self) {
        self.menu.to_playing();
    }

    // TODO: Change to a more game-specific state.
    pub fn state(&self) -> MenuState {
        self.menu.state()
    }

    fn render_background(&mut self) {
        self.canvas
            .copy(self.bg_texture, None, None)//, 0.0, None, false, false)
            .unwrap();
    }

    fn render_menu(&mut self) {
        // If our menu is visible, we're going to draw that onto our canvas.
        if self.menu.is_visible() {
            self.canvas
                .copy(
                    self.menu_texture,
                    self.menu.get_source_rect(),
                    self.dst_rect,
                )
                .unwrap();
        }
    }

    fn render_otter(&mut self) {
        // TODO: Render Otter in it's current state and form here.
    }

    pub fn update(&mut self) {
        // TODO: Add canvas drawing logic here.
        self.canvas.clear();
        self.render_background();
        self.render_menu();

        // TODO: Draw the otter on top of here as well.
        self.render_otter();
        self.canvas.present();
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

    let canvas = window.into_canvas().build().unwrap();
    let texture_creator = canvas.texture_creator();

    let background_path = Path::new("assets/background.bmp");

    // Load the background image
    let background_surface = Surface::load_bmp(background_path).unwrap();
    let bg_texture = texture_creator
        .create_texture_from_surface(&background_surface)
        .unwrap();

    let menu_path = Path::new("assets/menuScreens.bmp");
    let menu_surface = Surface::load_bmp(menu_path).unwrap();
    let menu_texture = texture_creator
        .create_texture_from_surface(&menu_surface)
        .unwrap();

    let menu = Menu::new();
    let otter = Otter::new();

    // Start Menu Screen
    let mut game = OtterSwag::new(canvas, menu, otter, &menu_texture, &bg_texture);

    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut frame: u32 = 0;

    // TODO: Figure out a way to encapsulate this logic inside of the Otter Swag struct
    // implementation.
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
        game.update()
    }
}
