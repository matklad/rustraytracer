use geom::{Point, UnitVector};

#[derive(Clone)]
pub struct Ray {
    pub origin: Point,
    pub direction: UnitVector
}


impl Ray {
    pub fn from_to(from: Point, to: Point) -> Ray {
        Ray {
            origin: from,
            direction: (to - from).direction()
        }
    }

    pub fn along(&self, t: f64) -> Point {
        self.origin + self.direction * t
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use props::{check_prop2};
    use geom::point::{Point};

    #[test]
    fn alog_inverse_from_to() {
        check_prop2(|from: Point, to: Point| {
            let ray = Ray::from_to(from, to);
            let t = (from - to).length();
            let p = ray.along(t);
            assert!((to - p).is_almost_zero())
        })
    }
}
