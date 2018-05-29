use specs::prelude::{Entities, Read, Join, LazyUpdate, ReadStorage, System, WriteStorage};
use game::resources::{PlayerInput, Time, Window};
use game::components::{Destructable, Position, Renderable, RenderableType, Rotation, Velocity};
use player::components::Player;
use assets::components::{Asset, Circle};
use bullets::components::Bullet;

#[derive(SystemData)]
pub struct Data<'a> {
    pub window: Read<'a, Window>,
    pub dt: Read<'a, Time>,
    pub input: Read<'a, PlayerInput>,
    pub updater: Read<'a, LazyUpdate>,
    pub entities: Entities<'a>,
    pub bullet: WriteStorage<'a, Bullet>,
    pub player: WriteStorage<'a, Player>,
    pub position: ReadStorage<'a, Position>,
    pub rotation: ReadStorage<'a, Rotation>,
}

pub struct BulletsSystem;

impl<'a> System<'a> for BulletsSystem {
    type SystemData = Data<'a>;

    fn run(&mut self, mut data: Data) {
        let entity_list: Vec<_> = (&data.bullet, &*data.entities, &data.position)
            .join()
            .filter(|&(_, _, pos)| {
                // TODO: How can this be written in a nicer way..?
                pos.0.x < 0.0 || pos.0.x > data.window.width || pos.0.y < 0.0
                    || pos.0.y > data.window.height
            })
            .map(|(_, e, _)| e)
            .collect();

        for entity in entity_list.iter() {
            // Delete entities that were found off screen
            data.entities.delete(*entity).unwrap();
        }

        let dt = &data.dt;
        let player = &mut data.player;
        let updater = &data.updater;
        let entities = &data.entities;

        if data.input.attack {
            // Create a new asteroid entity
            (&mut *player, &data.position, &data.rotation)
                .join()
                .for_each(|(player, pos, rot)| {
                    if dt.duration - player.last_fired > 0.2 {
                        player.last_fired = dt.duration;

                        let rads = (rot.degrees - 90.0).to_radians();
                        let x = rads.cos();
                        let y = rads.sin();

                        updater
                            .create_entity(&entities)
                            .with(Bullet)
                            .with(Destructable::new())
                            // 20.0 here is half the height of a player. This should probably
                            // be in a const or accessible in the Player component?
                            .with(Position::new(x * 20.0 + pos.0.x, y * 20.0 + pos.0.y))
                            .with(Circle::new(2.0))
                            // TODO: Bullet speed constant
                            .with(Velocity::new(x * 5.0, y * 5.0))
                            // TODO: This mesh does not need to be created dynamically
                            .with(Renderable::new(RenderableType::Mesh(Asset::new())))
                            .build();
                    }
                });
        }
    }
}
