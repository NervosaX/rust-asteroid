use ggez::graphics::Vector2;
use specs::prelude::{Read, Join, ReadStorage, System, WriteStorage};
use game::resources::{Time, PlayerInput};
use game::components::{Rotation, Velocity};
use player::components::Player;

#[derive(SystemData)]
pub struct Data<'a> {
    dt: Read<'a, Time>,
    input: Read<'a, PlayerInput>,
    player: ReadStorage<'a, Player>,
    velocity: WriteStorage<'a, Velocity>,
    rotation: WriteStorage<'a, Rotation>,
}

pub struct PlayerMovementSystem;

impl<'a> System<'a> for PlayerMovementSystem {
    type SystemData = Data<'a>;

    fn run(&mut self, mut data: Data) {
        const ROTATION_SPEED: f32 = 5.0;
        const THRUST_SPEED: f32 = 8.0;
        const MAX_SPEED: f32 = 8.0;

        let input = &data.input;
        let dt = &data.dt;

        (&data.player, &mut data.velocity, &mut data.rotation)
            .join()
            .for_each(|(_, vel, rot)| {
                if input.left {
                    rot.degrees -= ROTATION_SPEED;
                }
                if input.right {
                    rot.degrees += ROTATION_SPEED;
                }
                if input.up {
                    let rads = (rot.degrees - 90.0).to_radians();

                    let mut x = rads.cos() * dt.delta as f32 * THRUST_SPEED;
                    let mut y = rads.sin() * dt.delta as f32 * THRUST_SPEED;

                    // Prevent thrusters continuing forever...
                    if (vel.0.y + x).abs() > MAX_SPEED {
                        x = 0.0;
                    }
                    if (vel.0.y + y).abs() > MAX_SPEED {
                        y = 0.0;
                    }

                    vel.0 += Vector2::new(x, y);
                }
            });
    }
}
