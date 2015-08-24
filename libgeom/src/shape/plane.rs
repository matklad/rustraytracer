use Point;
use UnitVector;
use Dot;
use ray::Ray;
use super::{Intersection, Shape};


pub struct Plane {
    origin: Point,
    normal: UnitVector,
}


impl Plane {
    pub fn new(origin: Point, normal: UnitVector) -> Plane {
        Plane {
            origin: origin,
            normal: normal
        }
    }
}

impl Shape for Plane {
    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        // (x - self.origin).dot(self.normal) = 0
        // x = ray.origin + t * ray.direction

        // (ray.origin + t * ray.direction - self.origin).dot(self.normal) = 0;
        let o = ray.origin - self.origin;
        // o.dot(self.normal) + t * ray.direction.dot(self.normal) = 0
        let t = -o.dot(self.normal) / ray.direction.dot(self.normal);
        if t < 0.0 {
            None
        } else {
            let local_coordinates = [0.0, 0.0];
            Some(Intersection::new(t, ray.along(t), local_coordinates, self.normal))
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use props::check_prop;
    use geom::shape::Shape;
    use geom::shortcuts::{p, v};
    use geom::ray::Ray;

    #[test]
    fn test_plane_intersection() {
        let plane = Plane::new(
            p(0.0, 0.0, 0.0),
            v(1.0, 0.0, 0.0).direction());

        let ray_origin = p(10.0, 3.0, 4.0);
        check_prop(|(x, y, z): (f64, f64, f64)| {
            let ray = Ray::from_to(ray_origin, p(x, y, z));
            let intersection = plane.intersect(&ray);
            match intersection {
                None => assert!(x < 0.0),
                Some(i) => assert!(i.point[0].abs() < 1e-6)
            }
        })
    }

}
