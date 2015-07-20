use geom::{Point, UnitVector, Vector, Cross};
use super::Shape;

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
        n = av.cross(ac);
        Triangle {
            a: a,
            ab: ab,
            ac: ac,
            normals = [n; 3],
        }
    }

    pub fn with_normals(a: Point, b: Point, c: Point, normals: [UnitVector; 3]) -> Triangle {
        let ab = b - a;
        let ac = c - a;
        Triangle {
            a: a,
            ab: ab,
            ac: ac,
            normals = normals,
        }
    }
}

impl Shape for Triangle {
    fn intersect(&self, ray: &Ray) -> Option<Point> {
        // a + alpha ab + beta ac = ray.origin + t * ray.direction
        let n = self.a.cross(self.b);
        let t = (self.a - ray.origin).dot(n) / ray.direction.dot(n);
        if t < 0 {
            return None;
        }
        let p = ray.along(t) - self.a;

        let ort_ac = self.ac.cross(n);
        let ort_ab = self.ab.cross(n);
        let alpha = p.dot(ort_ac) / ab.dot(ort_ac);
        let beta = p.dot(ort_ab) / ac.dot(ort_ab);
        let f = |x| 0.0 < x < 1.0;
        if f(alpha) && f (beta) && f (1 - (alpha + beta)) {
            Some(p)
        } else {
            None
        }
    }
}
