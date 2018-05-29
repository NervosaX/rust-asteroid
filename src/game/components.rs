use ggez::graphics::{Vector2};
use specs::prelude::{VecStorage, World, Component};
use player::components::Player;
use asteroid::components::Asteroid;
use assets::components::{Circle, Polygon, Asset};
use bullets::components::Bullet;

#[derive(Debug, Clone)]
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

#[derive(Debug)]
pub enum RenderableType {
    Mesh(Asset),
}

#[derive(Debug)]
pub struct Renderable {
    pub renderable_type: RenderableType,
}

impl Renderable {
    pub fn new(rtype: RenderableType) -> Self {
        Self {
            renderable_type: rtype
        }
    }
}

impl Component for Renderable {
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


#[derive(Debug)]
pub struct AlwaysOnScreen;

impl Component for AlwaysOnScreen {
    type Storage = VecStorage<Self>;
}


#[derive(Debug)]
pub struct Destructable {
    pub destroyed: bool,
}

impl Destructable {
    pub fn new() -> Self {
        Self { destroyed: false }
    }
}

impl Component for Destructable {
    type Storage = VecStorage<Self>;
}

pub fn register_components(world: &mut World) {
    world.register::<Position>();
    world.register::<Rotation>();
    world.register::<Renderable>();
    world.register::<Velocity>();
    world.register::<Polygon>();
    world.register::<Player>();
    world.register::<Asteroid>();
    world.register::<Bullet>();
    world.register::<Circle>();
    world.register::<AlwaysOnScreen>();
    world.register::<Destructable>();
}
