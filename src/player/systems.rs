use ggez::graphics::Vector2;
use specs::prelude::{Fetch, Join, ReadStorage, System, WriteStorage};
use game::resources::{DeltaTime, PlayerInput, Window};
use game::components::{Position, Rotation, Velocity};
use player::components::Player;

#[derive(SystemData)]
pub struct Data<'a> {
    dt: Fetch<'a, DeltaTime>,
    input: Fetch<'a, PlayerInput>,
    window: Fetch<'a, Window>,
    player: ReadStorage<'a, Player>,
    velocity: WriteStorage<'a, Velocity>,
    rotation: WriteStorage<'a, Rotation>,
    position: WriteStorage<'a, Position>,
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
        let window = &data.window;

        (
            &data.player,
            &mut data.velocity,
            &mut data.rotation,
            &mut data.position,
        ).join()
            .for_each(|(_, vel, rot, pos)| {
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

                // Prevent the player from going off screen
                // TODO: move this to affect all objects
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
