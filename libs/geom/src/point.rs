use std::ops::{Add, Sub, Index};
use std::fmt;
use rand;
use rustc_serialize::{Decodable, Decoder};

use Vector;
use UnitVector;
use Axis;


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
}

impl Index<Axis> for Point {
    type Output = f64;
    fn index(&self, a: Axis) -> &f64 {
        &self.radius_vector[a]
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.radius_vector.fmt(f)
    }
}

impl rand::distributions::Distribution<Point> for rand::distributions::Standard {
    fn sample<R: rand::Rng + ?Sized>(&self, r: &mut R) -> Point {
        Point { radius_vector: r.gen() }
    }
}



impl Decodable for Point {
    fn decode<D: Decoder>(d: &mut D) -> Result<Point, D::Error> {
        let radius_vector: Vector = Decodable::decode(d)?;
        Ok(Point {radius_vector: radius_vector})
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
