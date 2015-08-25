extern crate rand;
extern crate rustc_serialize;
extern crate utils;

mod vector;
mod point;
mod ray;
pub mod shape;

pub use self::vector::{Vector, UnitVector};
pub use self::point::{Point};
pub use self::ray::Ray;

pub type Axis = u8;

pub trait Cross<T> {
    fn cross(self, rhs: T) -> Vector;
}

pub trait Dot<T> {
    fn dot(self, rhs: T) -> f64;
}
