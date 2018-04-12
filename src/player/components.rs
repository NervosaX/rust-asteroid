use ggez::graphics::{Point2};
use specs::prelude::{VecStorage, Component};

#[derive(Debug)]
pub struct Player;

impl Player {
    pub fn create_points(width: f32, height: f32) -> Vec<Point2> {
        let mut points = Vec::new();

        let bottom_left = Point2::new(0.0 - width / 2.0, 0.0 + height / 2.0);
        let top = Point2::new(0.0, 0.0 - height / 2.0);
        let bottom_right = Point2::new(0.0 + width / 2.0, 0.0 + height / 2.0);

        points.push(bottom_left);
        points.push(top);
        points.push(bottom_right);
        points.push(bottom_left);

        points
    }
}

impl Component for Player {
    type Storage = VecStorage<Self>;
}
