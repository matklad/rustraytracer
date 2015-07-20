extern crate rand;
use std::ops::{Add, Sub};
use std::fmt;
use geom::{Vector, UnitVector};


#[derive(Debug, Clone, Copy)]
pub struct Point {
    radius_vector: Vector,
}

impl Point {
    pub fn new(x: f64, y: f64, z: f64) -> Point {
        Point { radius_vector: Vector::new(x, y, z) }
    }

    pub fn direction_to(self, target: Point) -> UnitVector {
        (target - self).direction()
    }

    pub fn x(&self) -> f64 {
        self.radius_vector.x()
    }

    pub fn y(&self) -> f64 {
        self.radius_vector.y()
    }

    pub fn z(&self) -> f64 {
        self.radius_vector.z()
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.radius_vector.fmt(f)
    }
}

impl rand::Rand for Point {
    fn rand<R: rand::Rng>(rng: &mut R) -> Point {
        Point { radius_vector: Vector::rand(rng) }
    }
}

impl Sub for Point {
    type Output = Vector;

    fn sub(self, rhs: Point) -> Vector {
        self.radius_vector - rhs.radius_vector
    }
}

impl Add<Vector> for Point {
    type Output = Point;

    fn add(self, rhs: Vector) -> Point {
        Point{ radius_vector: self.radius_vector + rhs }
    }
}
