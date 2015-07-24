mod object;
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
pub use self::object::Object;
pub use self::image::{Image, Pixel};
pub use self::light::Light;
pub use self::filters::{NopFilter, SmoothingFilter};

pub struct Scene {
    camera: Camera,
    ambient_light: Color,
    background_color: Color,
    objects: Vec<Object>,
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
            objects: Vec::new(),
            lights: Vec::new(),
            filter: Box::new(filter)
        }
    }

    pub fn add_object(&mut self, object: Object) {
        self.objects.push(object);
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
                object: &Object, intersection: Intersection) -> Color {
        let mut result = self.colorize_ambient(object);
        for light in self.lights.iter() {
            if self.is_visible(light.position(), intersection.point) {
                let light_direction = light.position().direction_to(intersection.point);
                let normal = intersection.normal;
                result = result
                    + self.colorize_diffuse(&light, light_direction,
                                            object, normal)
                    + self.colorize_specular(view_direction,
                                             &light, light_direction,
                                             object, normal);
            }
        }
        result
    }

    fn colorize_ambient(&self, object: &Object) -> Color {
        object.color() * self.ambient_light
    }

    fn colorize_diffuse(&self, light: &Light, light_direction: UnitVector,
                        object: &Object, normal: UnitVector) -> Color {
        object.color() * light.color() * -(light_direction.dot(normal))
    }

    fn colorize_specular(&self,  view_direction: UnitVector,
                         light: &Light, light_direction: UnitVector,
                         object: &Object, normal: UnitVector) -> Color {

        let r = light_direction.reflect(normal);
        let k = (-r.dot(view_direction)).powf(3.0f64);
        // println!("{}", k);
        object.color() * light.color() * k
    }

    fn find_obstacle(&self, ray: &Ray) -> Option<(&Object, Intersection)> {
        for obj in self.objects.iter() {
            if let Some(point) = obj.shape().intersect(&ray) {
                return Some((obj, point))
            }
        }
        None
    }

    fn is_visible(&self, what: Point, from: Point) -> bool {
        let ray = Ray::from_to(from, what);
        let ray = Ray::from_to(ray.along(1e-6) , what);
        // FIXME: what if obstacle is behind a light source?
        self.find_obstacle(&ray).is_none()
    }
}
