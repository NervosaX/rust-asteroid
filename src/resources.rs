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
