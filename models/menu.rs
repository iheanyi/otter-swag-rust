use sdl2::rect::Rect;
use sdl2::render::Texture;

const menu_tile_size: (u32, u32) = (480, 320);

#[derive(Copy, Clone)]
pub enum MenuState {
    StartScreen { source_rect: Rect, is_visible: bool },
    Playing { source_rect: Rect, is_visible: bool },
    GameOver { source_rect: Rect, is_visible: bool },
}

impl MenuState {
    fn new_start() -> Self {
        let source_rect_start = Rect::new(12, 32, menu_tile_size.0, menu_tile_size.1);

        return MenuState::StartScreen {
            source_rect: source_rect_start,
            is_visible: true,
        };
    }

    fn new_playing() -> Self {
        // TODO: Transform to a dynamic point counter.
        let source_rect_start = Rect::new(12, 32, menu_tile_size.0, menu_tile_size.1);
        return MenuState::Playing {
            source_rect: source_rect_start,
            is_visible: false,
        };
    }

    fn new_game_over() -> Self {
        let source_rect_start = Rect::new(12, 505, menu_tile_size.0, menu_tile_size.1);

        return MenuState::GameOver {
            source_rect: source_rect_start,
            is_visible: true,
        };
    }
}

pub struct Menu<'a> {
    pub texture: Texture<'a>,
    pub state: MenuState,
}

impl<'a> Menu<'a> {
    pub fn new(texture: &'a Texture) -> Self {
        return Menu {
            state: MenuState::new_start(),
            texture: texture,
        };
    }

    pub fn to_start_screen(&mut self) {
        self.state = match self.state {
            MenuState::GameOver { .. } => MenuState::new_start(),
            _ => panic!("Invalid state transition!"),
        }
    }

    pub fn to_playing(&mut self) {
        self.state = match self.state {
            MenuState::StartScreen { .. } => MenuState::new_playing(),
            _ => panic!("Invalid state transition!"),
        }
    }

    pub fn to_game_over(&mut self) {
        self.state = match self.state {
            MenuState::Playing { .. } => MenuState::new_game_over(),
            _ => panic!("Invalid state transition!"),
        }
    }

    pub fn get_source_rect(&self) -> Rect {
        match self.state {
            MenuState::StartScreen { source_rect, .. } => return source_rect,
            MenuState::Playing { source_rect, .. } => return source_rect,
            MenuState::GameOver { source_rect, .. } => return source_rect,
        };
    }

    pub fn is_visible(&self) -> bool {
        match self.state {
            MenuState::StartScreen { is_visible, .. } => return is_visible,
            MenuState::Playing { is_visible, .. } => return is_visible,
            MenuState::GameOver { is_visible, .. } => return is_visible,
        };
    }
}
