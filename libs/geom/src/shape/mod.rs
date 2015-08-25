use std::cmp::{Ordering};
use Point;
use UnitVector;
use Ray;

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
    pub local_coordinates: [f64; 2],
    pub normal: UnitVector,
}

impl Intersection {
    pub fn new (t: f64,
                point: Point,
                local_coordinates: [f64; 2],
                normal: UnitVector) -> Intersection {

        assert!(!t.is_nan());
        Intersection {
            t: t,
            point: point,
            local_coordinates: local_coordinates,
            normal: normal
        }
    }
}

impl Ord for Intersection {
    fn cmp(&self, other: &Intersection) -> Ordering {
        self.t.partial_cmp(&other.t).unwrap()
    }
}

impl PartialEq for Intersection {
    fn eq(&self, other: &Intersection) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl PartialOrd for Intersection {
    fn partial_cmp(&self, other: &Intersection) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for Intersection {}


pub trait Shape: Send + Sync {
    fn intersect(&self, ray: &Ray) -> Option<Intersection>;
}
