pub struct Menu {
    state: MenuState,
    texture: Texture,
}

impl Menu {
    fn new(texture: Texture) -> Self {
        return Menu {
            state: StartState::new(),
            texture: texture,
        };
    }
}

const menu_tile_size: (i32, i32) = (480, 320);

trait MenuState {
    fn new() -> Self;
}

pub struct StartState {
    source_rect: Rect,
}

pub struct GameOverState {
    source_rect: Rect,
}

impl MenuState for StartState {
    fn new() -> Self {
        let source_rect_start = Rect::new(12, 32, menu_tile_size.0, menu_tile_size.1);

        return StartState {
            source_rect: source_rect_start,
        };
    }
}

impl MenuState for GameOverState {
    fn new() -> Self {
        let source_rect_start = Rect::new(505, 32, menu_tile_size.0, menu_tile_size.1);

        return GameOverState {
            source_rect: source_rect_start,
        };
    }
}
