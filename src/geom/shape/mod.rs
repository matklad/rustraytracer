mod sphere;
mod triangle;

use geom::{Point, UnitVector};
use geom::ray::{Ray};

pub use self::sphere::Sphere;
pub use self::triangle::Triangle;

pub struct Intersection {
    pub point: Point,
    pub normal: UnitVector
}

pub trait Shape {
    fn intersect(&self, ray: &Ray) -> Option<Intersection>;
}
