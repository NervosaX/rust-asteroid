use rand;
use rand::Rng;
use ggez::graphics::Point2;
use specs::prelude::{VecStorage, Component};
use assets::components::Polygon;

#[derive(Debug, Clone, Copy)]
pub enum AsteroidSize {
    Small,
    Medium,
    Large,
}

#[derive(Debug, Clone, Copy)]
pub struct Asteroid {
    size: AsteroidSize,
}

impl Asteroid {
    pub fn new(size: AsteroidSize) -> Self {
        Self { size }
    }

    pub fn to_polygon(&self) -> Polygon {
        let size = match self.size {
            AsteroidSize::Large => 90.0,
            AsteroidSize::Medium => 50.0,
            AsteroidSize::Small => 30.0,
        };

        Polygon::new(Asteroid::create_points(size))
    }

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
