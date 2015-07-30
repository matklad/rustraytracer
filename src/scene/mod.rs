mod primitive;
mod camera;
mod image;
mod light;
mod filters;

use geom::{UnitVector, Point, Dot};
use geom::shape::{Shape, Intersection};
use geom::ray::{Ray};
use color::Color;
use self::camera::Camera;
use self::image::new_image;
use self::filters::Filter;

pub use self::camera::CameraConfig;
pub use self::primitive::Primitive;
pub use self::image::{Image, Pixel};
pub use self::light::Light;
pub use self::filters::{NopFilter, SmoothingFilter};

pub struct Scene {
    camera: Camera,
    ambient_light: Color,
    background_color: Color,
    primitives: Vec<Primitive>,
    lights: Vec<Light>,
    filter: Box<Filter>,
}

pub struct SceneConfig {
    pub camera: CameraConfig,
    pub background: Color,
    pub ambient_light: Color,
}

impl Scene {
    pub fn new<F: Filter + 'static>(config: SceneConfig, filter: F) -> Scene {
        let config = filter.process_config(config);
        Scene {
            camera: Camera::new(config.camera),
            ambient_light: config.ambient_light,
            background_color: config.background,
            primitives: Vec::new(),
            lights: Vec::new(),
            filter: Box::new(filter)
        }
    }

    pub fn add_primitive(&mut self, primitive: Primitive) {
        self.primitives.push(primitive);
    }

    pub fn add_light(&mut self, light: Light) {
        self.lights.push(light);
    }

    pub fn render(&self) -> Image {
        let image = new_image(self.camera.resolution(), |x, y| {
            let ray = self.camera.cast_ray([x, y]);
            match self.find_obstacle(&ray) {
                Some((obj, point)) => self.colorize(ray.direction, &obj, point),
                None => self.background_color
            }
        });

        self.filter.process_image(image)
    }

    fn colorize(&self, view_direction: UnitVector,
                primitive: &Primitive, intersection: Intersection) -> Color {
        let mut result = primitive.colorize_ambient(self.ambient_light);
        let visible_lights = self.lights.iter()
            .filter(|&light| self.is_visible(light.position(), intersection.point));

        for light in visible_lights {
            let light_direction = light.position().direction_to(intersection.point);
            let normal = intersection.normal;
            result = result
                + primitive.colorize_diffuse(&light, light_direction, normal)
                + primitive.colorize_specular(view_direction, &light,
                                           light_direction, normal);
        }
        result
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

trait ColoredPrimitive {
    fn colorize_ambient(&self, ambient: Color) -> Color;
    fn colorize_diffuse(&self, light: &Light, light_direction: UnitVector,
                        normal: UnitVector) -> Color;
    fn colorize_specular(&self,  view_direction: UnitVector,
                         light: &Light, light_direction: UnitVector,
                         normal: UnitVector) -> Color;
}


impl ColoredPrimitive for Primitive {
    fn colorize_ambient(&self, ambient: Color) -> Color {
        self.color() * ambient
    }

    fn colorize_diffuse(&self, light: &Light, light_direction: UnitVector,
                        normal: UnitVector) -> Color {
        let k = (-light_direction.dot(normal)).max(0.0) * 0.9;
        self.color() * light.color() * k
    }

    fn colorize_specular(&self,  view_direction: UnitVector,
                         light: &Light, light_direction: UnitVector,
                         normal: UnitVector) -> Color {

        let r = light_direction.reflect(normal);
        let k = (-r.dot(view_direction)).max(0.0).powf(4.0f64);
        light.color() * k
    }
}
