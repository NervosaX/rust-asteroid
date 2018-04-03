use ggez::graphics::{Point2};

#[derive(Debug, Clone)]
pub struct Player {
    pub width: f32,
    pub height: f32,
    pub points: Vec<Point2>,
}

impl Player {
    pub fn new(width: f32, height: f32) -> Self {
        let mut points = Vec::new();

        let bottom_left = Point2::new(0.0 - width / 2.0, 0.0 + height / 2.0);
        let top = Point2::new(0.0, 0.0 - height / 2.0);
        let bottom_right = Point2::new(0.0 + width / 2.0, 0.0 + height / 2.0);

        points.push(bottom_left);
        points.push(top);
        points.push(bottom_right);
        points.push(bottom_left);

        Self {
            width,
            height,
            points,
        }
    }
}
