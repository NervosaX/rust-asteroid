use ggez::graphics::{Point2, Vector2};
use specs::prelude::{VecStorage, World, Component};
use player::components::Player;
use asteroid::components::Asteroid;
use assets::components::Asset;

#[derive(Debug)]
pub struct Position(pub Vector2);

impl Component for Position {
    type Storage = VecStorage<Self>;
}

impl Position {
    pub fn new(x: f32, y: f32) -> Self {
        Position(Vector2::new(x, y))
    }
}

#[derive(Debug, Default)]
pub struct Rotation {
    pub degrees: f32,
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

impl Component for Rotation {
    type Storage = VecStorage<Self>;
}

pub trait Shape {
    fn get_points(&self) -> &Vec<Point2>;

    fn overlaps(&self, _shape: &Shape) -> bool {
        true
    }
}

#[derive(Debug, Clone)]
pub enum Shapes {
    Player(Player),
    Asteroid(Asteroid),
}

#[derive(Debug, Clone)]
pub enum RenderableType {
    Shape(Shapes),
}

#[derive(Debug)]
pub struct Renderable {
    pub renderable_type: RenderableType,
}

impl Component for Renderable {
    type Storage = VecStorage<Self>;
}

#[derive(Debug)]
pub struct Controlled;

impl Component for Controlled {
    type Storage = VecStorage<Self>;
}

#[derive(Debug)]
pub struct Velocity(pub Vector2);

impl Velocity {
    pub fn new(x: f32, y: f32) -> Self {
        Velocity(Vector2::new(x, y))
    }
}

impl Component for Velocity {
    type Storage = VecStorage<Self>;
}

pub fn register_components(world: &mut World) {
    world.register::<Position>();
    world.register::<Rotation>();
    world.register::<Renderable>();
    world.register::<Controlled>();
    world.register::<Velocity>();
    world.register::<Asset>();
}
