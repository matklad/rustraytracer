extern crate rand;
extern crate rustc_serialize;
extern crate utils;

mod vector;
mod point;
pub mod ray;
pub mod shape;

pub use self::vector::{Vector, UnitVector};
pub use self::point::{Point};

pub type Axis = u8;


pub mod shortcuts {
    use super::vector::{Vector};
    use super::point::{Point};

    pub fn v(x: f64, y: f64, z: f64) -> Vector {
        Vector::new(x, y, z)
    }

    pub fn p(x: f64, y: f64, z: f64) -> Point {
        Point::new(x, y, z)
    }
}

pub trait Cross<T> {
    fn cross(self, rhs: T) -> Vector;
}

pub trait Dot<T> {
    fn dot(self, rhs: T) -> f64;
}
