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
