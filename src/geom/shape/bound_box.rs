use std::f64;
use std::iter::FromIterator;

use geom::ray::Ray;
use geom::{Point, Axis, Vector};


#[derive(Debug, Clone, Copy)]
pub struct BoundBox {
    p_min: Point,
    p_max: Point,
}

impl BoundBox {
    pub fn empty() -> BoundBox {
        BoundBox {
            p_min: Point::new(f64::INFINITY, f64::INFINITY, f64::INFINITY),
            p_max: Point::new(f64::NEG_INFINITY, f64::NEG_INFINITY, f64::NEG_INFINITY)
        }
    }
}

impl BoundBox {
    pub fn union(&self, rhs: &BoundBox) -> BoundBox {
        fn pmap<F>(f: F, a: &Point, b: &Point) -> Point
            where F: Fn(f64, f64) -> f64 {
                Point::new(f(a[0], b[0]), f(a[1], b[1]), f(a[2], b[2]))
            }

        BoundBox {
            p_min: pmap(f64::min, &self.p_min, &rhs.p_min),
            p_max: pmap(f64::max, &self.p_max, &rhs.p_max)
        }
    }

    pub fn center(&self) -> Point {
        return self.p_min + self.diag() * 0.5
    }

    pub fn longext_axis(&self) -> Axis {
        let d = self.diag();
        if d.x > d.y && d.x > d.z {
            0
        } else if d.y > d.z {
            1
        } else {
            2
        }
    }

    pub fn is_intersected(&self, ray: &Ray, mut max_t: f64) -> bool {
        let mut min_t: f64 = 0.0;
        for axis in (0..3) {
            let inv_dir = 1.0 / ray.direction[axis];
            let t1 = (self.p_min[axis] - ray.origin[axis]) * inv_dir;
            let t2 = (self.p_max[axis] - ray.origin[axis]) * inv_dir;
            let t_near = t1.min(t2);
            let t_far = t1.max(t2);
            min_t = min_t.max(t_near);
            max_t = max_t.min(t_far);
            assert!(!min_t.is_nan());
            assert!(!max_t.is_nan());
            if max_t < min_t {
                return false
            }
        }
        true
    }

    fn diag(&self) -> Vector {
        self.p_max - self.p_min
    }
}


impl FromIterator<Point> for BoundBox {
    fn from_iter<T>(iterator: T) -> BoundBox where T: IntoIterator<Item=Point> {
        iterator
            .into_iter()
            .map(|p| p.bound())
            .fold(BoundBox::empty(), |a, b| a.union(&b))
    }
}


pub trait Bound {
    fn bound(&self) -> BoundBox;
}

impl Bound for Point {
    fn bound(&self) -> BoundBox {
        BoundBox {
            p_min: self.clone(),
            p_max: self.clone()
        }
    }
}
