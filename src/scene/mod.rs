extern crate rustc_serialize;

mod camera;
mod filters;
mod image;
mod light;
mod material;
mod primitive;
mod renderer;

use std::str::FromStr;
use std::error::Error;
use std::{io, fs, fmt};

use geom::{Point, Vector, UnitVector};
use geom::shape::{Shape, Intersection, Mesh, Plane};
use geom::ray::{Ray};
use color::Color;
use self::rustc_serialize::json::Json;
use self::camera::{Camera, CameraConfig};
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

pub struct SceneConfig {
    pub camera: CameraConfig,
    pub background: Color,
    pub ambient_light: Color,
}

#[derive(Debug)]
pub struct ParseError(String);

fn error(s: &str) -> ParseError {
    ParseError(s.to_string())
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
    pub fn new(config: SceneConfig) -> Scene {
        Scene {
            camera: Camera::new(config.camera),
            ambient_light: config.ambient_light,
            background_color: config.background,
            primitives: Vec::new(),
            lights: Vec::new(),
        }
    }

    pub fn from_json(data: Json) -> Result<Scene, Box<Error>> {
        let conf = try!(data.find("camera").ok_or(error("wrong camera")));
        let camera = try!(read_camera(conf));
        let ambient_light = try!(data.find("ambient_light").and_then(read_color)
                                 .ok_or(error("wrong ambient")));
        let background_color = try!(data.find("background").and_then(read_color)
                                    .ok_or(error("wrong background")));

        let ps = try!(data.find("primitives")
                     .and_then(Json::as_array)
                     .ok_or(error("wrong primitives")));

        let primitives = try!(ps.iter().map(read_primitive)
                              .collect::<Result<Vec<Primitive>, _>>());

        let ls = try!(data.find("lights")
                      .and_then(Json::as_array)
                      .ok_or(error("wrong lights")));

        let lights = try!(ls.iter().map(read_light)
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

fn read_camera(data: &Json) -> Result<Camera, Box<Error>> {
    let position = try!(data.find("position").and_then(read_point)
                        .ok_or(error("wrong position")));
    let look_at = try!(data.find("look_at").and_then(read_point)
                       .ok_or(error("wrong look_at")));
    let focus_distance = try!(data.find("focus_distance").and_then(Json::as_f64)
                              .ok_or(error("wrong focus_distance")));
    let up = try!(data.find("up").and_then(read_direction)
                  .ok_or(error("wrong up")));
    let size = try!(data.find("size").and_then(read_dimention)
                    .ok_or(error("wrong size")));
    Ok(Camera::new(CameraConfig {
        position: position,
        look_at: look_at,
        focus_distance: focus_distance,
        up: up,
        size: size
    }))
}

fn read_coords(data: &Json) -> Option<[f64; 3]> {
    data.as_array()
        .and_then(|a| a.iter().map(Json::as_f64).collect::<Option<Vec<f64>>>())
        .and_then(|coords|
                  if coords.len() != 3 {
                      None
                  } else {
                      Some([coords[0], coords[1], coords[2]])
                  })
}

fn read_point(data: &Json) -> Option<Point> {
    read_coords(data).map(|c| Point::new(c[0], c[1], c[2]))
}

fn read_direction(data: &Json) -> Option<UnitVector> {
    read_coords(data).map(|c| Vector::new(c[0], c[1], c[2]).direction())
}

fn read_color(data: &Json) -> Option<Color> {
    data.as_string().and_then(|s| Color::from_str(s).ok())
}

fn read_dimention(data: &Json) -> Option<[f64; 2]> {
    data.as_array()
        .and_then(|a| a.iter().map(Json::as_f64).collect::<Option<Vec<f64>>>())
        .and_then(|d| if d.len() != 2 {None} else {Some([d[0], d[1]])})
}

fn read_primitive(data: &Json) -> Result<Primitive, Box<Error>> {
    let t = try!(data.find("type")
                 .and_then(Json::as_string)
                 .ok_or(error("bad primitive")));
    if t == "mesh" {
        let location = try!(data.find("location").and_then(Json::as_string)
                            .ok_or(error("bad primitive")));
        let color = try!(data.find("color").and_then(read_color)
                         .ok_or(error("bad primitive")));

        let mut file = try!(fs::File::open(&location).map(io::BufReader::new));
        let mesh = try!(Mesh::from_obj(&mut file));

        Ok(Primitive::new(mesh, Material {color: color, diffuse: 0.9, specular: 4.0}))
    } else if t == "plane" {
        let position = try!(data.find("position").and_then(read_point)
                           .ok_or(error("bad primitive")));
        let normal = try!(data.find("normal").and_then(read_direction)
                          .ok_or(error("bad primitive")));
        let color = try!(data.find("color").and_then(read_color)
                         .ok_or(error("bad primitive")));

        Ok(Primitive::new(Plane::new(position, normal),
                          Material {color: color, diffuse: 0.9, specular: 4.0}))
    } else {
        Err(Box::new(error("bad primitive")))
    }
}

fn read_light(data: &Json) -> Result<Light, Box<Error>> {
    let position = try!(data.find("position").and_then(read_point).ok_or(error("bad light")));
    let color = try!(data.find("color").and_then(read_color).ok_or(error("bad light")));
    Ok(Light::new(color, position))
}
