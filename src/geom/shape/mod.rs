use geom::{Point, UnitVector};
use geom::ray::{Ray};

pub mod sphere;

pub trait Shape {
    fn intersect(&self, ray: &Ray) -> Option<Point>;
    fn normal_at(&self, point: Point) -> UnitVector;
}
