use std::cmp::{Ordering};
use geom::{Point, UnitVector};
use geom::ray::{Ray};

mod mesh;
mod plane;
mod sphere;
mod triangle;
mod bound_box;

pub use self::mesh::Mesh;
pub use self::plane::Plane;
pub use self::sphere::Sphere;
pub use self::triangle::Triangle;

#[derive(Debug, Clone, Copy)]
pub struct Intersection {
    pub t: f64,
    pub point: Point,
    pub normal: UnitVector
}

impl Ord for Intersection {
    fn cmp(&self, other: &Intersection) -> Ordering {
        self.t.partial_cmp(&other.t).unwrap()
    }
}

impl PartialEq for Intersection {
    fn eq(&self, other: &Intersection) -> bool {
        self.cmp(other) == Ordering::Greater
    }
}

impl PartialOrd for Intersection {
    fn partial_cmp(&self, other: &Intersection) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for Intersection {}


pub trait Shape {
    fn intersect(&self, ray: &Ray) -> Option<Intersection>;
}
