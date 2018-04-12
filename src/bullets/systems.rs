use ggez::graphics::Vector2;
use specs::prelude::{Entities, Fetch, Join, LazyUpdate, ReadStorage, System, WriteStorage};
use game::resources::{DeltaTime, PlayerInput, Window};
use game::components::{Position, Renderable, RenderableType, Rotation, Velocity};
use player::components::Player;
use assets::components::{Asset, Circle};
use bullets::components::Bullet;

#[derive(SystemData)]
pub struct Data<'a> {
    pub window: Fetch<'a, Window>,
    pub dt: Fetch<'a, DeltaTime>,
    pub input: Fetch<'a, PlayerInput>,
    pub updater: Fetch<'a, LazyUpdate>,
    pub entities: Entities<'a>,
    pub bullet: WriteStorage<'a, Bullet>,
    pub player: ReadStorage<'a, Player>,
    pub position: ReadStorage<'a, Position>,
    pub rotation: ReadStorage<'a, Rotation>,
}

pub struct BulletsSystem;

impl<'a> System<'a> for BulletsSystem {
    type SystemData = Data<'a>;

    fn run(&mut self, data: Data) {
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
            data.entities.delete(*entity);
        }

        if data.input.attack {
            // Create a new asteroid entity
            (&data.player, &data.position, &data.rotation)
                .join()
                .for_each(|(_, pos, rot)| {
                    let rads = (rot.degrees - 90.0).to_radians();
                    let x = rads.cos();
                    let y = rads.sin();

                    data.updater
                        .create_entity(&data.entities)
                        .with(Bullet)
                        // 20.0 here is half the height of a player. This should probably
                        // be in a const or accessible in the Player component?
                        .with(Position::new(x * 20.0 + pos.0.x, y * 20.0 + pos.0.y))
                        .with(Circle::new(2.0))
                        // TODO: Bullet speed constant
                        .with(Velocity::new(x * 5.0, y * 5.0))
                        // TODO: This mesh does not need to be created dynamically
                        .with(Renderable::new(RenderableType::Mesh(Asset::new())))
                        .build();
                });
        }
    }
}
