use std::ops::Add;
use geo::{Polygon as GeoPolygon};
use geo::prelude::Intersects;
use ggez::graphics::{Mesh, Point2};
use specs::prelude::{VecStorage, Component};
use game::components::Position;


#[derive(Debug)]
pub struct Asset {
    pub mesh: Option<Mesh>,
}

impl Asset {
    pub fn new() -> Self {
        Self { mesh: None }
    }
}


#[derive(Debug, Clone)]
pub struct Polygon {
    pub points: Vec<Point2>,
}

impl Polygon {
    pub fn new(points: Vec<Point2>) -> Self {
        Self {points}
    }

    pub fn overlaps(&self, poly: &Polygon) -> bool {
        let poly_self: Vec<(f32, f32)> = self.points.iter().map(|p| (p.x, p.y)).collect();
        let poly_other: Vec<(f32, f32)> = poly.points.iter().map(|p| (p.x, p.y)).collect();

        let poly1 = GeoPolygon::new(poly_self.into(), vec![]);
        let poly2 = GeoPolygon::new(poly_other.into(), vec![]);

        poly1.intersects(&poly2)
    }
}

impl<'a> Add<&'a Position> for &'a Polygon {
    type Output = Polygon;

    fn add(self, other: &'a Position) -> Polygon {
        let points = self.points
            .iter()
            .map(|p| {
                Point2::new(p.x + other.0.x, p.y + other.0.y)
            })
            .collect();

        Polygon {
            points
        }
    }
}

impl Component for Polygon {
    type Storage = VecStorage<Self>;
}
