use specs::prelude::{Join, ReadStorage, System, WriteStorage, Entities, Entity};
use specs::world::{Index, EntitiesRes};
use bullets::components::Bullet;
use game::components::{Destructable, Position};
use assets::components::{Circle, Polygon};
use player::components::Player;
use asteroid::components::Asteroid;

#[derive(SystemData)]
pub struct Data<'a> {
    pub entities: Entities<'a>,
    pub polygon: ReadStorage<'a, Polygon>,
    pub circle: ReadStorage<'a, Circle>,
    pub player: ReadStorage<'a, Player>,
    pub bullet: ReadStorage<'a, Bullet>,
    pub position: ReadStorage<'a, Position>,
    pub asteroid: ReadStorage<'a, Asteroid>,
    pub destructable: WriteStorage<'a, Destructable>,
}

pub struct CollisionSystem;

impl<'a> System<'a> for CollisionSystem {
    type SystemData = Data<'a>;

    fn run(&mut self, mut data: Data) {
        let player: Option<Polygon> = (&data.polygon, &data.position, &data.player)
            .join()
            .map(|(ref poly, ref position, _)| {
                // Add the position to the polygon
                poly.clone() + position.clone()
            })
            .nth(0);

        if let Some(player) = player {
            (
                &data.polygon,
                &mut data.destructable,
                &data.position,
                &data.asteroid,
            ).join()
                .for_each(|(p, destructable, pos, _)| {
                    // Add the position to the polygon
                    let poly = p + pos;

                    if poly.overlaps_poly(&player) {
                        panic!("AHHH");
                        destructable.destroyed = true;
                    }
                });
        }

        let mut destructables: Vec<Index> = Vec::new();

        {
            let mut bullets = (
                &*data.entities,
                &data.circle,
                &data.destructable,
                &data.position,
                &data.bullet,
            ).join();

            let mut asteroids = (
                &*data.entities,
                &data.polygon,
                &data.destructable,
                &data.position,
                &data.asteroid,
            ).join();

            for (e1, circle, _, bpos, _) in &mut bullets {
                for (e2, polygon, _, apos, _) in &mut asteroids {
                    let bullet = &circle.to_polygon(12) + &bpos;
                    let asteroid = polygon + &apos;

                    if asteroid.overlaps_poly(&bullet) {
                        // Set both entities to be destroyed
                        destructables.push(e1.id());
                        destructables.push(e2.id());
                    }
                }
            }
        }

        // Set any destructables to destroyed on collision
        for d in destructables.iter() {
            for (e, mut destructable) in (&*data.entities, &mut data.destructable,).join() {
                if e.id() == *d {
                    destructable.destroyed = true;
                }
            }
        }
    }
}
