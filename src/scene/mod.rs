mod camera;
mod config;
mod light;
// FIXME: https://github.com/rust-lang/rust/issues/16264
pub mod material;
mod primitive;

use std::error::Error;
use std::collections::HashMap;

use geom::{Point, UnitVector, Ray};
use color::Color;
use self::camera::Camera;
use self::primitive::Primitive;
use self::config::read_primitive;

pub use self::light::LightSource;
pub use self::primitive::Intersection;
pub use self::camera::ScreenPoint;
pub use self::material::{Texture, Material};
pub use self::config::SceneConfig;


pub struct Scene {
    pub camera: Camera,
    pub ambient_light: Color,
    pub background_color: Color,
    pub lights: Vec<LightSource>,
    primitives: Vec<Primitive>,
    materials: Vec<Material>,
}


impl Scene {
    pub fn new(config: SceneConfig) -> Result<Scene, Box<Error>> {
        let mut materials = Vec::new();
        let mut material_index_map = HashMap::new();
        for (k, v) in config.materials {
            material_index_map.insert(k, materials.len());
            materials.push(Material::from(v));
        }

        let primitives = try!(config.primitives.into_iter()
                                               .map(|p| read_primitive(p, &material_index_map))
                                               .collect::<Result<Vec<Primitive>, _>>());
        let lights = config.lights.into_iter()
                                  .map(LightSource::from)
                                  .collect();

        Ok(Scene {
            camera: Camera::from(config.camera),
            ambient_light: config.ambient_light,
            background_color: config.background_color,
            lights: lights,
            primitives: primitives,
            materials: materials,
        })
    }

    pub fn is_visible(&self, what: Point, from: &Intersection) -> bool {
        let ray = Ray::from_to(from.geom.point, what);
        let ray = Ray::from_to(ray.along(1e-6), what);
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
            .filter_map(|obj| {
                let material = &self.materials[obj.material_idx];
                obj.shape.intersect(&ray)
                         .map(|g| Intersection { geom: g, material: material })
            })
            .min()
    }
}
