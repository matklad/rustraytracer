use geom::{Point, UnitVector, Vector, Cross, Dot};
use geom::ray::Ray;
use super::{Shape, Intersection};
use super::bound_box::{Bound, BoundBox};

pub struct Triangle {
    a: Point,
    ab: Vector,
    ac: Vector,
    normals: [UnitVector; 3],
}

impl Triangle {
    pub fn new(a: Point, b: Point, c: Point) -> Triangle {
        let ab = b - a;
        let ac = c - a;
        let n = ab.cross(ac).direction();
        Triangle {
            a: a,
            ab: ab,
            ac: ac,
            normals: [n; 3],
        }
    }

    pub fn with_normals(a: Point, b: Point, c: Point, normals: [UnitVector; 3]) -> Triangle {
        let ab = b - a;
        let ac = c - a;
        Triangle {
            a: a,
            ab: ab,
            ac: ac,
            normals: normals,
        }
    }

    fn local_coordinates(&self, point: Point) -> (f64, f64, f64) {
        let ort_ac = self.ac.cross(self.normal());
        let ort_ab = self.ab.cross(self.normal());
        let point = point - self.a;
        let alpha = point.dot(ort_ac) / self.ab.dot(ort_ac);
        let beta = point.dot(ort_ab) / self.ac.dot(ort_ab);
        let gamma = 1.0 - (alpha + beta);
        (alpha, beta, gamma)
    }

    fn normal(&self) -> Vector{
        self.ac.cross(self.ab)
    }

    fn interpolate_normal(&self, alpha: f64, beta: f64, gamma: f64) -> UnitVector {
        return (alpha * self.normals[1] +
                beta * self.normals[2] +
                gamma * self.normals[0]).direction()
    }
}


impl Bound for Triangle {
    fn bound(&self) -> BoundBox {
        self.a.bound()
            .union(&(self.a + self.ab).bound())
            .union(&(self.a + self.ac).bound())
    }
}


impl Shape for Triangle {
    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        // a + alpha ab + beta ac = ray.origin + t * ray.direction
        let t = (self.a - ray.origin).dot(self.normal()) / ray.direction.dot(self.normal());
        if t < 0.0 {
            return None;
        }
        let point = ray.along(t);
        let (alpha, beta, gamma) = self.local_coordinates(point);
        let f = |x| 0.0 < x && x < 1.0;
        if f(alpha) && f(beta) && f(gamma)  {
            Some(Intersection {
                t: t,
                point: point,
                normal: self.interpolate_normal(alpha, beta, gamma)
            })
        } else {
            None
        }
    }


}

#[cfg(test)]
mod test {
    use super::*;
    use geom::shape::Shape;
    use geom::shortcuts::p;
    use geom::ray::Ray;
    use props::check_prop2;

    #[test]
    fn test_triangle_intersection() {
        let t1 = Triangle::new(
            p(0.0, -1.0, -1.0),
            p(0.0, -1.0, 1.0),
            p(0.0, 1.0, 1.0));
        let t2 = Triangle::new(
            p(0.0, 1.0, 1.0),
            p(0.0, 1.0, -1.0),
            p(0.0, -1.0, -1.0));
        let origin = p(-1.0, 0.0, 0.0);
        let mut t1_hits = 0;
        let mut t2_hits = 0;

        check_prop2(|y: f64, z: f64| {
            let y = y % 1.0;
            let z = z % 1.0;
            let ray = Ray::from_to(origin, p(0.0, y, z));
            let i1 = t1.intersect(&ray);
            let i2 = t2.intersect(&ray);
            assert!((i1.is_some() || i2.is_some()) &&
                    !(i1.is_some() && i2.is_some()));
            if i1.is_some() {
                t1_hits += 1;
            }
            if i2.is_some() {
                t2_hits += 1;
            }
        });

        assert!(t1_hits > 10);
        assert!(t2_hits > 10);
    }
}
