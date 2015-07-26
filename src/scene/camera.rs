use geom::{Point, Vector, UnitVector, Cross};
use geom::ray::{Ray};
use super::Pixel;


struct Screen {
    center: Point,
    pub resolution: Pixel,
    basis: [Vector; 2],
}

pub struct Camera {
    position: Point,
    screen: Screen,
}

pub struct CameraConfig {
    pub position: Point,
    pub look_at: Point,
    pub focus_distance: f64,
    pub up: UnitVector,

    pub resolution: Pixel,
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
            resolution: [640, 480],
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
            resolution: config.resolution,
            basis: [right * config.size[0], -up * config.size[1]],
        };
        Camera {
            position: config.position,
            screen: screen
        }
    }

    pub fn resolution(&self) -> Pixel {
        self.screen.resolution
    }

    pub fn cast_ray(&self, pixel: Pixel) -> Ray {
        let mut relative = [0.0, 0.0];
        for i in 0..2 {
            let res = self.screen.resolution[i];
            assert!(pixel[i] < res);
            let pixel_width = 1.0 / (res as f64);
            relative[i] = (((pixel[i] as f64) + 0.5) * pixel_width) - 0.5;
            assert!(-0.5 < relative[i] && relative[i] < 0.5);
        }
        let target = self.screen.center
            + self.screen.basis[0] * relative[0]
            + self.screen.basis[1] * relative[1];

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
            resolution: [640, 480],
            size: [6.4, 4.8],
            ..Default::default()
        };
        let cam = Camera::new(config);
        check_prop2(|x: u32, y: u32| {
            let x = x % 640;
            let y = y % 480;
            let ray = cam.cast_ray([x, y]);
            let p = ray.along(10.0);
            assert!(-1.0 < p.x() && p.x() < 0.0);

            let x = p.y();
            let y = p.z();
            assert!(-3.2 < x && x < 3.2);
            assert!(-2.4 < y && y < 2.4);
        })
    }
}
