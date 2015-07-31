mod primitive;
mod camera;
mod image;
mod light;
mod filters;
mod renderer;

use geom::{Point};
use geom::shape::{Shape, Intersection};
use geom::ray::{Ray};
use color::Color;
use self::camera::Camera;

pub use self::camera::CameraConfig;
pub use self::primitive::Primitive;
pub use self::image::{Image, Pixel};
pub use self::light::Light;
pub use self::filters::{NopFilter, SmoothingFilter};
pub use self::renderer::Renderer;

pub struct Scene {
    camera: Camera,
    ambient_light: Color,
    background_color: Color,
    primitives: Vec<Primitive>,
    lights: Vec<Light>,
}

pub struct SceneConfig {
    pub camera: CameraConfig,
    pub background: Color,
    pub ambient_light: Color,
}

impl Scene {
    pub fn new(config: SceneConfig) -> Scene {
        Scene {
            camera: Camera::new(config.camera),
            ambient_light: config.ambient_light,
            background_color: config.background,
            primitives: Vec::new(),
            lights: Vec::new(),
        }
    }

    pub fn add_primitive(&mut self, primitive: Primitive) {
        self.primitives.push(primitive);
    }

    pub fn add_light(&mut self, light: Light) {
        self.lights.push(light);
    }

    fn find_obstacle(&self, ray: &Ray) -> Option<(&Primitive, Intersection)> {
        let mut result = None;
        for obj in self.primitives.iter() {
            if let Some(intersection) = obj.shape().intersect(&ray) {
                result = match result {
                    None => Some((obj, intersection)),
                    Some(previous) if intersection < previous.1 => Some((obj, intersection)),
                    _ => result
                }
            }
        }
        result
    }

    fn is_visible(&self, what: Point, from: Point) -> bool {
        let ray = Ray::from_to(from, what);
        let ray = Ray::from_to(ray.along(1e-6) , what);
        // FIXME: what if obstacle is behind a light source?
        self.find_obstacle(&ray).is_none()
    }
}
