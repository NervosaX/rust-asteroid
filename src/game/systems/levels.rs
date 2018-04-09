use specs::prelude::{Entities, FetchMut, Join, ReadStorage, System, SystemData, WriteStorage};
use game::resources::{GameWorld, State};

pub struct LevelsSystem;

#[derive(SystemData)]
pub struct Data<'a> {
    pub entities: Entities<'a>,
    pub game_world: FetchMut<'a, GameWorld>,
}

impl<'a> System<'a> for LevelsSystem {
    type SystemData = Data<'a>;

    fn run(&mut self, mut data: Data<'a>) {
        match data.game_world.state {
            State::NewLevel => {
                // Set up asteroids for this level
                let level = data.game_world.level;
                for _ in 1..level + 2 {

                }

                data.game_world.state = State::InProgress;
            }
            _ => {}
        }

        // (e,).join().for_each(|(e, game_world)| {

        // })

        // (&velocity, &mut position).join().for_each(|(vel, pos)| {
        //     pos.0 += vel.0;
        // })
    }
}
