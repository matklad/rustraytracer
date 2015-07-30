extern crate rand;

use std::ops::{Add, Sub, Div, Mul, Neg};
use std::fmt;

use super::{Cross, Dot};


#[derive(Debug, Clone, Copy)]
pub struct Vector {
    pub x: f64,
    pub y: f64,
    pub z: f64
}

impl Vector {
    pub fn new(x: f64, y: f64, z: f64) -> Vector {
        assert!(!(x.is_nan() || y.is_nan() || z.is_nan()));
        Vector { x: x, y: y, z: z}
    }

    pub fn is_almost_zero(self) -> bool {
        self.length() < 1e-6
    }

    pub fn is_almost_eq(self, rhs: Vector) -> bool {
        (self - rhs).is_almost_zero()
    }

    pub fn length(self) -> f64 {
        self.dot(self).sqrt()
    }

    pub fn direction(self) -> UnitVector {
         UnitVector { direction: self / self.length() }
    }
}

impl fmt::Display for Vector {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}


impl rand::Rand for Vector {
    fn rand<R: rand::Rng>(rng: &mut R) -> Vector {
        Vector::new(f64::rand(rng), f64::rand(rng), f64::rand(rng))
    }
}

impl Neg for Vector {
    type Output = Vector;

    fn neg(self) -> Vector {
        Vector::new(-self.x, -self.y, -self.z)
    }
}

impl Add for Vector {
    type Output = Vector;

    fn add(self, rhs: Vector) -> Vector {
        Vector::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl Sub for Vector {
    type Output = Vector;

    fn sub(self, rhs: Vector) -> Vector {
        Vector::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl Div<f64> for Vector {
    type Output = Vector;

    fn div(self, c: f64) -> Vector {
        let r = 1.0 / c;
        Vector::new(self.x * r, self.y * r, self.z * r)
    }
}

impl Mul<f64> for Vector {
    type Output = Vector;

    fn mul(self, c: f64) -> Vector {
        Vector::new(self.x * c, self.y * c, self.z * c)
    }
}


impl Mul<Vector> for f64 {
    type Output = Vector;

    fn mul(self, v: Vector) -> Vector {
        v * self
    }
}


#[derive(Debug, Clone, Copy)]
pub struct UnitVector {
    direction: Vector
}

impl UnitVector {
    pub fn reflect(self, axis: UnitVector) -> UnitVector {
        (self.direction + 2.0f64 * axis).direction()
    }
}

impl fmt::Display for UnitVector {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.direction.fmt(f)
    }
}

impl Neg for UnitVector {
    type Output = UnitVector;

    fn neg(self) -> UnitVector {
        UnitVector {
            direction: -self.direction
        }
    }
}

impl Div<f64> for UnitVector {
    type Output = Vector;

    fn div(self, c: f64) -> Vector {
        self.direction / c
    }
}

impl Mul<f64> for UnitVector {
    type Output = Vector;

    fn mul(self, c: f64) -> Vector {
        self.direction * c
    }
}

impl Mul<UnitVector> for f64 {
    type Output = Vector;

    fn mul(self, v: UnitVector) -> Vector {
        v * self
    }
}


impl Dot<Vector> for Vector {
    fn dot(self, rhs: Vector) -> f64 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }
}

impl Dot<UnitVector> for Vector {
    fn dot(self, rhs: UnitVector) -> f64 {
        self.dot(rhs.direction)
    }
}


impl Dot<UnitVector> for UnitVector {
    fn dot(self, rhs: UnitVector) -> f64 {
        self.direction.dot(rhs.direction)
    }
}

impl Dot<Vector> for UnitVector {
    fn dot(self, rhs: Vector) -> f64 {
        self.direction.dot(rhs)
    }
}



impl Cross<Vector> for Vector {
    fn cross(self, rhs: Vector) -> Vector {
        // self x y z
        // rhs  x y z
        Vector::new(self.y * rhs.z - self.z * rhs.y,
                    - (self.x * rhs.z - self.z * rhs.x),
                    self.x * rhs.y - self.y * rhs.x)
    }
}

impl Cross<UnitVector> for Vector {
    fn cross(self, rhs: UnitVector) -> Vector {
        self.cross(rhs.direction)
    }
}

impl Cross<UnitVector> for UnitVector {
    fn cross(self, rhs: UnitVector) -> Vector {
        self.direction.cross(rhs.direction)
    }
}

impl Cross<Vector> for UnitVector {
    fn cross(self, rhs: Vector) -> Vector {
        self.direction.cross(rhs)
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use geom::Cross;
    use props::{check_prop, check_prop2};

    #[test]
    fn addition_commutative() {
        check_prop2(|a: Vector, b: Vector| {
            let l = a + b;
            let r = b + a;
            assert!(l.is_almost_eq(r))
        })
    }

    #[test]
    fn subtraction_is_inverse() {
        check_prop(|a: Vector| {
            assert!((a - a).is_almost_zero());
        })
    }

    #[test]
    fn cross_with_self_is_zero() {
        check_prop(|a: Vector| {
            assert!(a.cross(a).is_almost_zero());
        })
    }

    #[test]
    fn mul_inverse_is_div() {
        check_prop2(|a: Vector, c: f64| {
            assert!(((a / c) * c).is_almost_eq(a))
        })
    }
}
