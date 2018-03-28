use specs::{Fetch, Join, ReadStorage, System, WriteStorage};
use resources::{PlayerInput, Window};
use components::{Controlled, Position, Rotation};

pub struct PlayerMovementSystem;

impl<'a> System<'a> for PlayerMovementSystem {
    type SystemData = (
        Fetch<'a, PlayerInput>,
        Fetch<'a, Window>,
        ReadStorage<'a, Controlled>,
        WriteStorage<'a, Rotation>,
        WriteStorage<'a, Position>,
    );

    fn run(&mut self, (input, window, controlled, mut rotation, mut position): Self::SystemData) {
        const SPEED: f32 = 5.0;

        (&controlled, &mut rotation, &mut position)
            .join()
            .for_each(|(_, rot, pos)| {
                if input.left {
                    rot.degrees -= SPEED;
                }

                if input.right {
                    rot.degrees += SPEED;
                }

                let rads = (rot.degrees - 90.0).to_radians();
                pos.x += SPEED * rads.cos();
                pos.y += SPEED * rads.sin();

                // Prevent the player from going off screen
                match pos {
                    &mut Position { x, .. } if x < 0.0 => pos.x = window.width,
                    &mut Position { x, .. } if x > window.width => pos.x = 0.0,
                    &mut Position { y, .. } if y < 0.0 => pos.y = window.height,
                    &mut Position { y, .. } if y > window.height => pos.y = 0.0,
                    _ => {}
                }
            });
    }
}
