use rand;
use rand::Rng;
use ggez::graphics::Point2;
use specs::prelude::{VecStorage, Component};

#[derive(Debug, Clone)]
pub struct Asteroid;

impl Asteroid {
    pub fn create_points(radius: f32) -> Vec<Point2> {
        // Ported from https://www.openprocessing.org/sketch/71739
        let x_offset = rand::thread_rng().gen_range(0.9, 2.0) as f32;
        let y_offset = rand::thread_rng().gen_range(0.9, 2.0) as f32;

        let x = (0.0 as f32).sin() * (radius / x_offset);
        let y = (0.0 as f32).cos() * (radius / y_offset);

        let mut points = Vec::new();
        points.push(Point2::new(x, y));

        for i in 1..6 {
            let x_offset = rand::thread_rng().gen_range(0.9, 2.0) as f32;
            let y_offset = rand::thread_rng().gen_range(0.9, 2.0) as f32;

            let x = (i as f32).sin() * (radius / x_offset);
            let y = (i as f32).cos() * (radius / y_offset);

            points.push(Point2::new(x, y));
        }

        points.push(Point2::new(x, y));
        points
    }
}

impl Component for Asteroid {
    type Storage = VecStorage<Self>;
}
