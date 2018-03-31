use ggez::graphics::Vector2;
use specs::{World, VecStorage};

#[derive(Debug, Component)]
#[component(VecStorage)]
pub struct Position(pub Vector2);

impl Position {
    pub fn new(x: f32, y: f32) -> Self {
        Position(Vector2::new(x, y))
    }
}

#[derive(Debug, Component, Default)]
#[component(VecStorage)]
pub struct Rotation {
    pub degrees: f32
}

impl Rotation {
    pub fn new(degrees: f32) -> Self {
        Self { degrees }
    }

    pub fn to_radians(&self) -> f32 {
        self.degrees.to_radians()
    }

    pub fn to_degrees(&self) -> f32 {
        self.degrees
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Shapes {
    Triangle { w: f32, h: f32 },
}

#[derive(Debug, Copy, Clone)]
pub enum RenderableType {
    Shape(Shapes),
}

#[derive(Debug, Component)]
#[component(VecStorage)]
pub struct Renderable {
    pub renderable_type: RenderableType,
}

#[derive(Debug, Component)]
#[component(VecStorage)]
pub struct Controlled;

#[derive(Debug, Component)]
#[component(VecStorage)]
pub struct Velocity(pub Vector2);

impl Velocity {
    pub fn new(x: f32, y: f32) -> Self {
        Velocity(Vector2::new(x, y))
    }
}

pub fn register_components(world: &mut World) {
    world.register::<Position>();
    world.register::<Rotation>();
    world.register::<Renderable>();
    world.register::<Controlled>();
    world.register::<Velocity>();
}
