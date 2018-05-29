use specs::prelude::{Read, Join, ReadStorage, System, WriteStorage};
use game::resources::Window;
use game::components::{Position, Velocity, AlwaysOnScreen};

#[derive(SystemData)]
pub struct Data<'a> {
    pub window: Read<'a, Window>,
    pub velocity: ReadStorage<'a, Velocity>,
    pub always_onscreen: ReadStorage<'a, AlwaysOnScreen>,
    pub position: WriteStorage<'a, Position>,
}

pub struct MovementSystem;

impl<'a> System<'a> for MovementSystem {
    type SystemData = Data<'a>;

    fn run(&mut self, mut data: Data) {
        let window = &data.window;

        (&data.velocity, &mut data.position)
            .join()
            .for_each(|(vel, pos)| {
                pos.0 += vel.0;
            });

        (&mut data.position, &data.always_onscreen)
            .join()
            .for_each(|(pos, _)| {
                match pos.0.x {
                    x if x < 0.0 => pos.0.x = window.width,
                    x if x > window.width => pos.0.x = 0.0,
                    _ => {}
                }

                match pos.0.y {
                    y if y < 0.0 => pos.0.y = window.height,
                    y if y > window.height => pos.0.y = 0.0,
                    _ => {}
                }
            });
    }
}
