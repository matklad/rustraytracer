mod camera;
mod light;
mod material;
mod primitive;

use std::error::Error;
use std::{io, fs};

use geom::{Point, UnitVector};
use geom::shape::{Shape, Intersection, Mesh, Plane};
use geom::ray::Ray;
use color::Color;
use self::camera::{Camera, CameraConfig};
use self::material::Material;

pub use self::light::Light;
pub use self::primitive::Primitive;
pub use self::camera::ScreenPoint;


pub struct Scene {
    pub camera: Camera,
    pub ambient_light: Color,
    pub background_color: Color,
    pub primitives: Vec<Primitive>,
    pub lights: Vec<Light>,
}


impl Scene {
    pub fn new(config: SceneConfig) -> Result<Scene, Box<Error>> {
        let primitives = try!(config.primitives.into_iter().map(read_primitive)
                              .collect::<Result<Vec<Primitive>, _>>());

        Ok(Scene {
            camera: Camera::new(config.camera),
            ambient_light: config.ambient_light,
            background_color: config.background_color,
            primitives: primitives,
            lights: config.lights,
        })
    }

    pub fn find_obstacle(&self, ray: &Ray) -> Option<(&Primitive, Intersection)> {
        let mut result = None;
        for obj in self.primitives.iter() {
            if let Some(intersection) = obj.shape.intersect(&ray) {
                result = match result {
                    None => Some((obj, intersection)),
                    Some(previous) if intersection < previous.1 => Some((obj, intersection)),
                    _ => result
                }
            }
        }
        result
    }

    pub fn is_visible(&self, what: Point, from: Point) -> bool {
        let ray = Ray::from_to(from, what);
        let ray = Ray::from_to(ray.along(1e-6) , what);
        // FIXME: what if obstacle is behind a light source?
        self.find_obstacle(&ray).is_none()
    }
}

#[derive(RustcDecodable)]
pub struct SceneConfig {
    camera: CameraConfig,
    ambient_light: Color,
    background_color: Color,
    primitives: Vec<PrimitiveConfig>,
    lights: Vec<Light>
}


#[derive(RustcDecodable)]
pub enum PrimitiveConfig {
    Mesh {
        location: String,
        material: Material
    },
    Plane {
        position: Point,
        normal: UnitVector,
        material: Material
    }
}



// fn read<T: Decodable>(data: &Json) -> Result<T, Box<Error>> {
//     let mut decoder = json::Decoder::new(data.clone());
//     let result = try!(Decodable::decode(&mut decoder));
//     Ok(result)
// }

fn read_primitive(conf: PrimitiveConfig) -> Result<Primitive, Box<Error>> {
    match conf {
        PrimitiveConfig::Mesh {location, material} => {
            let mut file = try!(fs::File::open(&location).map(io::BufReader::new));
            let mesh = try!(Mesh::from_obj(&mut file));
            Ok(Primitive::new(mesh, material))
        },
        PrimitiveConfig::Plane {position, normal, material} =>
            Ok(Primitive::new(Plane::new(position, normal), material))
    }
}
