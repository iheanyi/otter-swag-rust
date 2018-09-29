use sdl2::rect::Rect;
use sdl2::render::Texture;

const menu_tile_size: (u32, u32) = (480, 320);

enum MenuState {
    StartScreen { source_rect: Rect },
    Hidden { source_rect: Rect },
    GameOver { source_rect: Rect },
}

impl MenuState {
    fn new_start() -> Self {
        let source_rect_start = Rect::new(12, 32, menu_tile_size.0, menu_tile_size.1);

        return MenuState::StartScreen {
            source_rect: source_rect_start,
        };
    }

    fn new_hidden() -> Self {
        let source_rect_start = Rect::new(12, 32, menu_tile_size.0, menu_tile_size.1);
        return MenuState::Hidden {
            source_rect: source_rect_start,
        };
    }

    fn new_game_over() -> Self {
        let source_rect_start = Rect::new(12, 505, menu_tile_size.0, menu_tile_size.1);

        return MenuState::GameOver {
            source_rect: source_rect_start,
        };
    }
}

pub struct Menu<'a> {
    texture: Texture<'a>,
    state: MenuState,
}

impl<'a> Menu<'a> {
    fn new(texture: &'a Texture) -> Self {
        return Menu {
            state: MenuState::new_start(),
            texture: texture,
        };
    }

    fn to_start_screen(&mut self) {
        self.state = match self.state {
            MenuState::GameOver { .. } => MenuState::new_start(),
            _ => panic!("Invalid state transition!"),
        }
    }

    fn to_hidden(&mut self) {
        self.state = match self.state {
            MenuState::StartScreen { .. } => MenuState::new_hidden(),
            _ => panic!("Invalid state transition!"),
        }
    }

    fn to_game_over(&mut self) {
        self.state = match self.state {
            MenuState::Hidden { .. } => MenuState::new_game_over(),
            _ => panic!("Invalid state transition!"),
        }
    }
}
