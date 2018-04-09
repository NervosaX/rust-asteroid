#[derive(Clone, Default)]
pub struct DeltaTime {
    pub delta: f64,
}

#[derive(Debug, Clone, Default)]
pub struct PlayerInput {
    pub up: bool,
    pub left: bool,
    pub right: bool,
    pub attack: bool,
}

#[derive(Debug)]
pub struct Window {
    pub width: f32,
    pub height: f32,
}

impl Window {
    pub fn new(width: f32, height: f32) -> Self {
        Self { width, height }
    }
}



#[derive(Debug)]
pub enum State {
    NewLevel,
    InProgress,
    Dead,
}


#[derive(Debug)]
pub struct GameWorld {
    pub level: u32,
    pub state: State
}

impl GameWorld {
    pub fn new() -> Self {
        Self {
            level: 1,
            state: State::NewLevel
        }
    }
}
