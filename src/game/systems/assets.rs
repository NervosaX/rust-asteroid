 use ggez::Context;
use specs::prelude::{Join, ReadStorage, System, WriteStorage};
use ggez::graphics::{Point2, DrawMode, MeshBuilder};
use game::components::{Renderable, RenderableType};
use assets::components::{Circle, Polygon};

#[derive(SystemData)]
pub struct Data<'a> {
    pub polygon: ReadStorage<'a, Polygon>,
    pub circle: ReadStorage<'a, Circle>,
    pub renderable: WriteStorage<'a, Renderable>
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
        (&data.polygon, &mut data.renderable).join().for_each(|(polygon, ren)| {
            match ren.renderable_type {
                RenderableType::Mesh(ref mut asset) => {
                    let mesh = MeshBuilder::new()
                        .polyline(DrawMode::Line(1.0), &polygon.points)
                        .build(self.ctx).unwrap();

                    asset.mesh = Some(mesh);
                }
            }
        });

        (&data.circle, &mut data.renderable).join().for_each(|(circle, ren)| {
            match ren.renderable_type {
                RenderableType::Mesh(ref mut asset) => {
                    let mesh = MeshBuilder::new()
                        .circle(
                            DrawMode::Fill,
                            Point2::new(0.0, 0.0),
                            circle.radius,
                            1.0
                        )
                        .build(self.ctx).unwrap();

                    asset.mesh = Some(mesh);
                }
            }
        });
    }
}
