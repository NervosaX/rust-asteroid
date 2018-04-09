use specs::prelude::{VecStorage, Component};
use ggez::graphics::{Mesh};

#[derive(Debug)]
pub struct Asset {
    pub mesh: Option<Mesh>,
}

impl Component for Asset {
    type Storage = VecStorage<Self>;
}

impl Asset {
    pub fn new(mesh: Option<Mesh>) -> Self {
        Self { mesh }
    }
}
