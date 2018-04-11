use ggez::Context;
use ggez::graphics;
use ggez::graphics::{Point2, WHITE};
use specs::prelude::{Entities, Join, ReadStorage, System};
use game::components::{Position, Renderable, RenderableType, Rotation};

#[derive(SystemData)]
pub struct Data<'a> {
    pub entities: Entities<'a>,
    pub position: ReadStorage<'a, Position>,
    pub rotation: ReadStorage<'a, Rotation>,
    pub renderable: ReadStorage<'a, Renderable>,
}

pub struct RenderingSystem<'c> {
    ctx: &'c mut Context,
}

impl<'c> RenderingSystem<'c> {
    pub fn new(ctx: &'c mut Context) -> RenderingSystem<'c> {
        Self { ctx }
    }
}

impl<'a, 'c> System<'a> for RenderingSystem<'c> {
    type SystemData = Data<'a>;

    fn run(&mut self, data: Data) {
        let default_rotation = Rotation::default();

        // MeshBuilder::new()
        //     .circle(
        //         DrawMode::Fill,
        //         Point2::new(0.0, 10.0),
        //         1.0,
        //         1.0
        //     )
        //     .build(self.ctx)
        //     .and_then(|poly| {
        //         graphics::draw(
        //             self.ctx,
        //             &poly,
        //             Point2::new(10.0, 10.0),
        //             0.0)
        //         })
        //         .unwrap();

        for (e, pos, r) in (
            &*data.entities,
            &data.position,
            &data.renderable,
        ).join()
        {
            let rotation: &Rotation = data.rotation.get(e).unwrap_or_else(|| &default_rotation);

            graphics::set_color(self.ctx, WHITE).unwrap();

            match r.renderable_type {
                RenderableType::Mesh(ref asset) => {
                    if let Some(ref mesh) = asset.mesh {
                        graphics::draw(
                            self.ctx,
                            mesh,
                            Point2::new(pos.0.x, pos.0.y),
                            rotation.to_radians(),
                        ).unwrap();
                    }
                }
            }
        }
    }
}
