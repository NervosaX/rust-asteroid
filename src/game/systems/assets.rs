use ggez::Context;
use specs::prelude::{Join, ReadStorage, System, WriteStorage};
use ggez::graphics::{DrawMode, MeshBuilder};
use game::components::{Renderable, RenderableType, Shapes};
use assets::components::Asset;

#[derive(SystemData)]
pub struct Data<'a> {
    pub renderable: ReadStorage<'a, Renderable>,
    pub asset: WriteStorage<'a, Asset>,
}

pub struct AssetSystem<'c> {
    ctx: &'c mut Context,
}

impl<'c> AssetSystem<'c> {
    pub fn new(ctx: &'c mut Context) -> AssetSystem<'c> {
        Self { ctx }
    }
}

impl<'a, 'c> System<'a> for AssetSystem<'c> {
    type SystemData = Data<'a>;

    fn run(&mut self, mut data: Data) {
        (&data.renderable, &mut data.asset).join().for_each(|(ren, a)| {

            if a.mesh.is_none() {
                match ren.renderable_type {
                    RenderableType::Shape(ref shape) => {
                        match *shape {
                            Shapes::Player(ref player) => {
                                let mesh = MeshBuilder::new()
                                    .polyline(DrawMode::Line(1.0), &player.points)
                                    .build(self.ctx).unwrap();

                                a.mesh = Some(mesh);
                            }
                            Shapes::Asteroid(ref asteroid) => {
                                let mesh = MeshBuilder::new()
                                    .polyline(DrawMode::Line(1.0), &asteroid.points)
                                    .build(self.ctx).unwrap();

                                a.mesh = Some(mesh);
                            }
                        };
                    }
                }
            }
        })
    }
}
