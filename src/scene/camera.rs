use geom::{Point, Vector, UnitVector, Cross};
use geom::ray::{Ray};


struct Screen {
    center: Point,
    basis: [Vector; 2],
}

pub struct Camera {
    position: Point,
    screen: Screen,
}

#[derive(RustcDecodable)]
pub struct CameraConfig {
    pub position: Point,
    pub look_at: Point,
    pub focus_distance: f64,
    pub up: UnitVector,

    pub size: [f64; 2],
}

impl Default for CameraConfig {
    fn default() -> CameraConfig {
        use geom::shortcuts::{p, v};

        CameraConfig {
            position: p(0.0, 50.0, 0.0),
            look_at: p(0.0, 0.0, 0.0),
            focus_distance: 20.0,
            up: v(0.0, 0.0, 1.0).direction(),
            size: [6.4, 4.8]
        }
    }
}

impl Camera {
    pub fn new(config: CameraConfig) -> Camera {
        let ray_to_scren = Ray::from_to(config.position, config.look_at);
        let screen_center = ray_to_scren.along(config.focus_distance);
        let right = ray_to_scren.direction.cross(config.up).direction();
        let up = right.cross(ray_to_scren.direction).direction();
        let screen = Screen {
            center: screen_center,
            basis: [right * config.size[0], -up * config.size[1]],
        };
        Camera {
            position: config.position,
            screen: screen
        }
    }

    pub fn cast_ray(&self, screen_point: [f64; 2]) -> Ray {
        let target = self.screen.center
            + self.screen.basis[0] * screen_point[0]
            + self.screen.basis[1] * screen_point[1];

        return Ray::from_to(self.position, target);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use geom::shortcuts::{v, p};
    use props::{check_prop2};

    #[test]
    fn test_ray_casting() {
        let config = CameraConfig {
            position: p(-10.0, 0.0, 0.0),
            look_at: p(0.0, 0.0, 0.0),
            focus_distance: 10.0,
            up: v(0.0, 0.0, 1.0).direction(),
            size: [6.4, 4.8],
            ..Default::default()
        };
        let cam = Camera::new(config);
        check_prop2(|x: f64, y: f64| {
            let x = x % 0.5;
            let y = y % 0.5;
            let ray = cam.cast_ray([x, y]);
            let p = ray.along(10.0);
            assert!(-1.0 < p[0] && p[0] < 0.0);

            let x = p[1];
            let y = p[2];
            assert!(-3.2 < x && x < 3.2);
            assert!(-2.4 < y && y < 2.4);
        })
    }
}
