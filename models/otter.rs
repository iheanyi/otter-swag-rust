use sdl2::rect::Rect;

pub enum OtterState {
    Walking { source_rect: Rect },
    Falling { source_rect: Rect },
    Swimming { source_rect: Rect },
    Super { source_rect: Rect },
    Dead { source_rect: Rect },
}

impl OtterState {
    pub fn new_walking() -> Self {
        let source_rect = Rect::new(312, 0, 32, 32);

        return OtterState::Walking {
            source_rect: source_rect,
        };
    }
}

pub struct Otter {
    state: OtterState,
}

impl Otter {
    pub fn new() -> Self {
        return Otter {
            state: OtterState::new_walking(),
        };
    }

    pub fn get_source_rect(&self) -> Rect {
        return match self.state {
            OtterState::Walking { source_rect }
            | OtterState::Falling { source_rect }
            | OtterState::Swimming { source_rect }
            | OtterState::Super { source_rect }
            | OtterState::Dead { source_rect } => source_rect,
        };
    }
    pub fn is_dead(&self) -> bool {
        return match self.state {
            OtterState::Dead { .. } => true,
            _ => false,
        };
    }

    pub fn is_super(&self) -> bool {
        return match self.state {
            OtterState::Super { .. } => true,
            _ => false,
        };
    }
}
