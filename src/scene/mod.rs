extern crate rustc_serialize;

mod primitive;
mod camera;
mod image;
mod light;
mod filters;
mod renderer;

use std::str::FromStr;
use geom::{Point, Vector, UnitVector};
use geom::shape::{Shape, Intersection};
use geom::ray::{Ray};
use color::Color;
use self::rustc_serialize::json::Json;
use self::camera::{Camera, CameraConfig};

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

    pub fn from_json(data: Json) -> Result<Scene, String> {
        let conf = try!(data.find("camera").ok_or("wrong camera"));
        let camera = try!(read_camera(conf));
        let ambient_light = try!(data.find("ambient_light").and_then(read_color)
            .ok_or("wrong ambient"));
        let background_color = try!(data.find("background").and_then(read_color)
            .ok_or("wrong background"));

        Ok(Scene {
            camera: camera,
            ambient_light: ambient_light,
            background_color: background_color,
            primitives: Vec::new(),
            lights: Vec::new(),
        })
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

fn read_camera(data: &Json) -> Result<Camera, String> {
    let position = try!(data.find("position").and_then(read_point)
                        .ok_or("wrong position"));
    let look_at = try!(data.find("look_at").and_then(read_point)
                       .ok_or("wrong look_at"));
    let focus_distance = try!(data.find("focus_distance").and_then(Json::as_f64)
                              .ok_or("wrong focus_distance"));
    let up = try!(data.find("up").and_then(read_direction)
                  .ok_or("wrong up"));
    let size = try!(data.find("size").and_then(read_dimention)
                    .ok_or("wrong size"));
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
