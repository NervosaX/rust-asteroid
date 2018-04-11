use specs::prelude::{Entities, Fetch, FetchMut, LazyUpdate, System};
use game::resources::{GameWorld, State};
use game::components::{Position, Renderable, RenderableType, Velocity};
use asteroid::components::Asteroid;
use assets::components::{Asset, Polygon};

pub struct LevelsSystem;

#[derive(SystemData)]
pub struct Data<'a> {
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

                for _ in 1..level + 2 {
                    // Create a new asteroid entity
                    data.updater
                        .create_entity(&data.entities)
                        .with(Asteroid)
                        .with(Position::new(100.0, 100.0))
                        .with(Velocity::new(0.0, 0.0))
                        .with(Polygon::new(Asteroid::create_points(30.0)))
                        .with(Renderable::new(RenderableType::Mesh(Asset::new())))
                        .build();
                }

                data.game_world.state = State::InProgress;
            }
            _ => {}
        }
    }
}
