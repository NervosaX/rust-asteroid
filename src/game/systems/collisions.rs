use specs::prelude::{Join, ReadStorage, System};
use bullets::components::Bullet;
use game::components::Position;
use assets::components::{Circle, Polygon};
use player::components::Player;
use asteroid::components::Asteroid;

#[derive(SystemData)]
pub struct Data<'a> {
    pub polygon: ReadStorage<'a, Polygon>,
    pub circle: ReadStorage<'a, Circle>,
    pub player: ReadStorage<'a, Player>,
    pub bullet: ReadStorage<'a, Bullet>,
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

                    if poly.overlaps_poly(&player) {
                        panic!("Oh no!");
                    }
                });
        }

        let bullets: Vec<_> = (&data.circle, &data.position, &data.bullet)
            .join()
            .collect();
        let asteroids: Vec<_> = (&data.polygon, &data.position, &data.asteroid)
            .join()
            .collect();

        for &(circle, bpos, _) in bullets.iter() {
            for &(polygon, apos, _) in asteroids.iter() {
                let bullet = &circle.to_polygon(10) + &bpos;
                let player = polygon + &apos;

                if player.overlaps_poly(&bullet) {
                    panic!("Kaboom!");
                }
            }
        }
    }
}
