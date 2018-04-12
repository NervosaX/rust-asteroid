use specs::prelude::{VecStorage, Component};

#[derive(Debug)]
pub struct Bullet;

impl Component for Bullet {
    type Storage = VecStorage<Self>;
}
