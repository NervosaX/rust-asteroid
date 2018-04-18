use std::f32::consts::PI;
use std::ops::{Add};
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

pub struct Circle {
    pub radius: f32,
}

impl Circle {
    pub fn new(radius: f32) -> Self {
        Self { radius }
    }

    pub fn to_polygon(&self, segments: u32) -> Polygon {
        let segments: Vec<Point2> = (0..segments)
            .map(|i| {
                let mut a = 0.0;
                if i != 0 {
                    a = (1.0 / PI) * (i as f32);
                }
                let x = self.radius * a.cos();
                let y = self.radius * a.sin();

                Point2::new(x, y)
            })
            .collect();

        let mut points = vec![];

        // TODO: Can I do this in a more functional way?
        // flat_map would be good...
        for segment in segments.windows(2) {
            points.push(segment[0]);
            points.push(segment[1]);
        }

        // Join the end of the circle
        points.push(segments[0]);

        Polygon {
            points: points
        }
    }
}

impl Component for Circle {
    type Storage = VecStorage<Self>;
}

#[derive(Debug, Clone)]
/// Represents points that form a polygon
pub struct Polygon {
    pub points: Vec<Point2>,
}

impl Polygon {
    pub fn new(points: Vec<Point2>) -> Self {
        Self {points}
    }

    /// Tests if one polygon intersects another
    pub fn overlaps_poly(&self, poly: &Polygon) -> bool {
        let poly_self: Vec<(f32, f32)> = self.points.iter().map(|p| (p.x, p.y)).collect();
        let poly_other: Vec<(f32, f32)> = poly.points.iter().map(|p| (p.x, p.y)).collect();

        let poly1 = GeoPolygon::new(poly_self.into(), vec![]);
        let poly2 = GeoPolygon::new(poly_other.into(), vec![]);

        poly1.intersects(&poly2)
    }

    pub fn overlaps_circle(&self, circle: &Circle) -> bool {
        let poly = circle.to_polygon(30);
        poly.overlaps_poly(self)
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
