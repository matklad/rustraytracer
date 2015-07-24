mod sphere;
mod triangle;

use geom::{Point, UnitVector};
use geom::ray::{Ray};

pub use self::sphere::Sphere;
pub use self::triangle::Triangle;

pub trait Shape {
    fn intersect(&self, ray: &Ray) -> Option<Point>;
    fn normal_at(&self, point: Point) -> UnitVector;
}
