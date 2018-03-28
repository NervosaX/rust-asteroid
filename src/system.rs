use ggez::Context;
use ggez::graphics;
use ggez::graphics::{DrawMode, MeshBuilder, Point2, WHITE};
use specs::{Entities, Join, ReadStorage, System};
use components::{Position, Renderable, RenderableType, Rotation, Shapes};

pub struct RenderingSystem<'c> {
    ctx: &'c mut Context,
}

impl<'c> RenderingSystem<'c> {
    pub fn new(ctx: &'c mut Context) -> RenderingSystem<'c> {
        Self { ctx }
    }
}

impl<'a, 'c> System<'a> for RenderingSystem<'c> {
    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, Rotation>,
        ReadStorage<'a, Renderable>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (entities, position, rotation, renderable) = data;

        let default_rotation = Rotation::default();

        for (e, pos, r) in (&*entities, &position, &renderable).join() {
            let rotation: &Rotation =
                rotation.get(e).unwrap_or_else(|| &default_rotation);

            graphics::set_color(self.ctx, WHITE).unwrap();

            match r.renderable_type {
                RenderableType::Shape(shape) => {
                    match shape {
                        Shapes::Triangle { w, h } => {
                            let bottom_left = Point2::new(0.0 - w / 2.0, 0.0 + h / 2.0);
                            let top = Point2::new(0.0, 0.0 - h / 2.0);
                            let bottom_right = Point2::new(0.0 + w / 2.0, 0.0 + h / 2.0);

                            MeshBuilder::new()
                                .polyline(
                                    DrawMode::Line(1.0),
                                    &[bottom_left, top, bottom_right, bottom_left],
                                )
                                .build(self.ctx)
                                .and_then(|poly| {
                                    graphics::draw(
                                        self.ctx,
                                        &poly,
                                        Point2::new(pos.x + w / 2.0, pos.y - h / 2.0),
                                        rotation.to_radians())
                                })
                                // TODO: I don't know the best way to deal with these errors...
                                .expect("Failed to draw triangle poly");
                        }
                    };
                }
            }
        }
    }
}
