use specs::{Fetch, Join, WriteStorage, ReadStorage, System};
use resources::PlayerInput;
use components::{Controlled, Rotation};

pub struct PlayerMovementSystem;

impl<'a> System<'a> for PlayerMovementSystem {
    type SystemData = (
        Fetch<'a, PlayerInput>,
        ReadStorage<'a, Controlled>,
        WriteStorage<'a, Rotation>,
    );

    fn run(&mut self, (input, controlled, mut rotation): Self::SystemData) {
        (&controlled, &mut rotation).join().for_each(
            |(_, rot)| {
                if input.left {
                    rot.degrees -= 1.0;
                }

                if input.right {
                    rot.degrees += 1.0;
                }
            },
        );
    }
}
