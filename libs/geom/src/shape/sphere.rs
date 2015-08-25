use Point;
use UnitVector;
use Dot;
use ray::Ray;
use super::{Shape, Intersection};

pub struct Sphere {
    center: Point,
    radius: f64
}


impl Sphere {
    pub fn new(center: Point, radius: f64) -> Sphere {
        Sphere {
            center: center,
            radius: radius
        }
    }

    fn normal_at(&self, point: Point) -> UnitVector {
        return self.center.direction_to(point)
    }
}

impl Shape for Sphere {

    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        // (x - self.center)^2 == self.radius
        // x = ray.center + t * ray.direction
        let o = ray.origin - self.center;
        let k = ray.direction.dot(o);
        let c = o.dot(o) - self.radius * self.radius;

        let disc = k*k - c;
        if disc < 0.0 {
            return None;
        }

        let t = {
            let t1 = -k - disc.sqrt();
            let t2 = -k + disc.sqrt();
            if t1 > 0.0 {
                Some(t1)
            } else if t2 > 0.0 {
                Some(t2)
            } else {
                None
            }
        };

        t.map(|t| {
            let point = ray.along(t);
            let local_coordinates = [0.0, 0.0];
            Intersection::new(t, point, local_coordinates, self.normal_at(point))
        })
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use {Point, Cross};
    use shape::{Shape, Intersection};
    use ray::{Ray};

    use shortcuts::{p};
    use utils::props::check_prop2;

    #[test]
    fn hit_on_sphere() {
        let mut hits = 0;
        let mut misses = 0;
        let center = p(0.0, 0.0, 0.0);
        let radius = 0.5;
        let sphere = Sphere::new(center, radius);

        let is_on_sphere = |p: Point| {
            ((center - p).length() - radius) < 1e-6
        };

        let from = p(-10.0, 0.0, 0.0);

        check_prop2(|y: f64, z: f64| {
            let to = p(0.0, y % 1.0, z % 1.0);
            let ray = Ray::from_to(from, to);

            let is_on_ray = |p: Point| {
                ray.direction.cross(p - ray.origin).is_almost_zero()
            };

            match sphere.intersect(&ray) {
                None => misses += 1,
                Some(Intersection {point: p, ..}) => {
                    hits += 1;
                    assert!(is_on_sphere(p));
                    assert!(is_on_ray(p));
                }
            }
        });

        assert!(hits > 1);
        assert!(misses > 1);
    }
}
