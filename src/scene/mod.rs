mod camera;
mod filters;
mod image;
mod light;
mod material;
mod primitive;
mod renderer;

use std::error::Error;
use std::{io, fs, fmt};

use rustc_serialize::json::{self, Json};
use rustc_serialize::Decodable ;

use geom::{Point, UnitVector};
use geom::shape::{Shape, Intersection, Mesh, Plane};
use geom::ray::{Ray};
use color::Color;
use self::camera::Camera ;
use self::light::Light;
use self::primitive::Primitive;
use self::material::Material;

pub use self::image::{Image, Pixel};
pub use self::filters::{NopFilter, SmoothingFilter};
pub use self::renderer::Renderer;

pub struct Scene {
    camera: Camera,
    ambient_light: Color,
    background_color: Color,
    primitives: Vec<Primitive>,
    lights: Vec<Light>,
}


#[derive(Debug)]
pub struct ParseError(String);

fn error(s: &str) -> Box<Error> {
    Box::new(ParseError(s.to_string()))
}

impl Error for ParseError {
    fn description(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.description().fmt(f)
    }
}


impl Scene {

    pub fn from_json(data: Json) -> Result<Scene, Box<Error>> {
        let conf = try!(data.find("camera").ok_or(error("wrong camera")));
        let camera = try!(read_camera(conf));
        let ambient_light = try!(data.find("ambient_light").ok_or(error("wrong ambient"))
                                 .and_then(read::<Color>));
        let background_color = try!(data.find("background").ok_or(error("wrong background"))
                                    .and_then(read::<Color>));

        let ps = try!(data.find("primitives")
                     .and_then(Json::as_array)
                     .ok_or(error("wrong primitives")));

        let primitives = try!(ps.iter().map(read_primitive)
                              .collect::<Result<Vec<Primitive>, _>>());

        let ls = try!(data.find("lights")
                      .and_then(Json::as_array)
                      .ok_or(error("wrong lights")));

        let lights = try!(ls.iter().map(read::<Light>)
                          .collect::<Result<Vec<_>, _>>());

        Ok(Scene {
            camera: camera,
            ambient_light: ambient_light,
            background_color: background_color,
            primitives: primitives,
            lights: lights,
        })
    }

    fn find_obstacle(&self, ray: &Ray) -> Option<(&Primitive, Intersection)> {
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

    fn is_visible(&self, what: Point, from: Point) -> bool {
        let ray = Ray::from_to(from, what);
        let ray = Ray::from_to(ray.along(1e-6) , what);
        // FIXME: what if obstacle is behind a light source?
        self.find_obstacle(&ray).is_none()
    }
}


fn read<T: Decodable>(data: &Json) -> Result<T, Box<Error>> {
    let mut decoder = json::Decoder::new(data.clone());
    let result = try!(Decodable::decode(&mut decoder));
    Ok(result)
}

fn read_camera(data: &Json) -> Result<Camera, Box<Error>> {
    let config = try!(read(data));
    Ok(Camera::new(config))
}

fn read_primitive(data: &Json) -> Result<Primitive, Box<Error>> {
    let t = try!(data.find("type")
                 .and_then(Json::as_string)
                 .ok_or(error("bad primitive")));
    if t == "mesh" {
        let location = try!(data.find("location").and_then(Json::as_string)
                            .ok_or(error("bad primitive")));
        let material = try!(data.find("material").ok_or(error("bad primitive"))
                            .and_then(read::<Material>));

        let mut file = try!(fs::File::open(&location).map(io::BufReader::new));
        let mesh = try!(Mesh::from_obj(&mut file));

        Ok(Primitive::new(mesh, material))
    } else if t == "plane" {
        let position = try!(data.find("position").ok_or(error("bad primitive"))
                           .and_then(read::<Point>));
        let normal = try!(data.find("normal").ok_or(error("bad primitive"))
                          .and_then(read::<UnitVector>));
        let material = try!(data.find("material").ok_or(error("bad primitive"))
                            .and_then(read::<Material>));

        Ok(Primitive::new(Plane::new(position, normal), material))
    } else {
        Err(error("bad primitive"))
    }
}
