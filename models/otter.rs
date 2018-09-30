pub enum OtterState {
    Walking,
    Falling,
    Swimming,
    Super,
    Dead,
}

pub struct Otter {
    state: OtterState,
}

impl Otter {
    pub fn new() -> Self {
        return Otter {
            state: OtterState::Walking,
        };
    }

    pub fn is_dead(&self) -> bool {
        match self.state {
            OtterState::Dead => return true,
            _ => return false,
        }
    }

    pub fn is_super(&self) -> bool {
        match self.state {
            OtterState::Super => return true,
            _ => return false,
        }
    }
}
