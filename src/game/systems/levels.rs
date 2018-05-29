use rand;
use rand::Rng;
use specs::prelude::{Entities, Read, Write, LazyUpdate, System};
use game::resources::{GameWorld, State, Window};
use game::components::{AlwaysOnScreen, Position, Renderable, RenderableType, Velocity, Destructable};
use asteroid::components::{Asteroid, AsteroidSize};
use assets::components::Asset;

pub struct LevelsSystem;

#[derive(SystemData)]
pub struct Data<'a> {
    pub window: Read<'a, Window>,
    pub entities: Entities<'a>,
    pub game_world: Write<'a, GameWorld>,
    pub updater: Read<'a, LazyUpdate>,
}

impl<'a> System<'a> for LevelsSystem {
    type SystemData = Data<'a>;

    fn run(&mut self, mut data: Data<'a>) {
        match data.game_world.state {
            State::NewLevel => {
                let level = data.game_world.level;

                for _ in 0..level + 2 {
                    let rng = &mut rand::thread_rng();

                    let px = rng.gen_range(0, data.window.width as u32) as f32;
                    let py = rng.gen_range(0, data.window.height as u32) as f32;

                    let vx = rng.gen_range(-1.0, 1.0);
                    let vy = rng.gen_range(-1.0, 1.0);

                    let asteroid = Asteroid::new(AsteroidSize::Large);

                    // Create a new asteroid entity
                    data.updater
                        .create_entity(&data.entities)
                        .with(asteroid)
                        .with(Destructable::new())
                        .with(AlwaysOnScreen)
                        .with(Position::new(px, py))
                        .with(Velocity::new(vx, vy))
                        .with(asteroid.to_polygon())
                        .with(Renderable::new(RenderableType::Mesh(Asset::new())))
                        .build();
                }

                data.game_world.state = State::InProgress;
            }
            _ => {}
        }
    }
}
