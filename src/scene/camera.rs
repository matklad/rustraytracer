use geom::{Point, Vector, Ray, Cross};
use super::config::CameraConfig;


#[derive(Clone, Copy, Debug)]
pub struct ScreenPoint {
    pub x: f64,
    pub y: f64
}

impl ScreenPoint {
    pub fn new(x: f64, y: f64) -> ScreenPoint {
        ScreenPoint { x: x, y: y }
    }
    pub fn is_normalized(&self) -> bool {
        let within_bounds = |x| -1.0 <= x && x < 1.0;
        within_bounds(self.x) && within_bounds(self.y)
    }
}

impl From<[f64; 2]> for ScreenPoint {
    fn from(xy: [f64; 2]) -> ScreenPoint {
        ScreenPoint::new(xy[0], xy[1])
    }
}


struct Screen {
    center: Point,
    basis: [Vector; 2],
}


pub struct Camera {
    position: Point,
    screen: Screen,
}

impl Camera {
    pub fn cast_ray(&self, screen_point: ScreenPoint) -> Ray {
        let target = self.screen.center
        + self.screen.basis[0] * screen_point.x
        + self.screen.basis[1] * screen_point.y;

        return Ray::from_to(self.position, target);
    }
}

impl From<CameraConfig> for Camera {
    fn from(config: CameraConfig) -> Camera {
        let ray_to_scren = Ray::from_to(config.position, config.look_at);
        let screen_center = ray_to_scren.along(config.focus_distance);
        let right = ray_to_scren.direction.cross(config.up).direction();
        let up = right.cross(ray_to_scren.direction).direction();
        let screen = Screen {
            center: screen_center,
            basis: [right * config.size[0] / 2.0, -up * config.size[1] / 2.0],
        };
        Camera {
            position: config.position,
            screen: screen
        }
    }
}


    #[cfg(test)]
mod tests {
    use super::*;
    use scene::config::CameraConfig;
    use geom::shortcuts::{v, p};
    use utils::props::{check_prop2};


    #[test]
    fn test_ray_casting() {
        let config = CameraConfig {
            position: p(-10.0, 0.0, 0.0),
            look_at: p(0.0, 0.0, 0.0),
            focus_distance: 10.0,
            up: v(0.0, 0.0, 1.0).direction(),
            size: [6.4, 4.8],
        };
        let cam = Camera::from(config);
        check_prop2(|x: f64, y: f64| {
            let x = x % 1.0;
            let y = y % 1.0;
            let ray = cam.cast_ray(ScreenPoint::new(x, y));
            let p = ray.along(10.0);
            assert!(-1.0 < p[0] && p[0] < 0.0);

            let x = p[1];
            let y = p[2];
            assert!(-3.2 < x && x < 3.2);
            assert!(-2.4 < y && y < 2.4);
        })
    }
}
