use specs::prelude::{Entities, Read, Join, LazyUpdate, ReadStorage, System, WriteStorage};
use game::resources::{Time, PlayerInput, Window};
use game::components::{Position, Renderable, RenderableType, Rotation, Velocity};
use player::components::Player;
use assets::components::{Asset, Circle};
use bullets::components::Bullet;

use asteroid::components::Asteroid;
use game::components::Destructable;

#[derive(SystemData)]
pub struct Data<'a> {
    pub entities: Entities<'a>,
    pub asteroid: ReadStorage<'a, Asteroid>,
    pub destructable: ReadStorage<'a, Destructable>,
}

pub struct AsteroidSystem;

impl<'a> System<'a> for AsteroidSystem {
    type SystemData = Data<'a>;

    fn run(&mut self, data: Data) {
        (&*data.entities, &data.destructable)
            .join()
            .for_each(|(e, destructable)| {
                if destructable.destroyed {
                    data.entities.delete(e).unwrap();
                }
            });
    }
}
