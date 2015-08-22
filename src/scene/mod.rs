mod camera;
mod config;
mod light;
// FIXME: https://github.com/rust-lang/rust/issues/16264
pub mod material;
mod primitive;

use std::sync::Arc;
use std::error::Error;

use geom::{Point, UnitVector};
use geom::shape::Shape;
use geom::ray::Ray;
use color::Color;
use self::camera::Camera;
use self::config::read_primitive;

pub use self::light::LightSource;
pub use self::primitive::{Primitive, Intersection};
pub use self::camera::ScreenPoint;
pub use self::material::{Texture, Material};
pub use self::config::SceneConfig;


pub struct Scene {
    pub camera: Camera,
    pub ambient_light: Color,
    pub background_color: Color,
    pub primitives: Vec<Primitive>,
    pub lights: Vec<LightSource>,
}


impl Scene {
    pub fn new(config: SceneConfig) -> Result<Scene, Box<Error>> {
        let materials = config.materials.into_iter()
            .map(|(k, v)| (k, Arc::new(Material::from(v))))
            .collect();

        let primitives = try!(config.primitives.into_iter()
                              .map(|p| read_primitive(p, &materials))
                              .collect::<Result<Vec<Primitive>, _>>());
        let lights = config.lights.into_iter()
            .map(LightSource::from)
            .collect();

        Ok(Scene {
            camera: Camera::from(config.camera),
            ambient_light: config.ambient_light,
            background_color: config.background_color,
            primitives: primitives,
            lights: lights,
        })
    }

    pub fn is_visible(&self, what: Point, from: &Intersection) -> bool {
        let ray = Ray::from_to(from.geom.point, what);
        let ray = Ray::from_to(ray.along(1e-6) , what);
        // FIXME: what if obstacle is behind a light source?
        match self.find_obstacle(&ray) {
            None => true,
            Some(i) => {
                i.geom.t > (from.geom.point - what).length()
            }
        }
    }

    pub fn ray_from(&self, from: &Intersection, direction: UnitVector) -> Ray {
        Ray {
            origin: from.geom.point + direction * 1e-6,
            direction: direction
        }
    }

    pub fn find_obstacle(&self, ray: &Ray) -> Option<Intersection> {
        self.primitives
            .iter()
            .filter_map(|obj| obj.intersect(&ray))
            .min()
    }
}
