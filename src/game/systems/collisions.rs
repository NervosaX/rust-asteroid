use specs::prelude::{Join, ReadStorage, System};
use game::components::Position;
use assets::components::Polygon;
use player::components::Player;
use asteroid::components::Asteroid;

#[derive(SystemData)]
pub struct Data<'a> {
    pub polygon: ReadStorage<'a, Polygon>,
    pub player: ReadStorage<'a, Player>,
    pub position: ReadStorage<'a, Position>,
    pub asteroid: ReadStorage<'a, Asteroid>,
}

pub struct CollisionSystem;

impl<'a> System<'a> for CollisionSystem {
    type SystemData = Data<'a>;

    fn run(&mut self, data: Data) {
        let player: Option<Polygon> = (&data.polygon, &data.position, &data.player)
            .join()
            .map(|(ref poly, ref position, _)| {
                // Add the position to the polygon
                poly.clone() + position.clone()
            })
            .nth(0);

        if let Some(player) = player {
            (&data.polygon, &data.position, &data.asteroid)
                .join()
                .for_each(|(p, pos, _)| {
                    // Add the position to the polygon
                    let poly = p + pos;

                    if poly.overlaps(&player) {
                        panic!("Oh no!");
                    }
                });
        }
    }
}
