use sdl2::rect::Rect;

const MENU_TILE_SIZE: (u32, u32) = (480, 320);

// MenuState keeps track of the state of the menu screen. There's a state for when the game first
// starts, when the user is currently playing, and when it's game over.
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum MenuState {
    StartScreen { source_rect: Rect, is_visible: bool },
    Playing { source_rect: Rect, is_visible: bool },
    GameOver { source_rect: Rect, is_visible: bool },
}

impl MenuState {
    fn new_start() -> Self {
        let source_rect_start = Rect::new(12, 32, MENU_TILE_SIZE.0, MENU_TILE_SIZE.1);

        return MenuState::StartScreen {
            source_rect: source_rect_start,
            is_visible: true,
        };
    }

    fn new_playing() -> Self {
        // TODO: Transform to a dynamic point counter.
        let source_rect_start = Rect::new(12, 32, MENU_TILE_SIZE.0, MENU_TILE_SIZE.1);
        return MenuState::Playing {
            source_rect: source_rect_start,
            is_visible: false,
        };
    }

    fn new_game_over() -> Self {
        let source_rect_start = Rect::new(12, 505, MENU_TILE_SIZE.0, MENU_TILE_SIZE.1);

        return MenuState::GameOver {
            source_rect: source_rect_start,
            is_visible: true,
        };
    }

    pub fn as_str(&self) -> &'static str {
        match *self {
            MenuState::StartScreen { .. } => "start",
            MenuState::Playing { .. } => "playing",
            MenuState::GameOver { .. } => "game_over",
        }
    }
}

// Menu is actually a lightweight StateMachine for the game's menu.
pub struct Menu {
    state: MenuState,
}

impl Menu {
    pub fn new() -> Self {
        return Menu {
            state: MenuState::new_start(),
        };
    }

    pub fn state(&self) -> MenuState {
        return self.state;
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
        return match self.state {
            MenuState::StartScreen { source_rect, .. }
            | MenuState::Playing { source_rect, .. }
            | MenuState::GameOver { source_rect, .. } => source_rect,
        };
    }

    pub fn is_visible(&self) -> bool {
        return match self.state {
            MenuState::StartScreen { is_visible, .. }
            | MenuState::Playing { is_visible, .. }
            | MenuState::GameOver { is_visible, .. } => is_visible,
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup() -> Menu {
        return Menu::new();
    }

    #[test]
    fn it_starts_in_start_screen_state() {
        let menu = setup();
        assert_eq!(menu.state, MenuState::new_start());
    }

    #[test]
    fn it_transitions_from_to_playing_from_start() {
        let mut menu = setup();
        menu.to_playing();
        assert_eq!(menu.state, MenuState::new_playing());
    }

    #[test]
    #[should_panic]
    fn it_panics_from_start_screen_to_game_over() {
        let mut menu = setup();
        menu.to_game_over();
    }

    #[test]
    fn it_can_go_through_all_states() {
        let mut menu = setup();

        assert_eq!(menu.state, MenuState::new_start());
        menu.to_playing();
        assert_eq!(menu.state, MenuState::new_playing());
        menu.to_game_over();
        assert_eq!(menu.state, MenuState::new_game_over());
        menu.to_start_screen();
        assert_eq!(menu.state, MenuState::new_start());
    }
}
