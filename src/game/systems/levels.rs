use rand;
use rand::Rng;
use specs::prelude::{Entities, Fetch, FetchMut, LazyUpdate, System};
use game::resources::{GameWorld, State, Window};
use game::components::{AlwaysOnScreen, Position, Renderable, RenderableType, Velocity};
use asteroid::components::Asteroid;
use assets::components::{Asset, Polygon};

pub struct LevelsSystem;

#[derive(SystemData)]
pub struct Data<'a> {
    pub window: Fetch<'a, Window>,
    pub entities: Entities<'a>,
    pub game_world: FetchMut<'a, GameWorld>,
    pub updater: Fetch<'a, LazyUpdate>,
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

                    // Create a new asteroid entity
                    data.updater
                        .create_entity(&data.entities)
                        .with(Asteroid)
                        .with(AlwaysOnScreen)
                        .with(Position::new(px, py))
                        .with(Velocity::new(vx, vy))
                        .with(Polygon::new(Asteroid::create_points(80.0)))
                        .with(Renderable::new(RenderableType::Mesh(Asset::new())))
                        .build();
                }

                data.game_world.state = State::InProgress;
            }
            _ => {}
        }
    }
}
