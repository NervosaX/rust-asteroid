use specs::prelude::{Join, ReadStorage, System, WriteStorage};
use game::components::{Position, Velocity};

#[derive(SystemData)]
pub struct Data<'a> {
    pub velocity: ReadStorage<'a, Velocity>,
    pub position: WriteStorage<'a, Position>,
}

pub struct MovementSystem;

impl<'a> System<'a> for MovementSystem {
    type SystemData = Data<'a>;

    fn run(&mut self, mut data: Data) {
        (&data.velocity, &mut data.position).join().for_each(|(vel, pos)| {
            pos.0 += vel.0;
        })
    }
}
